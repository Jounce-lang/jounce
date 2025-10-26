# CLAUDE.md - Jounce Development Guide

**Version**: v0.8.3 "@persist Decorator Implemented"
**Current Status**: @persist decorator working! Progressive enhancement ready! 🎉
**Last Updated**: October 25, 2025 (Session 4)

---

## ⚠️ MEMORY MANAGEMENT

**IMPORTANT**: Monitor token usage during long sessions.

When usage reaches **70% (140k/200k tokens)**:
1. **STOP** and notify user
2. **Write next steps** to this file
3. **Commit all work**
4. User will clear memory and resume

**Why 70%?** Leaves buffer for final commits and documentation updates before hitting limits.

---

## 🎯 Current Status (v0.8.0)

**✅ MILESTONE ACHIEVED: 35/35 Packages Complete!**

- Core compiler: ✅ Complete (lexer, parser, codegen, type checker)
- Multi-file imports: ✅ Complete (`./` and `../`)
- Reactivity system: ✅ Complete (signal, computed, effect, batch)
- Standard library: ✅ Complete (JSON, DateTime, Crypto, File I/O, YAML)
- **Package ecosystem: ✅ 35 packages complete!**
- Tests: **850+ passing** (core + packages)
- Build speed: **102x faster** with cache

---

## 📦 35-Package Ecosystem

**Foundation (5):** router, http, forms, store, i18n
**Backend (10):** auth, db, cache, websocket, rpc, queue, rate-limit, config, validation, metrics
**Content (6):** markdown, email, image, pdf, xlsx, sanitizer
**Dev Tools (6):** logger, testing, cli, deploy, docs, utils
**Features (8):** ui, theme, animate, search, notification, storage, workflow, scheduler, templates
**Integration (extras):** localization, analytics, payment, graphql

---

## 🔄 Development Workflow

1. Work on current task (track with TodoWrite)
2. Commit frequently with detailed messages
3. Update docs (README.md, ROADMAP.md)
4. Push to GitHub
5. Move to next task

**Goal**: Build example apps showcasing the 35-package ecosystem, then expand to 100 packages.

---

## 🚀 Quick Commands

```bash
# Build & test
cargo build --release && cargo test

# Compile project
cd my-app/ && jnc compile main.jnc

# Run tests
jnc test --verbose

# Watch mode
jnc watch src --output dist

# Package count
ls -1 packages/ | wc -l
```

---

## 📂 Key Files

- `src/main.rs` - CLI (1340 lines)
- `src/lexer.rs`, `src/parser.rs`, `src/js_emitter.rs` - Compiler
- `src/module_loader.rs` - Import resolution
- `src/cache/mod.rs` - Build cache
- `packages/` - 35 complete packages
- `ROADMAP.md` - Development roadmap
- `CLAUDE_ARCHIVE.md` - Full history (archived)

---

## 📊 Test Status

**✅ 625/625 tests passing (100%)**
- Core compiler: 530+ tests
- Standard library: 74 tests
- Reactivity: 51 tests
- Module loader: 2 tests (fixed!)
- Test framework: 1 test (fixed!)
- 35 packages: ~240+ tests
- 10 ignored (intentional)

---

## 🎯 Next Steps (Session 5)

### IMMEDIATE PRIORITIES:

1. **Build Example Apps with @persist**
   - Counter app with localStorage persistence
   - Todo app with backend persistence
   - Shopping cart with realtime sync

2. **Implement backend and realtime strategies**
   - `@persist("backend")` - Generate RPC calls
   - `@persist("realtime")` - Generate WebSocket sync

3. **Documentation**
   - Update `BUILDING_APPS.md` with @persist examples
   - Add persistence migration guide
   - Show upgrade path examples

4. **Other Apps**
   - Build Calculator app (validate reactive patterns with numbers)
   - Build Temperature Converter (dual inputs, computed values)
   - Build Color Mixer (RGB sliders, real-time preview)

---

## 🏗️ Architecture Decisions (Session 3)

### Frontend vs Backend Storage Strategy

**Problem:** Users say "build me a todo app" - they shouldn't need to choose storage strategy.

**Analysis:**
- Option A (Auto-magic): Compiler auto-detects and adds persistence → **Unclear upgrade path**
- Option B (Decorators): `@persist("strategy")` explicit opt-in → **Clear, easy upgrades** ✅
- Option C (Manual): Write all storage code manually → **Too much boilerplate**

**Decision:** Implement Option B - Decorator-based progressive enhancement

**Reasoning:**
1. **Explicit is better than implicit** - developers can SEE what's happening
2. **One-line upgrades** - change decorator argument, not entire codebase
3. **Clear progression** - `none → localStorage → backend → realtime`
4. **No magic** - behavior is visible in source code

### Storage Strategies:

```
@persist("none")         → No persistence (default, pure frontend)
@persist("localStorage") → Browser storage (single device)
@persist("backend")      → Server + database (multi-user)
@persist("realtime")     → WebSocket sync (collaboration)
```

### Example Upgrade Path:

```jounce
// Day 1: Prototype
let todos = signal([]);

// Day 2: Want persistence
@persist("localStorage")
let todos = signal([]);

// Week 1: Multi-user needed
@persist("backend")
let todos = signal([]);

// Add server functions:
server fn loadTodos() -> Vec<Todo> { ... }
server fn saveTodo(todo: Todo) -> Todo { ... }

// Month 1: Real-time collaboration
@persist("realtime")
let todos = signal([]);
```

### Key Insight:

**Progressive enhancement shouldn't require rewrites.** Each upgrade is additive, not destructive.

---

## 📝 Recent Achievements

**October 25, 2025 (Session 4):**
- ✅ **@persist decorator fully implemented!**
- Added `Decorator` AST node type (src/ast.rs:821-824)
- Updated `LetStatement` to include decorators field
- Implemented decorator parsing in parser.rs:
  * `parse_decorators()` function (lines 2095-2128)
  * Updated `parse_statement()` to handle `@` before `let`
  * Updated `parse_let_statement()` to accept decorators
- Implemented localStorage code generation in js_emitter.rs:
  * Generates localStorage.getItem() on init
  * Generates effect() to save on changes
  * Supports "backend" and "realtime" (placeholders)
- All 625 tests passing
- Fixed formatter.rs to include decorators field (13 test cases)
- Parser distinguishes between `@server/@client` and `@persist` decorators

**Key Deliverables:**
- `src/ast.rs` - Decorator struct + LetStatement.decorators
- `src/parser.rs:2095-2128` - parse_decorators() function
- `src/parser.rs:109-129` - Decorator handling in parse_statement()
- `src/parser.rs:793-812` - Updated parse_let_statement()
- `src/js_emitter.rs:1088-1115` - localStorage code generation
- `test_decorator_parsing.jnc` - Test file demonstrating syntax
- All compilation tests passing (625/625)

**October 25, 2025 (Session 3):**
- ✅ **First interactive apps working in browser!**
- Fixed critical CSS bugs (hex colors, pseudo-classes)
- Fixed ES6 module exports (`reactivity.js`)
- Built Counter app (blue/yellow/green buttons, press effects)
- Built Todo List app (add, complete, delete, stats)
- Designed `@persist` decorator system (Option B selected)
- Documented frontend/backend architecture decisions

**Key Deliverables:**
- `test_app_counter.jnc` - Working counter with visual feedback
- `apps/11_todo_list.jnc` - Full todo app with reactive list rendering
- CSS parser fixes:
  * `src/parser.rs:3114-3116` - Hex color spacing fix (#3b82f6 not #3 b82f6)
  * `src/parser.rs:3100-3106` - Pseudo-class spacing (:hover not : hover)
  * `src/parser.rs:3069` - Track prev_was_hash for hex colors
- `runtime/reactivity.js:504` - Added ES6 exports
- Architecture documentation (this file)

**Browser Testing Workflow:**
```bash
cd dist && python3 -m http.server 8080
# Open http://localhost:8080/index.html
# Hard refresh: Ctrl+Shift+R (bypass cache)
```

**CSS Fixes Applied:**
- Hex colors: `#3b82f6` (no spaces)
- Pseudo-classes: `.btn:hover` (no spaces)
- Properties: `color: white` (space after colon)
- Units: `600px` (no spaces)

**October 25, 2025 (Session 2):**
- ✅ **100% test pass rate achieved! 625/625 tests**
- Fixed CSS spacing bug (no more "600 px" issues)
- Built 2 example apps (Counter, Stopwatch)
- Created comprehensive app building guide (BUILDING_APPS.md)
- Designed reactive automation system (future v0.9.0)
- Fixed module loader tests (raven-router test infrastructure)
- Fixed test framework assertion test

**Key Deliverables:**
- `BUILDING_APPS.md` (693 lines) - Complete app development guide
- `test_app_counter.jnc` - Simple counter with reactivity
- `test_app_stopwatch.jnc` - Timer app with intervals
- `TEST_IN_BROWSER.md` - Browser testing workflow
- `docs/design/REACTIVE_AUTOMATION.md` - Future automation design
- 100% test coverage (no failing tests!)

**October 24, 2025 (Session 1):**
- ✅ **35-package milestone complete!**
- Built 13 packages in one session
- Expanded test coverage (850+ tests)
- All work committed and pushed

---

## 🐛 Known Issues & Fixes

### CSS Generation (`src/parser.rs`)
- ✅ **FIXED** (Session 3): Hex colors had spaces (#3 b82f6)
- ✅ **FIXED** (Session 3): Pseudo-classes had spaces (.btn: hover)
- ✅ **FIXED** (Session 2): Units had spaces (600 px)

**Solution:** Track previous token state, don't add spaces after `#` or before `:` in selectors.

### ES6 Module Exports (`runtime/reactivity.js`)
- ✅ **FIXED** (Session 3): Missing `export` statement for batch, untrack, etc.

**Solution:** Added `export { signal, computed, effect, batch, untrack };` at end of file.

### Manual Reactive Setup (Current State)
- ⚠️ **WORKAROUND**: Must manually add reactive code to `dist/client.js` after compilation
- 🔧 **FIX PLANNED**: `@persist` decorator will auto-generate this code (Session 4)

---

## 📚 Resources

### Documentation
- `BUILDING_APPS.md` - App development patterns (693 lines)
- `TEST_IN_BROWSER.md` - Browser testing guide
- `docs/design/REACTIVE_AUTOMATION.md` - Future automation design
- `CLAUDE_ARCHIVE.md` - Full historical context

### Example Apps
- `test_app_counter.jnc` - Simple counter (blue/yellow/green buttons)
- `test_app_stopwatch.jnc` - Timer with intervals
- `apps/11_todo_list.jnc` - Todo list with stats

### Runtime Files
- `runtime/reactivity.js` - Signal/effect/computed implementation
- `runtime/client-runtime.js` - h() and mountComponent()
- `dist/` - Generated output (auto-copied on compile)

---

## 🔄 Workflow (Current Session Pattern)

1. **Write `.jnc` file** with style block + component
2. **Compile:** `cargo run -- compile app.jnc`
3. **Add reactive setup** to `dist/client.js` (manual for now)
4. **Test in browser:** `cd dist && python3 -m http.server 8080`
5. **Hard refresh:** Ctrl+Shift+R to bypass cache

**Next Session:** Steps 3-4 automated via `@persist` decorator!

---

**For full history, see `CLAUDE_ARCHIVE.md`**
