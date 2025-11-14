# Getting Started with Jounce

> **Note**: For authoritative language rules, see [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md).

**Version**: v0.8.3
**Status**: ✅ Production Ready (580/580 tests passing)
**Updated**: November 7, 2025

---

## Quick Navigation

Jounce has three primary documentation resources:

### 1. **Quick Start** → [README.md](../../README.md)
**"Hey, here's Jounce. Install it, look at this counter."**

- 2-minute installation
- Your first app (counter example)
- What makes Jounce different
- Quick links to everything

**Start here if**: You're brand new to Jounce

---

### 2. **Learn to be Productive** → [LEARN_JOUNCE.md](../guides/LEARN_JOUNCE.md)
**"Here's how to be productive with Jounce."**

- Tutorial 1: Counter (5 minutes)
- Tutorial 2: Todo App (15 minutes)
- Tutorial 3: Form Validation (20 minutes)
- Tutorial 4: Full-Stack App (30 minutes)
- Practical recipes and patterns

**Start here if**: You want hands-on tutorials

---

### 3. **Technical Reference** → [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md)
**"This is the truth about how Jounce works."**

- Complete grammar and constructs
- Execution model (server vs client)
- CLI command reference
- Language limitations
- Implemented vs planned features

**Start here if**: You need technical details or answers to "does Jounce support X?"

---

## Installation (2 Minutes)

```bash
# Clone the repository
git clone https://github.com/Jounce-lang/jounce-pre-production.git
cd jounce-pre-production

# Build the compiler
cargo build --release

# Add to PATH
export PATH="$PWD/target/release:$PATH"
```

**Verify installation:**
```bash
jnc --version
# jounce 0.8.3
```

---

## Your First App (5 Minutes)

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
    padding: 40px;
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

**Compile and run:**
```bash
jnc compile counter.jnc
cd dist && node server.js
# Open http://localhost:3000
```

---

## What's Next?

### Follow the Tutorials

Work through the practical tutorials in [LEARN_JOUNCE.md](../guides/LEARN_JOUNCE.md):

1. **Counter** (5 min) - Reactivity basics
2. **Todo App** (15 min) - State management
3. **Form Validation** (20 min) - Real-time validation
4. **Full-Stack App** (30 min) - Server functions and RPC

### Explore Examples

Browse [35+ working examples](../../examples/) organized by category:

- **Apps** - Full applications (todo, dashboard, admin panel)
- **Components** - Reusable UI components
- **Features** - Specific feature demonstrations
- **Patterns** - Common development patterns

### Read the Spec

When you need technical details, check [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md) for:

- Exact syntax rules
- What's implemented vs planned
- Language limitations
- CLI command reference

---

## Key Concepts

### Reactivity

```jounce
let count = signal<i32>(0);              // Mutable state
let doubled = computed<i32>(() => count.value * 2);  // Derived
effect(() => console.log(count.value.to_string()));  // Side effect
```

### Components

```jounce
component Button(props: { label: string, onClick: fn() }) {
    return <button onclick={props.onClick}>
        {props.label}
    </button>;
}
```

### Server Functions

```jounce
@server
fn getUsers() -> Vec<User> {
    return database.query("SELECT * FROM users");
}

component UserList() {
    let users = getUsers();  // Automatic RPC!
    return <div>...</div>;
}
```

### Scoped Styling

```jounce
style Card {
    padding: 20px;
    border: 1px solid #ddd;

    &:hover {
        box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    }
}
```

---

## Common Commands

```bash
# Create new project
jnc init my-app

# Compile .jnc file
jnc compile main.jnc

# Development mode (coming in v0.9.0)
jnc dev main.jnc

# Run tests (coming in v0.10.0)
jnc test

# Package management
jnc pkg add jounce-ui
jnc pkg install
jnc pkg publish
```

See [JOUNCE_SPEC.md § CLI Commands](../../JOUNCE_SPEC.md#cli-commands) for complete reference.

---

## Need Help?

- **Technical questions**: Check [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md)
- **Learning tutorials**: Follow [LEARN_JOUNCE.md](../guides/LEARN_JOUNCE.md)
- **Examples**: Browse [examples/](../../examples/)
- **Bugs**: [GitHub Issues](https://github.com/Jounce-lang/jounce-pre-production/issues)

---

**Ready to build?** Start with the [Quick Start in README.md](../../README.md) or dive into [tutorials in LEARN_JOUNCE.md](../guides/LEARN_JOUNCE.md)!

---

**Maintained by: The Jounce Project**
