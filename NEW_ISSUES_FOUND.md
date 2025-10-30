# New Issues Found - Session 21 Part 3

**Date**: October 28, 2025
**Status**: In Progress - Building 20 more apps
**Goal**: Find 10+ new issues

---

## ISSUES FOUND

### **Issue #12-1: Component Parameters Not Supported** 🔴 CRITICAL
**App**: 12-nested-components
**Severity**: HIGH
**Category**: Parser / Components

**Problem**:
```jounce
component Card(title: String, subtitle: String) -> JSX {
    //...
}
```
**Error**: `ParserError { message: "Expected LBrace, found Arrow", line: 4, column: 50 }`

**Root Cause**: Component parser doesn't support:
1. Parameters with types
2. Return type annotation (`-> JSX`)

**Current Workaround**: Use parameter-less components only

**Fix Needed**:
- Extend component parser to handle parameter list
- Support return type annotations
- Pass props to component functions

**Files**: `src/parser.rs` (component parsing)

**Priority**: HIGH - Components without props are very limited

---

### **Issue #13-1: Functions Inside Components Generate "Unsupported statement"** 🟡 MEDIUM
**App**: 13-conditional-jsx
**Severity**: MEDIUM
**Category**: Code Generation / Components

**Problem**:
```jounce
component App() {
    fn toggle() {
        show.set(!show.value);
    }
}
```
**Generated**: `// Unsupported statement`

**Root Cause**: JS emitter doesn't handle function definitions inside components

**Current Behavior**: Functions are commented out, causing runtime errors

**Fix Needed**:
- Support `fn` statements inside component bodies
- Generate proper JavaScript function declarations

**Files**: `src/js_emitter.rs`

**Priority**: MEDIUM - Can use inline lambdas as workaround

---

### **Issue #13-2: JSX Text Content Split by Spaces** 🟢 LOW
**App**: 13-conditional-jsx
**Severity**: LOW (cosmetic)
**Category**: Code Generation / JSX

**Problem**:
```jounce
<p>Showing content!</p>
```
**Generated**: `h('p', null, "Showing", "content", "!")`
**Should Be**: `h('p', null, "Showing content!")`

**Root Cause**: JSX text parser splits on spaces and generates separate text nodes

**Impact**: Works correctly but generates verbose code

**Fix Needed**:
- Combine consecutive text tokens into single string

**Files**: `src/js_emitter.rs` (JSX text generation)

**Priority**: LOW - Functional but not optimal

---

### **Issue #20-1: String Interpolation in Attributes Not Reactive** 🟡 MEDIUM
**App**: 20-dynamic-class
**Severity**: MEDIUM
**Category**: Code Generation / Reactivity

**Problem**:
```jounce
<button class="btn {active.value ? 'active' : 'inactive'}">
```
**Generated**: `{ class: "btn {active.value ? 'active' : 'inactive'}" }` (literal string!)
**Should Be**: Reactive expression that updates when `active` changes

**Root Cause**: String interpolation in JSX attributes not being detected as reactive expressions

**Impact**: Dynamic classes, styles, and other interpolated attributes don't update

**Fix Needed**:
- Detect `{...}` expressions inside string attributes
- Generate reactive wrapper for interpolated attributes
- Parse and evaluate the expression inside braces

**Files**: `src/parser.rs` (JSX attribute parsing), `src/js_emitter.rs` (attribute generation)

**Priority**: MEDIUM - Common pattern for dynamic styling

---

### **Issue #23-1: JSX Inside Lambda Return Statements Not Supported** 🔴 CRITICAL
**App**: 23-multiline-jsx
**Severity**: HIGH
**Category**: Parser / JSX

**Problem**:
```jounce
{items.value.map((item) => {
    return <p>Item: {item}</p>;
})}
```
**Error**: `ParserError { message: "Expected LAngle, found Colon", line: 7, column: 28 }`

**Root Cause**: Parser doesn't recognize JSX expressions inside return statements within lambdas

**Current Workaround**: Must use expression-body lambdas: `(item) => <p>{item}</p>` (but this also fails!)

**Actually**: Even `(item) => <p>{item}</p>` fails when inside JSX expressions!

**Fix Needed**:
- Track JSX context depth when parsing lambdas inside JSX
- Allow JSX parsing in lambda bodies when parent context is JSX
- Handle nested `{...}` expressions correctly

**Files**: `src/parser.rs` (JSX expression parsing, lambda parsing)

**Priority**: HIGH - Severely limits map/filter usage patterns

---

## 📊 SUMMARY

**Total Issues Found**: 5
- 🔴 **CRITICAL**: 2 (Component props, JSX in lambdas)
- 🟡 **MEDIUM**: 2 (Functions in components, String interpolation)
- 🟢 **LOW**: 1 (JSX text splitting)

**Apps Built**: 14 (Apps 12-25)
**Compile Success Rate**: 13/14 (92.9%)
**Failed**: 1 (App 23 - JSX in lambda)

**Most Critical**:
1. Issue #12-1: No component parameters
2. Issue #23-1: JSX inside lambdas broken

**Quick Wins**:
1. Issue #13-1: Support fn statements in components (easy)
2. Issue #13-2: Combine JSX text nodes (trivial)

**Medium Effort**:
1. Issue #20-1: String interpolation reactivity (moderate)

**Large Effort**:
1. Issue #12-1: Component props system (significant)
2. Issue #23-1: Nested JSX context tracking (complex)

---

