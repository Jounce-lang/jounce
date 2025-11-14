# Jounce Repository Consistency Pass 3: Parser Verification

**Date**: 2025-11-08
**Scope**: src/parser.rs, src/lexer.rs, src/errors.rs
**Status**: ✅ COMPLETE - All parser features verified

---

## Summary

Pass 3 verified that the Jounce parser implementation matches the documented syntax in JOUNCE_SPEC.md v0.8.3. All required features are correctly implemented.

---

## Verification Checklist

### ✅ 1. Lowercase Event Handlers (onclick, oninput, onchange)

**Requirement**: Parser must accept lowercase DOM-style event handlers, not React-style camelCase.

**Finding**: ✅ VERIFIED
- **Location**: src/parser.rs:2745-2806 `parse_jsx_attribute()`
- **Implementation**: Parser accepts any identifier as attribute name without validation
- **Evidence**: Event handlers are tokenized as `TokenKind::Identifier` by lexer
- **Test**: Compiled `/tmp/test_lowercase_events.jnc` successfully
- **Output**: Generated `{ onclick: handleClick }`, `{ oninput: (e) => ... }`, `{ onchange: (e) => ... }`

**Conclusion**: Parser does not enforce specific event handler names, correctly allowing both lowercase and camelCase. Code generation properly outputs lowercase events.

---

### ✅ 2. Arrow Functions in Component Bodies

**Requirement**: Components must support `let name = () => { ... };` syntax in component bodies.

**Finding**: ✅ VERIFIED
- **Location**:
  - Component body parsing: src/parser.rs:782-844 `parse_component_definition()`
  - Let statement parsing: src/parser.rs:1077-1095 `parse_let_statement()`
  - Arrow function parsing: src/parser.rs:1831-2008 `parse_lambda_or_grouped()`
  - Lambda body parsing: src/parser.rs:2010-2021 `parse_lambda_body()`

**Flow**:
1. Component body parses statements (line 819: `statements.push(self.parse_statement()?)`)
2. Let statement parses expression (line 1094: `let value = self.parse_expression(Precedence::Lowest)?`)
3. Expression parser handles lambda syntax (line 1838: `if self.consume_if_matches(&TokenKind::FatArrow)`)
4. Lambda body supports both block `{ ... }` and expression forms

**Test**: Compiled component with `let handleClick = () => { count.value = count.value + 1; };`

**Conclusion**: Full arrow function support in component bodies.

---

### ✅ 3. Explicit Return Statements

**Requirement**: Components must support explicit `return <div>...</div>;` syntax.

**Finding**: ✅ VERIFIED
- **Location**: src/parser.rs:823-836 (component body auto-conversion)
- **Implementation**: Parser accepts both explicit returns AND auto-converts implicit JSX expressions
- **Code**:
```rust
// Auto-convert implicit JSX returns:
if let Some(last_stmt) = statements.last_mut() {
    if let Statement::Expression(expr) = last_stmt {
        if matches!(expr, Expression::JsxElement(_)) {
            *last_stmt = Statement::Return(ReturnStatement {
                value: expr.clone(),
            });
        }
    }
}
```

**Conclusion**: Explicit returns work, plus parser provides helpful auto-conversion for implicit returns.

---

### ✅ 4. E_STY_001 Error Code for Style Errors

**Requirement**: Style parsing errors must use domain-specific error code E_STY_001 with helpful diagnostics.

**Finding**: ✅ VERIFIED
- **Location**:
  - Helper function: src/parser.rs:4235-4243 `style_error()`
  - Error type: src/errors.rs:17-23 `CompileError::StyleError`
  - Diagnostics: src/errors.rs:67-78 `to_diagnostic()`

**Implementation**:
```rust
fn style_error(&self, message: &str) -> CompileError {
    CompileError::StyleError {
        message: message.to_string(),
        line: self.current_token().line,
        column: self.current_token().column,
        code: "E_STY_001".to_string(),
    }
}
```

**Usage**: 15+ occurrences throughout style parsing (lines 4359, 4371, 4376, 4388, 4416, 4432, 4448, 4458, 4469, 4513, 4525, 4530, 4542, 4624, 4637)

**Test**: Compiled `tests/styles/nested_invalid.jnc` with deeply nested selectors
**Output**:
```
error: unsupported or malformed nested style rule
  --> tests/styles/nested_invalid.jnc:11:10
   10 |         color: red;
   11 |         &:hover {
      |              ^
   12 |             color: blue;
  [E_STY_001]
  help: Jounce currently supports one level of selector nesting inside `style <Component> { ... }`
  note: See docs/guides/SYNTAX_LIMITATIONS.md for current CSS syntax
```

**Conclusion**: E_STY_001 fully implemented with excellent diagnostic output.

---

### ✅ 5. One Level of Style Nesting

**Requirement**: Style blocks should support one level of nesting (e.g., `&:hover { ... }`) but reject deeper nesting.

**Finding**: ✅ VERIFIED
- **Location**: src/parser.rs:4247+ `parse_style_block()`
- **Implementation**: Parser validates nesting depth and returns E_STY_001 for violations
- **Test**: `tests/styles/nested_invalid.jnc` correctly rejects `&:hover { &:focus { ... } }`

**Conclusion**: Nesting validation correctly enforced.

---

## File Structure

**Discovered**: No separate jsx_parser.rs or style_parser.rs files
- All parsing logic consolidated in src/parser.rs (5044 lines)
- JSX parsing: lines 2700+
- Style parsing: lines 4235+
- Component parsing: lines 782+

---

## Tests Performed

### Test 1: Lowercase Event Handlers
```jounce
component EventTest() {
    let count = signal<i32>(0);
    let handleClick = () => { count.value = count.value + 1; };
    return <div>
        <button onclick={handleClick}>Click Me</button>
        <input type="text" oninput={(e) => console.log(e)} />
        <select onchange={(e) => console.log(e)}>
            <option>A</option>
        </select>
    </div>;
}
```
**Result**: ✅ Compiled successfully, generated correct lowercase events

### Test 2: Style Nesting Error
```jounce
component Box() {
    return <div class="box">Hi</div>;
}

style Box {
    .box {
        &:hover {
            &:focus {  // Too deep!
                color: green;
            }
        }
    }
}
```
**Result**: ✅ Correctly rejected with E_STY_001 error

---

## Key Parser Functions

| Function | Line | Purpose |
|----------|------|---------|
| `parse_component_definition()` | 782 | Parse component declarations |
| `parse_let_statement()` | 1077 | Parse let bindings |
| `parse_lambda_or_grouped()` | 1831 | Parse arrow functions |
| `parse_lambda_body()` | 2010 | Parse arrow function bodies |
| `parse_jsx_attribute()` | 2745 | Parse JSX attributes (including events) |
| `parse_style_block()` | 4247 | Parse style blocks |
| `style_error()` | 4235 | Create E_STY_001 errors |

---

## Recommendations

### ✅ No Changes Needed

The parser implementation is **fully aligned** with the documentation:
1. Event handlers accept any identifier (lowercase works)
2. Arrow functions fully supported in component bodies
3. Explicit returns supported with auto-conversion fallback
4. E_STY_001 error code implemented with excellent diagnostics
5. Style nesting properly validated

### Future Enhancements (Optional)

While not required for consistency, consider:
1. **Event name validation**: Add codegen warning if camelCase events detected (onClick → suggest onclick)
2. **Performance**: The parse_lambda_or_grouped() function is complex - could be optimized
3. **Error recovery**: Style parser could recover better from malformed CSS

---

## Conclusion

**Pass 3 Status**: ✅ COMPLETE

All parser features documented in JOUNCE_SPEC.md v0.8.3 are correctly implemented:
- Lowercase event handlers ✅
- Arrow functions in components ✅
- Explicit return statements ✅
- E_STY_001 error code ✅
- One-level style nesting ✅

**No parser changes required** - implementation matches specification perfectly.

---

**Verified By**: Claude (Automated Consistency Check)
**Next Steps**: Proceed to Pass 4 (src/ CLI/emitter/runtime files) when requested
