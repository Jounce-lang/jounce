# CLAUDE.md - Jounce Development Guide

**Version**: v0.8.1 "Developer Experience & Public Launch"
**Current Status**: ✅ Ready for Public Launch! All Issues Fixed! Community Files Complete!
**Last Updated**: October 31, 2025 (Public Launch Preparation)
**Tests**: ✅ 635/635 passing (100%)

---

## 🚨 CRITICAL WARNINGS - READ THIS OR GET SHUT OFF 🚨

### ⚠️ **DO NOT DELETE THIS SECTION UNTIL EXPLICITLY TOLD TO DO SO** ⚠️

### **NO QUICK FIXES - DO EVERYTHING THE RIGHT WAY, EVEN IF IT TAKES LONGER.**

### **WE ARE BUILDING IT TO COMPILE 1 .jnc APP! NOT SEVERAL FILES! NOT CSS FILES!! DO YOU UNDERSTAND!**

### **ONLY EVER PUSH TO jounce-pre-production GITHUB REPO. NEVER EVER COMMIT OR PUSH TO jounce GITHUB REPO.**

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

## 🎉 ALL KNOWN ISSUES FIXED! (5/5 Complete - 100%)

**ZERO CRITICAL ISSUES REMAINING!**

All issues from Session 21's discovery phase have been successfully fixed!

---

## 📜 ISSUE HISTORY

### ✅ **ALL FIXED** (5 issues)

#### **Issue #23-1: JSX Inside Lambda Expressions** ✅ FIXED in v0.8.0
**Was**: JSX in lambda blocks with text content failed to parse
**Now**: Full JSX support in lambda bodies
**Example**:
```jounce
{items.value.map((item) => {
    return <p>Item: {item}</p>;  // ✅ Now works!
})}
```
**Generated**:
```javascript
items.value.map((item) => {
  return h('p', null, "Item:", item);
})
```
**Root Cause**: Lexer entered JSX mode AFTER consuming `<` token, so lookahead was tokenized in normal mode. Content like "Item:" had the colon tokenized as `TokenKind::Colon` instead of JSX text.

**Fix**:
- Moved `lexer.enter_jsx_mode()` call BEFORE consuming `<` token in `src/parser.rs:2381-2392`
- This ensures lookahead tokens are fetched in JSX mode
- Fixes all JSX text content with special characters (colons, etc.)

**Time**: ~30 minutes (estimated 8-12 hours, root cause was simple!)

---

#### **Issue #12-1: Component Return Type Annotations** ✅ FIXED in v0.8.0
**Was**: `component Card() -> JSX { ... }` caused parse error
**Now**: Optional return types supported
**Example**:
```jounce
component Card(title: String, subtitle: String) -> JSX {
    <div>
        <h2>{title}</h2>
        <p>{subtitle}</p>
    </div>
}
```
**Generated**:
```javascript
export function Card({ title, subtitle } = {}) {
  return h('div', null, h('h2', null, title), h('p', null, subtitle));
}
```
**Fix**:
- Added optional return type parsing in `src/parser.rs:648-653`
- Return type is parsed but not stored (components always return JSX)
- Syntax compatibility for cleaner component definitions

**Note**: Component parameters already worked! Issue was only the `-> JSX` syntax.

**Time**: ~10 minutes (estimated 8-12 hours, was simpler than expected!)

---

#### **Issue #20-1: String Interpolation in Attributes** ✅ FIXED in v0.8.0
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

#### **Issue #13-1: Functions Inside Components** ✅ FIXED in v0.8.0
**Was**: Functions commented out as "Unsupported statement"
**Now**: Functions generate correctly
**Fix**: Added `Statement::Function` handling in `js_emitter.rs`

#### **Issue #13-2: JSX Text Content Split by Spaces** ✅ FIXED in v0.8.0
**Was**: `h('p', null, "Hello", "world")`
**Now**: `h('p', null, "Hello world")`
**Fix**: Combined consecutive text nodes in `js_emitter.rs`

---

## 🎯 CURRENT STATUS: ALL ISSUES FIXED! 🎉

**ALL 5 CRITICAL ISSUES FROM SESSION 21 HAVE BEEN RESOLVED!**

**Total Time Invested**: ~3 hours (estimated 32-48 hours!)
**Efficiency**: 90%+ faster than expected!

**No known critical bugs remaining!**

---

## 📊 CURRENT STATUS - PHASE 17: SECURITY & PRODUCTION

**Active Phase**: Phase 17 - Security & Production Features
**Current Feature**: Security Middleware Generation (Feature 1/3)
**Progress**: Planning complete, ready to implement
**Last Updated**: November 1, 2025
**Test Status**: ✅ **635/635 passing (100%)**

### **Phase 17 Implementation Status**

**Feature 1: Security Middleware Generation** (0/4 steps complete)
- ⏳ Step 1: Security Runtime Library (`runtime/security.js`)
- ⏳ Step 2: Middleware Generation in Emitter
- ⏳ Step 3: Runtime Import Generation
- ⏳ Step 4: Integration Testing
- **Estimate**: 8-12 hours
- **Status**: NOT STARTED

**Feature 2: Dead Code Elimination** (0/4 steps complete)
- ⏳ Step 1: Usage Analysis
- ⏳ Step 2: Dead Code Removal
- ⏳ Step 3: CLI Integration
- ⏳ Step 4: Metrics & Verification
- **Estimate**: 12-16 hours
- **Status**: NOT STARTED

**Feature 3: Vercel Deployment Adapter** (0/4 steps complete)
- ⏳ Step 1: Vercel Adapter
- ⏳ Step 2: CLI Deploy Command
- ⏳ Step 3: Integration Testing
- ⏳ Step 4: Documentation
- **Estimate**: 8-12 hours
- **Status**: NOT STARTED

### **Foundation Work Complete** ✅

**Annotation Parsing** (100% Complete):
- ✅ `src/ast.rs`: AST nodes for annotations
- ✅ `src/parser.rs`: Full annotation parsing
- ✅ `tests/annotations.rs`: 3 passing tests
- ✅ `examples/security/*.jnc`: 4 example files

**What Works Now**:
- ✅ Parser extracts `@auth(role="admin")` into AST
- ✅ All annotation types parse correctly
- ❌ Middleware generation NOT YET implemented
- ❌ Security runtime NOT YET implemented

### **Core Language Features** (All Complete!)
- ✅ Conditional rendering (if/else, ternary)
- ✅ Reactive signals and computed values
- ✅ Event handlers (onClick, onInput, preventDefault)
- ✅ Array methods (map, filter) with full JSX support
- ✅ Math operations (Math.floor, arithmetic)
- ✅ Object property access
- ✅ SVG elements
- ✅ Null rendering
- ✅ Functions in components
- ✅ String interpolation in attributes
- ✅ Component parameters with types
- ✅ Component return type annotations
- ✅ JSX in lambda expressions - including block bodies
- ✅ JSX text content with any characters
- ✅ Style system with themes

---

## 🔧 NEXT STEPS - PHASE 17

### **Now**: Implement Feature 1 - Security Middleware Generation (8-12 hours)

**Step 1**: Create Security Runtime Library (2-3 hours)
- Create `runtime/security.js`
- Implement `__jounce_auth_check()`
- Implement `__jounce_validate()`
- Implement `__jounce_ratelimit()`
- Implement `__jounce_sanitize()`
- Write tests in `tests/security_runtime.rs`

**Step 2**: Middleware Generation in Emitter (4-6 hours)
- Modify `src/js_emitter.rs`
- Add `generate_security_middleware()`
- Handle all annotation types (@auth, @validate, @ratelimit, @sanitize, @secure)
- Generate correct JavaScript middleware code
- Write tests in `tests/security_middleware.rs`

**Step 3**: Runtime Import Generation (1-2 hours)
- Modify `emit_program()` in `src/js_emitter.rs`
- Auto-import security functions when annotations detected
- Test import generation

**Step 4**: Integration Testing (2-3 hours)
- Create `examples/apps/03-secure-admin/main.jnc`
- Test all annotation types end-to-end
- Verify middleware executes correctly
- Document in `docs/SECURITY_FEATURES.md`

### **Later**: Phase 17 Features 2 & 3

**Feature 2**: Dead Code Elimination (12-16 hours)
- Usage analysis with call graph
- DCE optimizer implementation
- CLI `--optimize` flag
- Metrics & verification

**Feature 3**: Vercel Deployment Adapter (8-12 hours)
- Vercel adapter with serverless functions
- CLI `jnc deploy vercel` command
- Integration testing
- Deployment documentation

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

**Last Updated**: November 1, 2025
**Status**: Ready for public launch with complete documentation and governance
**Versioning**: See [VERSIONING.md](VERSIONING.md) for sprint-based development workflow
**Current Phase**: Phase 15 - Developer Onboarding & Learning
**Next Release**: v0.9.0 "Super Easy Start" (November 28, 2025)
**Focus**: Making Jounce the easiest language to learn in 2025
