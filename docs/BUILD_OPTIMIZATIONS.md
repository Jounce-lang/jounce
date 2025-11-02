# Jounce Build Optimizations

**Version**: v0.11.0 (Phase 17)
**Status**: Design Document
**Last Updated**: November 1, 2025

---

## 🎯 Goal

Reduce production bundle sizes by 30-50% through dead code elimination, tree shaking, and minification.

---

## 📊 Current State

**Development Builds**:
- No optimizations
- Full source maps
- All code included
- Fast compile times
- Typical bundle: ~200KB (uncompressed)

**Production Builds** (Current):
- Basic minification
- No dead code elimination
- No tree shaking
- Typical bundle: ~180KB (uncompressed)

**Production Builds** (Target):
- Full optimizations
- Dead code elimination
- Tree shaking
- Advanced minification
- **Target bundle: ~100KB (uncompressed)**
- **Target reduction: 45%**

---

## 🔧 Optimization Techniques

### 1. Dead Code Elimination (DCE)

**Definition**: Remove unused functions, variables, and imports.

**How It Works**:
1. Build call graph of all functions
2. Mark entry points (main, exports, components)
3. Traverse call graph from entry points
4. Mark all reachable functions
5. Remove unmarked (unreachable) functions

**Example**:
```jounce
// Source code
fn used_function() {
    console.log("Used");
}

fn unused_function() {  // Never called!
    console.log("Unused");
}

fn main() {
    used_function();
}
```

**After DCE**:
```jounce
fn used_function() {
    console.log("Used");
}

fn main() {
    used_function();
}

// unused_function was removed!
```

---

### 2. Tree Shaking

**Definition**: Remove unused exports from imported modules.

**How It Works**:
1. Analyze what's actually imported from each module
2. Only include code for used exports
3. Remove unused exports from bundles

**Example**:
```jounce
// utils.jnc exports 10 functions
export { add, subtract, multiply, divide, pow, sqrt, log, sin, cos, tan };

// app.jnc only uses one
import { add } from "./utils.jnc";

let result = add(1, 2);
```

**After Tree Shaking**:
```javascript
// Only `add` is included in bundle
// Other 9 functions are removed
export function add(a, b) { return a + b; }
```

**Savings**: If each function is 100 bytes, we save 900 bytes!

---

### 3. Minification

**Definition**: Reduce code size through compression techniques.

**Techniques**:

**A. Identifier Shortening**:
```javascript
// Before
function calculateTotalPrice(items) {
    let totalPrice = 0;
    for (let item of items) {
        totalPrice += item.price;
    }
    return totalPrice;
}

// After
function a(b){let c=0;for(let d of b)c+=d.price;return c}
```

**B. Whitespace Removal**:
```javascript
// Before (readable)
function add(a, b) {
    return a + b;
}

// After (minified)
function add(a,b){return a+b}
```

**C. Comment Removal**:
```javascript
// Before
// This function adds two numbers
function add(a, b) {
    return a + b;  // Return sum
}

// After
function add(a,b){return a+b}
```

**D. Constant Folding**:
```javascript
// Before
let x = 2 + 3 * 4;

// After
let x = 14;
```

**E. Boolean Simplification**:
```javascript
// Before
if (condition === true) { ... }

// After
if (condition) { ... }
```

**Typical Savings**: 30-40% size reduction

---

### 4. Code Splitting

**Definition**: Split code into chunks loaded on demand.

**Strategies**:

**A. Route-Based Splitting**:
```jounce
// Each route gets its own bundle
route("/home", () => import("./Home.jnc"));    // home-[hash].js
route("/about", () => import("./About.jnc"));  // about-[hash].js
route("/contact", () => import("./Contact.jnc")); // contact-[hash].js
```

**B. Component Lazy Loading**:
```jounce
// Lazy load heavy components
let LazyChart = lazy(() => import("./Chart.jnc"));

component App() {
    <div>
        <h1>Dashboard</h1>
        <Suspense fallback={<p>Loading chart...</p>}>
            <LazyChart data={data} />
        </Suspense>
    </div>
}
```

**Benefits**:
- Initial bundle: ~50KB (core app)
- Route chunks: ~20KB each (loaded on demand)
- Total transferred: Only what's needed!

---

## 🏗️ Implementation Architecture

### Build Pipeline

```
Source Code (.jnc files)
        ↓
    [Lexer & Parser]
        ↓
   Abstract Syntax Tree (AST)
        ↓
    [Semantic Analysis]
        ↓
    [Optimizer] ← NEW!
     ├─ Dead Code Elimination
     ├─ Tree Shaking
     ├─ Constant Folding
     └─ Unused Import Removal
        ↓
    [Code Generator]
        ↓
   JavaScript Output
        ↓
    [Minifier] ← NEW!
     ├─ Identifier Shortening
     ├─ Whitespace Removal
     ├─ Comment Removal
     └─ Boolean Simplification
        ↓
   Optimized Bundle
```

---

## 💻 CLI Flags

### Development Build (Default)
```bash
jnc build
```
- No optimizations
- Source maps included
- Fast compilation
- Easy debugging

### Production Build
```bash
jnc build --release
```
- All optimizations enabled
- Dead code elimination
- Tree shaking
- Minification
- No source maps (unless --sourcemap)

### Production with Source Maps
```bash
jnc build --release --sourcemap
```
- All optimizations
- Source maps included
- Easier debugging in production

### Minification Only
```bash
jnc build --minify
```
- Only minification (no DCE/tree shaking)
- Useful for debugging optimization issues

### Bundle Analysis
```bash
jnc build --release --analyze
```
- Generates bundle size report
- Shows largest modules
- Identifies optimization opportunities
- Creates `bundle-analysis.html`

### Custom Optimization Level
```bash
jnc build --optimize 0  # No optimization
jnc build --optimize 1  # Basic (minify only)
jnc build --optimize 2  # Standard (DCE + minify)
jnc build --optimize 3  # Aggressive (DCE + tree shaking + minify)
```

---

## 📈 Optimization Levels

| Level | DCE | Tree Shake | Minify | Code Split | Typical Savings |
|-------|-----|------------|--------|------------|-----------------|
| 0 (dev) | ❌ | ❌ | ❌ | ❌ | 0% |
| 1 (basic) | ❌ | ❌ | ✅ | ❌ | 30% |
| 2 (standard) | ✅ | ❌ | ✅ | ❌ | 40% |
| 3 (aggressive) | ✅ | ✅ | ✅ | ✅ | 50% |

**Default**:
- `jnc build` → Level 0
- `jnc build --release` → Level 3

---

## 🔍 Dead Code Elimination Algorithm

### Call Graph Analysis

```rust
struct CallGraph {
    functions: HashMap<String, FunctionNode>,
    entry_points: Vec<String>,
}

struct FunctionNode {
    name: String,
    calls: Vec<String>,      // Functions this function calls
    is_exported: bool,       // Is this an export?
    is_reachable: bool,      // Marked as reachable?
}

impl CallGraph {
    fn mark_reachable(&mut self) {
        // 1. Mark all entry points as reachable
        for entry in &self.entry_points {
            self.mark_function_reachable(entry);
        }

        // 2. Mark all exports as reachable
        for (name, node) in &self.functions {
            if node.is_exported {
                self.mark_function_reachable(name);
            }
        }
    }

    fn mark_function_reachable(&mut self, name: &str) {
        if let Some(node) = self.functions.get_mut(name) {
            if !node.is_reachable {
                node.is_reachable = true;

                // Recursively mark called functions
                let calls = node.calls.clone();
                for called in calls {
                    self.mark_function_reachable(&called);
                }
            }
        }
    }

    fn remove_unreachable(&mut self) -> Vec<String> {
        let mut removed = Vec::new();
        self.functions.retain(|name, node| {
            if !node.is_reachable {
                removed.push(name.clone());
                false
            } else {
                true
            }
        });
        removed
    }
}
```

---

## 🧪 Testing Strategy

### Unit Tests

```rust
#[test]
fn test_remove_unused_function() {
    let source = r#"
        fn used() {
            console.log("used");
        }

        fn unused() {
            console.log("unused");
        }

        fn main() {
            used();
        }
    "#;

    let optimized = optimize_with_dce(source);

    // unused() should be removed
    assert!(!optimized.contains("fn unused"));
    assert!(optimized.contains("fn used"));
}
```

### Integration Tests

```bash
# Test bundle size reduction
jnc build examples/large-app.jnc
# Before: 250KB

jnc build examples/large-app.jnc --release
# After: 125KB (50% reduction)
```

---

## 📊 Benchmark Targets

| Metric | Dev Build | Production Build | Improvement |
|--------|-----------|------------------|-------------|
| **Bundle Size** | 200 KB | 100 KB | 50% |
| **Gzipped Size** | 60 KB | 30 KB | 50% |
| **Parse Time** | 50 ms | 40 ms | 20% |
| **Compile Time** | 2s | 8s | -300% (slower OK for prod) |

---

## 🚀 Implementation Phases

### Phase 1: Basic Dead Code Elimination (Week 1)
- [ ] Build call graph from AST
- [ ] Identify entry points (main, exports)
- [ ] Mark reachable functions
- [ ] Remove unreachable functions
- [ ] Test with real codebases

### Phase 2: Tree Shaking (Week 2)
- [ ] Track imports and exports
- [ ] Identify unused exports
- [ ] Remove unused module code
- [ ] Handle re-exports
- [ ] Test with package imports

### Phase 3: Minification (Week 2)
- [ ] Implement identifier shortening
- [ ] Remove whitespace and comments
- [ ] Constant folding
- [ ] Boolean simplification
- [ ] Test with real codebases

### Phase 4: Code Splitting (Week 3)
- [ ] Identify split points (routes, lazy imports)
- [ ] Generate multiple bundles
- [ ] Runtime chunk loading
- [ ] Preloading strategies
- [ ] Test with routing examples

---

## 🎯 Success Metrics

- ✅ 30-50% bundle size reduction
- ✅ No breaking changes to working code
- ✅ 100% test coverage on optimizer
- ✅ Production build < 10 seconds (for typical app)
- ✅ Dev build stays fast (< 2 seconds)

---

## 🔗 Related Documents

- [Security System](./SECURITY_SYSTEM.md)
- [Deployment Guide](./DEPLOYMENT_GUIDE.md)
- [CLI Build Flags](./CLI_BUILD_FLAGS.md)

---

**Last Updated**: November 1, 2025
**Phase**: 17 - Security & Production Features
**Sprint**: 17.2 - Production Build Optimizations
