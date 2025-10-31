# Jounce Documentation

**Complete documentation for the Jounce programming language and framework**

---

## 🚀 Quick Start

**New to Jounce?**
1. [Main README](../README.md) - Project overview
2. [Getting Started](GETTING_STARTED.md) - Step-by-step tutorial
3. [Starter Templates](../templates/) - Production-ready starting points

---

## 📚 Core Documentation

### Language Features
- [CSS Utilities](CSS_UTILITIES.md) - 457 Tailwind-inspired utility classes
- [Error Messages](ERROR_MESSAGES.md) - Enhanced error codes and help

### User Guides (guides/)
- [Full-Stack Development](guides/FULLSTACK_GUIDE.md) - Complete development guide
- [JSX Lexer Usage](guides/JSX_LEXER_USAGE.md) - JSX parsing guide
- [JSX AST Guide](guides/JSX_AST_GUIDE.md) - JSX abstract syntax tree
- [CSS Syntax](guides/CSS_SYNTAX.md) - CSS and styling guide
- [Module System](guides/MODULE_SYSTEM.md) - Package and imports
- [Compiler Guide](guides/COMPILER-GUIDE.md) - Compilation process
- [LSP Features](guides/LSP_FEATURES.md) - Language Server Protocol
- [Code Formatting](guides/CODE_FORMATTING.md) - Code style guide

### Advanced Topics
- [Closure Implementation](guides/CLOSURE_IMPLEMENTATION_SUMMARY.md) - How closures work
- [Double Colon Operator](guides/DOUBLE_COLON_OPERATOR.md) - `::` syntax
- [Parser Limitations](guides/PARSER_LIMITATIONS.md) - Known limitations

---

## 🏗️ Architecture (architecture/)

Technical design documentation:
- System architecture
- Component design
- Data flow diagrams

---

## 🔧 Technical (technical/)

Implementation details:
- Codegen progress
- Standard library completion
- Deployment configuration
- Annotation status

---

## 📖 Tutorials (tutorials/)

Step-by-step learning materials

---

## 🎯 Design (design/)

Design documents and specifications

---

## 📦 API Reference (api/)

Auto-generated API documentation

---

## 📁 Organization

### Active Documentation
- Root `docs/` - Current, maintained documentation
- Root `docs/*.md` - Core feature docs
- `guides/` - User-facing guides
- `technical/` - Technical implementation docs
- `architecture/` - System design docs
- `tutorials/` - Learning materials
- `api/` - API reference

### Archived
- `docs/archive/` - Superseded documentation
- `docs/archived/` - Historical documents
- `docs/sessions/` - Development session notes
- `docs/planning/` - Old planning documents

---

## 🔍 Finding Documentation

**By Topic:**
```bash
# Getting started
cat docs/GETTING_STARTED.md

# CSS utilities reference
cat docs/CSS_UTILITIES.md

# Error messages
cat docs/ERROR_MESSAGES.md

# Full-stack guide
cat docs/guides/FULLSTACK_GUIDE.md
```

**Search all docs:**
```bash
find docs/ -name "*.md" | grep -v archive | grep -v archived
```

---

## 📝 Adding Documentation

**Guidelines:**
1. Place in appropriate subdirectory
2. Use clear, descriptive filenames
3. Include table of contents for long docs
4. Link to related documentation
5. Keep examples up-to-date
6. Add to this index when adding new docs

**File Naming:**
- Use `SCREAMING_SNAKE_CASE.md` for technical docs
- Use `Title_Case.md` for guides
- Use descriptive names (not `doc1.md`)

---

## 🆕 Recent Additions

**October 2025 (v0.8.1):**
- `CSS_UTILITIES.md` - Complete CSS utilities reference (457 classes)
- `ERROR_MESSAGES.md` - Enhanced error messages guide (20+ error codes)
- Updated `GETTING_STARTED.md` - Includes starter templates

**October 2025 (v0.8.0):**
- Package ecosystem documentation
- Production readiness summary
- Phase 1 completion docs

---

## 🔗 External Resources

- [GitHub Repository](https://github.com/Jounce-lang/Jounce)
- [Issue Tracker](https://github.com/Jounce-lang/Jounce/issues)
- [Changelog](../CHANGELOG.md)
- [Roadmap](../ROADMAP.md)

---

**Last Updated**: October 31, 2025 (v0.8.1)
