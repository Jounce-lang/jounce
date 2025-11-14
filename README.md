# 🎉 Jounce

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/Jounce-lang/Jounce)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.8.3-orange)](https://github.com/Jounce-lang/Jounce/releases)
[![Tests](https://img.shields.io/badge/tests-580%2F580-brightgreen)](TESTING_GUIDE.md)
[![Docs](https://img.shields.io/badge/docs-complete-blue)](docs/)

**A single-file, full-stack reactive language that compiles to Node.js.**

> ✅ **Production Ready**: 580/580 tests passing, zero known bugs, fine-grained reactivity, comprehensive safety features.

---

## Golden Docs

For authoritative language rules, see **[JOUNCE_SPEC.md](JOUNCE_SPEC.md)**.

**Core Documentation**:
- **[JOUNCE_SPEC.md](JOUNCE_SPEC.md)** - Authoritative language specification
- **[docs/guides/LEARN_JOUNCE.md](docs/guides/LEARN_JOUNCE.md)** - Learning guide and tutorials
- **[docs/JOUNCE_DO_AND_DONT.md](docs/JOUNCE_DO_AND_DONT.md)** - Common mistakes and best practices

---

## Targets (v0.8.x)

**Primary Output**: JavaScript
- `server.js` - Node.js backend
- `client.js` - Browser frontend
- `styles.css` - Scoped component styles
- `index.html` - HTML entry point

**Experimental Output**: WebAssembly (--target wasm)
- May fail for some language constructs
- Best-effort in v0.8.x
- If JS succeeds but WASM fails, the build is still considered successful

---

## What is Jounce?

Write your entire full-stack web app in **one file**. No build config, no bundler setup, no frontend/backend split.

```
my-app/main.jnc  →  jnc compile  →  dist/my-app/
                                     ├── server.js    (Node.js backend)
                                     ├── client.js    (Browser frontend)
                                     ├── styles.css   (Scoped styles)
                                     └── index.html   (Entry point)
```

**Key Features**:
- 🔥 **Fine-grained reactivity** - signals, computed, effects
- 🎯 **Component-based** - JSX with typed props
- 🎨 **Scoped styling** - CSS that doesn't leak
- 🔒 **Type-safe** - Static type checking
- 🚀 **Auto-RPC** - Call server functions from client
- 📦 **Package ecosystem** - 35+ packages, full registry

---

## Quick Start (2 Minutes)

### Install

```bash
git clone https://github.com/Jounce-lang/jounce-pre-production.git
cd jounce-pre-production
cargo build --release
```

Add to PATH:
```bash
export PATH="$PWD/target/release:$PATH"
```

### Your First App

Create `counter.jnc`:
```jounce
component Counter() {
    let count = signal<i32>(0);

    return <div>
        <h1>Count: {count.value}</h1>
        <button onclick={() => count.value++}>+1</button>
    </div>;
}

style Counter {
    padding: 20px;
    text-align: center;

    button {
        font-size: 20px;
        padding: 10px 20px;
        cursor: pointer;
    }

    button:hover {
        background: #007bff;
        color: white;
    }
}
```

Compile and run:
```bash
jnc compile counter.jnc
cd dist/counter && node server.js
# Open http://localhost:3000
```

> **Note**: Each compiled app goes to `dist/<app-folder>/` to prevent conflicts when building multiple apps.

**That's it!** You just built a full-stack reactive web app.

---

## What Makes Jounce Different?

### 1. One File, Full Stack

No more context switching between frontend and backend:

```jounce
// Server-side function
@server
fn getUser(id: i32) -> Result<User, string> {
    return database.query("SELECT * FROM users WHERE id = ?", id);
}

// Client-side component
component UserProfile() {
    let user = getUser(123);  // Automatic RPC!

    return <div>
        <h1>{user.name}</h1>
        <p>{user.email}</p>
    </div>;
}
```

### 2. True Reactivity

Fine-grained updates, no virtual DOM overhead:

```jounce
let count = signal<i32>(0);
let doubled = computed<i32>(() => count.value * 2);

effect(() => {
    console.log("Doubled: " + doubled.value.to_string());
});

count.value = 5;  // Effect runs automatically
```

### 3. Scoped Styling

CSS that never leaks outside your component:

```jounce
style Card {
    padding: 20px;
    border: 1px solid #ddd;

    // Nested selectors
    h2 {
        margin: 0;
    }

    // Pseudo-classes
    &:hover {
        box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    }
}
```

### 4. Type Safety

Catch bugs at compile time:

```jounce
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

add(5, 10);      // ✅ Works
add(5, "ten");   // ❌ Compile error: type mismatch
```

---

## Learn More

### 📖 Documentation

- **[JOUNCE_SPEC.md](JOUNCE_SPEC.md)** - Complete language specification
- **[LEARN_JOUNCE.md](docs/guides/LEARN_JOUNCE.md)** - Comprehensive tutorial
- **[Examples](examples/)** - 35+ working examples

### 🎓 Tutorials

Start with these examples:

1. **Counter** - Reactivity basics ([examples/apps/01-click-counter/](examples/apps/01-click-counter/))
2. **Todo App** - State management ([examples/apps/todo-app/](examples/apps/todo-app/))
3. **Form Validation** - Real-time validation ([examples/apps/16-form-validation/](examples/apps/16-form-validation/))
4. **Dashboard** - Component composition ([examples/apps/35-crm-dashboard/](examples/apps/35-crm-dashboard/))

### 🚀 Next Steps

```bash
# Start a new project
jnc init my-app

# Development mode with hot reload
jnc dev my-app/main.jnc

# Add packages
jnc pkg add jounce-router
jnc pkg add jounce-db

# Build for production
jnc compile --release my-app/main.jnc
```

---

## Core Concepts

### Reactivity

```jounce
// Mutable state
let count = signal<i32>(0);

// Derived values
let doubled = computed<i32>(() => count.value * 2);

// Side effects
effect(() => {
    console.log("Count: " + count.value.to_string());
});

// Batch updates
batch(() => {
    count.value = 10;
    count.value = 20;  // Only one effect run
});
```

### Components

```jounce
component Button(props: { label: string, onClick: fn() }) {
    return <button onclick={props.onClick}>
        {props.label}
    </button>;
}

component App() {
    let count = signal<i32>(0);

    return <div>
        <h1>Count: {count.value}</h1>
        <Button
            label="Increment"
            onClick={() => count.value++}
        />
    </div>;
}
```

### Server Functions

```jounce
@server
fn fetchPosts() -> Vec<Post> {
    return database.query("SELECT * FROM posts");
}

component PostList() {
    let posts = fetchPosts();  // Automatic RPC

    return <div>
        {posts.map((post) =>
            <article>
                <h2>{post.title}</h2>
                <p>{post.body}</p>
            </article>
        )}
    </div>;
}
```

### Module System

```jounce
// utils.jnc
pub fn formatDate(date: DateTime) -> string {
    return date.format("%Y-%m-%d");
}

// main.jnc
use ./utils::{formatDate};
use ./components::{Card as UICard};  // Import aliasing (v0.8.3+)

component App() {
    let today = formatDate(DateTime.now());
    return <UICard title={today} />;
}
```

---

## Package Ecosystem

35+ packages available:

**UI & Styling**:
- `jounce-ui` - Component library (buttons, inputs, modals)
- `jounce-theme` - Theming system with dark mode
- `jounce-animate` - Animation utilities

**Backend**:
- `jounce-db` - Database adapters (PostgreSQL, SQLite)
- `jounce-auth` - Authentication (JWT, OAuth)
- `jounce-cache` - Caching (LRU, Redis)

**Utilities**:
- `jounce-router` - Client-side routing
- `jounce-http` - HTTP client
- `jounce-forms` - Form handling and validation

[Browse all packages](packages/)

---

## What's New in v0.8.3

### Import Aliasing
```jounce
use ./widgets::{Button as WidgetButton};
use ./types::{User as UserType};
```

### Advanced Styling
```jounce
style Component {
    // Child combinators
    > .item {
        padding: 10px;
    }

    // Pseudo-classes
    &:hover {
        background: #f0f0f0;
    }
}
```

### Explicit `pub` Keyword
```jounce
pub fn publicFunction() -> i32 { ... }
fn privateFunction() -> i32 { ... }
```

### Type Narrowing (if-let)
```jounce
if let Some(value) = option {
    console.log(value);
}
```

### Package Registry
```bash
jnc pkg publish  # Publish to registry
jnc pkg add jounce-ui  # Install packages
```

[See full changelog](CHANGELOG.md)

---

## Development Roadmap

**Current**: v0.8.3 "Enhanced Language Features"
**Next**: v0.9.0 "Super Easy Start" (November 2025)

Upcoming features:
- 📚 Interactive tutorial system (tutorial.jounce.dev)
- 🎥 Video courses and screencasts
- 📖 Documentation overhaul with cookbook
- 🎨 20+ copy-paste example apps
- 🎮 Visual playground (play.jounce.dev) - December 2025
- 🔧 VSCode extension pro - December 2025
- 🚀 v1.0.0 stable release - Q2 2026

[View full roadmap](docs/project/ROADMAP.md)

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Ways to contribute**:
- 🐛 Report bugs
- 💡 Suggest features
- 📝 Improve documentation
- 🔧 Submit pull requests
- 📦 Create packages

---

## Developer Tooling

Run `./scripts/verify-examples.sh` to ensure examples stay spec-compliant.

---

## Community & Support

- **GitHub Issues**: [Report bugs or request features](https://github.com/Jounce-lang/jounce-pre-production/issues)
- **Discussions**: [Ask questions and share projects](https://github.com/Jounce-lang/jounce-pre-production/discussions)
- **Documentation**: [Complete guides and API reference](docs/)
- **Examples**: [35+ working examples](examples/)

---

## Why Jounce?

### For Solo Developers
- Write full-stack apps without context switching
- No build configuration needed
- Deploy with one command

### For Teams
- One language, one file structure
- Type safety prevents bugs
- Easy code review (everything in one place)

### For Learners
- Clean syntax familiar from Rust and JavaScript
- Comprehensive error messages
- Interactive tutorials coming soon

---

## Status: Production Ready ✅

- ✅ **580/580 tests passing** (100%)
- ✅ **0 critical bugs**
- ✅ **Fine-grained reactivity** system
- ✅ **Type safety** with inference
- ✅ **Package ecosystem** (35+ packages)
- ✅ **Runtime safety** (3-layer protection)

---

## License

MIT License - see [LICENSE](LICENSE) for details.

---

## Quick Links

- [Language Specification](JOUNCE_SPEC.md)
- [Tutorial Guide](docs/guides/LEARN_JOUNCE.md)
- [API Reference](docs/)
- [Examples](examples/)
- [Changelog](CHANGELOG.md)
- [Roadmap](docs/project/ROADMAP.md)

---

**🚀 Ready to build? [Start with the Quick Start](#quick-start-2-minutes)**

---

> Docs verified in CI: examples + local links ✅

---

**Maintained by: The Jounce Project**
