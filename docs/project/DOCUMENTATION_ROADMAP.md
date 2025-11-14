# Jounce Documentation Roadmap

**Last Updated**: November 4, 2025

This guide shows you **exactly where to start** and **what order to read** the documentation.

---

## 📚 For LLMs Learning Jounce

**Goal**: Learn the language from scratch to build Jounce applications.

### Reading Order:

1. **START HERE**: [`LEARN_JOUNCE.md`](LEARN_JOUNCE.md) - Complete language guide ⭐
   - Core syntax
   - Components and reactivity
   - JSX and styling
   - What works vs what doesn't
   - Complete examples

2. **THEN READ**: [`SYNTAX_LIMITATIONS.md`](SYNTAX_LIMITATIONS.md) - What NOT to do
   - Common mistakes
   - Jounce vs JavaScript syntax
   - Jounce vs Rust syntax

3. **THEN EXPLORE**: Tutorial starters (hands-on learning)
   - [`templates/tutorial-starters/counter/`](templates/tutorial-starters/counter/) - Simple counter
   - [`templates/tutorial-starters/todo/`](templates/tutorial-starters/todo/) - Todo app
   - [`templates/tutorial-starters/form/`](templates/tutorial-starters/form/) - Form handling

4. **OPTIONAL**: Real-world examples
   - [`examples/apps/01-click-counter/`](examples/apps/01-click-counter/) - Click counter
   - [`examples/apps/03-secure-admin/`](examples/apps/03-secure-admin/) - Admin dashboard

**After this, you should be able to build any Jounce application!**

---

## 👨‍💻 For Human Developers

**Goal**: Understand what Jounce is and how to use it.

### Reading Order:

1. **START HERE**: [`README.md`](README.md) - Project overview ⭐
   - What is Jounce?
   - Why use Jounce?
   - Quick start guide
   - Feature highlights

2. **THEN READ**: [`LEARN_JOUNCE.md`](LEARN_JOUNCE.md) - Complete language guide
   - Detailed syntax explanations
   - Reactivity system deep dive
   - Complete examples

3. **THEN READ**: [`SYNTAX_LIMITATIONS.md`](SYNTAX_LIMITATIONS.md) - Avoid common mistakes
   - What doesn't work and why
   - Alternative syntax

4. **THEN BUILD**: Tutorial starters (hands-on)
   - Start with [`templates/tutorial-starters/counter/`](templates/tutorial-starters/counter/)
   - Progress to more complex examples

5. **OPTIONAL**: Advanced topics
   - [`CONTRIBUTING.md`](CONTRIBUTING.md) - Contributing to Jounce
   - [`TESTING_GUIDE.md`](TESTING_GUIDE.md) - Testing strategies
   - [`VERSIONING.md`](VERSIONING.md) - Version management

---

## 🔧 For Jounce Compiler Developers

**Goal**: Understand the compiler internals and contribute to the project.

### Reading Order:

1. **START HERE**: [`CLAUDE.md`](CLAUDE.md) - Development guide ⭐
   - Compiler architecture
   - Current status and issues
   - Development workflow
   - Critical warnings and best practices

2. **THEN READ**: Session archives (historical context)
   - Latest session docs show recent changes
   - Issue resolution history
   - Design decisions

3. **THEN EXPLORE**: Source code
   - `src/parser.rs` - Parsing logic
   - `src/js_emitter.rs` - Code generation
   - `src/reactive_analyzer.rs` - Reactivity tracking
   - `src/ast.rs` - AST definitions

4. **THEN TEST**: Test suite
   - `cargo test --lib` - Run all tests
   - `tests/` - Integration tests
   - Tutorial starters compilation

---

## 📖 Complete Documentation Index

### Core Documentation (Start Here)

| File | Purpose | Audience |
|------|---------|----------|
| [`README.md`](README.md) | Project overview, quick start | Everyone |
| [`LEARN_JOUNCE.md`](LEARN_JOUNCE.md) | Complete language guide | LLMs, Developers |
| [`SYNTAX_LIMITATIONS.md`](SYNTAX_LIMITATIONS.md) | What doesn't work | LLMs, Developers |

### Development Documentation

| File | Purpose | Audience |
|------|---------|----------|
| [`CLAUDE.md`](CLAUDE.md) | Compiler development guide | Contributors, AI assistants |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Contribution guidelines | Contributors |
| [`TESTING_GUIDE.md`](TESTING_GUIDE.md) | Testing strategies | Contributors |
| [`VERSIONING.md`](VERSIONING.md) | Version management | Maintainers |

### Session Archives (Historical)

| File | Purpose |
|------|---------|
| `SESSION_*.md` | Development session summaries |
| `PHASE_*.md` | Feature implementation phases |
| `*_COMPLETE.md` | Completed feature documentation |

### Examples and Templates

| Directory | Contents |
|-----------|----------|
| [`templates/tutorial-starters/`](templates/tutorial-starters/) | Beginner-friendly templates |
| [`examples/apps/`](examples/apps/) | Complete example applications |
| [`examples/projects/`](examples/projects/) | Real-world projects |

---

## 🎯 Quick Navigation

### I want to...

**...learn Jounce from scratch**
→ Start with [`LEARN_JOUNCE.md`](LEARN_JOUNCE.md)

**...understand what Jounce is**
→ Start with [`README.md`](README.md)

**...avoid syntax errors**
→ Read [`SYNTAX_LIMITATIONS.md`](SYNTAX_LIMITATIONS.md)

**...build my first app**
→ Go to [`templates/tutorial-starters/counter/`](templates/tutorial-starters/counter/)

**...contribute to the compiler**
→ Start with [`CLAUDE.md`](CLAUDE.md)

**...see working examples**
→ Browse [`examples/apps/`](examples/apps/)

**...understand the test suite**
→ Read [`TESTING_GUIDE.md`](TESTING_GUIDE.md)

---

## 📌 Documentation Principles

### For Writers

1. **Start with the user's goal** - What are they trying to do?
2. **Show, don't tell** - Include complete, working examples
3. **Be explicit about limitations** - Say what doesn't work
4. **Provide context** - Explain why, not just what
5. **Keep it current** - Update docs when features change

### For Readers

1. **Follow the reading order** - Don't skip around randomly
2. **Try the examples** - Compile and run them
3. **Experiment** - Break things and learn
4. **Ask questions** - Open issues if something is unclear
5. **Contribute back** - Improve docs when you find gaps

---

## 🔄 Keeping Documentation Current

### When to Update Docs

- ✅ New feature added → Update `LEARN_JOUNCE.md` and `CLAUDE.md`
- ✅ Syntax changed → Update `SYNTAX_LIMITATIONS.md`
- ✅ Bug fixed → Update `CLAUDE.md` issue list
- ✅ Breaking change → Update `README.md` and `LEARN_JOUNCE.md`
- ✅ New example added → Update this roadmap

### Documentation Checklist

Before releasing a new version:

- [ ] README.md updated with new features
- [ ] LEARN_JOUNCE.md includes new syntax/features
- [ ] SYNTAX_LIMITATIONS.md reflects current limitations
- [ ] CLAUDE.md updated with resolved issues
- [ ] Session archive created for major changes
- [ ] Tutorial starters still compile
- [ ] Examples still work

---

## 🆘 Need Help?

### For LLMs
- Re-read [`LEARN_JOUNCE.md`](LEARN_JOUNCE.md) for syntax questions
- Check [`SYNTAX_LIMITATIONS.md`](SYNTAX_LIMITATIONS.md) for what doesn't work
- Examine tutorial starters for working examples

### For Humans
- Open a GitHub issue for questions
- Check existing issues for similar problems
- Join the community discussions

### For Contributors
- Read [`CLAUDE.md`](CLAUDE.md) for development workflow
- Check session archives for historical context
- Run `cargo test --lib` to verify changes

---

**Last Updated**: November 4, 2025
**Version**: 0.8.3

**This roadmap will be updated as documentation evolves.**
