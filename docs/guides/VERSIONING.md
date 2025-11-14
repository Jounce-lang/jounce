# Jounce Versioning & Release Management

> **Note**: For authoritative language rules, see [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md).

**Last Updated**: November 7, 2025
**Current Version**: v0.8.3 "Enhanced Language Features"
**System**: Semantic Versioning with Sprint-Based Development

---

## 📋 Table of Contents

1. [High-Level Workflow Hierarchy](#1-high-level-workflow-hierarchy)
2. [Versioning System (Semantic Versioning)](#2-versioning-system-semantic-versioning)
3. [Git Branching & Tags](#3-git-branching--tags)
4. [Project Folder & Package Versioning](#4-project-folder--package-versioning)
5. [Sprint & Task Naming](#5-sprint--task-naming)
6. [Daily Flow Template](#6-daily-flow-template)
7. [Example Timeline](#7-example-timeline)
8. [Optional Advanced Tags](#8-optional-advanced-tags)

---

## 1. High-Level Workflow Hierarchy

Jounce uses a four-tier hierarchy to organize work from strategic planning down to individual commits.

| Level | Purpose | Duration | Example Name |
|-------|---------|----------|--------------|
| **Phase** | 4 major roadmap epochs (big architectural or feature leaps) | 3+ months | Phase 1 — Compiler Core |
| **Sprint** | Short development cycles within a phase | 1-2 weeks | Sprint 1.3 — Parser rewrite |
| **Release** | Public milestone or patch | Anytime | v0.3.0 |
| **Daily Work** | Individual commits / dev logs | 1-5 per day | feat: add BLAST optimizer |

### How They Connect

```
Phase 1: Compiler Core (3 months)
  ├─ Sprint 1.1: Lexer (2 weeks) → Release v0.1.0
  ├─ Sprint 1.2: Parser (2 weeks) → Release v0.2.0
  └─ Sprint 1.3: Optimizer (2 weeks) → Release v0.3.0
      └─ Daily commits:
          ├─ feat: add BLAST optimizer
          ├─ fix: tokenizer edge cases
          └─ docs: update README
```

### Key Principles

- **Phases** define strategic milestones (e.g., "Compiler Core", "Package Ecosystem", "Production Ready")
- **Sprints** are time-boxed development cycles focused on specific deliverables
- **Releases** are public-facing version numbers following semantic versioning
- **Daily Work** is tracked through conventional commits (feat:, fix:, docs:, refactor:)

---

## 2. Versioning System (Semantic Versioning)

We use **Semantic Versioning (SemVer)** as described at [semver.org](https://semver.org).

### Version Format: MAJOR.MINOR.PATCH

Example: `v0.3.1`

### Increment When...

| Type | Increment When... | Example |
|------|-------------------|---------|
| **MAJOR** | Backwards-incompatible change | 0.x.x → 1.0.0 |
| **MINOR** | Add new feature / partial phase done | 0.2.x → 0.3.0 |
| **PATCH** | Fixes / optimizations / docs | 0.3.0 → 0.3.1 |

### Detailed Rules

#### MAJOR Version (Breaking Changes)
**Increment when making backwards-incompatible changes:**
- API removals or signature changes
- File format changes that break existing code
- Removed features
- Language syntax changes that break existing `.jnc` files

**Example**: `0.9.x → 1.0.0`

**Until 1.0**, every MINOR bump = a new public milestone (your "phase" milestones).

#### MINOR Version (New Features)
**Increment when adding functionality in a backwards-compatible manner:**
- New language features (e.g., async/await, pattern matching)
- New standard library modules
- Partial completion of a phase
- New packages added to ecosystem

**Example**: `0.2.x → 0.3.0`

#### PATCH Version (Bug Fixes & Docs)
**Increment when making backwards-compatible bug fixes:**
- Bug fixes
- Performance optimizations
- Documentation updates
- Refactoring (no user-facing changes)

**Example**: `0.3.0 → 0.3.1`

### Pre-1.0.0 Special Note

**Until 1.0**, every MINOR bump = a new public milestone (your "phase" milestones).

**Tip**: Until 1.0.0, every MINOR bump = a new public milestone (your "phase" milestones will likely align with 0.1.0, 0.2.0, etc.).

---

## 3. Git Branching & Tags

### Branch Strategy

| Branch Type | Purpose | Naming Example |
|-------------|---------|----------------|
| **main** | Public, production-ready code | `main` |
| **dev** | Active working branch | `dev` |
| **feature branches** | Temporary, focused work | `feature/ast-parser` |
| **hotfix** | Urgent fixes | `hotfix/package-registry-bug` |

### Tag Strategy

**Tags** are used to lock releases:

```bash
# Standard release
git tag -a v0.2.0 -m "Phase 1 partial release: Jounce compiler emits WASM + JS"
git push origin v0.2.0

# Beta release
git tag -a v0.3.0-beta1 -m "Beta release for testing"
git push origin v0.3.0-beta1
```

### Workflow Example

```bash
# 1. Daily work happens on feature branches
git checkout -b feature/reactive-signals dev
# ... make changes ...
git commit -m "feat: add reactive signal support"
git push origin feature/reactive-signals

# 2. Merge to dev when ready
git checkout dev
git merge feature/reactive-signals
git push origin dev

# 3. When sprint is complete, merge to main and tag
git checkout main
git merge dev
git tag -a v0.3.0 -m "Sprint 1.2 - Parser rewrite"
git push origin main
git push origin v0.3.0
```

### Command Examples

```bash
# Create and push a tag
git tag -a v0.3.0 -m "Phase 1 partial release: Jounce compiler emits WASM + JS"
git push origin v0.3.0

# List all tags
git tag -l

# Delete a tag (if needed)
git tag -d v0.3.0
git push origin :refs/tags/v0.3.0
```

---

## 4. Project Folder & Package Versioning

### Multiple Sub-Packages

If you're using multiple sub-packages:
- `compiler/` → 0.3.0
- `cli/` → 0.3.0
- `registry/` → 0.2.5

**They all share the same phase but can have independent patch versions.**

### Versioning Files

Use a single **CHANGELOG.md** at the root with structured entries:

```markdown
## [0.3.0] - 2025-11-01

### Added
- WASM emitter
- CLI command 'jounce build'

### Fixed
- Tokenizer edge cases
```

### Example Structure

```
jounce/
├── Cargo.toml          # version = "0.3.0"
├── CHANGELOG.md        # Root changelog
├── compiler/
│   └── Cargo.toml      # version = "0.3.0"
├── cli/
│   └── Cargo.toml      # version = "0.3.0"
└── registry/
    └── Cargo.toml      # version = "0.2.5" (independent patch)
```

---

## 5. Sprint & Task Naming

### Three-Layer Task Structure

Use a 3-layer task structure in Notion or GitHub Projects:

| Layer | Example | Purpose |
|-------|---------|---------|
| **Phase** | Phase 1 — Core Compiler | 3-month theme |
| **Sprint** | Sprint 1.2 — Optimizer | 2-week work block |
| **Task** | #312: Implement dead code elimination | daily deliverable |

### Naming Patterns

#### Naming pattern for commits or issues:

**Format**:
```
phase1/sprint.2: add parser combinator module
```

**Breakdown**:
- `phase1` — Which major phase
- `/sprint.2` — Which sprint within the phase
- `: add parser combinator module` — What was done

#### Task Naming Examples

```
#312: Implement dead code elimination
#313: Add WASM memory optimization
#314: Fix parser regression in array literals
```

### Sprint Board Organization

Example in Notion or GitHub Projects:

```
Phase 1: Core Compiler
├── Sprint 1.1: Lexer
│   ├── #101: Implement tokenizer
│   ├── #102: Add JSX mode support
│   └── #103: Write lexer tests
├── Sprint 1.2: Parser
│   ├── #201: Build AST nodes
│   ├── #202: Implement expression parsing
│   └── #203: Add error recovery
└── Sprint 1.3: Optimizer
    ├── #301: Implement constant folding
    ├── #302: Add dead code elimination
    └── #303: Optimize WASM output
```

---

## 6. Daily Flow Template

### Start of Day

```bash
# 1. Pull latest changes
git pull origin dev

# 2. Define 3 tasks (≤3)
# Write tasks in todo.md or Notion

# 3. Create feature branch (if needed)
git checkout -b feature/my-feature dev
```

### During the Day

**Commit style**: Use conventional commits

```bash
git commit -m "feat: add new feature"
git commit -m "fix: resolve bug in parser"
git commit -m "refactor: simplify lexer logic"
git commit -m "docs: update API documentation"
```

### End of Day

```bash
# 1. Commit + push to dev
git add .
git commit -m "feat: implement reactive signals"
git push origin dev

# 2. Update sprint board or Notion
# Mark completed tasks, update progress

# 3. Optional tag for daily snapshot
git tag -a v0.3.0-dev.20251101 -m "Daily snapshot: Nov 1, 2025"
git push origin v0.3.0-dev.20251101
```

### Commit Message Convention

Follow [Conventional Commits](https://www.conventionalcommits.org/):

| Type | When to Use | Example |
|------|-------------|---------|
| `feat:` | New feature | `feat: add reactive signals` |
| `fix:` | Bug fix | `fix: resolve parser crash` |
| `refactor:` | Code refactoring | `refactor: simplify error handling` |
| `docs:` | Documentation | `docs: update README` |
| `test:` | Tests | `test: add parser unit tests` |
| `chore:` | Maintenance | `chore: update dependencies` |
| `perf:` | Performance | `perf: optimize lexer speed` |

---

## 7. Example Timeline

This is a realistic timeline for a new language project:

| Time | Milestone | Example Tag |
|------|-----------|-------------|
| **Month 1** | Compiler MVP ready | v0.1.0 |
| **Month 2** | Registry and package manager working | v0.2.0 |
| **Month 3** | CLI + Docs + public showcase | v0.3.0 |
| **Month 6** | Jounce stable 1.0 | v1.0.0 |

### Detailed Example

```
Month 1: Compiler MVP
├── Week 1-2: Sprint 1.1 — Lexer → v0.0.1-alpha.1
├── Week 3-4: Sprint 1.2 — Parser → v0.1.0-beta.1
└── Release: v0.1.0 (Compiler MVP)

Month 2: Package Manager
├── Week 5-6: Sprint 2.1 — Registry API → v0.1.1
├── Week 7-8: Sprint 2.2 — CLI commands → v0.1.2
└── Release: v0.2.0 (Package Manager)

Month 3: Production Ready
├── Week 9-10: Sprint 3.1 — Documentation → v0.2.1
├── Week 11-12: Sprint 3.2 — Public showcase → v0.2.2
└── Release: v0.3.0 (Public Launch)

Month 6: Stable Release
└── Release: v1.0.0 (Language Lock)
```

---

## 8. Optional Advanced Tags

If you like clarity in pre-releases:

### Tag Types

| Tag Type | Purpose | Example |
|----------|---------|---------|
| **Beta** | Public testing | `v0.3.0-beta.1` |
| **Alpha** | Internal testing only | `v0.3.0-alpha.4` |
| **Dev** | Bleeding edge, daily snapshots | `v0.3.0-dev.20251101` |

### When to Use

- **Beta**: Feature-complete but needs testing
- **Alpha**: Work in progress, may have breaking changes
- **Dev**: Daily snapshots for team coordination

### Examples

```bash
# Beta release (public testing)
git tag -a v0.3.0-beta.1 -m "Beta 1 for v0.3.0"
git push origin v0.3.0-beta.1

# Alpha release (internal testing)
git tag -a v0.3.0-alpha.4 -m "Alpha 4 for v0.3.0"
git push origin v0.3.0-alpha.4

# Dev snapshot (daily build)
git tag -a v0.3.0-dev.20251101 -m "Dev snapshot: Nov 1, 2025"
git push origin v0.3.0-dev.20251101
```

---

## 📊 Jounce Version History

### Current Status (November 2025)

- **Current Version**: v0.8.3 "Enhanced Language Features"
- **Tests**: 580/580 passing (100%)
- **Status**: ✅ Production ready, zero known bugs

### Version Timeline

| Version | Date | Phase | Highlights |
|---------|------|-------|------------|
| v0.1.0 | Oct 20, 2025 | Phase 1 | Initial compiler implementation |
| v0.2.0 | Oct 22, 2025 | Phase 1 | Language core complete (100%) |
| v0.3.0 | Oct 24, 2025 | Phase 10 | Production ready, 638/638 tests |
| v0.4.0 | Oct 24, 2025 | Phase 12 | Fine-grained reactivity |
| v0.6.0 | Oct 24, 2025 | Phase 14 | 15 packages (ecosystem expansion) |
| v0.7.0 | Oct 24, 2025 | Packages | 20 packages (queue, markdown) |
| v0.8.0 | Oct 24, 2025 | Packages | 35 packages (milestone achieved!) |
| v0.8.1 | Oct 31, 2025 | Developer UX | Public launch preparation |
| v0.8.2 | Nov 5, 2025 | Runtime Safety | Runtime safety nets & production audit |
| v0.8.3 | Nov 7, 2025 | Language Features | Import aliasing, explicit pub, docs cleanup |

### Next Milestones

| Version | Target | Phase | Goals |
|---------|--------|-------|-------|
| v0.9.0 | Nov 2025 | Database | Database integration + example apps |
| v0.10.0 | Dec 2025 | Router | Client/server routing system |
| v0.11.0 | Jan 2026 | Forms | Form validation & error handling |
| v1.0.0 | Q2 2026 | Language Lock | Stable API, 50+ packages |

---

## 🔧 Tools & Automation

### Version Bumping

```bash
# Update version in Cargo.toml
cargo set-version 0.3.1

# Or manually edit:
# version = "0.3.1"
```

### Changelog Generation

Use [git-cliff](https://github.com/orhun/git-cliff) or [conventional-changelog](https://github.com/conventional-changelog/conventional-changelog):

```bash
# Generate changelog
git cliff --tag v0.3.0 > CHANGELOG.md

# Or use conventional-changelog
conventional-changelog -p angular -i CHANGELOG.md -s
```

### Release Automation

Create a release script (`scripts/release.sh`):

```bash
#!/bin/bash
VERSION=$1

# 1. Update version
cargo set-version $VERSION

# 2. Update changelog
git cliff --tag v$VERSION > CHANGELOG.md

# 3. Commit changes
git add .
git commit -m "chore: release v$VERSION"

# 4. Tag release
git tag -a v$VERSION -m "Release v$VERSION"

# 5. Push
git push origin main
git push origin v$VERSION

# 6. Create GitHub release
gh release create v$VERSION --generate-notes
```

---

## 📚 References

- [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
- [Git Flow](https://nvie.com/posts/a-successful-git-branching-model/)

---

## ✅ Quick Reference

### Creating a New Release

```bash
# 1. Ensure all changes are committed
git status

# 2. Update version number
# Edit Cargo.toml: version = "0.3.1"

# 3. Update CHANGELOG.md
# Add release notes under new version header

# 4. Commit version bump
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to v0.3.1"

# 5. Create tag
git tag -a v0.3.1 -m "Release v0.3.1: Bug fixes and optimizations"

# 6. Push to GitHub
git push origin main
git push origin v0.3.1

# 7. Create GitHub release (optional)
gh release create v0.3.1 --generate-notes
```

### Hotfix Workflow

```bash
# 1. Create hotfix branch from main
git checkout -b hotfix/critical-bug main

# 2. Fix the bug
git commit -m "fix: resolve critical security issue"

# 3. Bump patch version
# Edit Cargo.toml: version = "0.3.1" → "0.3.2"

# 4. Merge to main
git checkout main
git merge hotfix/critical-bug

# 5. Tag and push
git tag -a v0.3.2 -m "Hotfix: Security patch"
git push origin main
git push origin v0.3.2

# 6. Merge back to dev
git checkout dev
git merge main
git push origin dev
```

---

**Last Updated**: November 7, 2025

---

**Maintained by: The Jounce Project**

**Questions?** See [CONTRIBUTING.md](CONTRIBUTING.md) or [open an issue](https://github.com/Jounce-lang/Jounce/issues)
