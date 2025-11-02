# Release Checklist

**Guide for preparing Jounce for public release**

**Versioning System**: See [VERSIONING.md](VERSIONING.md) for our sprint-based development workflow
**Last Updated**: November 1, 2025

---

## 🎯 Pre-Release Checklist

### 1. Code Quality ✅

- [x] All tests passing (635/635)
- [x] Zero critical bugs
- [x] Code formatted (`cargo fmt`)
- [x] No compiler warnings
- [x] Examples compile successfully
- [x] Documentation up to date

### 2. Community Files ✅

- [x] CODE_OF_CONDUCT.md created
- [x] CONTRIBUTING.md created
- [x] SECURITY.md created
- [x] LICENSE file (MIT)
- [x] Issue templates (.github/ISSUE_TEMPLATE/)
- [x] Pull request template
- [ ] FUNDING.yml (optional - add when ready)

### 3. Documentation ✅

- [x] README.md polished
- [x] CHANGELOG.md updated
- [x] ROADMAP.md current
- [x] All directory READMEs present
- [x] Getting started guide
- [x] Deployment guide
- [x] Testing guide

### 4. GitHub Repository Settings

#### Branch Protection (main)
- [ ] Require pull request reviews (1 reviewer minimum)
- [ ] Require status checks to pass
- [ ] Require branches to be up to date
- [ ] Disable force pushes
- [ ] Disable deletions

#### Security
- [ ] Enable Dependabot security updates
- [ ] Enable Dependabot version updates
- [ ] Enable CodeQL analysis
- [ ] Add SECURITY.md to repository
- [ ] Set up security policy

#### Features
- [ ] Enable GitHub Discussions
- [ ] Disable Wiki (use docs/ instead)
- [ ] Enable Issues
- [ ] Add issue labels:
  - `bug`, `enhancement`, `documentation`
  - `good first issue`, `help wanted`
  - `question`, `wontfix`, `duplicate`

#### Topics (GitHub tags)
- [ ] Add topics: `programming-language`, `compiler`, `rust`, `javascript`, `webassembly`, `reactivity`, `full-stack`, `ai-native`

### 5. Release Preparation

#### Determine Release Type

See [VERSIONING.md](VERSIONING.md) for complete details. In summary:

| Type | When to Use | Example |
|------|-------------|---------|
| **MAJOR** | Breaking changes | 0.9.x → 1.0.0 |
| **MINOR** | New features (backwards-compatible) | 0.8.x → 0.9.0 |
| **PATCH** | Bug fixes and docs | 0.8.1 → 0.8.2 |

#### Version Bump

```bash
# 1. Determine new version (see VERSIONING.md)
# - MAJOR: Breaking changes (0.x.x → 1.0.0)
# - MINOR: New features (0.8.x → 0.9.0)
# - PATCH: Bug fixes (0.8.1 → 0.8.2)

# 2. Update version in Cargo.toml
version = "0.9.0"

# 3. Update version in README.md badges
# Find and replace version numbers

# 4. Create git tag
git tag -a v0.9.0 -m "Release v0.9.0 - Example Applications"
git push origin v0.9.0
```

**For Sprint-Based Releases**:
- Each sprint should target a specific release (see ROADMAP.md)
- Example: Sprint 15.2 → v0.9.0 (MINOR - new example apps)

#### Release Notes Template
```markdown
# Jounce v0.8.1 "Developer Experience" 🚀

## 🎉 Highlights

- 457 CSS utility classes (Tailwind-inspired)
- Enhanced error messages (20+ error codes)
- 4 production-ready starter templates
- Complete documentation overhaul

## ✨ New Features

### CSS Utilities
- Auto-included in every compilation
- 457 classes covering layout, spacing, typography, colors
- Zero configuration required
- 19KB total (5KB gzipped)

### Error Messages
- Beautiful ANSI-colored diagnostics
- Helpful suggestions with 💡 icons
- Code examples showing correct usage
- 20+ error codes (E001-E079)

### Starter Templates
1. Minimal Counter (5 mins, Beginner)
2. Todo App (15 mins, Intermediate)
3. Form App (20 mins, Intermediate)
4. Dashboard (15 mins, Intermediate)

## 🐛 Bug Fixes

- None (zero known bugs!)

## 📚 Documentation

- Added READMEs for all directories
- Created CONTRIBUTING.md
- Created CODE_OF_CONDUCT.md
- Created SECURITY.md
- Updated all guides to v0.8.1

## 📊 Stats

- **Tests**: 635/635 passing (100%)
- **Lines of Code**: ~40,000 (Rust)
- **Documentation**: 3,000+ lines added
- **Examples**: 35+ working examples
- **Packages**: 35/100 complete

## 🚀 Quick Start

\`\`\`bash
# Install
cargo install jounce

# Try a template
cp -r templates/minimal-counter my-app
cd my-app
jnc compile main.jnc
cd dist && node server.js
\`\`\`

## 🔗 Resources

- [Documentation](https://jounce-lang.github.io/docs)
- [Examples](./examples/)
- [Templates](./templates/)
- [Roadmap](./ROADMAP.md)

## 💬 Feedback

- [Report a bug](https://github.com/Jounce-lang/Jounce/issues/new?template=bug_report.md)
- [Request a feature](https://github.com/Jounce-lang/Jounce/issues/new?template=feature_request.md)
- [Ask a question](https://github.com/Jounce-lang/Jounce/discussions)

## ❤️ Contributors

Thank you to all contributors who made this release possible!

## 🎯 What's Next

See [ROADMAP.md](./ROADMAP.md) for upcoming features:
- Hot reload & HMR
- Visual builder
- More example apps
- Package ecosystem expansion

---

**Full Changelog**: https://github.com/Jounce-lang/Jounce/compare/v0.8.0...v0.8.1
```

### 6. Release Process

#### Step 1: Prepare
```bash
# Ensure main is up to date
git checkout main
git pull origin main

# Run full test suite
cargo test --lib
./scripts/test_all_examples.sh

# Build release binary
cargo build --release

# Verify version
cargo run --release -- --version
```

#### Step 2: Tag Release
```bash
# Create annotated tag
git tag -a v0.8.1 -m "Release v0.8.1 - Developer Experience"

# Push tag
git push origin v0.8.1
```

#### Step 3: Create GitHub Release
1. Go to https://github.com/Jounce-lang/Jounce/releases/new
2. Select tag: `v0.8.1`
3. Release title: `v0.8.1 - Developer Experience`
4. Description: Use template above
5. Attach binaries (if applicable)
6. Mark as "Latest release"
7. Publish!

#### Step 4: Announce
- [ ] Post in GitHub Discussions
- [ ] Tweet/social media announcement
- [ ] Update website (if applicable)
- [ ] Reddit /r/programming, /r/rust (if appropriate)
- [ ] Hacker News "Show HN" (if appropriate)

---

## 📋 Post-Release

### Immediate
- [ ] Verify release appears correctly on GitHub
- [ ] Test installation: `cargo install jounce`
- [ ] Check documentation links work
- [ ] Monitor for issues

### Within 24 Hours
- [ ] Respond to initial feedback
- [ ] Fix any critical bugs
- [ ] Update project boards

### Within 1 Week
- [ ] Publish blog post (if applicable)
- [ ] Gather community feedback
- [ ] Plan next release

---

## 🚨 Emergency Rollback

If critical issues are discovered:

```bash
# Yank release from crates.io (if published)
cargo yank --vers 0.8.1

# Delete tag
git tag -d v0.8.1
git push origin :refs/tags/v0.8.1

# Delete GitHub release
# (Do this manually on GitHub)

# Revert and fix
git revert <commit-hash>
# Or
git reset --hard <previous-commit>

# Release hotfix
git tag -a v0.8.2 -m "Hotfix for v0.8.1"
```

---

## 📊 Release Metrics

**Track:**
- GitHub stars
- Issues opened/closed
- Pull requests
- Downloads (crates.io)
- Website traffic
- Community growth

---

## 🎯 Beta vs Stable

### Beta Release (Current)
- Version: `v0.8.x-beta`
- Audience: Early adopters
- Stability: Production-ready, but API may change
- Support: Best effort

### Stable Release (Future v1.0)
- Version: `v1.0.0+`
- Audience: Production users
- Stability: Guaranteed API stability
- Support: LTS for major versions

---

## ✅ Ready for Release?

**All checkboxes above marked?**
- Repository settings configured
- Community files in place
- Documentation complete
- Tests passing
- Release notes drafted

**Then you're ready to release Jounce to the world!** 🎉

---

---

## 📖 Additional Resources

- **[VERSIONING.md](VERSIONING.md)** - Complete versioning and sprint workflow guide
  - Daily flow template
  - Sprint and task naming conventions
  - Git branching strategy
  - Advanced tagging (beta, alpha, dev)

- **[ROADMAP.md](ROADMAP.md)** - Phase and sprint roadmap
  - Current phase and sprint details
  - Release calendar
  - Success metrics

- **[CHANGELOG.md](CHANGELOG.md)** - Detailed release history
  - All releases with version numbers
  - Features, fixes, and improvements

---

**Last Updated**: November 1, 2025
**Current Version**: v0.8.1
**Next Release**: v0.9.0 (November 21, 2025)
**Workflow**: Phase 15, Sprint 15.2 (Blog Platform)
