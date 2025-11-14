# Getting Started with Jounce - Quick Start

**Last Updated**: November 1, 2025
**Version**: v0.9.0
**Time to First App**: 5 minutes

---

## 🚀 Quick Start

```bash
# 1. Install Jounce via Cargo
cargo install jounce

# 2. Create your first app
echo 'component App() {
    let count = signal(0);

    return <div class="container p-8">
        <h1 class="text-4xl mb-4">Count: {count.value}</h1>
        <button onClick={() => count.value = count.value + 1} class="btn btn-primary">
            Click Me!
        </button>
    </div>;
}' > app.jnc

# 3. Compile and run it!
jnc compile app.jnc
cd dist && node server.js
```

**Open http://localhost:3000** - Your app is running! 🎉

---

## 📦 Installation

### Method 1: Cargo (Rust) - **Currently Available**

```bash
cargo install jounce
```

### Verify Installation

```bash
jnc --version
# Output: jnc 0.9.0
```

---

## ⚡ Your First App (10 Lines)

Create `app.jnc`:

```jounce
component App() {
    let count = signal(0);

    return <div class="container p-8">
        <h1 class="text-4xl mb-4">Count: {count.value}</h1>
        <button onClick={() => count.value = count.value + 1} class="btn btn-primary">
            Click Me!
        </button>
    </div>;
}
```

**Compile and run**:
```bash
jnc compile app.jnc
cd dist && node server.js
```

**Open http://localhost:3000** - That's it! You've built a reactive app in 10 lines.

---

## 🎓 What You Just Built

Your app has:
- ✅ **Reactive state** (`signal`)
- ✅ **Event handling** (`onClick`)
- ✅ **Automatic UI updates** (when count changes)
- ✅ **Component architecture** (`component App`)
- ✅ **Utility CSS classes** (`btn`, `btn-primary`)
- ✅ **Full-stack output** (server.js + client.js + WASM)

---

## 📖 Next Steps

### 1. Use a Starter Template
**Skip the boilerplate** - Start with a template:
```bash
jnc new my-app
cd my-app
# Edit templates/tutorial-starters/counter/main.jnc (or todo, form, dashboard)
jnc compile templates/tutorial-starters/counter/main.jnc
cd dist && node server.js
```

### 2. Read the Docs
**Deep dive** - Full documentation in `docs/` directory:
- [Language Guide](../docs/README.md)
- [CSS Utilities](../docs/CSS_UTILITIES.md)
- [Examples](../examples/)

---

## 🛠️ Available CLI Commands

```bash
# Create new project structure
jnc new my-app

# Compile a .jnc file to JavaScript + WASM
jnc compile app.jnc

# Serve compiled output (basic server)
jnc serve
```

**Note**: Additional commands like `jnc init`, `jnc dev`, `jnc deploy` are planned for future releases.

---

## 🎨 Using Utility Classes

Jounce includes **457 utility classes** out of the box:

```jounce
<div class="container mx-auto p-8">
  <h1 class="text-4xl font-bold text-primary mb-4">
    Title
  </h1>
  <button class="btn btn-primary btn-lg rounded">
    Click Me
  </button>
</div>
```

**Categories**:
- Layout: `flex`, `grid`, `container`
- Spacing: `p-4`, `m-8`, `mx-auto`
- Typography: `text-xl`, `font-bold`
- Colors: `text-primary`, `bg-blue-500`
- Components: `btn`, `card`, `badge`

[See full CSS reference →](./CSS_UTILITIES.md)

---

## 🐛 Troubleshooting

### Port 3000 already in use
**Solution**: Edit `dist/server.js` and change the port:
```javascript
const PORT = process.env.PORT || 8080;
```

### Compilation errors
**Solution**: Check syntax against working examples:
```bash
# Test with a known-good template
jnc compile templates/tutorial-starters/counter/main.jnc
```

---

## 💡 Core Concepts (2-Minute Overview)

### Signals (Reactive State)
```jounce
let count = signal(0);           // Create
console.log(count.value);         // Read
count.value = 5;                  // Update
// UI updates automatically! ✨
```

### Computed Values
```jounce
let count = signal(5);
let doubled = computed(() => count.value * 2);
// doubled updates automatically when count changes!
```

### Components
```jounce
component Card(title: String) {
    return <div class="card">
        <h2>{title}</h2>
    </div>;
}

// Use it
<Card title="Hello" />
```

### Event Handlers
```jounce
function handleClick() {
    console.log("Clicked!");
}

<button onClick={handleClick}>
    Click Me
</button>
```

### Styling
```jounce
style MyComponent {
    background-color: #f5f5f5;
    padding: 20px;

    :hover {
        background-color: #e0e0e0;
    }
}
```

---

## 📚 Learning Resources

### 📖 Documentation
- Language Guide - `docs/README.md`
- CSS Reference - `docs/CSS_UTILITIES.md`
- Example Apps - `examples/apps/`

### 🎮 Example Templates
- Counter - `templates/tutorial-starters/counter/`
- Todo - `templates/tutorial-starters/todo/`
- Form - `templates/tutorial-starters/form/`
- Dashboard - `templates/tutorial-starters/dashboard/`

---

## ❓ FAQ

**Q: Do I need to know Rust?**
A: No! Jounce syntax is similar to JavaScript/TypeScript.

**Q: What's the difference between `signal()` and `createSignal()`?**
A: `signal()` is the current API. `createSignal()` was an older API that's no longer supported.

**Q: Is it production-ready?**
A: The compiler is stable (635/635 tests passing), but some developer tools are still in development.

**Q: How big are the bundles?**
A: Tiny! ~50KB for a full app (gzipped).

**Q: Why doesn't `jnc dev` work?**
A: The `jnc dev` command is planned but not yet implemented. Currently use: `jnc compile app.jnc && cd dist && node server.js`

---

## 🆘 Get Help

- **GitHub**: [github.com/jrez-soft/jounce-pre-production](https://github.com/jrez-soft/jounce-pre-production)
- **Issues**: [github.com/jrez-soft/jounce-pre-production/issues](https://github.com/jrez-soft/jounce-pre-production/issues)

---

## 🎯 What's Next?

Choose your path:

1. **Learn** → Explore example templates in `templates/tutorial-starters/`
2. **Build** → Create your own app with `jnc new my-app`
3. **Contribute** → Check `CONTRIBUTING.md` to help build Jounce!

---

**Ready to build amazing apps?** 🚀

**Start here**:
```bash
jnc new my-app
cd my-app
jnc compile templates/tutorial-starters/counter/main.jnc
cd dist && node server.js
```

---

**Last Updated**: November 1, 2025
**Questions?** [Open an issue](https://github.com/jrez-soft/jounce-pre-production/issues)
