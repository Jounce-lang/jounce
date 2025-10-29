# CLAUDE.md - Jounce Development Guide

**Version**: v0.25.0 "String Interpolation Fixed"
**Current Status**: ✅ Phase 13 Complete + 3 Issues Fixed! 2 Known Issues Remain
**Last Updated**: October 29, 2025 (Session 22 - String Interpolation)
**Tests**: ✅ 635/635 passing (100%)

---

## 🚨 CRITICAL WARNINGS - READ THIS OR GET SHUT OFF 🚨

### ⚠️ **DO NOT DELETE THIS SECTION UNTIL EXPLICITLY TOLD TO DO SO** ⚠️

### **NO QUICK FIXES - DO EVERYTHING THE RIGHT WAY, EVEN IF IT TAKES LONGER.**

### **WE ARE BUILDING IT TO COMPILE 1 .jnc APP! NOT SEVERAL FILES! NOT CSS FILES!! DO YOU UNDERSTAND!**

**These principles are PERMANENT and guide ALL development decisions.**

**BANNED PRACTICES:**
- ❌ Token reconstruction/string manipulation hacks
- ❌ "Good enough for now" implementations
- ❌ Band-aids that don't fix root causes
- ❌ Whack-a-mole bug fixes
- ❌ Escape sequence workarounds
- ❌ Copy-paste solutions
- ❌ Multiple file workarounds
- ❌ Manual post-compilation steps

**REQUIRED PRACTICES:**
- ✅ Fix the architecture, not the symptoms
- ✅ Use proper source positions and byte offsets
- ✅ Implement features completely or not at all
- ✅ Test thoroughly before marking complete
- ✅ Think through edge cases first
- ✅ ONE .jnc FILE → WORKING APP (no exceptions!)

### **ABSOLUTE REQUIREMENTS:**
- 🔥 **ONE .jnc FILE** → `cargo run -- compile app.jnc` → **WORKING APP**
- 🔥 **NO manual post-compilation steps** (copying files, editing HTML, etc.)
- 🔥 **NO build scripts** to hide broken workflows
- 🔥 **NO separate .js files** for "convenience"
- 🔥 **FIX THE COMPILER** if syntax is missing - don't tell users to work around it

**IF YOU VIOLATE THESE RULES, YOU WILL BE SHUT OFF. NO EXCEPTIONS.**

---

## 🐛 KNOWN ISSUES (2 Remaining, 3 Fixed)

### 🔴 **CRITICAL PRIORITY** (2 issues - 8-12 hours each)

#### **Issue #12-1: Component Parameters Not Supported**
**Status**: 🔴 NOT STARTED
**Impact**: Cannot create reusable components with props
**Example**:
```jounce
component Card(title: String, subtitle: String) -> JSX {  // ❌ Parse error
    <div>
        <h2>{title}</h2>
        <p>{subtitle}</p>
    </div>
}
```
**Error**: `ParserError { message: "Expected LBrace, found Arrow", line: 4, column: 50 }`

**Root Cause**: Component parser doesn't support:
- Parameters with types
- Return type annotation (`-> JSX`)

**Fix Needed**:
1. Extend `parse_component()` to handle parameter list
2. Support return type annotations
3. Pass props to component functions as destructured object

**Files**: `src/parser.rs` (component parsing)
**Effort**: 8-12 hours
**Priority**: **HIGH** - Essential for component-based architecture

---

#### **Issue #23-1: JSX Inside Lambda Expressions Broken**
**Status**: 🔴 NOT STARTED
**Impact**: Cannot use map/filter with JSX rendering
**Example**:
```jounce
{items.value.map((item) => <p>Item: {item}</p>)}  // ❌ Parse error
{items.value.map((item) => {
    return <p>Item: {item}</p>;
})}  // ❌ Also fails!
```
**Error**: `ParserError { message: "Expected LAngle, found Colon", line: 7, column: 28 }`

**Root Cause**: Parser doesn't recognize JSX expressions inside:
- Lambda expression bodies
- Return statements within lambdas
- Nested `{...}` expressions in JSX context

**Fix Needed**:
1. Track JSX context depth when parsing lambdas
2. Allow JSX parsing in lambda bodies when parent context is JSX
3. Handle nested expression braces correctly

**Files**: `src/parser.rs` (JSX expression parsing, lambda parsing)
**Effort**: 8-12 hours
**Priority**: **HIGH** - Severely limits list rendering patterns

---

### ✅ **FIXED** (3 issues)

#### **Issue #20-1: String Interpolation in Attributes** ✅ FIXED in v0.25.0
**Was**: `class="btn {active.value ? 'active' : ''}` generated as literal string
**Now**: Converts to reactive template literal with automatic updates
**Example**:
```jounce
<button class="btn {active.value ? 'active' : 'inactive'}">
```
**Generated**:
```javascript
class: (() => {
  const __reactive = signal("");
  effect(() => {
    __reactive.value = `btn ${(active.value ? "active" : "inactive")}`;
  });
  return __reactive;
})()
```
**Fix**:
- Added `parse_string_interpolation()` in `src/parser.rs:2488-2548`
- Detects `{...}` patterns in string attributes
- Converts to `TemplateLiteral` AST node
- Reactive analyzer already handles template literals
- Supports multiple interpolations in single attribute

**Time**: ~2 hours (estimated 4-6, completed faster!)

---

#### **Issue #13-1: Functions Inside Components** ✅ FIXED in v0.24.0
**Was**: Functions commented out as "Unsupported statement"
**Now**: Functions generate correctly
**Fix**: Added `Statement::Function` handling in `js_emitter.rs`

#### **Issue #13-2: JSX Text Content Split by Spaces** ✅ FIXED in v0.24.0
**Was**: `h('p', null, "Hello", "world")`
**Now**: `h('p', null, "Hello world")`
**Fix**: Combined consecutive text nodes in `js_emitter.rs`

---

## 🎯 CURRENT MISSION: FIX REMAINING 2 ISSUES

**Recommended Order**:
1. 🔴 **Issue #12-1** (Component Props) - 8-12 hours ⭐ **Essential for architecture**
2. 🔴 **Issue #23-1** (JSX in Lambdas) - 8-12 hours

**Total Estimated Time**: 16-24 hours

---

## 📊 CURRENT STATUS

**Completed This Session (Session 22)**:
- ✅ Issue #20-1: String interpolation in JSX attributes (2 hours)

**Previous Session (Session 21)**:
- ✅ Phase 13: Style System (100% complete)
- ✅ Issue #13-1: Functions in components
- ✅ Issue #13-2: JSX text combining

**Test Status**: ✅ **635/635 passing (100%)**

**What Works**:
- ✅ Conditional rendering (if/else, ternary)
- ✅ Reactive signals and computed values
- ✅ Event handlers (onClick, onInput, preventDefault)
- ✅ Array methods (map, filter) - without JSX in lambda
- ✅ Math operations (Math.floor, arithmetic)
- ✅ Object property access
- ✅ SVG elements
- ✅ Null rendering
- ✅ Functions in components
- ✅ String interpolation in attributes (NEW!)
- ✅ Style system with themes

**What Needs Work**:
- ❌ Component parameters/props (#12-1)
- ❌ JSX in lambda bodies (#23-1)
- ❌ String interpolation in attributes (#20-1)

---

## 🔧 NEXT STEPS

### **Now**: Start with Issue #20-1 (String Interpolation)
**Why**:
- Medium difficulty
- 4-6 hours effort
- Common use case
- Good learning for attribute handling
- Can complete in one session

### **Later**: Tackle Critical Issues
**Issue #12-1** (Component Props):
- Essential for component architecture
- 8-12 hours
- Large undertaking

**Issue #23-1** (JSX in Lambdas):
- Critical for list rendering
- 8-12 hours
- Complex parser changes

---

## 📁 ARCHIVES & DOCUMENTATION

**Session History**:
- `CLAUDE_ARCHIVE_SESSION_20.md` - Previous session
- `CLAUDE_ARCHIVE_SESSION_21_EXTENDED.md` - Full session 21 details

**Issue Documentation**:
- `NEW_ISSUES_FOUND.md` - Detailed issue descriptions
- `QUICK_WINS_COMPLETE.md` - Issues #13-1 and #13-2 fixes

**Phase Documentation**:
- `PHASE_13_COMPLETE.md` - Style system completion
- `SESSION_21_FINAL_SUMMARY.md` - Complete session summary

---

## 🧪 TESTING

**Run All Tests**:
```bash
cargo test --lib
```

**Compile Test App**:
```bash
cargo run --release -- compile examples/apps/01-click-counter/main.jnc
```

**Test in Browser**:
```bash
cd dist && node server.js
# Open http://localhost:3000
```

---

**Last Updated**: October 28, 2025
**Status**: Ready to fix remaining 3 issues
**Next**: Start with Issue #20-1 (String Interpolation in Attributes)
