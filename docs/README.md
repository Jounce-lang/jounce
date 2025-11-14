# Jounce Documentation

**Version**: v0.8.3 "Enhanced Language Features"
**Last Updated**: November 7, 2025
**Status**: Production Ready (580/580 tests passing)

**Complete documentation for the Jounce programming language and framework**

---

## 📖 Documentation Hierarchy

**Authoritative specification** (in order of precedence):

1. **[JOUNCE_SPEC.md](../JOUNCE_SPEC.md)** — **THE BOSS DOCUMENT**
   - Canonical language behavior
   - Complete syntax reference
   - Type system rules
   - Runtime semantics
   - **If there's a conflict, this document wins**

2. **[LEARN_JOUNCE.md](guides/LEARN_JOUNCE.md)** — Teaching guide
   - Learn by building 5 apps
   - Best practices
   - Common patterns
   - **Must align with JOUNCE_SPEC.md**

3. **[README.md](../README.md)** — Quick start
   - Installation
   - First app in 5 minutes
   - Key features overview

---

## 🚀 Quick Start Paths

**New to Jounce?** Choose your path:

### Path 1: Just Ship It (5 minutes)
1. [Main README](../README.md) - Install and run first app
2. [Starter Templates](../templates/) - Copy a working template
3. Start building

### Path 2: Learn Properly (90 minutes)
1. [Main README](../README.md) - Install
2. [LEARN_JOUNCE.md](guides/LEARN_JOUNCE.md) - Build 5 apps
3. [JOUNCE_SPEC.md](../JOUNCE_SPEC.md) - Reference when stuck

### Path 3: I Read Specs (Reference)
1. [JOUNCE_SPEC.md](../JOUNCE_SPEC.md) - Complete specification
2. [API References](api/) - Detailed APIs
3. [Guides](guides/) - Specific topics

---

## 📚 Documentation Map

### Core Documents (Root Level)
- **[JOUNCE_SPEC.md](../JOUNCE_SPEC.md)** - Complete language specification
- **[README.md](../README.md)** - Quick start guide
- **[CHANGELOG.md](../CHANGELOG.md)** - Version history

### User Guides (`guides/` - 34 files)
- **[LEARN_JOUNCE.md](guides/LEARN_JOUNCE.md)** - Primary tutorial (5 apps)
- **[SYNTAX_LIMITATIONS.md](guides/SYNTAX_LIMITATIONS.md)** - Complete syntax limitations reference
- **[QUICKSTART.md](guides/QUICKSTART.md)** - Quick reference redirect
- **[FULLSTACK_GUIDE.md](guides/FULLSTACK_GUIDE.md)** - Full-stack development
- **[REACTIVITY_USER_GUIDE.md](guides/REACTIVITY_USER_GUIDE.md)** - Signals & reactivity
- **[STYLE_SYSTEM_USER_GUIDE.md](guides/STYLE_SYSTEM_USER_GUIDE.md)** - CSS and styling
- **[PACKAGE_MANAGER_GUIDE.md](guides/PACKAGE_MANAGER_GUIDE.md)** - Package management
- **[MODULE_SYSTEM.md](guides/MODULE_SYSTEM.md)** - Imports and modules
- **[DEPLOYMENT_GUIDE.md](guides/DEPLOYMENT_GUIDE.md)** - Deploy to production
- **[TESTING_GUIDE.md](guides/TESTING_GUIDE.md)** - Testing your apps
- **[SECURITY_SYSTEM.md](guides/SECURITY_SYSTEM.md)** - Security features
- **[ERROR_MESSAGES.md](guides/ERROR_MESSAGES.md)** - Understanding errors
- **[CSS_UTILITIES.md](guides/CSS_UTILITIES.md)** - 457 utility classes
- **[COMPONENT_PROPS_GUIDE.md](guides/COMPONENT_PROPS_GUIDE.md)** - Props and components
- See [guides/](guides/) for complete list

### API References (`api/` - 9 files)
- **[REACTIVITY_API.md](api/REACTIVITY_API.md)** - signal(), computed(), effect()
- **[STDLIB_API_REFERENCE.md](guides/STDLIB_API_REFERENCE.md)** - 200+ stdlib functions
- **[STDLIB_MODULES_REFERENCE.md](guides/STDLIB_MODULES_REFERENCE.md)** - JSON, DateTime, etc.
- **[YAML_MODULE.md](api/YAML_MODULE.md)** - YAML parsing API
- See [api/](api/) for complete list

### Design Documents (`design/` - 7 files)
- **[REACTIVITY_SYSTEM.md](design/REACTIVITY_SYSTEM.md)** - Reactivity design (historical)
- **[SSR_DEV_SERVER_DESIGN.md](design/SSR_DEV_SERVER_DESIGN.md)** - SSR architecture
- See [design/](design/) for complete list

### Development Logs (`development/` - 29 files)
- Session-by-session development history
- Sprint completion reports
- Task summaries
- See [development/](development/) for complete list

### Project Management (`project/` - 6 files)
- **[ROADMAP.md](project/ROADMAP.md)** - Future plans
- **[PRODUCTION_AUDIT.md](project/PRODUCTION_AUDIT.md)** - Readiness audit
- **[RELEASE_CHECKLIST.md](project/RELEASE_CHECKLIST.md)** - Release process
- **[DOCUMENTATION_ROADMAP.md](project/DOCUMENTATION_ROADMAP.md)** - Docs planning
- See [project/](project/) for complete list

### Archives (`archive/` - Historical Only)
- **DO NOT USE** - Outdated documentation
- See [archive/ARCHIVED.md](archive/ARCHIVED.md) for index
- Kept for historical reference only
- All content superseded by current docs

---

## 🔍 Finding What You Need

### By Task

**"I want to build my first app"**
→ [LEARN_JOUNCE.md](guides/LEARN_JOUNCE.md) Tutorial 1: Counter

**"How does X work?"**
→ [JOUNCE_SPEC.md](../JOUNCE_SPEC.md) - Search for X

**"I got an error"**
→ [ERROR_MESSAGES.md](guides/ERROR_MESSAGES.md) - Error guide

**"How do I deploy?"**
→ [DEPLOYMENT_GUIDE.md](guides/DEPLOYMENT_GUIDE.md) - Deployment

**"What CSS classes are available?"**
→ [CSS_UTILITIES.md](guides/CSS_UTILITIES.md) - All 457 utilities

**"How do signals work?"**
→ [REACTIVITY_API.md](api/REACTIVITY_API.md) - Complete API

**"Something doesn't work"**
→ Check [JOUNCE_SPEC.md](../JOUNCE_SPEC.md) first (authoritative)
→ If still stuck: [Open an issue](https://github.com/Jounce-lang/jounce-pre-production/issues)

### By Searching

**All curated docs are linked from this file** - use your browser's search (Ctrl/Cmd+F)

**Command line search:**
```bash
# Search all active docs (excludes archives)
find docs/ -name "*.md" -not -path "*/archive/*" -exec grep -l "keyword" {} \;

# Search guides only
grep -r "keyword" docs/guides/

# Search API references
grep -r "keyword" docs/api/
```

---

## 📝 Documentation Standards

### File Naming Conventions
- **Specifications**: `JOUNCE_SPEC.md` (SCREAMING_SNAKE_CASE)
- **Guides**: `LEARN_JOUNCE.md`, `FULLSTACK_GUIDE.md` (SCREAMING_SNAKE_CASE)
- **API References**: `REACTIVITY_API.md` (SCREAMING_SNAKE_CASE)
- **Use descriptive names**, not `doc1.md`

### Content Guidelines
1. **Align with JOUNCE_SPEC.md** - Don't contradict the spec
2. **Include version/date** at top of each doc
3. **Link to related docs** for deeper info
4. **Keep examples up-to-date** with current syntax
5. **Add to this index** when creating new docs

---

## 🆕 Recent Updates

**November 7, 2025 (v0.8.3):**
- Documentation cleanup and consolidation
- Moved 20+ outdated docs to archive/
- Established JOUNCE_SPEC.md as authoritative source
- Reorganized guides/ and project/ folders

**November 5, 2025 (v0.8.2):**
- Runtime safety nets and production audit
- Frozen signals, dev-mode checks
- 3-layer protection system

**October 31, 2025 (v0.8.1):**
- CSS utilities (457 classes)
- Enhanced error messages (20+ codes)
- Starter templates

---

## ⚠️ If You Can't Find Something

1. **Search this index** (Ctrl/Cmd+F)
2. **Check [JOUNCE_SPEC.md](../JOUNCE_SPEC.md)** - It's the boss
3. **Look in [guides/](guides/)** - 33 topic-specific guides
4. **Still stuck?** [Open an issue](https://github.com/Jounce-lang/jounce-pre-production/issues) - We'll add it to the docs

---

## 🔗 External Resources

- [GitHub Repository](https://github.com/Jounce-lang/jounce-pre-production)
- [Issue Tracker](https://github.com/Jounce-lang/jounce-pre-production/issues)
- [Changelog](../CHANGELOG.md)
- [Roadmap](project/ROADMAP.md)

---

**Last Updated**: November 7, 2025 (v0.8.3)
