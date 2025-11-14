# Jounce Package Manager - Complete Guide

**Version**: v0.8.3
**Last Updated**: November 7, 2025
**Status**: ✅ Production Ready (Package Registry Server Complete)

The Jounce package manager (`jnc pkg`) provides a complete solution for managing dependencies, packages, and builds in your Jounce projects.

> **Quick Start**: See [README.md](../../README.md) for installation and basic usage
> **Technical Details**: See [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md) for package system specification

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Package Management](#package-management)
3. [Developer Experience](#developer-experience)
4. [Build Cache](#build-cache)
5. [Registry Integration](#registry-integration)
6. [Command Reference](#command-reference)

---

## Getting Started

### Initializing a New Package

Create a new `jounce.toml` manifest in your project:

```bash
jnc pkg init
```

This creates a `jounce.toml` file with package metadata:

```toml
[package]
name = "my-package"
version = "0.1.0"
authors = ["Developer <dev@example.com>"]
description = ""
license = "MIT"
repository = ""
homepage = ""
keywords = []

[dependencies]

[dev-dependencies]
```

---

## Package Management

### Installing Dependencies

Install all dependencies from `jounce.toml`:

```bash
jnc pkg install
```

**Output:**
```
📦 Resolving dependencies...
📥 Installing 5 packages...
  📥 Installing jounce-store @ 1.0.0
  📥 Downloading jounce-store v1.0.0
  ...
✅ All dependencies installed!
```

**Features:**
- ✅ Transitive dependency resolution
- ✅ Circular dependency detection
- ✅ Dependency deduplication
- ✅ Lock file generation (`jounce.lock`)

### Adding Dependencies

Add a new dependency to your project:

```bash
jnc pkg add jounce-store
jnc pkg add jounce-http --version "^0.1.0"
jnc pkg add test-pkg --dev  # Add to dev-dependencies
```

### Removing Dependencies

Remove a dependency:

```bash
jnc pkg remove jounce-store
```

### Updating Dependencies

Update all dependencies to latest compatible versions:

```bash
jnc pkg update
```

This removes the lock file and re-resolves all dependencies to find the newest compatible versions.

---

## Developer Experience

### Viewing the Dependency Tree

Visualize your complete dependency graph:

```bash
jnc pkg tree
```

**Output:**
```
full-stack-demo v0.1.0
├── jounce-forms v1.0.0
│   └── jounce-store v1.0.0
├── jounce-http v0.1.0
├── jounce-i18n v1.0.0
│   └── jounce-store v1.0.0
├── jounce-router v0.1.0
└── jounce-store v1.0.0
```

**Features:**
- Beautiful Unicode tree visualization
- Shows transitive dependencies with nesting
- Alphabetically sorted for easy reading

### Checking for Outdated Packages

Check if any dependencies have newer versions:

```bash
jnc pkg outdated
```

**Output:**
```
Checking for outdated dependencies...

📦 some-package
   Current: 1.0.0 | Latest: 2.0.0 | Wanted: ^1.0.0

💡 Run 'jnc pkg update' to update to latest compatible versions
```

### Listing Installed Packages

View all installed packages:

```bash
jnc pkg list
```

**Output:**
```
Installed packages:

📦 jounce-store @ 1.0.0
📦 jounce-i18n @ 1.0.0
   Dependencies: jounce-store
📦 jounce-forms @ 1.0.0
   Dependencies: jounce-store
📦 jounce-http @ 0.1.0
📦 jounce-router @ 0.1.0

✅ Total: 5 packages
```

### Package Information

Get detailed information about a package:

```bash
jnc pkg info jounce-store
```

**Output:**
```
Fetching package information...

📦 jounce-store
   Advanced state management library for Jounce applications

Latest version: 1.0.0

Available versions:
   • 1.0.0

Statistics:
   Total downloads: 156
   Downloads (last month): 42
   Repository: https://github.com/jounce/jounce-store

Keywords: state, reactive, store, signals

✅ Installed: v1.0.0
```

---

## Build Cache

The package manager includes an intelligent build cache system to speed up compilation.

### Cache Location

Build artifacts are cached in:
```
~/.jnc/cache/
```

### Viewing Cache Statistics

```bash
jnc pkg cache
```

**Output:**
```
Build Cache Statistics:

Location: /Users/you/.jnc/cache
Cached packages: 3

Cached builds:
  📦 jounce-store@1.0.0 (compiled 2h ago)
  📦 jounce-http@0.1.0 (compiled 5m ago)
  📦 jounce-forms@1.0.0 (compiled 1d ago)
```

### Clearing the Cache

```bash
jnc pkg clean
```

**Output:**
```
✅ Build cache cleared
```

### How Caching Works

1. **Source Hashing**: Package source files are hashed to detect changes
2. **Cache Validation**: Before using cache, hashes are verified
3. **Automatic Invalidation**: Cache is invalidated when source changes
4. **Timestamp Tracking**: Shows when each package was last compiled

---

## Registry Integration

**New in v0.8.3**: Full package registry server with authentication, JWT tokens, and rate limiting!

### Searching for Packages

Search the registry for packages:

```bash
jnc pkg search http
```

**Output:**
```
Found 2 packages:

📦 jounce-http @ 0.1.0
   HTTP client and server for Jounce
   Keywords: http, client, server, api
   Downloads: 203 | Score: 8.50

📦 jounce-api @ 1.2.0
   REST API utilities
   Keywords: api, rest, http
   Downloads: 89 | Score: 7.20
```

### Publishing Packages

First, register an account:

```bash
jnc pkg register
```

**v0.8.3 Features**:
- Secure password hashing with bcrypt
- JWT-based authentication
- Email validation
- Rate limiting protection

Then login:

```bash
jnc pkg login
```

Finally, publish your package:

```bash
jnc pkg publish
```

**Requirements:**
- Valid `jounce.toml` with all required fields
- Unique package name
- Valid semantic version
- Source files in `src/` directory

**Registry Features (v0.8.3)**:
- ✅ User registration & authentication
- ✅ Package publishing & versioning
- ✅ Download statistics tracking
- ✅ Search with keyword matching
- ✅ Rate limiting (100 req/15min)
- ✅ SQLite database backend
- ✅ JWT token authentication

---

## Command Reference

### Core Commands

| Command | Description |
|---------|-------------|
| `jnc pkg init` | Initialize new package manifest |
| `jnc pkg install` | Install all dependencies |
| `jnc pkg add <name>` | Add a dependency |
| `jnc pkg remove <name>` | Remove a dependency |
| `jnc pkg update` | Update dependencies to latest versions |

### Developer Tools

| Command | Description |
|---------|-------------|
| `jnc pkg tree` | Display dependency tree |
| `jnc pkg outdated` | Check for outdated dependencies |
| `jnc pkg list` | List installed packages |
| `jnc pkg info <name>` | Show package details |

### Registry

| Command | Description |
|---------|-------------|
| `jnc pkg search <query>` | Search for packages |
| `jnc pkg register` | Register new account |
| `jnc pkg login` | Login to registry |
| `jnc pkg publish` | Publish package |

### Build Cache

| Command | Description |
|---------|-------------|
| `jnc pkg cache` | Show cache statistics |
| `jnc pkg clean` | Clear build cache |

---

## File Structure

A typical Jounce project with dependencies:

```
my-project/
├── jounce.toml          # Package manifest
├── jounce.lock          # Dependency lock file
├── src/
│   └── main.jnc         # Source code
├── jounce_modules/      # Downloaded dependencies
│   ├── jounce-store/
│   ├── jounce-http/
│   └── jounce-forms/
└── dist/                # Compiled output
```

Global cache location:

```
~/.jnc/
├── credentials.json     # Registry authentication (JWT tokens)
└── cache/               # Build cache
    ├── index.json       # Cache metadata
    └── builds/          # Compiled artifacts
```

---

## Advanced Features

### Transitive Dependencies

The package manager automatically resolves and installs transitive dependencies:

```toml
# Your jounce.toml
[dependencies]
jounce-i18n = "^1.0.0"
```

If `jounce-i18n` depends on `jounce-store`, both will be installed automatically.

### Circular Dependency Detection

The package manager detects circular dependencies and provides clear error messages:

```
❌ Circular dependency detected: package-a -> package-b -> package-c -> package-a
```

### Semantic Versioning

Version specifiers follow semver:

- `1.0.0` - Exact version
- `^1.0.0` - Compatible (>=1.0.0, <2.0.0)
- `~1.0.0` - Patch level (>=1.0.0, <1.1.0)
- `>=1.0.0` - Greater than or equal

### Lock File (`jounce.lock`)

The lock file ensures reproducible builds:

```toml
version = "1"

[[packages]]
name = "jounce-store"
version = "1.0.0"
dependencies = []

[packages.source]
type = "Registry"
url = "https://packages.jounce.dev/jounce-store/1.0.0"

[[packages]]
name = "jounce-i18n"
version = "1.0.0"
dependencies = ["jounce-store"]

[packages.source]
type = "Registry"
url = "https://packages.jounce.dev/jounce-i18n/1.0.0"
```

---

## Best Practices

### 1. Version Pinning

Use `^` for libraries (allows minor updates):
```toml
[dependencies]
jounce-store = "^1.0.0"  # Allows 1.x.x
```

Use exact versions for critical dependencies:
```toml
[dependencies]
core-lib = "2.5.0"  # Exact version
```

### 2. Keep Dependencies Updated

Regularly check for outdated dependencies:

```bash
jnc pkg outdated
jnc pkg update
```

### 3. Use Lock Files

Always commit `jounce.lock` to version control for reproducible builds.

### 4. Clean Cache Periodically

Clear old build artifacts:

```bash
jnc pkg clean
```

### 5. Minimal Dependencies

Only add dependencies you actually need. Check with:

```bash
jnc pkg tree  # See what you're really pulling in
```

---

## Troubleshooting

### Lock File Not Found

```
❌ jounce.lock not found. Run 'jnc pkg install' first.
```

**Solution**: Run `jnc pkg install` to create the lock file.

### Circular Dependency

```
❌ Circular dependency detected: A -> B -> C -> A
```

**Solution**: Review your dependencies and break the circular reference.

### Package Not Found

```
❌ Package 'unknown-package' not found in registry
```

**Solution**:
- Check the package name spelling
- Search the registry: `jnc pkg search unknown`
- Verify the package exists in the registry

### No Compatible Version

```
❌ No compatible version found for package @ ^2.0.0
```

**Solution**:
- Check available versions: `jnc pkg info package`
- Adjust version requirement in `jounce.toml`
- Update dependencies: `jnc pkg update`

### Authentication Failed

```
❌ Authentication failed. Please login.
```

**Solution**:
```bash
jnc pkg login
```

---

## Performance

### Cache Hit Rates

With caching enabled:
- **First install**: Full download and compilation
- **Subsequent installs**: Instant (from cache)
- **After source change**: Selective recompilation

### Parallel Operations

The package manager performs several operations in parallel:
- Dependency resolution
- Package downloads
- Cache validation

---

## Available Packages (v0.8.3)

Browse the full package ecosystem at the registry. Popular packages include:

**UI & Components**:
- `jounce-ui` - Component library
- `jounce-theme` - Theming system
- `jounce-animate` - Animation utilities

**Backend**:
- `jounce-db` - Database adapters
- `jounce-auth` - Authentication
- `jounce-cache` - Caching

**Utilities**:
- `jounce-router` - Client routing
- `jounce-http` - HTTP client
- `jounce-forms` - Form handling

See [README.md § Package Ecosystem](../../README.md#package-ecosystem) for complete list.

---

## What's Next?

- **Getting Started**: See [README.md](../../README.md) for installation
- **Tutorials**: See [LEARN_JOUNCE.md](../guides/LEARN_JOUNCE.md) for practical examples
- **Technical Reference**: See [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md) for complete specification

---

## Future Roadmap (v0.9.0+)

- [ ] Workspace support (monorepos)
- [ ] Private registry support
- [ ] Package aliasing
- [ ] Patch dependencies
- [ ] Dependency overrides
- [ ] Security vulnerability audit
- [ ] Performance benchmarks

---

**Version**: v0.8.3 "Enhanced Language Features"
**Status**: ✅ Production Ready (580/580 tests passing)
**Registry**: ✅ Complete with authentication, JWT, rate limiting
