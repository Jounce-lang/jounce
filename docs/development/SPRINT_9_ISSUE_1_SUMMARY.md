# Sprint 9 - Issue #1: JSX Expressions in Elements - PARTIAL FIX ✅

**Date**: 2025-10-21
**Duration**: 2.5 hours
**Status**: Partial fix - basic use cases working, nested edge case remains

---

## Problem Statement

JSX expressions like `<span>{stock}</span>` were failing with error:
```
No prefix parse function for JsxText("stock}")
```

### Root Cause

**Critical timing issue with lexer state management:**

1. Parser buffers tokens in `current` and `peek` fields
2. When parsing `<span>{stock}`:
   - Parser is at `span`, peek is at `>`
   - Parser consumes `span`, now current = `>`, **peek is fetched**
   - At this point, lexer is NOT in JSX mode yet
   - Lexer tokenizes `{` as `LBrace` (not `JsxOpenBrace`)
   - Lexer does NOT increment `brace_depth`
3. Parser enters JSX mode (sets `jsx_mode = true`, `jsx_depth = 1`)
4. Parser consumes `>`, now current = `{` (LBrace)
5. Parser calls `parse_jsx_children()`
6. Next token fetch from lexer:
   - Lexer checks: `jsx_mode && jsx_depth > 0 && brace_depth == 0`
   - **All true!** So lexer reads `stock}` as JSX text
   - This is wrong - it should parse `stock` as identifier

**The Bug**: `brace_depth` should be 1 (we're inside `{}`), but it's 0 because the `{` was tokenized as `LBrace` before JSX mode was active.

---

## Solution

**Manual `brace_depth` management in parser:**

### Files Modified

**src/lexer.rs** (+8 lines):
```rust
pub fn increment_brace_depth(&mut self) {
    self.brace_depth += 1;
}

pub fn decrement_brace_depth(&mut self) {
    if self.brace_depth > 0 {
        self.brace_depth -= 1;
    }
}
```

**src/parser.rs** (+9 lines, in `parse_jsx_opening_tag_with_mode_check`):
```rust
if self.current_token().kind == TokenKind::RAngle {
    self.lexer.enter_jsx_mode();

    // CRITICAL BUG FIX: If peek token is LBrace, manually increment brace_depth
    // LIMITATION: Only works for root-level JSX elements
    if !was_jsx_mode && self.peek_token().kind == TokenKind::LBrace {
        self.lexer.increment_brace_depth();
    }
}
```

### How It Works

1. After entering JSX mode, check if peek token is `LBrace`
2. If yes, manually increment `brace_depth` to compensate
3. This ensures lexer knows we're inside `{}` and won't read JSX text
4. **Important**: Only do this for root-level JSX elements (`!was_jsx_mode`)

---

## What Works ✅

### Test Case 1: Standalone JSX Expressions
```raven
let result = <span>{stock}</span>;
// ✅ WORKS
```

### Test Case 2: JSX Expression with Text
```raven
let result = <span>{stock} in stock</span>;
// ✅ WORKS
```

### Test Case 3: Ternary with JSX (Not Nested)
```raven
let result = in_stock ?
    <span>{stock} in stock</span> :
    <span>Out of stock</span>;
// ✅ WORKS
```

### Test Case 4: Field Access in JSX
```raven
struct Product { stock: i32 }
let product = Product { stock: 5 };
let result = <span>{product.stock} items</span>;
// ✅ WORKS
```

---

## Known Limitation ❌

### Nested JSX Inside JSX Expressions

```raven
// ❌ FAILS
return <div>
    {in_stock ?
        <span>{stock} in stock</span> :
        <span>Out of stock</span>
    }
</div>;
```

**Error**: `No prefix parse function for JsxText("in_stock ?")`

### Why It Fails

For nested JSX elements:
```
<div>                           // JSX element 1 (was_jsx_mode = false)
  {                             // Peek token buffered as LBrace
    <span>                      // JSX element 2 (was_jsx_mode = true!)
      {stock}                   // Another LBrace, but we don't increment for nested
    </span>
  }
</div>
```

The fix only increments `brace_depth` for root elements (`!was_jsx_mode`). For nested elements, the condition is false, so we don't increment, and the same bug occurs.

### Workaround

**Use ternary at statement level:**
```raven
// Instead of nested:
// <div>{cond ? <span>{x}</span> : <span>y</span>}</div>

// Use statement-level ternary:
let content = cond ? <span>{x}</span> : <span>y</span>;
return <div>{content}</div>;
```

---

## Why Full Fix Is Complex

A complete fix requires **architectural changes** to lexer/parser interaction:

### Option 1: Two-Pass Tokenization
- First pass: Parse structure, determine JSX regions
- Second pass: Tokenize with correct modes
- **Complexity**: High, requires major refactor

### Option 2: Backtracking Tokenizer
- Re-tokenize when entering JSX mode
- Reset lexer position and re-scan
- **Complexity**: Medium, but impacts performance

### Option 3: Token Stream Transformation
- Post-process token stream to convert `LBrace` → `JsxOpenBrace` in JSX regions
- **Complexity**: Medium, but requires tracking context

All options require 4-8 hours of work and risk breaking existing functionality.

---

## Test Results

- ✅ **All 221 tests passing** (0 failures, 9 ignored)
- ✅ **All 11 JSX parser tests passing**
- ✅ **No regressions**

---

## Recommendation

**Accept partial fix** for now because:
1. **Most common use case works**: `<span>{expr}</span>` pattern
2. **Simple workaround exists**: Use statement-level ternary
3. **Time investment**: 2.5 hours already spent
4. **Risk vs reward**: Full fix risks breaking working features

**Future improvement**: Tag as "enhancement" for v0.2.0 when doing larger refactors.

---

## Examples of Working Patterns

```raven
// ✅ Direct expression
<div>{user.name}</div>

// ✅ Expression with text
<span>{count} items</span>

// ✅ Ternary (not nested)
let badge = in_stock ? <span class="green">Available</span> : <span class="red">Sold Out</span>;
return <div>{badge}</div>;

// ✅ Multiple expressions
<div>
    <span>{first_name}</span>
    <span>{last_name}</span>
</div>

// ✅ Nested elements (no nested expressions)
<div>
    <span>Static: {value}</span>
</div>
```

---

## Impact on Example Apps

Most real-world patterns are covered. The nested ternary pattern can be refactored using the workaround.

**Next Steps**: Test ecommerce app to see if this fix unblocks it.
