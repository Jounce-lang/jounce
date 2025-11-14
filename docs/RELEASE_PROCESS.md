# Jounce Release Process (v0.8.x → v0.9.x)

> **Note**: For authoritative language rules, see [JOUNCE_SPEC.md](../JOUNCE_SPEC.md).

This document outlines the standard release process for Jounce compiler versions.

---

## Pre-Release Checklist

### 1. Run Release Guard

Run the comprehensive validation suite:

```bash
make verify
```

This executes:
- ✅ Example verification (syntax + compilation)
- ✅ Documentation link validation
- ✅ Maintainer footer verification
- ✅ Golden docs consistency checks
- ✅ Full Rust test suite

**All checks must pass before proceeding.**

---

### 2. Run Test Suite

Verify all unit and integration tests pass:

```bash
cargo test --tests
```

Expected output: `test result: ok. XXX passed; 0 failed`

---

### 3. Update Version Numbers

Update version in the following files:

**JOUNCE_SPEC.md**:
```diff
-**Version**: v0.8.3
+**Version**: v0.9.0
```

**README.md**:
```diff
-**Current**: v0.8.3 "Enhanced Language Features"
+**Current**: v0.9.0 "Next Release Name"
```

**Cargo.toml**:
```diff
-version = "0.8.1"
+version = "0.9.0"
```

---

### 4. Update CHANGELOG.md

Add release notes at the top of CHANGELOG.md:

```markdown
## [v0.9.0] - YYYY-MM-DD

### Added
- Feature 1
- Feature 2

### Fixed
- Bug fix 1
- Bug fix 2

### Changed
- Breaking change 1
```

---

### 5. Commit Version Bump

```bash
git add JOUNCE_SPEC.md README.md Cargo.toml CHANGELOG.md
git commit -m "chore: Bump version to v0.9.0"
```

---

### 6. Tag Release

Create annotated git tag:

```bash
git tag -a v0.9.0 -m "Release v0.9.0"
```

---

### 7. Push Tags

Push commits and tags to remote:

```bash
git push origin main
git push origin v0.9.0
```

---

### 8. Confirm CI Green

Monitor GitHub Actions:
- Verify all workflow checks pass
- Confirm `release-guard` job succeeds on tag

---

## Post-Release

### 9. Publish to Crates.io (Optional)

If publishing to crates.io:

```bash
cargo publish
```

---

### 10. Create GitHub Release

1. Go to GitHub Releases page
2. Click "Draft a new release"
3. Select tag `v0.9.0`
4. Copy CHANGELOG.md section as release notes
5. Publish release

---

## Version Numbering

Jounce follows semantic versioning:

- **Major (X.0.0)**: Breaking changes, incompatible API changes
- **Minor (0.X.0)**: New features, backward-compatible
- **Patch (0.0.X)**: Bug fixes, backward-compatible

Current series: **v0.8.x** → Next: **v0.9.x** → Future: **v1.0.0**

---

## Emergency Hotfix Process

For critical bugs in production:

1. Create hotfix branch from tag: `git checkout -b hotfix/v0.8.4 v0.8.3`
2. Apply fix and tests
3. Run `make verify`
4. Update version to v0.8.4 (patch bump)
5. Commit, tag, and push
6. Merge back to main

---

## Rollback Procedure

If a release has critical issues:

1. Identify last stable tag: `git tag -l`
2. Create rollback branch: `git checkout -b rollback/v0.8.3 v0.8.3`
3. Push as new patch version or revert commits on main
4. Communicate issue to users

---

## Checklist Summary

Before any release:

- [ ] `make verify` passes
- [ ] `cargo test --tests` passes
- [ ] Version numbers updated (SPEC, README, Cargo.toml)
- [ ] CHANGELOG.md updated
- [ ] Commits pushed
- [ ] Tag created and pushed
- [ ] CI confirms green
- [ ] Release notes published

---

**Maintained by: **The Jounce Project****
