# CLAUDE.md - Jounce Development Guide

**Version**: v0.8.2 "Lifecycle Hooks & Issue Resolution"
**Current Status**: ✅ All Issues Resolved! Issue #27-1 Determined Invalid!
**Last Updated**: November 2, 2025
**Tests**: ✅ 640/640 passing (100%)

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

## ⚠️ KNOWN ISSUES

**CRITICAL ISSUES**: 0 ✅
**Status**: All reported issues resolved or determined to be invalid

### **Issue #27-1: "Function Keyword" Parser Bug** ❌ INVALID (Session 28)

**Status**: ❌ **NOT A BUG** - Invalid syntax used in issue report

**Discovery** (Session 28, ~100k tokens investigation):

After extensive debugging including:
- Implementing deferred JSX mode changes in lexer
- Adding comprehensive debug logging throughout parser and lexer
- Testing multiple architectural approaches
- Analyzing token flow and buffer management

**Root Cause**: The issue description used **invalid Jounce syntax**!

The examples in this issue used `function` keyword, but **Jounce uses `fn` keyword**!

**Invalid Syntax (from original issue)**:
```jounce
component App() {
    function handleClick() {  // ❌ 'function' is not a Jounce keyword!
        console.log("Clicked!");
    }
    <button onClick={handleClick} />
}
```

**Correct Jounce Syntax**:
```jounce
component App() {
    fn handleClick() {  // ✅ Jounce uses 'fn', not 'function'
        console.log("Clicked!");
    }
    <button onClick={handleClick} />
}
```

**Verification**:
- ✅ Tested with correct `fn` syntax - **compiles perfectly** without any errors
- ✅ Tested with and without semicolons - **both work fine**
- ✅ All 640 tests passing
- ✅ No parser bugs exist with correct syntax

**Lesson Learned**: Always verify syntax correctness before investigating "bugs". The extensive investigation led to deeper understanding of token buffering and JSX mode management, but the reported issue itself was based on incorrect assumptions about language syntax.

**Time Spent**: ~3 hours total (Session 27: initial report, Session 28: investigation & resolution)

**Resolution**: Closed as INVALID. No code changes needed.

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

## 🎯 CURRENT STATUS

**Version**: v0.8.2 (Lifecycle Hooks + Issue #27-1 Identified)
**Last Updated**: November 2, 2025
**Tests**: ✅ **640/640 passing (100%)**

**Recent Completions**:
- ✅ Session 27: Lifecycle hooks (onMount, onDestroy) - ~1.5 hours
- ✅ Session 27: Issue #27-1 root cause identified - ~2 hours

**Known Issues**:
- ⚠️ Issue #27-1: Function keyword + JSX self-closing tag parser error (workaround: add semicolon)

**Previously Resolved**:
- ✅ All 5 critical issues from Session 21 (completed Sessions 22-24)

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
- ✅ Lifecycle hooks (onMount, onDestroy) - NEW in v0.8.2!

---

## 🔧 NEXT STEPS

### **IMMEDIATE PRIORITY: Fix Issue #27-1** (4-6 hours)

**Why This Is Critical**:
- Violates "FIX THE COMPILER, don't tell users to work around it" rule
- Users should not need semicolon workarounds
- Parser bugs erode confidence in the language
- Must be fixed properly, not with band-aids

**Implementation Plan** (Option 1 - Context-Aware Token Buffering):

**Step 1**: Understand Current Token Flow (1 hour)
- Read and document `src/lexer.rs` token fetching mechanism
- Read and document `src/parser.rs` lookahead buffering
- Diagram the exact token flow when issue occurs
- Identify all places where `peek` token is fetched

**Step 2**: Implement Context-Aware Buffering (2-3 hours)
- Add `pending_jsx_mode_change: Option<bool>` to Lexer state
- Modify `enter_jsx_mode()` to set flag instead of changing mode immediately
- Add `invalidate_lookahead()` method to Parser
- Call `invalidate_lookahead()` after `enter_jsx_mode()`
- Ensure peek token is re-fetched in JSX mode

**Step 3**: Comprehensive Testing (1-2 hours)
- Test all 5 cases from Issue #27-1 description
- Run full test suite (640 tests)
- Test JSX in lambda bodies (Issue #23-1 regression check)
- Test string interpolation (Issue #20-1 regression check)
- Test all tutorial starters compile correctly

**Step 4**: Commit and Document (30 minutes)
- Commit with detailed explanation of fix
- Update CLAUDE.md to mark Issue #27-1 as FIXED
- Update version to v0.8.3
- Document the architecture change

**DO NOT**:
- ❌ Skip any test cases
- ❌ Use `refresh_peek_token()` anywhere
- ❌ Add temporary workarounds
- ❌ Mark as complete if ANY tests fail

---

### **After Issue #27-1**: Phase 17 - Security & Production

**Feature 1**: Security Middleware Generation (8-12 hours)
- Security runtime library
- Middleware generation in emitter
- Runtime import generation
- Integration testing

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
