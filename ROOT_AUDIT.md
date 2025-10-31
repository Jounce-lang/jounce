# Root Directory Audit - Complete File Analysis

**Date**: October 31, 2025

---

## ✅ KEEP - Essential Files/Directories

### Build & Configuration
- `Cargo.toml` - Rust project configuration (required)
- `Cargo.lock` - Rust dependency lockfile (required)
- `package.json` - Node.js dependencies for runtime (required)
- `package-lock.json` - Node.js lockfile (required)

### Documentation (7 files)
- `README.md` - Main project documentation (essential for GitHub)
- `CHANGELOG.md` - Version history and release notes (required)
- `CLAUDE.md` - Development instructions for AI assistant (active development)
- `ROADMAP.md` - Project roadmap and future plans (active)
- `DEPLOYMENT_GUIDE.md` - Deployment instructions (user-facing)
- `GETTING_STARTED.md` - Quick start guide (user-facing)
- `TESTING_GUIDE.md` - Testing instructions (developer-facing)

### Source & Core Directories
- `src/` - Source code (ESSENTIAL)
- `docs/` - Documentation files (user-facing)
- `examples/` - Example applications (user-facing)
- `templates/` - Starter templates (NEW - user-facing)
- `packages/` - Package ecosystem (active development)
- `tests/` - Integration tests (required)
- `benches/` - Performance benchmarks (active)
- `archive/` - Historical documentation (organized storage)

### Build Output (Git-ignored)
- `target/` - Rust build artifacts (gitignored)
- `dist/` - Compilation output (gitignored)
- `node_modules/` - Node dependencies (gitignored)

### Active Projects
- `docs-site/` - Live documentation website (active)
- `registry/` - Package registry server (active infrastructure)
- `runtime/` - Client/server runtime files (core functionality)
- `vscode-extension/` - VSCode extension (active tooling)
- `scripts/` - Build and utility scripts (infrastructure)

---

## 🔄 MOVE - Files in Wrong Location

### Test Files (Move to tests/)
- `test_all_examples.sh` → Move to `scripts/test_all_examples.sh`
- `test_all_new_operators.jnc` → Move to `tests/operators/test_all_new_operators.jnc`
- `test_logical_assign.jnc` → Move to `tests/operators/test_logical_assign.jnc`
- `test_modules/` → Move to `tests/modules/`
- `test_new_operators.jnc` → Move to `tests/operators/test_new_operators.jnc`
- `test_nullish_simple.jnc` → Move to `tests/operators/test_nullish_simple.jnc`
- `test_optional_chain.jnc` → Move to `tests/operators/test_optional_chain.jnc`
- `test/` → Merge into `tests/` (duplicate)
- `test-project/` → Move to `tests/fixtures/test-project/`

### Scripts
- `watch.sh` → Move to `scripts/watch.sh`

### Apps
- `apps/11_todo_list.jnc` → Move to `examples/apps/11-todo-list/main.jnc` (for consistency)

---

## ❓ INVESTIGATE - Unclear Purpose

### vscode-raven/
**Status**: Old VSCode extension (Raven was the old name)
**Action**: ARCHIVE or DELETE
**Reason**: Superseded by `vscode-extension/`, "Raven" was old project name

---

## 📊 Summary

**KEEP**: 24 files/directories (essential)
**MOVE**: 11 files (wrong location)
**INVESTIGATE**: 1 directory (vscode-raven)

---

## 🎯 Recommended Actions

### Immediate (High Priority)
1. Move test files to `tests/` directory
2. Move scripts to `scripts/` directory
3. Consolidate `test/` into `tests/`
4. Move `apps/` content to `examples/apps/`

### Review (Medium Priority)
1. Check if `vscode-raven/` is still needed
2. If not, archive or delete it

### After Cleanup
- Root should have ~15 items (mostly directories + 7 essential docs)
- All test files organized in `tests/`
- All scripts organized in `scripts/`
- Clean separation of concerns

---

## 🔍 File-by-File Justification

| File/Dir | Keep? | Reason |
|----------|-------|--------|
| `Cargo.toml` | ✅ | Rust project config |
| `Cargo.lock` | ✅ | Rust dependency lockfile |
| `package.json` | ✅ | Node.js dependencies |
| `package-lock.json` | ✅ | Node.js lockfile |
| `README.md` | ✅ | Main docs |
| `CHANGELOG.md` | ✅ | Version history |
| `CLAUDE.md` | ✅ | Dev instructions |
| `ROADMAP.md` | ✅ | Future plans |
| `DEPLOYMENT_GUIDE.md` | ✅ | Deployment guide |
| `GETTING_STARTED.md` | ✅ | Quick start |
| `TESTING_GUIDE.md` | ✅ | Testing guide |
| `src/` | ✅ | Source code |
| `docs/` | ✅ | Documentation |
| `examples/` | ✅ | Example apps |
| `templates/` | ✅ | Starter templates |
| `packages/` | ✅ | Package ecosystem |
| `tests/` | ✅ | Integration tests |
| `benches/` | ✅ | Benchmarks |
| `archive/` | ✅ | Historical docs |
| `target/` | ✅ | Build artifacts (gitignored) |
| `dist/` | ✅ | Output (gitignored) |
| `node_modules/` | ✅ | Dependencies (gitignored) |
| `docs-site/` | ✅ | Live docs website |
| `registry/` | ✅ | Package registry |
| `runtime/` | ✅ | Runtime files |
| `vscode-extension/` | ✅ | VSCode extension |
| `scripts/` | ✅ | Build scripts |
| `test_all_examples.sh` | 🔄 | Move to scripts/ |
| `test_*.jnc` | 🔄 | Move to tests/operators/ |
| `test_modules/` | 🔄 | Move to tests/modules/ |
| `test/` | 🔄 | Merge into tests/ |
| `test-project/` | 🔄 | Move to tests/fixtures/ |
| `watch.sh` | 🔄 | Move to scripts/ |
| `apps/` | 🔄 | Move to examples/apps/ |
| `vscode-raven/` | ❓ | Archive/delete if old |

---

**Next Step**: Execute the moves and cleanup!
