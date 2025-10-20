# Closure and Lambda Support Status

## ✅ Completed (as of today)

### 1. Function Type Syntax
- **Added**: `TypeExpression::Function(Vec<TypeExpression>, Box<TypeExpression>)`
- **Syntax**: `fn(i32, String) -> bool`
- **File**: `src/ast.rs:260`

### 2. Parser Support for Function Types
- **Feature**: Parse function type expressions like `fn(T1, T2) -> R`
- **File**: `src/parser.rs:452-465`

### 3. Typed Let Bindings
- **Feature**: Optional type annotations on let statements
- **Syntax**: `let x: i32 = 42;` or `let func: fn(i32) -> i32 = add_ten;`
- **Files**:
  - AST: `src/ast.rs:36-40`
  - Parser: `src/parser.rs:500-514`

### 4. Type System Integration
- **Semantic Analyzer**: Handles function types (returns `ResolvedType::Unknown` placeholder)
- **Type Checker**: Converts to `Type::Function` for Hindley-Milner inference
- **RPC Generator**: Formats as TypeScript arrow functions `(arg0: Type) => ReturnType`

### 5. Lambda Expression Parsing
- **Feature**: Parse lambda syntax `|x| x + 10` and `|x, y| x + y`
- **AST**: `Expression::Lambda` with parameters, body, and captures
- **File**: `src/parser.rs:338-356`

## ⚠️ Limitations (Not Yet Implemented)

### 1. Lambda Code Generation
**Problem**: Lambdas compile inline as expressions, but:
- Cannot be stored in variables
- Cannot be passed as function arguments
- Cannot capture variables from outer scope

**Why**: WASM doesn't have native function pointers. Requires:
- Function table (`table` section in WASM)
- `call_indirect` instruction support
- Closure environment management

**Example that FAILS**:
```raven
fn test() -> i32 {
    let add_ten = |x| x + 10;  // ❌ Lambda stored in variable
    return add_ten(5);          // ❌ Variable called as function
}
```

### 2. Lambda Parameter Type Inference
**Problem**: Lambda parameters without type annotations get `Type::Any`, causing type errors.

**Example that FAILS**:
```raven
let adder = |x| x + 10;  // ❌ Can't infer type of 'x'
```

**Workaround**: Use explicit type annotation on the let binding:
```raven
let adder: fn(i32) -> i32 = |x| x + 10;  // ✅ Type known from annotation
```

###3. Closure Capture
**Problem**: Closures cannot capture variables from outer scope.

**Example that FAILS**:
```raven
fn make_adder(n: i32) -> fn(i32) -> i32 {
    return |x| x + n;  // ❌ Cannot capture 'n'
}
```

**Why**: Requires:
- Capture analysis to detect free variables
- Environment allocation (heap or stack)
- Closure struct with function pointer + environment pointer

### 4. Higher-Order Functions
**Problem**: Functions cannot accept or return function values.

**Example that FAILS**:
```raven
fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
    return f(x);  // ❌ Cannot call function parameter
}
```

## ✅ What DOES Work

### Example 1: Type-Annotated Let Bindings
```raven
fn main() -> i32 {
    let x: i32 = 42;
    let name: String = "Alice";
    let func: fn(i32) -> i32 = add_ten;  // Function reference (parsed, but not callable)
    return x;
}
```

### Example 2: Inline Lambdas (Limited)
```raven
// Lambdas work inline in some contexts (like JSX event handlers)
component Button() {
    <button onClick={|e| console::log("clicked")}>
        Click me
    </button>
}
```

## 🚀 Implementation Plan for Full Support

### Phase 1: Function Tables (2-3 days)
1. Generate WASM function table
2. Add functions to table during codegen
3. Implement `call_indirect` for function calls
4. Support function parameters and return values

### Phase 2: Closure Environments (3-4 days)
1. Implement capture analysis
2. Generate closure structs (function index + environment)
3. Allocate environment on heap
4. Pass environment pointer to closure functions

### Phase 3: Type Inference (2-3 days)
1. Bidirectional type inference for lambdas
2. Infer lambda parameter types from context
3. Unification with expected function types

### Phase 4: Optimization (1-2 days)
1. Inline simple lambdas
2. Eliminate unused captures
3. Stack-allocate small environments

**Total Estimated Time**: 8-12 days of focused development

## 📚 References

**WASM Function Tables**:
- [WebAssembly Tables](https://webassembly.github.io/spec/core/syntax/modules.html#tables)
- [`call_indirect` instruction](https://webassembly.github.io/spec/core/exec/instructions.html#xref-syntax-instructions-syntax-call-indirect-mathsf-call-indirect-x-y)

**Closure Implementation**:
- [Implementing Closures in Rust](https://rustc-dev-guide.rust-lang.org/closure.html)
- [Lambda Lifting](https://en.wikipedia.org/wiki/Lambda_lifting)

**Type Inference**:
- [Bidirectional Type Checking](https://arxiv.org/pdf/1908.05839.pdf)
- [Hindley-Milner Type Inference](https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system)

## 🎯 Current Status Summary

**Parser & Type System**: ✅ Fully functional
**Code Generation**: ⚠️ Basic support only
**Full Closures**: ❌ Not implemented

The foundation is solid, but runtime support for first-class functions needs significant work.
