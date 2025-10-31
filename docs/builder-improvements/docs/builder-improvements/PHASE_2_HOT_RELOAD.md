# Phase 2: Hot Reload + Live Preview

**Timeline:** 2-3 weeks
**Effort:** Medium
**Impact:** Very High
**Dependencies:** None (can run parallel to Phase 1)
**Status:** Not started

---

## 🎯 Goal

Eliminate manual recompilation and browser refreshes. When a user saves their `.jnc` file, the browser automatically updates in < 500ms.

**Problem solved:** Users currently run `cargo run -- compile` and refresh browser 10+ times per session. Slow and tedious.

**Solution:** File watcher + auto-compile + WebSocket → instant browser refresh.

---

## 📋 What We're Building

### **The Development Loop:**

**Current workflow (slow):**
```
1. Edit main.jnc
2. Save file
3. Switch to terminal
4. Run: cargo run -- compile main.jnc
5. Wait 2-5 seconds
6. Switch to browser
7. Click refresh (Cmd+R)
8. See changes
9. Repeat 10-20 times
```

**New workflow (fast):**
```
1. Edit main.jnc
2. Save file (Cmd+S)
3. See changes instantly in browser
4. Repeat effortlessly
```

**Time savings:** 5 seconds → 0.5 seconds per iteration
**10 iterations:** 50 seconds → 5 seconds (90% faster!)

---

## 🏗️ Architecture

### **System Components:**

```
┌─────────────────────────────────────────────┐
│  User edits main.jnc in VSCode             │
│  Saves file (Cmd+S)                         │
└─────────────────┬───────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────┐
│  File Watcher (Rust)                        │
│  - Detects .jnc, .css file changes         │
│  - Triggers recompilation                   │
└─────────────────┬───────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────┐
│  Jounce Compiler (Fast Mode)                │
│  - Incremental compilation (only changed)   │
│  - Generates client.js, styles.css          │
│  - < 500ms compile time                     │
└─────────────────┬───────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────┐
│  WebSocket Server (Rust)                    │
│  - Notifies browser of changes              │
│  - Sends: { type: "reload", files: [...] } │
└─────────────────┬───────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────┐
│  Browser (Auto-refresh)                     │
│  - Listens for WebSocket messages           │
│  - Reloads changed modules                  │
│  - Preserves app state (optional)           │
└─────────────────────────────────────────────┘
```

---

## 🔧 Technical Implementation

### **1. File Watcher (notify crate)**

**Cargo.toml:**
```toml
[dependencies]
notify = "6.1"
tokio = { version = "1.0", features = ["full"] }
```

**Implementation:**
```rust
// src/watch.rs
use notify::{Watcher, RecursiveMode, Event};
use std::path::Path;

pub struct JounceWatcher {
    watcher: notify::RecommendedWatcher,
    tx: tokio::sync::mpsc::Sender<String>,
}

impl JounceWatcher {
    pub fn new(watch_path: &Path, tx: tokio::sync::mpsc::Sender<String>) -> Result<Self> {
        let mut watcher = notify::recommended_watcher(move |res: Result<Event>| {
            match res {
                Ok(event) => {
                    for path in event.paths {
                        if path.extension() == Some("jnc") ||
                           path.extension() == Some("css") {
                            tx.blocking_send(path.to_string_lossy().to_string()).ok();
                        }
                    }
                }
                Err(e) => eprintln!("Watch error: {}", e),
            }
        })?;

        watcher.watch(watch_path, RecursiveMode::Recursive)?;
        Ok(Self { watcher, tx })
    }
}

// Usage
pub async fn watch_and_compile(project_path: &Path) {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    let _watcher = JounceWatcher::new(project_path, tx)?;

    println!("🔥 Watching for changes...");

    while let Some(file_path) = rx.recv().await {
        println!("📝 Changed: {}", file_path);

        // Debounce (wait 100ms for multiple changes)
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Recompile
        match compile_project(&file_path) {
            Ok(_) => {
                println!("✅ Compiled successfully");
                // Notify browser via WebSocket
                notify_browser_reload().await;
            }
            Err(e) => {
                eprintln!("❌ Compilation error: {}", e);
                // Send error to browser
                notify_browser_error(&e).await;
            }
        }
    }
}
```

---

### **2. WebSocket Server (tokio-tungstenite)**

**Cargo.toml:**
```toml
[dependencies]
tokio-tungstenite = "0.20"
futures-util = "0.3"
```

**Implementation:**
```rust
// src/dev_server.rs
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tokio::net::{TcpListener, TcpStream};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct DevServer {
    clients: Arc<RwLock<Vec<tokio_tungstenite::WebSocketStream<TcpStream>>>>,
}

impl DevServer {
    pub async fn start(port: u16) -> Result<Self> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        let clients = Arc::new(RwLock::new(Vec::new()));

        let clients_clone = clients.clone();
        tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let ws_stream = accept_async(stream).await.unwrap();
                clients_clone.write().await.push(ws_stream);
            }
        });

        println!("🌐 WebSocket server running on ws://localhost:{}", port);
        Ok(Self { clients })
    }

    pub async fn notify_reload(&self) {
        let message = Message::Text(r#"{"type":"reload"}"#.to_string());
        let mut clients = self.clients.write().await;

        // Send to all connected clients
        for client in clients.iter_mut() {
            client.send(message.clone()).await.ok();
        }
    }

    pub async fn notify_error(&self, error: &str) {
        let message = Message::Text(format!(
            r#"{{"type":"error","message":"{}"}}"#,
            error.replace('"', "\\\"")
        ));

        let mut clients = self.clients.write().await;
        for client in clients.iter_mut() {
            client.send(message.clone()).await.ok();
        }
    }
}
```

---

### **3. Browser Client (JavaScript)**

**File: dist/hot-reload.js**
```javascript
// Hot reload client
(function() {
    let ws;
    let reconnectAttempts = 0;
    const maxReconnectAttempts = 10;

    function connect() {
        ws = new WebSocket('ws://localhost:3001');

        ws.onopen = () => {
            console.log('🔥 Hot reload connected');
            reconnectAttempts = 0;
        };

        ws.onmessage = (event) => {
            const data = JSON.parse(event.data);

            if (data.type === 'reload') {
                console.log('🔄 Reloading...');

                // Option 1: Full page reload (simple)
                window.location.reload();

                // Option 2: Module reload (advanced, preserves state)
                // reloadModules(data.files);
            } else if (data.type === 'error') {
                console.error('❌ Compilation error:', data.message);
                showErrorOverlay(data.message);
            }
        };

        ws.onclose = () => {
            console.log('🔌 Hot reload disconnected');

            if (reconnectAttempts < maxReconnectAttempts) {
                setTimeout(() => {
                    reconnectAttempts++;
                    connect();
                }, 1000 * reconnectAttempts);
            }
        };

        ws.onerror = (error) => {
            console.error('WebSocket error:', error);
        };
    }

    // Show compilation errors as overlay
    function showErrorOverlay(message) {
        let overlay = document.getElementById('jounce-error-overlay');

        if (!overlay) {
            overlay = document.createElement('div');
            overlay.id = 'jounce-error-overlay';
            overlay.style.cssText = `
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: rgba(0, 0, 0, 0.9);
                color: #ff6b6b;
                padding: 40px;
                font-family: monospace;
                font-size: 14px;
                z-index: 999999;
                overflow: auto;
            `;
            document.body.appendChild(overlay);
        }

        overlay.innerHTML = `
            <div style="max-width: 800px; margin: 0 auto;">
                <h2 style="color: #ff6b6b; margin-top: 0;">
                    ❌ Compilation Error
                </h2>
                <pre style="white-space: pre-wrap; line-height: 1.5;">
${message}
                </pre>
                <button onclick="document.getElementById('jounce-error-overlay').remove()"
                        style="margin-top: 20px; padding: 10px 20px; cursor: pointer;">
                    Dismiss
                </button>
            </div>
        `;
    }

    // Start connection
    connect();
})();
```

**Include in index.html:**
```html
<head>
    <!-- ... -->
    <script src="/hot-reload.js"></script>
</head>
```

---

### **4. CLI Command**

**New command:**
```bash
jounce dev
# or
cargo run -- dev

# With options
jounce dev --port 3000 --watch src/
```

**Implementation:**
```rust
// src/cli.rs
pub enum Command {
    Compile { file: String },
    Dev { port: Option<u16>, watch_path: Option<String> },
}

pub async fn run_dev_server(port: u16, watch_path: &str) {
    println!("🔥 Starting Jounce dev server...");

    // 1. Start WebSocket server
    let dev_server = DevServer::start(3001).await.unwrap();

    // 2. Start HTTP server (for serving files)
    let http_server = HttpServer::start(port).await;

    // 3. Start file watcher
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    let _watcher = JounceWatcher::new(Path::new(watch_path), tx).unwrap();

    println!("✅ Dev server ready!");
    println!("   📡 HTTP: http://localhost:{}", port);
    println!("   🔌 WebSocket: ws://localhost:3001");
    println!("   👀 Watching: {}", watch_path);

    // 4. Handle file changes
    while let Some(file_path) = rx.recv().await {
        match compile_project(&file_path) {
            Ok(_) => dev_server.notify_reload().await,
            Err(e) => dev_server.notify_error(&e.to_string()).await,
        }
    }
}
```

---

## ⚡ Performance Optimizations

### **1. Incremental Compilation**

**Problem:** Recompiling entire app is slow (2-5 seconds)

**Solution:** Only recompile changed files

```rust
pub struct CompilationCache {
    // Hash of file contents → compiled output
    cache: HashMap<String, CompiledModule>,
}

impl CompilationCache {
    pub fn compile_if_changed(&mut self, file_path: &str) -> Result<CompiledModule> {
        let content = fs::read_to_string(file_path)?;
        let hash = calculate_hash(&content);

        if let Some(cached) = self.cache.get(&hash) {
            // File unchanged, return cached result
            return Ok(cached.clone());
        }

        // File changed, recompile
        let compiled = compile_file(file_path)?;
        self.cache.insert(hash, compiled.clone());
        Ok(compiled)
    }
}
```

**Result:** 2-5 seconds → 200-500ms

---

### **2. Debouncing**

**Problem:** User saves multiple times in quick succession

**Solution:** Wait 100ms after last change before compiling

```rust
let mut last_change = Instant::now();

while let Some(file_path) = rx.recv().await {
    last_change = Instant::now();

    // Wait for 100ms of inactivity
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Check if another change came in
    if last_change.elapsed() < Duration::from_millis(100) {
        continue; // Skip, more changes coming
    }

    // Compile now
    compile_project(&file_path)?;
}
```

---

### **3. Parallel Compilation**

**Problem:** Multiple files changed at once

**Solution:** Compile in parallel

```rust
use rayon::prelude::*;

pub fn compile_multiple_files(files: Vec<String>) -> Result<()> {
    files.par_iter()
        .try_for_each(|file| compile_file(file))?;
    Ok(())
}
```

---

## 🎨 User Experience Enhancements

### **1. Compilation Progress**

Show progress in terminal and browser:

**Terminal:**
```
🔥 Jounce dev server running
   http://localhost:3000

📝 Changed: src/main.jnc
⏳ Compiling...
✅ Compiled in 234ms
🔄 Browser refreshed
```

**Browser (top-right corner):**
```
┌─────────────────┐
│ ⏳ Compiling... │
└─────────────────┘

↓ (500ms later)

┌──────────────────┐
│ ✅ Updated!      │
└──────────────────┘
```

---

### **2. Error Overlay**

When compilation fails, show error in browser:

```
┌─────────────────────────────────────┐
│ ❌ Compilation Error                │
│                                     │
│ src/main.jnc:15:20                  │
│                                     │
│ 13 │   let count = signal(0);       │
│ 14 │                                │
│ 15 │   return <button onclick={count++}>
│                                  ^  │
│                                     │
│ Unexpected token '+'                │
│ Expected ')' or ','                 │
│                                     │
│ [Dismiss]                           │
└─────────────────────────────────────┘
```

---

### **3. Success Notification**

Brief toast notification:

```
┌──────────────┐
│ ✅ Updated!  │
└──────────────┘
(fades out after 1 second)
```

---

## 🧪 Advanced Features (Optional)

### **1. Hot Module Replacement (HMR)**

**What:** Reload only changed components without full page refresh

**Benefits:** Preserves app state (form inputs, scroll position, etc.)

**Implementation:**
```javascript
// hot-reload.js (advanced)
function reloadModule(modulePath) {
    // Remove old module
    const oldScript = document.querySelector(`script[src="${modulePath}"]`);
    if (oldScript) oldScript.remove();

    // Load new module
    const newScript = document.createElement('script');
    newScript.src = modulePath + '?t=' + Date.now(); // Bust cache
    newScript.type = 'module';
    document.head.appendChild(newScript);

    // Re-mount component
    newScript.onload = () => {
        // Preserve state
        const state = getCurrentAppState();

        // Remount
        mountComponent('App', document.getElementById('app'));

        // Restore state
        restoreAppState(state);
    };
}
```

**Complexity:** High (requires state serialization)
**ROI:** Medium (nice-to-have, not critical)

---

### **2. CSS Hot Reload**

**What:** Reload CSS without refreshing page

**Implementation:**
```javascript
function reloadCSS() {
    const links = document.querySelectorAll('link[rel="stylesheet"]');
    links.forEach(link => {
        const href = link.href.split('?')[0];
        link.href = href + '?t=' + Date.now();
    });
}
```

**Complexity:** Low
**ROI:** High (instant style updates!)

---

### **3. Error Recovery**

**What:** Try to recover from errors automatically

**Example:**
```
User: Saves file with syntax error
System: Shows error overlay
User: Fixes error, saves again
System: Auto-removes error overlay, reloads
```

---

## 📦 Deliverables

### **Week 1: Basic File Watching**
- [ ] Implement file watcher (notify crate)
- [ ] Trigger recompilation on .jnc changes
- [ ] Terminal output showing what changed
- [ ] Manual browser refresh still required

### **Week 2: WebSocket + Auto-Reload**
- [ ] WebSocket server implementation
- [ ] Browser client (hot-reload.js)
- [ ] Auto browser refresh on successful compile
- [ ] Error overlay for compilation failures

### **Week 3: Polish + Optimization**
- [ ] Incremental compilation (cache)
- [ ] Debouncing (avoid redundant compiles)
- [ ] Success notifications
- [ ] CSS hot reload
- [ ] Better error messages

---

## 🚀 Testing Plan

### **Manual Tests:**

1. **Basic reload:**
   - Start dev server
   - Edit main.jnc, save
   - Browser should auto-refresh

2. **Error handling:**
   - Introduce syntax error
   - Save file
   - Error overlay should appear
   - Fix error, save
   - Overlay should disappear

3. **Multiple files:**
   - Edit main.jnc and styles.css
   - Save both
   - Should recompile once (debounced)

4. **Reconnection:**
   - Start dev server
   - Open browser
   - Restart dev server
   - Browser should reconnect

### **Performance Tests:**

1. **Compile speed:**
   - Small file (< 100 lines): < 200ms
   - Medium file (100-500 lines): < 500ms
   - Large file (500+ lines): < 1s

2. **Reload speed:**
   - Time from save to browser update: < 1s total

---

## 📊 Success Metrics

**Week 1 goals:**
- File watcher detects changes reliably
- Recompilation triggers automatically
- Zero manual `cargo run` commands

**Week 2 goals:**
- Browser refreshes automatically
- < 1 second from save to updated browser
- Error overlay works correctly

**Week 3 goals:**
- < 500ms compile time (incremental)
- CSS hot reload (no page refresh)
- Zero crashes or disconnects

---

## 🔗 Next Steps

After Phase 2 completes:
1. Users can iterate 10x faster
2. Foundation for Visual Builder (Phase 3)
3. Real-time preview for drag-and-drop

---

## 📝 Open Questions

1. **HMR vs Full Reload?**
   - Start with full reload (simpler)
   - Add HMR later if needed

2. **Port configuration?**
   - Default: HTTP on 3000, WebSocket on 3001
   - Allow custom via CLI flags

3. **Watch all files or just .jnc?**
   - Watch: .jnc, .css
   - Ignore: .md, .git, node_modules

4. **Error format?**
   - Show in terminal + browser?
   - Use Rust error format (nice colors)?

**Decision needed before Week 1!**

---

## 🎯 Ready to Start?

**First task:** Implement basic file watcher (1-2 days)
**Then:** Add WebSocket server (2-3 days)
**Finally:** Polish UX (1 week)

See [IMPLEMENTATION_NOTES.md](./IMPLEMENTATION_NOTES.md) for coding tips!
