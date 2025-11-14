# Jounce Package Manager - Quick Start Guide

> **Note**: For authoritative language rules, see [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md).

Complete guide to using the Jounce package manager (`jnc pkg`).

## Table of Contents

1. [Creating Your First Package](#creating-your-first-package)
2. [Using Packages in Your App](#using-packages-in-your-app)
3. [Publishing to the Registry](#publishing-to-the-registry)
4. [Common Workflows](#common-workflows)

---

## Creating Your First Package

### Step 1: Initialize Package

```bash
mkdir my-ui-lib
cd my-ui-lib
jnc pkg init
```

This creates a `jounce.toml` manifest:

```toml
[package]
name = "my-ui-lib"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = ""
license = "MIT"
repository = ""
homepage = ""
keywords = []

[dependencies]

[dev-dependencies]

[build]
target = ""
optimize = false
ssr = false
hydrate = false

[features]
```

### Step 2: Create Your Package Structure

```bash
mkdir src
touch src/lib.jnc
```

### Step 3: Add Components (src/lib.jnc)

```jounce
// src/lib.jnc - Main library file

/// A simple button component
pub component Button(
    text: String,
    onClick: fn(),
    variant: String // "primary" | "secondary" | "danger"
) {
    let buttonClass = match variant {
        "primary" => "btn btn-primary",
        "secondary" => "btn btn-secondary",
        "danger" => "btn btn-danger",
        _ => "btn",
    };

    <button class={buttonClass} on:click={onClick}>
        {text}
    </button>
}

/// A card container component
pub component Card(title: String, children: Vec<Node>) {
    <div class="card">
        <div class="card-header">
            <h3>{title}</h3>
        </div>
        <div class="card-body">
            {children}
        </div>
    </div>
}

/// Input component with validation
pub component Input(
    value: Signal<String>,
    placeholder: String,
    type_: String // "text" | "email" | "password"
) {
    <input
        type={type_}
        placeholder={placeholder}
        value={value.value}
        on:input={|e| value.set(e.target.value)}
    />
}
```

### Step 4: Update Package Metadata

Edit `jounce.toml`:

```toml
[package]
name = "my-ui-lib"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "A simple UI component library for Jounce"
license = "MIT"
repository = "https://github.com/username/my-ui-lib"
keywords = ["ui", "components", "design-system"]

[dependencies]
# No dependencies for this simple example

[features]
default = ["button", "card", "input"]
button = []
card = []
input = []
```

---

## Using Packages in Your App

### Step 1: Initialize Your App

```bash
mkdir my-app
cd my-app
jnc pkg init
```

### Step 2: Add Dependencies

Manually edit `jounce.toml`:

```toml
[package]
name = "my-app"
version = "1.0.0"
authors = ["You <you@example.com>"]

[dependencies]
# Official packages from Jounce registry
raven-ui = "^1.0.0"          # UI components
raven-store = "^1.0.0"       # State management
raven-http = "^0.1.0"        # HTTP client
```

Or use the CLI:

```bash
# Note: These commands will work once the registry is running
jnc pkg add raven-ui
jnc pkg add raven-store@1.0.0  # Specific version
```

### Step 3: Install Dependencies

```bash
jnc pkg install
```

Output:
```
📦 Resolving dependencies...
📥 Installing 3 packages...
✅ All dependencies installed!
```

This creates:
- `jounce.lock` - Lock file for reproducible builds
- `.jnc_packages/` - Downloaded packages directory

### Step 4: Import and Use Components

Create `src/main.jnc`:

```jounce
// Import from installed packages
// Note: Packages are downloaded to .jnc_packages/
use ./.jnc_packages/raven-ui/src/lib::{Button, Card, Input};
use ./.jnc_packages/raven-store/src/lib::{createStore};

fn main() {
    let count = signal(0);

    <div>
        <h1>My App</h1>

        <Card title="Counter Demo">
            <p>Count: {count.value}</p>
            <Button
                text="Increment"
                onClick={|| count.set(count.value + 1)}
                variant="primary"
            />
        </Card>
    </div>
}
```

### Step 5: Compile and Run

```bash
jnc compile src/main.jnc
node dist/server.js
```

---

## Publishing to the Registry

### Step 1: Register Account (First Time Only)

```bash
jnc pkg register
```

You'll be prompted for:
- Username
- Email
- Password

### Step 2: Login

```bash
jnc pkg login
```

This stores your auth token in `~/.jnc/credentials`.

### Step 3: Prepare for Publishing

Ensure your `jounce.toml` is complete:

```toml
[package]
name = "my-ui-lib"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "A simple UI component library"  # Required
license = "MIT"                                 # Required
repository = "https://github.com/you/my-ui-lib"
keywords = ["ui", "components"]                 # Recommended
```

### Step 4: Publish

```bash
jnc pkg publish
```

Output:
```
📦 Building package tarball...
📤 Publishing my-ui-lib v0.1.0...
✅ Published successfully!

View at: https://jounce-registry.fly.dev/packages/my-ui-lib
```

---

## Common Workflows

### Updating Dependencies

```bash
# Check for outdated packages
jnc pkg outdated

# Update all to latest compatible versions
jnc pkg update

# Update specific package
jnc pkg add raven-ui@1.2.0
```

### Managing Dependencies

```bash
# Add dependency
jnc pkg add package-name

# Add with specific version
jnc pkg add package-name@1.2.3

# Remove dependency
jnc pkg remove package-name

# List installed packages
jnc pkg list

# Show dependency tree
jnc pkg tree
```

### Package Information

```bash
# Search registry
jnc pkg search "ui components"
jnc pkg search raven

# Get package info
jnc pkg info raven-ui

# Show package details
jnc pkg info raven-ui@1.0.0
```

### Cache Management

```bash
# Show cache stats
jnc pkg cache

# Clean cache
jnc pkg clean
```

### Security

```bash
# Audit for vulnerabilities
jnc pkg audit
```

---

## Best Practices

### 1. Version Your Package Properly

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** version (1.0.0 → 2.0.0): Breaking changes
- **MINOR** version (1.0.0 → 1.1.0): New features, backward compatible
- **PATCH** version (1.0.0 → 1.0.1): Bug fixes

### 2. Use Appropriate Version Ranges

```toml
[dependencies]
# Caret: Allow MINOR and PATCH updates
raven-ui = "^1.2.3"     # 1.2.3 <= version < 2.0.0

# Tilde: Allow PATCH updates only
raven-http = "~1.2.3"   # 1.2.3 <= version < 1.3.0

# Exact: Pin to specific version
raven-store = "=1.2.3"  # Exactly 1.2.3
```

### 3. Commit Your Lock File

Always commit `jounce.lock` to ensure reproducible builds:

```bash
git add jounce.lock
git commit -m "Update dependencies"
```

### 4. Use `pub` Keyword for Exports

Mark public API clearly:

```jounce
// Public API
pub component Button() { ... }
pub fn helper() { ... }

// Private implementation details
fn internal_helper() { ... }
```

### 5. Document Your Package

Create a `README.md`:

```markdown
# my-ui-lib

A simple UI component library for Jounce.

## Installation

\`\`\`bash
jnc pkg add my-ui-lib
\`\`\`

## Usage

\`\`\`jounce
use ./.jnc_packages/my-ui-lib/src/lib::{Button, Card};

fn main() {
    <Button text="Click me" onClick={|| console.log("Clicked!")} />
}
\`\`\`
```

### 6. Use Features for Optional Functionality

```toml
[features]
default = ["button", "card"]
button = []
card = []
input = []
all = ["button", "card", "input"]
```

Users can enable features:

```toml
[dependencies]
my-ui-lib = { version = "^1.0.0", features = ["all"] }
```

---

## Available Official Packages

The Jounce registry currently hosts:

| Package | Version | Description |
|---------|---------|-------------|
| **raven-ui** | 1.0.0 | UI component library (Button, Input, Card, etc.) |
| **raven-store** | 1.0.0 | Advanced state management with signals |
| **raven-http** | 0.1.0 | HTTP client for API requests |
| **raven-forms** | 1.0.0 | Form handling and validation |
| **raven-router** | 0.1.0 | Client-side routing for SPAs |
| **raven-i18n** | 1.0.0 | Internationalization (i18n) library |

Browse more:
```bash
jnc pkg search ""
```

---

## Troubleshooting

### "Registry connection failed"

The registry might not be running. Check:

```bash
curl http://localhost:4000/health
```

### "Authentication required"

Login to the registry:

```bash
jnc pkg login
```

### "Package not found"

Search for the package:

```bash
jnc pkg search package-name
```

### "Lock file mismatch"

Reinstall dependencies:

```bash
rm jounce.lock
jnc pkg install
```

---

## Next Steps

1. **Read the full guide**: See [LEARN_JOUNCE.md](./LEARN_JOUNCE.md) for complete language documentation
2. **Explore examples**: Check out example apps in `examples/apps/`
3. **Join the community**: Report issues at https://github.com/jrezin1201/jounce

---

*Last updated: November 2025 - Jounce v0.8.3*
