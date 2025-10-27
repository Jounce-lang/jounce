# Phase 2: Package Ecosystem Integration - Implementation Plan

**Status:** Planning Complete, Ready to Execute
**Created:** October 26, 2025
**Estimated Duration:** 2-3 days
**Priority:** P0 - CRITICAL FOR ECOSYSTEM

---

## 🎯 Mission

Transform 35 existing packages (850+ tests) from "code that exists" to "features that work" by implementing full package import resolution and compilation.

**Success Metric:** Users can write `use jounce::db::{Database, Query};` and it actually works!

---

## 📊 Current State

### What Exists ✅
- ✅ 35 packages in `packages/` directory
- ✅ 850+ tests passing in package code
- ✅ Multi-file imports work (`./` and `../` relative paths)
- ✅ Module loader has basic structure (`src/module_loader.rs`)
- ✅ Package.toml files exist for all packages

### What's Broken ❌
- ❌ Can't `use jounce::db` - parser doesn't recognize package imports
- ❌ Package resolution not implemented
- ❌ Packages don't compile to JavaScript
- ❌ No package bundling with applications
- ❌ No package module registration

### The Gap
Users can import local files (`import { foo } from "./bar.jnc"`), but standard library packages like `jounce-db`, `jounce-auth`, etc. are completely inaccessible.

---

## 🎯 Goals

### Primary Goals (Must Have)
1. **Package Import Syntax** - Support `use jounce::db::{Database};`
2. **Package Resolution** - Find packages in `packages/jounce-<name>/`
3. **Package Compilation** - Compile package .jnc files to JavaScript
4. **Package Bundling** - Include packages in application output
5. **Test with Real Package** - Get `jounce-db` working end-to-end

### Secondary Goals (Should Have)
6. **Multiple Package Support** - `jounce-auth`, `jounce-config`, `jounce-validation`
7. **Example Apps** - Update 3-5 examples to use real packages
8. **Documentation** - Update FEATURES.md with working package examples

### Stretch Goals (Nice to Have)
9. **Package Caching** - Cache compiled packages (like file cache)
10. **Version Management** - Basic semver support in package.toml
11. **Dependency Resolution** - Handle package-to-package dependencies

---

## 📋 Detailed Task Breakdown

### Day 1: Foundation (6-8 hours)

#### Task 1.1: Research & Understanding (1 hour)
- [x] Read `src/module_loader.rs` implementation
- [ ] Identify where local imports are resolved
- [ ] Trace code path from `use` statement → file loading
- [ ] Document current architecture

**Deliverable:** Architecture notes in this file

---

#### Task 1.2: Test Current Behavior (30 mins)
- [ ] Create test file: `test_package_import.jnc`
- [ ] Try: `use jounce::db::{Database};`
- [ ] Document exact error message
- [ ] Identify where the error occurs (lexer/parser/module_loader)

**Expected Result:** Parser error or module resolution error

---

#### Task 1.3: Parser Support for Package Imports (2 hours)
**Location:** `src/parser.rs`

Current syntax works:
```jounce
import { foo } from "./bar.jnc";  // Relative path
```

Need to support:
```jounce
use jounce::db::{Database, Query};  // Package path
use jounce::auth::*;                // Wildcard import
use jounce::config::env;            // Single item
```

**Changes Required:**
1. Add `use` keyword to lexer (if not exists)
2. Parse `jounce::package::item` syntax (module path with `::`)
3. Create AST node: `UseStatement` or extend `ImportStatement`
4. Parse import items: `{Database, Query}`, `*`, or single identifier

**Test Cases:**
```jounce
use jounce::db::{Database};
use jounce::db::{Database, Query, Transaction};
use jounce::auth::*;
use jounce::config::env;
```

**Deliverable:** Parser tests passing for `use` syntax

---

#### Task 1.4: Package Path Resolution (2-3 hours)
**Location:** `src/module_loader.rs`

**Current Implementation:**
```rust
// Resolves "./bar.jnc" relative to current file
fn resolve_import(&mut self, import_path: &str, current_file: &Path) -> Result<PathBuf>
```

**New Implementation Needed:**
```rust
// Resolves "jounce::db" to "packages/jounce-db/src/lib.jnc"
fn resolve_package_import(&mut self, package_path: &str) -> Result<PathBuf>
```

**Algorithm:**
1. Parse `jounce::db` → package name = `db`
2. Look for `packages/jounce-db/` directory
3. Read `packages/jounce-db/package.toml` for entry point (default: `src/lib.jnc`)
4. Return path to entry file
5. Handle errors: package not found, malformed package.toml, missing entry point

**Edge Cases:**
- Package doesn't exist → clear error message
- Package.toml missing → default to `src/lib.jnc`
- Circular package dependencies → detect and error
- Package has no exports → empty module

**Deliverable:** Unit tests for package resolution

---

#### Task 1.5: Package Compilation (1-2 hours)
**Location:** `src/module_loader.rs`, `src/js_emitter.rs`

**Current Implementation:**
- Compiles local .jnc files to JavaScript
- Merges imports into single AST

**New Requirements:**
1. Compile package entry point (lib.jnc)
2. Follow package's internal imports
3. Generate JavaScript for package code
4. Export package symbols

**Integration Points:**
- Use existing `parse_module()` function
- Use existing `compile_to_js()` codegen
- Add package code to bundle

**Deliverable:** Package compiles to valid JavaScript

---

### Day 2: Integration & Testing (6-8 hours)

#### Task 2.1: Module Registration (2 hours)
**Location:** `src/js_emitter.rs`

**Challenge:** JavaScript needs to know about imported symbols

**Current Output (local imports):**
```javascript
// bar.js exports
export function foo() { ... }

// main.js imports
import { foo } from './bar.js';
```

**Required Output (package imports):**
```javascript
// jounce-db package bundle
const Database = { /* package code */ };
const Query = { /* package code */ };

// main.js uses package
// Symbols available in scope
```

**Options:**
1. **ES6 Modules** - Generate proper import/export statements
2. **Global Object** - `window.jounce = { db: { Database, Query } }`
3. **Module Bundler** - Bundle packages into single file

**Recommended:** Option 1 (ES6 Modules) - cleanest, most standard

**Deliverable:** Package exports accessible in application code

---

#### Task 2.2: Test with jounce-db Package (3-4 hours)
**Location:** `packages/jounce-db/`

**Test Application:**
```jounce
// test_db_app.jnc
use jounce::db::{Database, Query};

fn main() {
    let db = Database::connect("postgres://localhost/test");
    let users = db.query("SELECT * FROM users");
    println!("Users: {}", users);
}
```

**Test Steps:**
1. Compile test_db_app.jnc
2. Verify no parser errors
3. Verify module resolution finds jounce-db
4. Verify package compiles to JavaScript
5. Verify generated JavaScript has Database and Query
6. Run in browser/Node.js and verify no runtime errors
7. Verify Database methods are callable

**Success Criteria:**
- ✅ Compiles without errors
- ✅ Generated JavaScript contains package code
- ✅ Database and Query symbols are accessible
- ✅ Methods can be called (even if they don't connect to real DB)

**Deliverable:** Working example app using jounce-db

---

#### Task 2.3: Error Handling & Messages (1 hour)

**Scenarios to Handle:**
1. Package not found
   - Error: `Package 'jounce::foo' not found. Did you mean 'jounce::db'?`
2. Package exists but item not exported
   - Error: `'Foo' is not exported from package 'jounce::db'. Available exports: Database, Query`
3. Circular package dependencies
   - Error: `Circular package dependency: jounce::db → jounce::auth → jounce::db`
4. Malformed package.toml
   - Error: `Failed to parse package.toml for 'jounce::db': <toml error>`

**Deliverable:** Friendly error messages for all failure cases

---

#### Task 2.4: Integration Tests (1 hour)

**Location:** `src/integration_tests.rs`

**Test Cases:**
```rust
#[test]
fn test_package_import_simple() {
    let source = r#"
        use jounce::db::{Database};

        fn main() {
            let db = Database::connect("test");
        }
    "#;

    let result = compile_source(source);
    assert!(result.is_ok());
}

#[test]
fn test_package_import_multiple() {
    let source = r#"
        use jounce::db::{Database, Query};
    "#;
    // ...
}

#[test]
fn test_package_not_found() {
    let source = r#"
        use jounce::nonexistent::{Foo};
    "#;

    let result = compile_source(source);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}
```

**Deliverable:** 5-10 integration tests covering happy path and errors

---

### Day 3: Multiple Packages & Polish (4-6 hours)

#### Task 3.1: Additional Packages (2-3 hours)

**Test with:**
1. **jounce-auth** (authentication package)
2. **jounce-config** (environment config)
3. **jounce-validation** (form validation)

**For Each Package:**
1. Write test application using package
2. Verify compilation works
3. Verify JavaScript output correct
4. Document any issues

**Deliverable:** 3 more packages working

---

#### Task 3.2: Update Example Apps (1-2 hours)

**Examples to Update:**
1. `examples/projects/task-dashboard/` - Use jounce-db, jounce-auth
2. `examples/projects/devboard/` - Use jounce-websocket, jounce-metrics
3. Create new: `examples/packages/db-example/` - Simple CRUD app

**For Each Example:**
1. Replace fake imports with real package imports
2. Test compilation
3. Update README with instructions
4. Add to test suite

**Deliverable:** 3-5 working examples using real packages

---

#### Task 3.3: Documentation (1 hour)

**Files to Update:**
1. **FEATURES.md** - Mark packages as "✅ Integrated"
2. **BUILDING_APPS.md** - Add package import tutorial
3. **CLAUDE.md** - Update status to "Phase 2 Complete"
4. **COMPREHENSIVE_AUDIT.md** - Mark Phase 2 as done

**New Documentation:**
- Create **PACKAGE_GUIDE.md** with:
  - How to use packages
  - Available packages list
  - How to create packages (future)

**Deliverable:** Accurate, updated documentation

---

## 🧪 Testing Strategy

### Unit Tests
- [ ] Parser tests for `use` syntax (5 tests)
- [ ] Package resolution tests (5 tests)
- [ ] Module loader tests (5 tests)

### Integration Tests
- [ ] Full compilation with package imports (10 tests)
- [ ] Error handling tests (5 tests)

### End-to-End Tests
- [ ] jounce-db working in real app
- [ ] jounce-auth working in real app
- [ ] Multiple packages in one app

### Manual Testing
- [ ] Compile example apps
- [ ] Run in browser
- [ ] Verify no console errors
- [ ] Test package functionality

**Total Tests:** ~30-40 new tests

---

## ⚠️ Risk Mitigation

### Risk 1: Parser Changes Break Existing Code
**Mitigation:** Run full test suite after each parser change
**Fallback:** Revert parser changes, use different syntax

### Risk 2: Module Loader Complexity Explodes
**Mitigation:** Start simple (single package), iterate
**Fallback:** Implement MVP only (jounce-db), defer others

### Risk 3: JavaScript Module System Conflicts
**Mitigation:** Test with both ES6 modules and CommonJS
**Fallback:** Use global object if modules don't work

### Risk 4: Package Dependencies Cause Circular Issues
**Mitigation:** Detect cycles early, fail fast with clear error
**Fallback:** Ban package-to-package imports in v1

### Risk 5: Takes Longer Than 3 Days
**Mitigation:** Time-box each task, cut scope if needed
**Fallback:** Ship jounce-db only, defer others to Phase 2.5

---

## 📅 Timeline

### Day 1 (6-8 hours)
- **Morning:** Tasks 1.1-1.3 (Parser support)
- **Afternoon:** Tasks 1.4-1.5 (Package resolution & compilation)
- **EOD Checkpoint:** Parser can handle `use` syntax, packages can be found

### Day 2 (6-8 hours)
- **Morning:** Tasks 2.1-2.2 (Module registration, jounce-db test)
- **Afternoon:** Tasks 2.3-2.4 (Error handling, integration tests)
- **EOD Checkpoint:** jounce-db works end-to-end

### Day 3 (4-6 hours)
- **Morning:** Task 3.1 (More packages)
- **Afternoon:** Tasks 3.2-3.3 (Examples, documentation)
- **EOD Checkpoint:** Phase 2 complete, ready to merge

**Total:** 16-22 hours over 3 days

---

## ✅ Success Criteria

**Phase 2 is DONE when:**

1. ✅ Parser supports `use jounce::package::{Item};` syntax
2. ✅ Module loader resolves package paths to `packages/jounce-<name>/`
3. ✅ Packages compile to JavaScript
4. ✅ Package exports are accessible in applications
5. ✅ jounce-db works in a real test application
6. ✅ At least 3 packages integrated (db, auth, config)
7. ✅ 3-5 example apps using real packages
8. ✅ 30-40 new tests passing
9. ✅ Documentation updated
10. ✅ No regressions (all 625 existing tests still pass)

**Bonus Success:**
- ✅ 10+ packages working
- ✅ Package caching implemented
- ✅ Package-to-package imports work

---

## 📝 Architecture Notes (Task 1.1) ✅

### 🎉 MAJOR DISCOVERY: Infrastructure Already Exists!

**The module loader is mostly built!** This is WAY better than expected!

### Current Import Flow
```
1. Parser encounters `use` statement → creates UseStatement AST node
2. UseStatement has:
   - path: Vec<Identifier> (e.g., ["raven", "router"])
   - imports: Vec<Identifier> (e.g., ["Router", "Route"]) or empty for wildcard

3. ModuleLoader.merge_imports(program):
   - Collects all UseStatement nodes
   - For each use statement:
     a. resolve_module_path(["raven", "router"]) → PathBuf
     b. Searches in: test_modules/, packages/, aloha-shirts/
     c. Converts snake_case → kebab-case (raven_router → raven-router)
     d. Looks for: packages/raven-router/src/lib.jnc
     e. Parses the module file
     f. extract_exports() → HashMap<String, ExportedSymbol>
     g. Inserts imported symbols into main program AST

4. Result: Imported functions/structs/enums available in main program
```

### What Already Works ✅
1. **UseStatement AST node exists** (src/ast.rs line 4)
2. **Module resolution** - Finds packages in `packages/` directory
3. **Snake to kebab conversion** - `raven_router` → `raven-router`
4. **Export extraction** - All top-level items exported automatically
5. **Symbol merging** - Imported items added to AST
6. **Wildcard imports** - `use foo::*` supported
7. **Selective imports** - `use foo::{Bar, Baz}` supported
8. **Circular dependency detection** - Prevents infinite loops
9. **Module caching** - Modules only loaded once

### What Might Need Work ⚠️
1. **Parser syntax** - Need to verify `use jounce::db::{Database}` parses correctly
2. **Package prefix** - Does `jounce::db` work, or just `db`?
3. **JavaScript emission** - Are imports included in generated JS?
4. **Runtime availability** - Do imported symbols work at runtime?

### Key Code Locations
- `src/ast.rs` - UseStatement definition (check line 4 import)
- `src/module_loader.rs:59-153` - resolve_module_path() - Package resolution
- `src/module_loader.rs:156-218` - load_module() - Parse and cache modules
- `src/module_loader.rs:293-390` - merge_imports() - Insert imports into AST
- `src/parser.rs` - Need to find use statement parser
- `src/js_emitter.rs` - Need to check if imports are emitted

### Next Steps
✅ Architecture understood - MUCH simpler than expected!
➡️ Task 1.2: Test if `use jounce::db::{Database}` actually works right now!

---

## 🚀 Ready to Execute

**Next Step:** Begin Task 1.1 - Research & Understanding

**Command to start:**
```bash
# Read and understand module_loader.rs
# Document findings in this file
```

---

**This plan will be updated as we progress. Mark tasks complete with ✅**
