# CLAUDE.md - Jounce Development Guide

**Version**: v0.8.6 "Session 6 - Compiler Fixes 75% Complete"
**Current Status**: Fixing single-file workflow (Phase 1-3 ✅, Phase 4 next)
**Last Updated**: October 26, 2025 (Session 6)

---

## ⚠️ CRITICAL PRINCIPLE: ONE .jnc FILE → FULL APP

**THE GOLDEN RULE:**
```
main.jnc (ONE FILE) → cargo compile → Working App
```

**NEVER:**
- ❌ Create separate .js files for app logic
- ❌ Require manual copying of files after compilation
- ❌ Split functionality across multiple files "for convenience"
- ❌ Use build scripts as a workaround for incomplete compilation

**ALWAYS:**
- ✅ Put ALL code in the .jnc file (HTML, CSS, JavaScript logic)
- ✅ Compile should produce a COMPLETE working app
- ✅ Users should only run: `cargo run -- compile app.jnc` then open browser
- ✅ If current Jounce syntax doesn't support something, FIX THE COMPILER

**Why this matters:** Jounce's entire value proposition is ONE SOURCE FILE. Taking shortcuts defeats the purpose and creates confusion!

---

## 📍 Current Status (v0.8.6)

**✅ MILESTONE ACHIEVED: 35/35 Packages Complete!**

- Core compiler: ✅ Complete (lexer, parser, codegen, type checker)
- Multi-file imports: ✅ Complete (`./` and `../`)
- Reactivity system: ✅ Complete (signal, computed, effect, batch)
- Standard library: ✅ Complete (JSON, DateTime, Crypto, File I/O, YAML)
- **Package ecosystem: ✅ 35 packages complete!**
- Tests: **625+ passing** (core + packages)
- Build speed: **102x faster** with cache

**35-Package Ecosystem:**
- **Foundation (5):** router, http, forms, store, i18n
- **Backend (10):** auth, db, cache, websocket, rpc, queue, rate-limit, config, validation, metrics
- **Content (6):** markdown, email, image, pdf, xlsx, sanitizer
- **Dev Tools (6):** logger, testing, cli, deploy, docs, utils
- **Features (8):** ui, theme, animate, search, notification, storage, workflow, scheduler, templates
- **Integration (extras):** localization, analytics, payment, graphql

---

## 🔴 SESSION 5 REALITY CHECK (October 26, 2025)

### **CRITICAL TRUTH: The Compiler Was NOT Built for Single-File Reactive Apps**

**What Was Wrong:**
- Created Phase 15 example apps with **FAKE single-file workflow**
- Required 690 lines of manual JavaScript (`client-app.js`) copied after compilation
- Created build scripts (`build.sh`) to hide the broken workflow
- Made it LOOK like "one .jnc file → working app" but it was a **LIE**
- All example apps (Counter, Todo, Blog) required manual post-compilation steps

### **What the Compiler ACTUALLY Did:**

| Feature | Status | Reality |
|---------|--------|---------|
| JSX → JavaScript | ✅ Works | Generates `h()` function calls correctly |
| CSS Extraction | ✅ Works | `style {}` blocks compile to `styles.css` |
| Function Compilation | ✅ Works | Jounce functions become JavaScript |
| Struct Literals | ✅ Works | `Post { id: 1 }` (Rust-style only) |
| **Object Literals** | ❌ **BROKEN** | `{ id: 1, name: "test" }` → Parser error |
| **Reactive Wiring** | ❌ **MISSING** | Signals don't auto-connect to DOM |
| **Component Mounting** | ❌ **INCOMPLETE** | No automatic initialization |
| **Inline Event Handlers** | ❌ **BROKEN** | `onClick={() => ...}` doesn't work |
| **Script Blocks** | ❌ **MISSING** | No way to embed raw JavaScript |
| **Single-File Apps** | ❌ **LIE** | ALL apps need manual post-compilation |

### **Example of the Broken Workflow:**

**What Was Claimed:**
```bash
cargo run -- compile main.jnc  # ONE COMMAND
cd dist && python3 -m http.server 8080  # DONE!
```

**What Actually Happened:**
```bash
cargo run -- compile main.jnc           # Compile
cp client-app.js dist/                  # ❌ Manual copy
# Edit dist/index.html manually         # ❌ Manual step
# Add 690 lines of reactive JavaScript  # ❌ Wire everything up
cd dist && python3 -m http.server 8080  # Finally works
```

---

## 🟢 SESSION 6 PROGRESS (October 26, 2025)

### **FIXING THE COMPILER - 3/4 Phases Complete (75%)**

**Token Usage**: 90k/200k (45%)

### ✅ **Phase 1 COMPLETE: Object Literal Support**

**What Works:**
```jounce
let post = { id: 1, title: "Hello", tags: ["rust", "jounce"] };
```

**Files Changed:**
- `src/ast.rs` - Added `ObjectLiteral` variant
- `src/parser.rs` - Added `parse_object_literal()` function
- `src/js_emitter.rs` - Generates `{ key: value }` JavaScript
- All compiler passes updated

**Commit**: 5cde04d

---

### ✅ **Phase 2 COMPLETE: Script Block Support**

**What Works:**
```jounce
<script>
  console.log("App initialized!");
  function initApp() { ... }
</script>
```

**Files Changed:**
- `src/code_splitter.rs:41` - Initialize script_blocks vector
- `src/code_splitter.rs:71-74` - Collect ScriptBlock statements
- `src/js_emitter.rs:650-657` - Emit raw JavaScript to client.js

**Commit**: 47de187

---

### ✅ **Phase 3 COMPLETE: Event Handlers with Arrow Functions**

**What Works:**
```jounce
// Multi-param arrow functions
let add = (a, b) => a + b;

// No-param arrow functions
let greet = () => "Hello";

// In JSX attributes
<button onClick={() => 42}>Click</button>
<input onChange={(e) => e.target.value} />
```

**Problem Solved:** Arrow functions like `(a, b) => ...` were parsed as tuples, not lambdas

**Files Changed:**
- `src/parser.rs:1556-1582` - Enhanced `parse_lambda_or_grouped()`
  * Check for `=>` after tuple parsing
  * Convert tuple elements to lambda parameters if `=>` found

**Commit**: 9f45a5b

---

## 🎯 PHASE 4: TRUE SINGLE-FILE APP (NEXT)

### **Goal**: Build a complete interactive app in ONE .jnc file

**What We Have Now:**
- ✅ Object literals for data structures: `{ id: 1, name: "test" }`
- ✅ Script blocks for initialization: `<script>...</script>`
- ✅ Arrow functions for event handlers: `onClick={() => ...}`

**Test Plan:**
1. Create a simple counter app in ONE .jnc file
2. Use `<script>` blocks for reactive setup
3. Use event handlers for interactivity
4. Compile: `cargo run -- compile app.jnc`
5. Verify: Works in browser with ZERO manual steps

### **Example App to Build:**

```jounce
// counter_app.jnc - Complete app in one file

<script>
  // Initialize reactive state
  const count = signal(0);

  // Auto-update DOM when count changes
  effect(() => {
    const el = document.getElementById('count');
    if (el) el.textContent = count.value;
  });

  // Mount app to DOM
  window.addEventListener('DOMContentLoaded', () => {
    const root = document.getElementById('app');
    if (root) mountComponent(Counter, root);
  });
</script>

component Counter() {
    return <div>
        <h1>Counter: <span id="count">0</span></h1>
        <button onClick={() => count.value++}>+</button>
        <button onClick={() => count.value--}>-</button>
        <button onClick={() => count.value = 0}>Reset</button>
    </div>;
}

fn main() {
    // Entry point (called from index.html)
}
```

### **Success Criteria:**

- [ ] Compiles without errors
- [ ] Generated `dist/client.js` contains script block code
- [ ] Generated `dist/index.html` loads app correctly
- [ ] Opens in browser at http://localhost:8080
- [ ] Buttons work (increment, decrement, reset)
- [ ] Count updates reactively
- [ ] **ZERO manual steps** after compilation

### **Testing Workflow:**

```bash
# 1. Compile
cargo run -- compile counter_app.jnc

# 2. Serve (no manual file copying!)
cd dist && python3 -m http.server 8080

# 3. Open browser
# Open http://localhost:8080/index.html

# 4. Test interactivity
# Click +, -, Reset buttons
# Verify count updates
```

---

## 📝 Documentation Strategy

**Primary Documents:**
- **FEATURES.md** - Single source of truth for what's implemented
- **EXAMPLE_APPS.md** - User-facing tutorials and app showcase
- **CLAUDE.md** (this file) - Current status and next steps
- **ROADMAP.md** - High-level phases and timeline
- **CLAUDE_ARCHIVE.md** - Full historical context

**Rule**: Check FEATURES.md BEFORE building anything to avoid duplicates!

---

## 📊 Test Status

**✅ 625/625 tests passing (100%)**
- Core compiler: 530+ tests
- Standard library: 74 tests
- Reactivity: 51 tests
- 35 packages: ~240+ tests
- 10 ignored (intentional)

---

## 🚀 Quick Commands

```bash
# Build & test
cargo build --release && cargo test

# Compile project
cargo run -- compile main.jnc

# Run tests
cargo test --lib

# Serve app
cd dist && python3 -m http.server 8080
```

---

## 📚 Key Files

### Compiler
- `src/main.rs` - CLI (1340 lines)
- `src/lexer.rs`, `src/parser.rs`, `src/js_emitter.rs` - Compiler
- `src/code_splitter.rs` - Code splitting logic
- `src/module_loader.rs` - Import resolution
- `src/cache/mod.rs` - Build cache
- `packages/` - 35 complete packages

### Documentation
- `FEATURES.md` - What's implemented (800+ lines)
- `EXAMPLE_APPS.md` - User tutorials (500+ lines)
- `BUILDING_APPS.md` - Development patterns (693 lines)
- `CLAUDE_ARCHIVE.md` - Full history

### Runtime
- `runtime/reactivity.js` - Signal/effect/computed
- `runtime/client-runtime.js` - h() and mountComponent()
- `dist/` - Generated output

---

## ⏭️ Next Session (Session 7)

**IMMEDIATE TASK: Phase 4 - Build Single-File Test App**

1. Create `examples/single-file-counter/main.jnc`
2. Implement counter with `<script>` + event handlers
3. Compile: `cargo run -- compile main.jnc`
4. Test in browser
5. Verify ZERO manual steps needed
6. Document workflow

**If successful:**
- ✅ Single-file workflow WORKS!
- ✅ Can delete all build scripts from example apps
- ✅ Can rebuild blog platform as true single file
- ✅ Ready for production use!

**If issues found:**
- Debug and fix compiler
- Update FEATURES.md with limitations
- Plan additional fixes

**Token Budget**: Have 110k tokens remaining (55%) - plenty of room!

---

**For full session history, see `CLAUDE_ARCHIVE.md`**
