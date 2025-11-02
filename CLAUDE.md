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

## 📊 CURRENT STATUS

**Completed This Session (Session 24)**:
- ✅ Issue #23-1: JSX in lambda expressions (30 minutes) - **FINAL ISSUE!**

**Previous Sessions**:
- ✅ Session 23: Issue #12-1 (Component return types - 10 minutes)
- ✅ Session 22: Issue #20-1 (String interpolation - 2 hours)
- ✅ Session 21: Phase 13 + Issues #13-1, #13-2

**Test Status**: ✅ **635/635 passing (100%)**

**What Works** (Everything!):
- ✅ Conditional rendering (if/else, ternary)
- ✅ Reactive signals and computed values
- ✅ Event handlers (onClick, onInput, preventDefault)
- ✅ Array methods (map, filter) with full JSX support (NEW!)
- ✅ Math operations (Math.floor, arithmetic)
- ✅ Object property access
- ✅ SVG elements
- ✅ Null rendering
- ✅ Functions in components
- ✅ String interpolation in attributes
- ✅ Component parameters with types
- ✅ Component return type annotations
- ✅ JSX in lambda expressions - including block bodies (NEW!)
- ✅ JSX text content with any characters (NEW!)
- ✅ Style system with themes

**What Needs Work**:
- ✨ **NOTHING! All known critical issues fixed!**

---

## 🔧 NEXT STEPS

### **Now**: Phase 15 - Developer Onboarding & Learning
**Goal**: Make Jounce the easiest language to learn in 2025

**Sprint 15.1** (Nov 1-7, 2025) - Interactive Tutorial System:
- [ ] Build tutorial.jounce.dev with 10 progressive lessons
- [ ] In-browser editor with live preview
- [ ] "Jounce Certified Developer" badge on completion
- [ ] Target: Users go from zero to first app in under 1 hour

**Sprint 15.2** (Nov 8-14, 2025) - Video Course:
- [ ] YouTube channel with "Jounce in 100 Seconds"
- [ ] 10 short tutorial videos (1-3 mins each)
- [ ] 4 live coding sessions (30 mins each)
- [ ] Target: 10,000+ views in first month

**Sprint 15.3** (Nov 15-21, 2025) - Documentation Overhaul:
- [ ] Rewrite Getting Started guide
- [ ] Create Tutorial Hub with 15 tutorials
- [ ] Build Cookbook with common patterns
- [ ] Write migration guides (React, Vue, Svelte, Next.js)
- [ ] Target: Time-to-first-app under 10 minutes

**Sprint 15.4** (Nov 22-28, 2025) - Example App Library:
- [ ] Build examples.jounce.dev with 20+ examples
- [ ] One-click deploy to Vercel/Fly.io
- [ ] Searchable gallery with filters
- [ ] Target: 1000+ deployments in first month

**Release**: v0.9.0 "Super Easy Start" (November 28, 2025)

### **Later**: Phase 16 - Developer Productivity
**Goal**: 10x developer productivity with world-class tooling

**Deliverables**:
- Visual playground (play.jounce.dev)
- VSCode extension pro with AI assistance
- CLI power tools (doctor, init, add, deploy)
- Developer dashboard (dashboard.jounce.dev)

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
