# Developer Tools Priority List

**Added:** October 31, 2025
**Purpose:** Tools that help BOTH users AND Claude+me build Jounce faster

---

## 🎯 Goal

After Quick Wins, shift focus to **meta-tools** that accelerate the development loop:
- Help users debug their apps
- Help us build Jounce features faster
- Help Claude understand the codebase better
- Reduce iteration time for everyone

---

## 🛠️ High-Priority Developer Tools

### 1. **AST Viewer/Inspector** ⭐⭐⭐⭐⭐
**Why:** See exactly what the parser produces
**Helps:**
- Debugging parser issues instantly
- Understanding how code is interpreted
- Validating new syntax features
- Teaching users how Jounce works

**Implementation:**
```bash
jounce ast main.jnc --format=json
jounce ast main.jnc --format=tree --visual
```

**Output:**
```
Program
├── Component: App
│   ├── Parameters: []
│   └── Body: Block
│       ├── Let: count = signal(0)
│       └── Return: JsxElement<div>
```

**Time:** 2-3 days
**Impact:** VERY HIGH

---

### 2. **Compilation Visualizer** ⭐⭐⭐⭐⭐
**Why:** See each phase of compilation
**Helps:**
- Understanding the pipeline
- Finding bottlenecks
- Debugging codegen issues
- Validating transformations

**Implementation:**
```bash
jounce compile main.jnc --visualize
```

**Output:**
```
Phase 1: Lexing          ✓ 542 tokens (0.3ms)
Phase 2: Parsing         ✓ 1 component, 5 statements (1.2ms)
Phase 3: Type Checking   ✓ 0 errors (0.8ms)
Phase 4: Borrow Checking ✓ 0 errors (0.5ms)
Phase 5: JS Generation   ✓ 234 lines (2.1ms)
Phase 6: CSS Generation  ✓ 45 rules (0.4ms)

📊 Total: 5.3ms
```

**Time:** 3-4 days
**Impact:** VERY HIGH

---

### 3. **Error Replay System** ⭐⭐⭐⭐⭐
**Why:** Reproduce bugs instantly
**Helps:**
- Debugging reported issues
- Creating regression tests
- Sharing reproducible cases
- Building test suite

**Implementation:**
```bash
# User hits error, gets a replay ID
jounce compile broken.jnc
# Error: ... [Replay ID: abc123]

# We can reproduce it exactly
jounce replay abc123

# Saves to test suite
jounce replay abc123 --save-test
```

**Captures:**
- Input file(s)
- Compiler version
- Exact error
- Environment info
- Compiler flags

**Time:** 2-3 days
**Impact:** VERY HIGH (saves hours of debugging)

---

### 4. **Test Generator** ⭐⭐⭐⭐
**Why:** Auto-generate tests from examples
**Helps:**
- Expanding test coverage
- Catching regressions
- Validating features
- Documentation that can't drift

**Implementation:**
```bash
jounce generate-tests examples/apps/*
```

**Output:**
```rust
#[test]
fn test_app_01_click_counter() {
    let source = include_str!("../examples/apps/01-click-counter/main.jnc");
    let result = compile(source);
    assert!(result.is_ok());
    assert!(result.unwrap().client_js.contains("signal(0)"));
}
```

**Time:** 2-3 days
**Impact:** HIGH

---

### 5. **Language Server (LSP)** ⭐⭐⭐⭐⭐
**Why:** IDE integration (VSCode, etc.)
**Helps:**
- Autocomplete
- Go to definition
- Find references
- Real-time error checking
- Refactoring support

**Implementation:**
```bash
jounce lsp --stdio
```

**Features:**
- Syntax highlighting
- Autocomplete for Jounce keywords
- Type hints
- Error squiggles
- Quick fixes

**Time:** 1-2 weeks
**Impact:** REVOLUTIONARY (makes Jounce feel professional)

---

### 6. **Code Formatter** ⭐⭐⭐⭐
**Why:** Consistent style automatically
**Helps:**
- Code reviews
- Collaboration
- Examples consistency
- Professional feel

**Implementation:**
```bash
jounce fmt main.jnc
jounce fmt --check main.jnc  # CI mode
```

**Time:** 3-4 days
**Impact:** HIGH

---

### 7. **REPL (Read-Eval-Print-Loop)** ⭐⭐⭐
**Why:** Test snippets quickly
**Helps:**
- Experimenting with syntax
- Teaching/learning
- Quick prototyping
- Testing expressions

**Implementation:**
```bash
jounce repl
>>> let x = signal(5)
>>> x.value
5
>>> x.value = 10
>>> x.value
10
```

**Time:** 4-5 days
**Impact:** MEDIUM-HIGH

---

### 8. **Performance Profiler** ⭐⭐⭐
**Why:** Find compilation bottlenecks
**Helps:**
- Optimizing compiler
- Scaling to large projects
- Understanding costs
- Prioritizing improvements

**Implementation:**
```bash
jounce compile main.jnc --profile
```

**Output:**
```
Function                Time      %      Calls
parse_expression        1.2ms    45%    127
generate_jsx            0.8ms    30%    43
type_check              0.4ms    15%    89
...
```

**Time:** 2-3 days
**Impact:** MEDIUM

---

### 9. **Documentation Generator** ⭐⭐⭐
**Why:** Auto-docs from code
**Helps:**
- Keeping docs updated
- API reference
- Package documentation
- Onboarding users

**Implementation:**
```bash
jounce doc src/ --output docs/
```

**Time:** 3-4 days
**Impact:** MEDIUM-HIGH

---

### 10. **Diff Viewer** ⭐⭐⭐⭐
**Why:** See what changed in generated code
**Helps:**
- Understanding compiler changes
- Reviewing codegen improvements
- Debugging regressions
- Teaching compiler internals

**Implementation:**
```bash
jounce diff main.jnc --before=v0.26.0 --after=v0.27.0
```

**Time:** 1-2 days
**Impact:** MEDIUM

---

## 📅 Proposed Timeline

### **Phase 1: Critical Dev Tools (2 weeks)**
Week 1:
- ✅ Quick Wins (continue current)
- Day 1-2: AST Viewer
- Day 3-4: Compilation Visualizer

Week 2:
- Day 1-2: Error Replay System
- Day 3-4: Test Generator
- Day 5: Integration & polish

### **Phase 2: Professional Tools (3 weeks)**
Week 3-5:
- Week 3-4: Language Server (LSP) - biggest lift
- Week 5: Code Formatter

### **Phase 3: Nice-to-Haves (1-2 weeks)**
- REPL
- Performance Profiler
- Doc Generator
- Diff Viewer

---

## 🎯 Success Metrics

**AST Viewer:**
- Reduces parser debugging time by 80%
- Used in every parser change

**Compilation Visualizer:**
- See bottlenecks clearly
- Optimize top 3 slowest phases

**Error Replay:**
- Reproduce 100% of reported bugs
- 50+ saved test cases

**Test Generator:**
- 100+ generated tests
- Zero manual test writing

**LSP:**
- VSCode extension published
- Autocomplete working
- 80% of users use it

**Code Formatter:**
- All examples formatted consistently
- Used in CI/CD

---

## 💡 Why These Tools Matter

### For Users:
- Professional IDE experience (LSP)
- Clear error messages (Visualizer)
- Consistent code style (Formatter)
- Learn faster (AST Viewer, REPL)

### For Us (Developers):
- Debug 10x faster (AST, Visualizer, Replay)
- Catch regressions (Test Generator)
- Understand performance (Profiler)
- Ship features faster (all of the above)

### For Claude:
- See AST structure clearly
- Understand compilation flow
- Debug issues independently
- Generate better code examples

---

## 🚀 Immediate Next Steps

After Quick Wins complete:

1. **Day 1:** AST Viewer (basic version)
2. **Day 2:** AST Viewer (visual tree + JSON export)
3. **Day 3:** Compilation Visualizer (phase timing)
4. **Day 4:** Compilation Visualizer (detailed output)
5. **Day 5:** Error Replay System

Then reassess based on which tool had most impact.

---

## 📝 Notes

- Start with simplest versions
- Get feedback early
- Iterate based on actual usage
- Some tools can be combined (AST + Visualizer)
- LSP is biggest but most impactful
- Error Replay will save us countless hours

---

**Priority:** HIGH
**Start Date:** After Quick Wins (est. Nov 2-3, 2025)
**Estimated Completion:** December 2025 (all 10 tools)

**Let's build tools that build Jounce faster!** 🚀
