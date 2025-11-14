# Scripts

**Build, test, and development automation scripts**

---

## 🚀 Build Scripts

### build-for-deployment.sh
**Production build preparation**
```bash
./scripts/build-for-deployment.sh
```
- Builds release binary
- Runs all tests
- Generates production artifacts
- Validates deployment readiness

### compile-wasm.js
**WebAssembly compilation helper**
```bash
node scripts/compile-wasm.js input.jnc
```
- Compiles Jounce to WASM
- Optimizes output
- Generates sourcemaps

---

## 🧪 Test Scripts

### test_all_examples.sh
**Test all example applications**
```bash
./scripts/test_all_examples.sh
```
- Compiles every example in `examples/apps/`
- Validates successful compilation
- Reports any failures
- Summary statistics at end

### test_registry.sh
**Test package registry**
```bash
./scripts/test_registry.sh
```
- Tests registry server endpoints
- Validates package publishing
- Checks package downloads
- Integration tests

---

## 🛠️ Development Scripts

### watch.sh
**File watcher for auto-recompilation**
```bash
./scripts/watch.sh app.jnc
```
- Watches source file for changes
- Auto-recompiles on save
- Shows compilation errors
- Optional browser refresh

Usage:
```bash
# Watch and compile
./scripts/watch.sh examples/counter.jnc

# With auto-refresh
./scripts/watch.sh examples/counter.jnc --refresh
```

### hmr-server.js
**Hot Module Replacement server**
```bash
node scripts/hmr-server.js
```
- WebSocket server for HMR
- Pushes updates to browser
- No full page reload
- Preserves application state

### debug-helper.js
**Debugging utilities**
```bash
node scripts/debug-helper.js <command>
```
Commands:
- `ast` - Print AST for file
- `tokens` - Show tokenization
- `codegen` - Show generated code
- `trace` - Full compilation trace

---

## 🌐 Server Scripts

### serve.py
**Simple HTTP server for development**
```bash
python scripts/serve.py
```
- Serves `dist/` directory
- Default port: 8000
- Auto-opens browser
- CORS enabled

Usage:
```bash
# Default (port 8000)
python scripts/serve.py

# Custom port
python scripts/serve.py 3000

# Specific directory
python scripts/serve.py --dir dist/
```

---

## 📦 Utility Scripts

### rpm.js
**Release and package management**
```bash
node scripts/rpm.js <command>
```
Commands:
- `version` - Bump version
- `tag` - Create git tag
- `publish` - Publish to registry
- `release` - Full release process

### install.cmd
**Windows installation script**
```cmd
scripts\install.cmd
```
- Installs Jounce on Windows
- Sets up PATH
- Configures environment
- Installs dependencies

---

## 🎯 Common Workflows

### Development
```bash
# 1. Watch and auto-compile
./scripts/watch.sh app.jnc

# 2. Start dev server (in another terminal)
python scripts/serve.py

# 3. Open http://localhost:8000
```

### Testing
```bash
# Test all examples
./scripts/test_all_examples.sh

# Test specific app
jnc compile examples/apps/01-counter/main.jnc && echo "✓ Success"

# Test registry
./scripts/test_registry.sh
```

### Building
```bash
# Development build
cargo build

# Production build
./scripts/build-for-deployment.sh

# WASM build
node scripts/compile-wasm.js app.jnc
```

### Releasing
```bash
# 1. Bump version
node scripts/rpm.js version patch

# 2. Run tests
./scripts/test_all_examples.sh

# 3. Build release
./scripts/build-for-deployment.sh

# 4. Publish
node scripts/rpm.js publish
```

---

## 📝 Adding New Scripts

**Guidelines:**
1. Use descriptive names
2. Add to this README
3. Include usage examples
4. Add error handling
5. Support `--help` flag
6. Make executable: `chmod +x script.sh`

**Template:**
```bash
#!/bin/bash
# Script: my-script.sh
# Description: What this script does
# Usage: ./scripts/my-script.sh [options]

set -e  # Exit on error

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --help)
            echo "Usage: $0 [options]"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
    shift
done

# Main logic here
echo "Running script..."
```

---

## 🔗 Resources

- [Deployment Guide](../DEPLOYMENT_GUIDE.md)
- [Testing Guide](../TESTING_GUIDE.md)
- [Development Setup](../docs/guides/DEVELOPMENT_SETUP.md)

---

**Last Updated**: November 7, 2025 (v0.8.3)
