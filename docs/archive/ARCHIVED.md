# Archived Documentation

**Archived Date**: November 7, 2025
**Reason**: Historical development logs and outdated implementation docs

---

## Why Archived?

This folder contains historical documentation from Jounce's early development (October 2025). All content here is **outdated** and kept for historical reference only.

**Current documentation** lives in:
- [README.md](../../README.md) - Quick start guide
- [LEARN_JOUNCE.md](../guides/LEARN_JOUNCE.md) - Tutorials
- [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md) - Complete specification
- [docs/guides/](../guides/) - User guides
- [docs/api/](../api/) - API references

---

## Contents

### Development Logs (~400KB)
Historical progress reports from October 2025:
- **CLAUDE_PHASE*.md** - Phase 1-9 development sessions
- **DAY*.md** - Daily progress reports (Day 5-7)
- **DEVELOPMENT_PLAN_3_PHASES.md** - Original 3-phase plan
- **STATUS.md** - Old status reports

**Status**: ⚠️ Outdated - See [VERSIONING.md](../guides/VERSIONING.md) for current timeline

### Implementation Docs (~100KB)
Old implementation/deployment guides:
- **DEMO.md** - Early demo documentation
- **DEPLOY_RAVEN_FILE.md** - Outdated deployment (uses "raven" branding)
- **DEPLOYMENT_STATUS.md** - Old deployment status
- **HTTP_CLIENT_README.md** - Early HTTP client docs
- **IMPLEMENTATION_SUMMARY.md** - Phase completion summaries
- **LEARNINGS.md** - Development lessons learned
- **PERFORMANCE_BENCHMARKS.md** - Early benchmarks
- **PROGRESS.md** - Old progress tracking
- **QUICK_DEPLOY.md** - Outdated quick deploy guide
- **REGISTRY_TEST_REPORT.md** - Early registry tests
- **TESTING.md** - Old testing docs

**Status**: ⚠️ Outdated - See current guides in [docs/guides/](../guides/)

### Architecture (~60KB)
Historical design documents from October 18-23, 2025:
- **COMPILER_PIPELINE_STATUS.md** (Oct 18) - "90% complete" status from 3 weeks ago
- **HTTP_CLIENT_DESIGN.md** - HTTP client design (now implemented)
- **PERFORMANCE_OPTIMIZATION_DESIGN.md** (Oct 23) - Performance plans
- **REACTIVITY_IMPLEMENTATION.md** - Reactivity implementation summary
- **STDLIB_DESIGN.md** - Standard library architecture

**Status**: ⚠️ Historical - These describe work completed in v0.4.0-v0.6.0

**Current implementation details**: See [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md)

### Static Site (RavensOne Era)
- **docs-site-ravensone/** - Static HTML docs from pre-rebrand era
  - Wrong branding ("RavensOne" instead of "Jounce")
  - Outdated content (pre-v0.8.3)
  - See [docs-site-ravensone/ARCHIVED.md](docs-site-ravensone/ARCHIVED.md)

---

## Historical Context

These documents were created during Jounce's initial development sprint in **October 2025**, when the project was still being built from scratch. They document:

- **Week 1 (Oct 14-18)**: Compiler pipeline development
- **Week 2 (Oct 19-23)**: Reactivity system, stdlib, HTTP client
- **Week 3 (Oct 24-31)**: Package ecosystem, testing, initial release

**Current version** (v0.8.3) is ~3 weeks ahead of these docs with:
- ✅ 580/580 tests passing (vs ~200 tests in these docs)
- ✅ Complete language features (import aliasing, pub keyword, if-let)
- ✅ Production-ready runtime with 29/29 reactivity tests
- ✅ Full package registry with JWT auth
- ✅ Comprehensive safety nets (frozen signals, dev-mode checks)

---

## Future Documentation

If you need to reference implementation history:
1. **Git history** - More accurate than these docs
2. **VERSIONING.md** - Official release timeline
3. **Session logs** - Detailed session-by-session progress (docs/development/)

Do not use archived docs as current reference - they contain outdated information, old branding ("raven"), and incomplete feature descriptions.

---

**Status**: ⚠️ ARCHIVED - Historical reference only
**Last Updated**: November 7, 2025
