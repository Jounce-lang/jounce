# Foundational Fixes for Jounce Gotchas

**Status**: Proposed fixes for confirmed compiler issues
**Priority**: CRITICAL - These cause silent failures that compile but break at runtime

---

## Confirmed Gotchas (Tested)

### ✅ Gotcha #1: Forgetting `.value` on Signals
**Status**: **CONFIRMED** - Compiles, destroys reactivity at runtime

**Test Result**:
```jounce
let count = signal(0);
count = count + 1;  // Compiles but overwrites signal object!
```

**Generated JavaScript**:
```javascript
let count = signal(0);
count = (count + 1);  // ❌ Destroys signal, assigns number
```

**Foundational Fix Options**:

1. **Type Checker Fix** (Recommended):
   - Location: `src/type_checker.rs`
   - Detection: When a variable initialized with `signal()` is reassigned
   - Error: `"Cannot assign non-signal value to signal variable 'count'. Did you mean 'count.value = ...'?"`
   - Implementation:
     ```rust
     // In type_checker.rs
     fn check_assignment(&mut self, var_name: &str, expr_type: Type) {
         if let Some(var_type) = self.env.get(var_name) {
             if is_signal_type(var_type) && !is_signal_type(&expr_type) {
                 self.errors.push(format!(
                     "Cannot assign {} to signal variable '{}'. Did you mean '{}.value = ...'?",
                     type_name(&expr_type), var_name, var_name
                 ));
             }
         }
     }
     ```

2. **Semantic Analyzer Fix** (Auto-correct):
   - Location: `src/semantic_analyzer.rs`
   - Transform: `count = expr` → `count.value = expr` when `count` is a signal
   - Risk: Might hide developer mistakes, less explicit

3. **Runtime Fix** (Safety net):
   - Location: `dist/reactivity.js`
   - Make signal objects frozen/immutable
   - Implementation:
     ```javascript
     export function signal(initialValue) {
         const s = { value: initialValue, subscribers: new Set() };
         Object.freeze(s);  // Prevent reassignment
         return s;
     }
     ```

**Recommendation**: Implement #1 (Type Checker) + #3 (Runtime) for defense-in-depth.

---

### ✅ Gotcha #3: `await` Inside JSX
**Status**: **CONFIRMED** - Compiles to syntax error

**Test Result**:
```jounce
component App() {
    <div>{await fetch("/api/data")}</div>
}
```

**Generated JavaScript**:
```javascript
export function App({} = {}) {
  return h('div', null, await fetch("/api/data"));  // ❌ Syntax error!
}
```

**Problem**: `await` in non-async function is a JavaScript syntax error.

**Foundational Fix Options**:

1. **Parser Rejection** (Recommended):
   - Location: `src/parser.rs`
   - Detection: `await` keyword inside JSX expression
   - Error: `"await cannot be used inside JSX. Use signals and onMount instead"`
   - Implementation:
     ```rust
     // In parser.rs - parse_jsx_expression()
     fn parse_jsx_expression(&mut self) -> Result<Expr, String> {
         let expr = self.parse_expression()?;
         if contains_await(&expr) {
             return Err("await cannot be used inside JSX. Fetch data in onMount() and store in a signal.".to_string());
         }
         Ok(expr)
     }
     ```

2. **Component Auto-Async**:
   - Transform component to `async function`
   - Risk: Components can't be async in most frameworks
   - Not recommended

**Recommendation**: Implement #1 (Parser Rejection) with helpful error message.

---

### ✅ Gotcha #8: `.length()` vs `.length`
**Status**: **CONFIRMED** - Compiles to runtime error

**Test Result**:
```jounce
let count = items.value.length();  // Should be .length property
```

**Generated JavaScript**:
```javascript
let count = items.value.length();  // ❌ TypeError at runtime
```

**Foundational Fix Options**:

1. **Type Checker Fix** (Recommended):
   - Location: `src/type_checker.rs`
   - Detection: Method call `.length()` on array/string type
   - Error: `".length is a property, not a method. Use 'items.length' without parentheses"`
   - Implementation:
     ```rust
     // In type_checker.rs - check_method_call()
     fn check_method_call(&mut self, receiver: &Expr, method: &str) -> Result<Type, String> {
         let receiver_type = self.infer_expr(receiver)?;

         match (receiver_type, method) {
             (Type::Array(_), "length") | (Type::String, "length") => {
                 return Err(format!(
                     ".length is a property, not a method. Use '.length' without parentheses."
                 ));
             }
             _ => { /* normal method lookup */ }
         }
     }
     ```

2. **Parser Transform** (Auto-correct):
   - Location: `src/parser.rs`
   - Transform: `.length()` → `.length` automatically
   - Risk: Hides mistakes, might confuse developers

**Recommendation**: Implement #1 (Type Checker) - fail fast with clear error.

---

## Additional Critical Gotchas (Needs Testing)

### Gotcha #2: Not Using Signals (`let mut` vs `signal()`)

**Foundational Fix**:
- **Lint Warning** in components:
  - Location: `src/semantic_analyzer.rs`
  - Detection: `let mut` inside component scope
  - Warning: `"Mutable variable 'x' in component won't trigger re-renders. Did you mean 'let x = signal(...)'?"`

---

### Gotcha #4: Async `.map()` Returning Promises

**Foundational Fix**:
- **Type Checker**:
  - Location: `src/type_checker.rs`
  - Detection: `.map()` callback returns `Promise<T>`
  - Error: `"map() callback returns Promise. JSX expects synchronous values. Pre-fetch data with Promise.all() before rendering"`

---

### Gotcha #5: Signal Shadowing

**Foundational Fix**:
- **Scope Analyzer**:
  - Location: `src/semantic_analyzer.rs`
  - Detection: Variable with same name as outer scope signal
  - Warning: `"Variable 'count' shadows signal from outer scope, breaking reactivity"`

---

### Gotcha #6: Side Effects in `computed()`

**Foundational Fix**:
- **Static Analysis** (Advanced):
  - Location: `src/semantic_analyzer.rs`
  - Detection: Function calls, I/O operations inside `computed()` callback
  - Warning: `"computed() should be pure. Found side effect: console.log(). Use effect() instead"`
  - Challenge: Hard to detect all side effects statically

**Alternative - Runtime Fix**:
- Implement "strict mode" that tracks side effects:
  ```javascript
  let __computing = false;
  export function computed(fn) {
      return () => {
          __computing = true;
          const result = fn();
          __computing = false;
          return result;
      };
  }

  // Instrument console.log, fetch, etc
  const originalLog = console.log;
  console.log = (...args) => {
      if (__computing) {
          throw new Error("Side effect detected in computed()");
      }
      originalLog(...args);
  };
  ```

---

### Gotcha #7: Missing Cleanup in `onMount`

**Foundational Fix**:
- **Static Analysis**:
  - Location: `src/semantic_analyzer.rs`
  - Detection: `setInterval`, `setTimeout`, `addEventListener` in `onMount()` without `return () => ...`
  - Warning: `"onMount() uses setInterval but doesn't return cleanup. This causes memory leaks"`

---

### Gotcha #12: JSX Without Parentheses

**Foundational Fix**:
- **Parser Improvement**:
  - Location: `src/parser.rs`
  - Better JSX parsing to not require parentheses
  - This is a parser quality issue, not a semantic one

---

### Gotcha #13: Object Spread `{...obj}`

**Test Needed**: Check if spread operator is supported

**Foundational Fix**:
- If NOT supported:
  - **Parser Rejection**: Error with helpful message
  - Error: `"Object spread {...obj} not yet supported. Use manual field copying or Todo { ...fields }"`

---

### Gotcha #14: Reactivity Tracking Requires `.value`

**Foundational Fix**:
- Same as Gotcha #1 - Type checker enforcement

---

### Gotcha #17: Range Loops Over Signals Not Reactive

**Foundational Fix**:
- **Static Analysis**:
  - Location: `src/semantic_analyzer.rs`
  - Detection: `for i in 0..signal.value` pattern
  - Warning: `"Range 'for' loop over signal.value evaluates once. Wrap in effect() for reactivity"`

---

## Implementation Priority

### Phase 1: Critical Type Checker Fixes (Prevent Silent Failures)
1. ✅ **Gotcha #1**: Signal reassignment detection
2. ✅ **Gotcha #8**: `.length()` vs `.length` detection
3. ✅ **Gotcha #3**: `await` in JSX rejection

**Estimate**: 4-6 hours
**Impact**: Prevents most critical silent failures

### Phase 2: Advanced Static Analysis (Lint Warnings)
1. **Gotcha #2**: `let mut` in components warning
2. **Gotcha #5**: Signal shadowing detection
3. **Gotcha #7**: Missing cleanup detection
4. **Gotcha #17**: Non-reactive range loops warning

**Estimate**: 8-12 hours
**Impact**: Guides developers to correct patterns

### Phase 3: Runtime Safety Nets
1. **Gotcha #1**: Frozen signal objects
2. **Gotcha #6**: Computed side effect detection (dev mode)

**Estimate**: 4-6 hours
**Impact**: Catches issues in development

---

## Testing Strategy

For each fix:
1. ✅ Write test case that SHOULD fail compilation
2. ✅ Verify error message is helpful
3. ✅ Write test case with correct pattern that SHOULD compile
4. ✅ Add to regression test suite

---

## Compiler Files to Modify

1. **`src/type_checker.rs`**:
   - Add signal type tracking
   - Add method vs property detection
   - Add await-in-JSX detection

2. **`src/semantic_analyzer.rs`**:
   - Add scope analysis for shadowing
   - Add pattern detection for memory leaks
   - Add reactivity warnings

3. **`src/parser.rs`**:
   - Reject `await` in JSX expressions
   - Better JSX parsing

4. **`dist/reactivity.js`**:
   - Add frozen signals
   - Add dev mode side effect detection

---

**Status**: Ready to implement Phase 1
**Next Steps**:
1. Implement Type Checker fixes for Gotchas #1, #3, #8
2. Add comprehensive test cases
3. Update error message documentation
