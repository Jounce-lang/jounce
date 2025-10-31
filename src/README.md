# Jounce Compiler Source Code

**Source code for the Jounce programming language compiler**

---

## 📂 Module Structure

### Core Compiler Pipeline
- **`lib.rs`** - Main compiler entry point and pipeline
- **`main.rs`** - CLI interface and command handling
- **`lexer.rs`** - Tokenization (source → tokens)
- **`parser.rs`** - Parsing (tokens → AST)
- **`ast.rs`** - Abstract Syntax Tree definitions
- **`token.rs`** - Token types and definitions

### Analysis Passes
- **`semantic_analyzer.rs`** - Semantic analysis (scope, variables)
- **`type_checker.rs`** - Type checking and inference
- **`types.rs`** - Type system definitions
- **`borrow_checker.rs`** - Borrow checking for safety
- **`reactive_analyzer.rs`** - Reactivity analysis for signals

### Code Generation
- **`codegen.rs`** - WASM codegen (AST → WASM)
- **`js_emitter.rs`** - JavaScript emitter (AST → JS)
- **`css_generator.rs`** - CSS generation from style blocks
- **`css_utilities.rs`** - CSS utility class generation (457 classes)
- **`utility_generator.rs`** - Utility class system
- **`utility_config.rs`** - Utility configuration

### Optimization
- **`wasm_optimizer.rs`** - WASM optimization
  - Dead code elimination
  - Constant folding
  - Function inlining

### Runtime & Infrastructure
- **`vdom.rs`** - Virtual DOM implementation
- **`cache/`** - Compilation caching
- **`ssr.rs`** - Server-side rendering
- **`hydration.rs`** - Client-side hydration
- **`reactive.rs`** - Reactive state management

### Advanced Features
- **`router.rs`** - Client-side routing
- **`forms.rs`** - Form handling and validation
- **`animation.rs`** - Animation system
- **`design_tokens.rs`** - Design token parsing
- **`macros.rs`** - Macro expansion

### Package System
- **`module_loader.rs`** - Module/package loading
- **`package_manager/`** - Package management
- **`stdlib.rs`** - Standard library

### Developer Tools
- **`diagnostics.rs`** - Error diagnostics and formatting
- **`error_help.rs`** - Error help text database (20+ codes)
- **`errors.rs`** - Error types
- **`lsp/`** - Language Server Protocol
- **`formatter.rs`** - Code formatter
- **`doc_generator.rs`** - Documentation generator
- **`profiler.rs`** - Performance profiling
- **`watcher.rs`** - File watching

### Build & Deploy
- **`deployer/`** - Deployment utilities
- **`code_splitter.rs`** - Code splitting
- **`rpc_generator.rs`** - RPC stub generation
- **`source_map.rs`** - Source map generation
- **`js_minifier.rs`** - JavaScript minification
- **`hmr.rs`** - Hot Module Replacement
- **`wasm_runtime.rs`** - WASM runtime support
- **`test_framework/`** - Testing framework

---

## 🔄 Compilation Pipeline

```
Source Code (.jnc file)
    ↓
Lexer (lexer.rs)
    ↓
Tokens
    ↓
Parser (parser.rs)
    ↓
AST (ast.rs)
    ↓
Macro Expansion (macros.rs)
    ↓
Module Resolution (module_loader.rs)
    ↓
Semantic Analysis (semantic_analyzer.rs)
    ↓
Type Checking (type_checker.rs)
    ↓
Borrow Checking (borrow_checker.rs)
    ↓
Reactivity Analysis (reactive_analyzer.rs)
    ↓
Code Generation
    ├── WASM (codegen.rs)
    ├── JavaScript (js_emitter.rs)
    └── CSS (css_generator.rs)
    ↓
Optimization (wasm_optimizer.rs)
    ↓
Output Files
    ├── server.js
    ├── client.js
    ├── app.wasm
    ├── styles.css
    └── index.html
```

---

## 🧩 Key Components

### Lexer
**File**: `lexer.rs`
- Tokenizes source code
- Handles JSX mode switching
- Tracks line/column positions
- **~1200 lines**

### Parser
**File**: `parser.rs`
- Recursive descent parser
- Produces AST
- Handles JSX syntax
- **~4000 lines**

### AST
**File**: `ast.rs`
- Node definitions for all language constructs
- Component, Function, Expression, Statement types
- **~1500 lines**

### JS Emitter
**File**: `js_emitter.rs`
- Emits JavaScript code
- Handles reactivity wrapping
- Client/server code splitting
- **~2500 lines**

### CSS Utilities
**File**: `css_utilities.rs`
- Generates 457 utility classes
- Tailwind-inspired design
- Auto-included in compilation
- **~800 lines**

### Error Help
**File**: `error_help.rs`
- 20+ error codes (E001-E079)
- Helpful suggestions
- Pattern matching
- **~350 lines**

---

## 📊 Statistics

**Total Lines**: ~40,000 lines of Rust
**Modules**: 40+ modules
**Tests**: 635 integration tests
**Pass Rate**: 100%

---

## 🛠️ Development

### Building
```bash
cargo build --release
```

### Testing
```bash
cargo test --lib
```

### Running
```bash
cargo run --release -- compile app.jnc
```

### Benchmarking
```bash
cargo bench
```

---

## 📝 Contributing

**Guidelines:**
1. Follow Rust conventions
2. Add tests for new features
3. Update documentation
4. Run `cargo fmt` before committing
5. Ensure all tests pass

**Adding a new module:**
1. Create `my_module.rs`
2. Add to `lib.rs`: `pub mod my_module;`
3. Write tests
4. Update this README

---

## 🔗 Resources

- [Compiler Architecture](../docs/architecture/)
- [Technical Documentation](../docs/technical/)
- [Testing Guide](../TESTING_GUIDE.md)

---

**Last Updated**: October 31, 2025 (v0.8.1)
**Lines of Code**: ~40,000
**Modules**: 40+
**Tests**: 635 passing
