# Live Reload Development Workflow

Jounce supports rapid development with auto-recompilation and browser live reload!

## Quick Start

**Terminal 1 - Auto-Recompile:**
```bash
./watch.sh examples/single-file-counter/main.jnc
```

**Terminal 2 - Live Server:**
```bash
# Option A: live-server (auto-reloads browser)
npm install -g live-server
live-server dist --port=8080

# Option B: Python (manual refresh required)
cd dist && python3 -m http.server 8080
```

**Browser:** Open http://localhost:8080

Now edit your `.jnc` file and see changes instantly in the browser!

---

## How It Works

1. **`watch.sh`** monitors your `.jnc` file for changes
2. When you save, it automatically runs `cargo run -- compile`
3. **`live-server`** detects the changed `dist/` files and reloads your browser

**Result:** Edit → Save → See changes in ~1 second!

---

## Installation

### macOS
```bash
brew install fswatch
npm install -g live-server
```

### Linux
```bash
sudo apt-get install inotify-tools
npm install -g live-server
```

### Without File Watchers
The script falls back to polling every 2 seconds if no file watcher is installed.

---

## Full HMR (Hot Module Replacement)

True HMR (state-preserving hot reload) is not yet implemented. This would require:

- WebSocket connection between server and browser
- Module replacement logic in the runtime
- State serialization/deserialization
- Dependency graph tracking

**Current solution** (auto-recompile + live-server) provides 80% of the value with much less complexity!

---

## Tips

- Use `live-server` for automatic browser refresh
- Use browser DevTools to keep console open across reloads
- Add `debugger;` statements for quick breakpoints
- Test reactivity by watching signals update in real-time

---

## Example Session

```bash
# Terminal 1
$ ./watch.sh examples/single-file-counter/main.jnc
👀 Watching: examples/single-file-counter/main.jnc
📦 Compiling on save...
✨ Compilation complete! (10.73ms)

# Terminal 2
$ live-server dist --port=8080
Serving "dist" at http://127.0.0.1:8080
Ready for changes

# Now edit main.jnc and save...
# Browser auto-refreshes!
```

Enjoy rapid Jounce development! 🚀
