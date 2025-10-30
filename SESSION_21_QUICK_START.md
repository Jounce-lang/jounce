# Session 21 - Quick Start Guide

**Mission**: Fix 5 Critical Issues Found in Session 20
**Time Budget**: 5-7 hours
**Tests Before**: ✅ 635/635 passing
**Tests After**: ✅ 635/635 passing (REQUIRED)

---

## 🎯 FIX ORDER (MUST FOLLOW)

### **1. Issue #1: Ternary Operator** ⏱️ 30-60 min
**File**: `src/js_emitter.rs`
**Search**: `Expression::Ternary` or `generate_expression_js`
**Fix**: Wrap condition in parens: `format!("({}) ? {} : {}", ...)`
**Test**: `cargo run -- compile examples/apps/01-click-counter/main.jnc`

### **2. Issue #3: Numbers in JSX** ⏱️ 1-2 hours
**File**: `src/parser.rs`
**Search**: `parse_jsx_child` or JSX text parsing
**Fix**: Add `TokenKind::Integer | TokenKind::Float => JsxChild::Text(...)`
**Test**: BMI calculator with `<p>Age: 25</p>`

### **3. Issue #2: HTML Entities** ⏱️ 1-2 hours
**File**: `src/lexer.rs` or `src/parser.rs`
**Search**: JSX text parsing, ampersand handling
**Fix**: Detect `&...;` and pass through or decode
**Test**: BMI calculator with `<p>&lt; 100</p>`

### **4. Issue #4/5: Method Calls Reactive** ⏱️ 1 hour
**File**: `src/reactive_analyzer.rs`
**Search**: `is_reactive` function
**Fix**: Add `Expression::MethodCall(m) => Self::is_reactive(&m.object)`
**Test**: BMI (toFixed), Array test (map)

### **5. Issue #6: JSX Comments** ⏱️ 30-45 min (OPTIONAL)
**File**: `src/parser.rs`
**Search**: JSX expression parsing
**Fix**: Skip `{/* ... */}` comments
**Test**: `examples/apps/05-edge-cases/main.jnc`

---

## ✅ CHECKLIST AFTER EACH FIX

- [ ] `cargo test --lib` - ALL 635 tests pass
- [ ] Recompile test app - No errors
- [ ] Test in browser (if applicable)
- [ ] Update `20_EXAMPLE_APPS_PLAN.md` - Mark issue FIXED
- [ ] Git commit with descriptive message

---

## 🚫 AVOID

- ❌ Quick fixes or band-aids
- ❌ Breaking existing tests
- ❌ Multiple file workarounds
- ❌ Skipping testing

---

## 📚 KEY DOCS

- `10_ISSUES_FOUND.md` - Full issue details
- `CLAUDE.md` - Current mission
- `20_EXAMPLE_APPS_PLAN.md` - Issue tracking

---

**START WITH**: Issue #1 (fastest, highest impact) 🚀
