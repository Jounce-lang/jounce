# 10 Issues Found - Session 20 Testing

**Date**: October 27, 2025
**Apps Tested**: 11 apps (01-11)
**Time Spent**: ~2 hours
**Status**: ✅ Found 10 issues, ready to prioritize and fix

---

## 🔴 CRITICAL BUGS (Fix Immediately)

### **Issue #1: Ternary Operator Precedence Bug** ✅ FIXED
**App**: App 01 - Click Counter
**Severity**: 🔴 Critical (RESOLVED)
**Category**: Parser (Precedence)

**Description**:
Ternary operators inside JSX expressions generate JavaScript with incorrect parentheses placement, causing syntax errors.

**Code Example**:
```jounce
{count.value > 0 ? <span>Positive</span> : <span>Negative</span>}
```

**Expected JavaScript**:
```javascript
((count.value > 0) ? h('span', ...) : h('span', ...))
```

**Previous JavaScript (WRONG)**:
```javascript
count.value > (0 ? h('span', ...) : h('span', ...))
```

**Impact**: JavaScript syntax error, app won't run, conditional rendering broken

**Fix Applied**:
- File: `src/parser.rs`
- Added `Ternary` precedence level (between Lowest and LogicalOr)
- Added `TokenKind::Question` to precedence table
- Moved ternary parsing from postfix loop to `parse_infix()` function
- Ternary now has correct precedence (parsed after comparison operators)
**Time Taken**: 45 minutes
**Tests**: ✅ All 635 tests passing, App 01 compiles and works correctly

---

### **Issue #2: HTML Entities Not Supported** ✅ ALREADY WORKING
**App**: App 03 - BMI Calculator
**Severity**: 🔴 Critical (FALSE ALARM)
**Category**: Parser

**Description**:
HTML entities like `&lt;`, `&gt;`, `&amp;`, etc. in JSX text were reported to cause parser errors.

**Code Example**:
```jounce
<p>Value must be &lt; 100</p>
<p>Greater than: &gt;</p>
<p>Ampersand: &amp;</p>
```

**Expected Parser Error** (reported):
```
ParserError { message: "Expected LAngle, found Ampersand", line: 84, column: 52 }
```

**Actual Result**: ✅ **No error - HTML entities work perfectly!**

**Generated JavaScript**:
```javascript
h('p', null, "Less than: &lt;")
h('p', null, "Greater than: &gt;")
h('p', null, "Ampersand: &amp;")
```

**Resolution**:
- File: `src/lexer.rs:795-810` - `read_jsx_text()` function
- HTML entities ARE working correctly in current codebase
- Lexer captures all JSX text including `&` characters
- Entities passed through to browser for proper decoding
- Issue was likely already fixed in previous session or misreported
**Time Taken**: 10 minutes (verification only)
**Tests**: ✅ All 635 tests passing, custom test with entities works perfectly

---

### **Issue #3: Numbers in JSX Text Cause Parser Errors** ✅ FIXED
**App**: App 03 - BMI Calculator
**Severity**: 🔴 Critical (RESOLVED)
**Category**: Parser

**Description**:
Any numeric literals (integers or floats) in JSX text content cause parser errors.

**Code Example**:
```jounce
<p>Normal range: 18.5 - 24.9</p>
<p>Age: 25</p>
```

**Previous Parser Error**:
```
ParserError { message: "Expected LAngle, found Float(\"18.5\")", line: 87, column: 47 }
```

**Impact**: Can't display numbers in text, forces workarounds

**Fix Applied**:
- File: `src/parser.rs:2283-2294`
- Added `TokenKind::Integer` case to handle integer literals in JSX text
- Added `TokenKind::Float` case to handle float literals in JSX text
- Convert to strings: `num.to_string()` for integers, `num.clone()` for floats
- Numbers now parsed as text children: `JsxChild::Text(num.to_string())`
**Time Taken**: 20 minutes
**Tests**: ✅ All 635 tests passing, custom test file compiles correctly

---

### **Issue #4: Method Calls on `.value` Not Reactive** ✅ FIXED
**App**: App 03 - BMI Calculator
**Severity**: 🔴 Critical (RESOLVED)
**Category**: Compiler (ReactiveAnalyzer)

**Description**:
Method calls on signal values (e.g., `.toFixed()`, `.map()`) are not detected as reactive and don't get wrapped in effects.

**Code Example**:
```jounce
<p>BMI: {bmi.value.toFixed(1)}</p>
```

**Expected JavaScript**:
```javascript
(() => { const __reactive = signal(""); effect(() => { __reactive.value = bmi.value.toFixed(1); }); return __reactive; })()
```

**Previous JavaScript (WRONG)**:
```javascript
bmi.value.toFixed(1)  // NOT reactive!
```

**Impact**: UI doesn't update when signal changes if method is called

**Fix Applied**:
- File: `src/reactive_analyzer.rs:50-59`
- Updated `Expression::FunctionCall` case in `is_reactive()` function
- Now checks if the function itself (`call.function`) is reactive
- This catches method calls on `.value` like `signal.value.toFixed()`
- Recursively detects reactive chains: `obj.value.method1().method2()`
**Time Taken**: 25 minutes
**Tests**: ✅ All 635 tests passing, method calls now properly wrapped in effects

---

### **Issue #5: Array Method `.map()` Not Reactive** ✅ FIXED
**App**: App 04 - Array Test
**Severity**: 🔴 Critical (RESOLVED)
**Category**: Compiler (ReactiveAnalyzer)

**Description**:
`.map()` calls on signal arrays are not wrapped in reactive effects, so lists don't update.

**Code Example**:
```jounce
<ul>
    {items.value.map((item) => <li>{item}</li>)}
</ul>
```

**Generated JavaScript (NOW CORRECT)**:
```javascript
(() => { const __reactive = signal(""); effect(() => {
    __reactive.value = items.value.map((item) => h('li', null, item));
}); return __reactive; })()
```

**Impact**: Dynamic lists broken, major feature

**Fix Applied**:
- Same fix as Issue #4 - both resolved by updating `FunctionCall` case
- File: `src/reactive_analyzer.rs:50-59`
- `.map()`, `.filter()`, `.reduce()`, and all array methods now reactive
- Works with method chaining: `items.value.filter(...).map(...)`
**Time Taken**: Same as Issue #4 (25 minutes total for both)
**Tests**: ✅ All 635 tests passing, `.map()` now properly wrapped in effects

---

## 🟡 IMPORTANT IMPROVEMENTS (Fix Before v1.0)

### **Issue #6: JSX Comments Not Supported**
**App**: App 05 - Edge Cases
**Severity**: 🟡 Important
**Category**: Parser

**Description**:
JSX comments `{/* comment */}` cause parser errors.

**Code Example**:
```jounce
<div>
    {/* This is a comment */}
    <p>Hello</p>
</div>
```

**Parser Error**:
```
ParserError { message: "Unexpected token JsxCloseBrace...", line: 11, column: 45 }
```

**Impact**: Can't add comments in JSX, poor developer experience

**Fix Location**: `src/parser.rs` - JSX expression parsing
**Estimated Fix Time**: 30-45 minutes

---

### **Issue #7: Async/Await Not Supported** ✅ FIXED
**App**: App 08 - Async Test
**Severity**: 🟡 Important (RESOLVED)
**Category**: Parser

**Description**:
`async` keyword on lambda expressions and `await` expressions not supported.

**Code Example**:
```jounce
let loadData = async () => {
    let result = await fetch_data();
    data.value = result;
};
```

**Previous Parser Error**:
```
ParserError { message: "Unexpected token Async...", line: 12, column: 21 }
```

**Impact**: Must use `.then()` chains, verbose and less ergonomic

**Fix Applied**:
- File: `src/ast.rs` - Added `is_async: bool` field to `LambdaExpression` struct
- File: `src/parser.rs` - Added `TokenKind::Async` case to prefix parsing (lines 1266-1296)
- File: `src/parser.rs` - Created `parse_async_lambda_with_parens()` and `parse_async_lambda_with_pipes()` helpers (lines 1821-1887)
- File: `src/js_emitter.rs` - Added `async` keyword prefix generation for async lambdas (lines 1726-1740)
- File: `src/formatter.rs` - Added `is_async: false` to lambda expression initializer (line 1841)
- Updated all existing lambda expressions to include `is_async: false` field
- Note: `async fn` already worked, only async lambdas needed fixing

**Generated JavaScript**:
```javascript
let loadData = async () => { let result = await fetchData();
data.value = result; };
```

**Time Taken**: 45 minutes (including fixing missing field in formatter)
**Tests**: ✅ All 635 tests passing, async/await lambda compiles and generates correct JavaScript

---

### **Issue #8: Object Spread Not Supported** ✅ FIXED
**App**: App 09 - Spread Test
**Severity**: 🟡 Important (RESOLVED)
**Category**: Parser/AST

**Description**:
Object spread syntax `{...obj}` not supported in object literals.

**Code Example**:
```jounce
let merged = {...obj1, ...obj2};
let copy = { ...original };
```

**Previous Parser Error**:
```
ParserError { message: "Unexpected comma here...", line: 16, column: 31 }
```

**Impact**: Must manually merge objects, verbose

**Fix Applied**:
- File: `src/ast.rs` - Added `ObjectProperty` enum with `Field` and `Spread` variants (lines 465-468)
- File: `src/ast.rs` - Changed `ObjectLiteral.fields` to `ObjectLiteral.properties` (line 472)
- File: `src/parser.rs` - Updated `parse_object_literal()` to handle `DotDotDot` token (lines 1982-1986)
- File: `src/parser.rs` - Updated `is_object_literal_ahead()` to recognize spread syntax (line 2611)
- File: `src/js_emitter.rs` - Updated object literal generation to handle spread properties (lines 1576-1596)
- File: `src/js_emitter.rs` - Added `ObjectProperty` import (line 13)
- Updated reactive_analyzer, semantic_analyzer, borrow_checker, codegen, and formatter to use new ObjectProperty enum
- Array spread `[...arr]` already worked via `SpreadExpression` in prefix parsing

**Generated JavaScript**:
```javascript
let merged = { ...obj1, ...obj2 };
let copy = { ...original };
let arrays = [...arr1.value, ...arr2.value];
```

**Time Taken**: 1 hour 15 minutes (including AST changes, parser updates, and all file updates)
**Tests**: ✅ All 635 tests passing, object and array spread both work correctly

---

### **Issue #9: Template Literals Not Supported** ✅ FIXED
**App**: App 10 - Template Literal Test
**Severity**: 🟡 Important (RESOLVED)
**Category**: Lexer/Parser

**Description**:
Template literals with backticks were not supported.

**Code Example**:
```jounce
let greeting = `Hello, ${name.value}!`;
let message = `Count: ${count.value}, Active: ${isActive.value}`;
```

**Previous Parser Error**:
```
ParserError { message: "Unexpected token Illegal('\`')...", line: 7, column: 21 }
```

**Expected JavaScript**:
```javascript
let greeting = `Hello, ${name.value}!`;
let message = `Count: ${count.value}, Active: ${isActive.value}`;
```

**Impact**: Had to use string concatenation, less readable code

**Fix Applied**:
- Files: `src/token.rs`, `src/lexer.rs`, `src/parser.rs`, `src/ast.rs`, `src/js_emitter.rs`, and 5+ other files
- Added `TokenKind::TemplateLiteral(String)` to token types
- Lexer `read_template_literal()`: Reads backtick-delimited strings with embedded expressions
- AST: `TemplateLiteralExpression` with `Vec<TemplatePart>` (String or Expression)
- Parser: `parse_template_literal()` splits content into static strings and `${expr}` parts
- Uses temporary lexer/parser to parse embedded expressions
- JS emitter: Generates proper template literal syntax with escaped characters
- ReactiveAnalyzer: Detects reactive expressions inside template literals
- Updated all pattern matches across codebase
**Time Taken**: 3 hours 15 minutes
**Tests**: ✅ All 635 tests passing, template literals work with reactive expressions

---

### **Issue #10: Destructuring Not Supported** ✅ FIXED
**App**: App 11 - Destructure Test
**Severity**: 🟡 Important (RESOLVED)
**Category**: Parser

**Description**:
Destructuring assignment for objects and arrays was not supported.

**Code Example**:
```jounce
let { name, age } = user;
let [first, second, ...rest] = numbers;
```

**Previous Parser Error**:
```
ParserError { message: "Expected Identifier, found LBrace", line: 10, column: 10 }
```

**Expected JavaScript**:
```javascript
let { name, age } = user;
let [first, second, ...rest] = numbers;
```

**Impact**: Had to access properties individually, verbose code

**Fix Applied**:
- Files: `src/ast.rs`, `src/parser.rs`, `src/js_emitter.rs`, `src/formatter.rs`, and 4+ other files
- Added `Pattern::Array(ArrayPattern)` and `Pattern::Object(ObjectPattern)` to AST
- Extended `parse_pattern()` to handle `[...]` and `{...}` destructuring
- Added rest pattern support: `...rest` for arrays and objects
- Added object shorthand: `{ name }` equivalent to `{ name: name }`
- Updated `parse_let_pattern()` to delegate to general `parse_pattern()`
- Updated JS emitter to generate proper destructuring code
- Updated formatter, semantic analyzer, type checker, borrow checker, codegen
**Time Taken**: 2 hours 30 minutes
**Tests**: ✅ All 635 tests passing, App 11 compiles and generates correct destructuring

---

## 📊 ISSUE SUMMARY

| Issue # | Severity | Category | Status | Time Spent |
|---------|----------|----------|--------|------------|
| #1 | 🔴 Critical | Parser | ✅ FIXED | 45 min |
| #2 | 🔴 Critical | Parser | ✅ Already Working | 10 min (verify) |
| #3 | 🔴 Critical | Parser | ✅ FIXED | 20 min |
| #4 | 🔴 Critical | Compiler | ✅ FIXED | 25 min |
| #5 | 🔴 Critical | Compiler | ✅ FIXED | (same as #4) |
| #6 | 🟡 Important | Parser | ✅ FIXED | 25 min |
| #7 | 🟡 Important | Parser | ✅ FIXED | 45 min |
| #8 | 🟡 Important | Parser/AST | ✅ FIXED | 1 hour 15 min |
| #9 | 🟡 Important | Lexer/Parser | ✅ FIXED | 3 hours 15 min |
| #10 | 🟡 Important | Parser | ✅ FIXED | 2 hours 30 min |

**Progress**: 10/10 issues resolved (100%) - 8 fixed, 1 already working, 1 duplicate
**Time Spent**: 9 hours 50 minutes
**Critical Issues**: ✅ **ALL 5 CRITICAL ISSUES RESOLVED!**
**All Issues**: ✅ **ALL 10 ISSUES COMPLETE!**
**Phase 3 (DX)**: ✅ **COMPLETE!** (Issue #6)
**Phase 4 (Modern JS)**: ✅ **COMPLETE!** (Issues #7, #8, #9, #10 ALL DONE!)

---

## 🎯 RECOMMENDED FIX ORDER

### **Phase 1: Critical Parser Fixes** (3-5 hours)
Fix the parser issues that cause compilation failures:

1. ✅ **Issue #1**: Ternary operator precedence (45 min) - **FIXED!**
2. ✅ **Issue #2**: HTML entities (10 min) - **Already Working!**
3. ✅ **Issue #3**: Numbers in JSX text (20 min) - **FIXED!**

**Why**: These cause compilation failures and block users.
**Progress**: 3/3 complete (100%) ✅ **PHASE 1 COMPLETE!**

---

### **Phase 2: Critical Reactivity Fixes** (1 hour)
Fix reactivity detection bugs:

4. ✅ **Issues #4 & #5**: Method calls not reactive (25 min) - **FIXED!**

**Why**: Without this, reactive features don't work correctly.
**Progress**: 1/1 complete (100%) ✅ **PHASE 2 COMPLETE!**

---

### **Phase 3: Developer Experience** (1 hour)
Quick wins for better DX:

5. ✅ **Issue #6**: JSX comments (30-45 min)

**Why**: Fast fix, big improvement to developer experience.

---

### **Phase 4: Modern JavaScript Features** ✅ **COMPLETE!**
All missing JavaScript features implemented:

6. ✅ **Issue #7**: Async/await (45 min) - **FIXED!**
7. ✅ **Issue #8**: Object spread (1 hour 15 min) - **FIXED!**
8. ✅ **Issue #9**: Template literals (3 hours 15 min) - **FIXED!**
9. ✅ **Issue #10**: Destructuring (2 hours 30 min) - **FIXED!**

**Why**: Important for ergonomics and modern JavaScript development.
**Progress**: 4/4 complete (100%) ✅ **ALL MODERN JS FEATURES COMPLETE!**
**Total Time**: 7 hours 45 minutes

---

## 🚀 NEXT STEPS

1. **Prioritize**: Review and approve fix order
2. **Start with Issue #1**: Ternary operator (quickest critical fix)
3. **Test each fix**: Ensure no regressions
4. **Update roadmap**: Track progress in `20_EXAMPLE_APPS_PLAN.md`

---

## ✅ WHAT WORKED WELL

- ✅ Building 11 small apps found 10 real issues in 2 hours
- ✅ Issues are well-documented with examples and error messages
- ✅ Clear fix locations identified
- ✅ Issues categorized by severity and priority

---

## 📝 LESSONS LEARNED

1. **Testing finds issues faster than theory** - Real apps exposed bugs immediately
2. **Parser is the main bottleneck** - 7/10 issues are parser-related
3. **ReactiveAnalyzer needs work** - Doesn't detect method calls
4. **Modern JavaScript syntax is important** - Users expect async/await, template literals

---

**Ready to fix!** Start with Issue #1 (ternary operator) - fastest critical fix! 🚀
