# Jounce CLI Build Flags Reference

**Version**: v0.11.0 (Phase 17)
**Last Updated**: November 1, 2025

---

## 🚀 Quick Start

```bash
# Development build (fast, unoptimized)
jnc build

# Production build (slow, optimized)
jnc build --release

# Production with source maps
jnc build --release --sourcemap

# Analyze bundle size
jnc build --release --analyze
```

---

## 📋 All Flags

### `--release`
**Build for production with all optimizations**

```bash
jnc build --release
```

**Enables**:
- Dead code elimination
- Tree shaking
- Minification
- Code splitting
- No source maps

**Use When**: Deploying to production

---

### `--minify`
**Only minify code (no other optimizations)**

```bash
jnc build --minify
```

**Enables**:
- Identifier shortening
- Whitespace removal
- Comment removal

**Use When**: Debugging optimization issues, want smaller bundles but keep DCE off

---

### `--sourcemap`
**Generate source maps**

```bash
jnc build --release --sourcemap
```

**Generates**: `.map` files for debugging

**Use When**: Need to debug production code

---

### `--analyze`
**Generate bundle size analysis**

```bash
jnc build --release --analyze
```

**Generates**: `bundle-analysis.html`

**Shows**:
- Largest modules
- Bundle composition
- Optimization opportunities

**Use When**: Optimizing bundle size

---

### `--optimize <level>`
**Set optimization level (0-3)**

```bash
jnc build --optimize 0  # None
jnc build --optimize 1  # Basic (minify only)
jnc build --optimize 2  # Standard (DCE + minify)
jnc build --optimize 3  # Aggressive (all optimizations)
```

**Levels**:
- `0`: No optimization (dev)
- `1`: Minification only
- `2`: DCE + Minification
- `3`: DCE + Tree Shaking + Minification + Code Splitting

---

### `--no-dce`
**Disable dead code elimination**

```bash
jnc build --release --no-dce
```

**Use When**: Debugging DCE issues

---

### `--no-tree-shaking`
**Disable tree shaking**

```bash
jnc build --release --no-tree-shaking
```

**Use When**: Debugging tree shaking issues

---

### `--no-splitting`
**Disable code splitting**

```bash
jnc build --release --no-splitting
```

**Use When**: Want single bundle file

---

### `--output <dir>`
**Specify output directory**

```bash
jnc build --output dist/
```

**Default**: `./dist`

---

### `--target <env>`
**Set target environment**

```bash
jnc build --target browser    # Browser (default)
jnc build --target node        # Node.js
jnc build --target deno        # Deno
```

---

## 📊 Comparison Matrix

| Flag | Dev Build | --minify | --release | --optimize 3 |
|------|-----------|----------|-----------|--------------|
| **DCE** | ❌ | ❌ | ✅ | ✅ |
| **Tree Shaking** | ❌ | ❌ | ✅ | ✅ |
| **Minify** | ❌ | ✅ | ✅ | ✅ |
| **Code Split** | ❌ | ❌ | ✅ | ✅ |
| **Source Maps** | ✅ | ❌ | ❌ | ❌ |
| **Bundle Size** | 200KB | 140KB | 100KB | 100KB |
| **Compile Time** | 2s | 3s | 8s | 8s |

---

## 🎯 Common Use Cases

### Development
```bash
jnc build
# or
jnc dev  # With hot reload
```

### Production Deployment
```bash
jnc build --release
```

### Production with Debugging
```bash
jnc build --release --sourcemap
```

### Bundle Size Investigation
```bash
jnc build --release --analyze
```

### Testing Optimizations
```bash
# Test without DCE
jnc build --release --no-dce

# Test without tree shaking
jnc build --release --no-tree-shaking
```

---

**See Also**:
- [Build Optimizations](./BUILD_OPTIMIZATIONS.md)
- [Deployment Guide](./DEPLOYMENT_GUIDE.md)
