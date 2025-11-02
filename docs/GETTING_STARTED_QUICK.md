# Getting Started with Jounce - 5-Minute Quick Start

**Last Updated**: November 1, 2025
**Version**: v0.9.0 "Super Easy Start"
**Time to First App**: 5 minutes

---

## 🚀 Quick Start (3 Commands)

```bash
# 1. Install Jounce
curl -fsSL https://jounce.dev/install.sh | sh

# 2. Create your first app
jnc init my-app
cd my-app

# 3. Run it!
jnc dev
```

**Open http://localhost:3000** - Your app is running! 🎉

---

## 📦 Installation Methods

### Method 1: One-Line Install (Recommended)

**macOS / Linux**:
```bash
curl -fsSL https://jounce.dev/install.sh | sh
```

**Windows** (PowerShell):
```powershell
irm https://jounce.dev/install.ps1 | iex
```

### Method 2: Cargo (Rust)

```bash
cargo install jounce
```

### Method 3: NPM (Node.js)

```bash
npm install -g jounce-cli
```

### Verify Installation

```bash
jnc --version
# Output: jounce 0.9.0
```

---

## ⚡ Your First App (10 Lines)

Create `app.jnc`:

```jounce
component App() {
    let count = createSignal(0);

    <div class="container p-8">
        <h1 class="text-4xl mb-4">Count: {count.value}</h1>
        <button onClick={() => count.set(count.value + 1)} class="btn btn-primary">
            Click Me!
        </button>
    </div>
}
```

**Compile and run**:
```bash
jnc compile app.jnc
jnc dev
```

**That's it!** You've built a reactive app in 10 lines.

---

## 🎓 What You Just Built

Your app has:
- ✅ **Reactive state** (`createSignal`)
- ✅ **Event handling** (`onClick`)
- ✅ **Automatic UI updates** (when count changes)
- ✅ **Component architecture** (`component App`)
- ✅ **Utility CSS classes** (`btn`, `btn-primary`)

---

## 📖 Next Steps

### 1. Take the Interactive Tutorial (50 mins)
**Learn by doing** - Interactive lessons in your browser:
```
https://tutorial.jounce.dev
```

### 2. Use a Starter Template
**Skip the boilerplate** - Start with a template:
```bash
jnc init my-app --template todo
jnc init my-blog --template blog
jnc init my-dashboard --template dashboard
```

### 3. Read the Docs
**Deep dive** - Full documentation:
```
https://docs.jounce.dev
```

---

## 🛠️ Common CLI Commands

```bash
# Create new project
jnc init my-app

# Compile code
jnc compile app.jnc

# Development server with hot reload
jnc dev

# Build for production
jnc build --release

# Deploy to production
jnc deploy

# Check project health
jnc doctor

# Add packages
jnc add jounce-router
jnc add jounce-db

# Run tests
jnc test
```

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

### Command not found: jnc
**Solution**: Add to PATH
```bash
# macOS/Linux
echo 'export PATH="$HOME/.jounce/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Windows
# Add %USERPROFILE%\.jounce\bin to PATH
```

### Port 3000 already in use
**Solution**: Use a different port
```bash
jnc dev --port 8080
```

### Compilation errors
**Solution**: Check syntax
```bash
jnc check app.jnc  # Check for errors
jnc format app.jnc  # Auto-format code
```

### Can't install on Windows
**Solution**: Use WSL2
```bash
wsl --install
# Then use Linux install method
```

---

## 📚 Learning Resources

### 🎥 Video Tutorials
- [Jounce in 100 Seconds](https://youtube.com/jounce) - Quick overview
- [Build a Todo App](https://youtube.com/jounce/todo) - 15-min tutorial
- [Full Course](https://youtube.com/jounce/course) - 2-hour deep dive

### 📖 Documentation
- [Language Guide](https://docs.jounce.dev/guide)
- [API Reference](https://docs.jounce.dev/api)
- [Cookbook](https://docs.jounce.dev/cookbook) - Common patterns

### 🎮 Interactive
- [Tutorial](https://tutorial.jounce.dev) - 10 lessons, 50 minutes
- [Playground](https://play.jounce.dev) - Try code in browser
- [Examples](https://examples.jounce.dev) - 20+ copy-paste examples

---

## 💡 Core Concepts (2-Minute Overview)

### Signals (Reactive State)
```jounce
let count = createSignal(0);     // Create
console.log(count.value);         // Read
count.set(5);                     // Update
// UI updates automatically! ✨
```

### Computed Values
```jounce
let count = createSignal(5);
let doubled = computed(() => count.value * 2);
// doubled updates automatically when count changes!
```

### Components
```jounce
component Card(title: String) {
    <div class="card">
        <h2>{title}</h2>
    </div>
}

// Use it
<Card title="Hello" />
```

### Event Handlers
```jounce
<button onClick={() => doSomething()}>
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

## 🚀 Deploy to Production

### Option 1: Vercel (One Command)
```bash
jnc deploy --platform vercel
```

### Option 2: Fly.io
```bash
jnc deploy --platform fly
```

### Option 3: Docker
```bash
jnc build --docker
docker run -p 3000:3000 my-app
```

---

## ❓ FAQ

**Q: Do I need to know Rust?**
A: No! Jounce syntax is similar to JavaScript/TypeScript.

**Q: Can I use npm packages?**
A: Yes! Import any npm package.

**Q: Is it production-ready?**
A: Yes! 635/635 tests passing, zero known bugs.

**Q: How big are the bundles?**
A: Tiny! ~50KB for a full app (gzipped).

**Q: Does it work with VS Code?**
A: Yes! Install the Jounce extension.

---

## 🆘 Get Help

- **Discord**: [discord.jounce.dev](https://discord.jounce.dev)
- **GitHub**: [github.com/Jounce-lang/Jounce](https://github.com/Jounce-lang/Jounce)
- **Discussions**: [github.com/Jounce-lang/Jounce/discussions](https://github.com/Jounce-lang/Jounce/discussions)
- **Twitter**: [@JounceJS](https://twitter.com/JounceJS)

---

## 🎯 What's Next?

Choose your path:

1. **Learn** → [Take the tutorial](https://tutorial.jounce.dev)
2. **Build** → [Use a template](https://examples.jounce.dev)
3. **Explore** → [Try the playground](https://play.jounce.dev)

---

**Ready to build amazing apps?** 🚀

**Start here**: `jnc init my-app && cd my-app && jnc dev`

---

**Last Updated**: November 1, 2025
**Questions?** [Ask on Discord](https://discord.jounce.dev) or [GitHub Discussions](https://github.com/Jounce-lang/Jounce/discussions)
