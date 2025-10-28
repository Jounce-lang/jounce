# 🚀 Getting Started with Jounce

Welcome! This guide will get you up and running with Jounce in **5 minutes**.

---

## ⚡ 30-Second Quick Start

```bash
# 1. Build the compiler
cargo build --release

# 2. Test everything works
./test_all_examples.sh

# 3. Run your first app
cargo run --release -- compile test_fine_grained_reactivity.jnc
cd dist && node server.js

# 4. Open browser to http://localhost:3000
```

**That's it!** You should see a working counter app with automatic reactivity! 🎉

---

## 📚 What You Get

Jounce gives you:
- ✅ **Fine-grained reactivity** - Like Solid.js, but at compile-time
- ✅ **Full-stack** - Server functions + database + UI in ONE file
- ✅ **Fast** - Sub-15ms compile times
- ✅ **Simple** - Just use `.value`, everything else is automatic

---

## 🎯 The 5-Minute Tutorial

### **Step 1: Your First App (2 minutes)**

Create `my-first-app.jnc`:

```jounce
component MyApp() {
    let count = signal(0);

    return <div>
        <h1>Count: {count.value}</h1>
        <button onclick={() => { count.value = count.value + 1; }}>
            Click me!
        </button>
    </div>;
}

fn main() {
    console.log("My first Jounce app!");
}
```

### **Step 2: Compile It (10 seconds)**

```bash
cargo run --release -- compile my-first-app.jnc
```

### **Step 3: Run It (10 seconds)**

```bash
cd dist && node server.js
```

### **Step 4: Test It (1 minute)**

1. Open `http://localhost:3000`
2. Click the button
3. Watch the count increment automatically! ✨

**No manual DOM updates. No useState. No useEffect. Just `.value` and it works!**

---

## 🧠 Key Concepts (90 seconds)

### **1. Signals = Reactive Values**

```jounce
let count = signal(0);       // Create
let value = count.value;     // Read
count.value = 10;            // Write → UI updates automatically!
```

### **2. Computed = Derived Values**

```jounce
let doubled = computed(() => {
    return count.value * 2;
});
// Auto-recalculates when count changes!
```

### **3. JSX = UI**

```jounce
return <div>
    <h1>{count.value}</h1>     <!-- Updates automatically! -->
</div>;
```

### **4. Server Functions = Backend**

```jounce
server fn get_user(id: int) {
    script {
        const db = global.db;
        return db.prepare('SELECT * FROM users WHERE id = ?').get(id);
    }
}

// Call from client:
get_user(123).then(user => console.log(user));
```

**That's everything you need to know!**

---

## 📦 Example Apps

We've included **6 production-ready examples**:

### **1. Counter** (Beginner)
Learn signal basics
```bash
cargo run --release -- compile test_fine_grained_reactivity.jnc
```

### **2. Shopping Cart** (Beginner)
Arrays, computed totals
```bash
cargo run --release -- compile examples/reactivity/shopping-cart.jnc
```

### **3. Form Validation** (Intermediate)
Real-time validation, multiple signals
```bash
cargo run --release -- compile examples/reactivity/form-validation-simple.jnc
```

### **4. Search & Filter** (Intermediate)
Complex filtering, multiple filters
```bash
cargo run --release -- compile examples/reactivity/search-filter.jnc
```

### **5. Dashboard** (Advanced)
Computed metrics, derived calculations
```bash
cargo run --release -- compile examples/reactivity/dashboard.jnc
```

### **6. Todo App** (Advanced - Full-Stack)
Database, server functions, complete app
```bash
cargo run --release -- compile examples/apps/todo-app/main_reactive.jnc
```

**After compiling, run**: `cd dist && node server.js`

---

## 🎨 Build Your Own App

### **Template: Simple App**

```jounce
component MyComponent() {
    // 1. Create signals for state
    let name = signal("");

    // 2. Create computed values
    let greeting = computed(() => {
        return "Hello, " + name.value + "!";
    });

    // 3. Render UI
    return <div>
        <input
            oninput={(e) => { name.value = e.target.value; }}
            placeholder="Enter your name"
        />
        <h1>{greeting.value}</h1>
    </div>;
}

fn main() {
    console.log("App starting...");
}
```

### **Template: Full-Stack App**

```jounce
// SERVER: Database function
server fn get_data() {
    script {
        const db = global.db;
        return db.prepare('SELECT * FROM items').all();
    }
}

// CLIENT: Component
component App() {
    let items = signal([]);

    onMount(() => {
        get_data().then((data) => {
            items.value = data;
        });
    });

    return <div>
        <h1>Items: {items.value.length}</h1>
    </div>;
}

fn main() {
    console.log("Full-stack app!");
}
```

---

## 🔥 Common Patterns

### **Pattern: Form Input**
```jounce
let email = signal("");

<input
    oninput={(e) => { email.value = e.target.value; }}
    placeholder="Email"
/>
<p>You typed: {email.value}</p>
```

### **Pattern: Toggle**
```jounce
let isOpen = signal(false);

<button onclick={() => { isOpen.value = !isOpen.value; }}>
    Toggle
</button>

{isOpen.value ?
    <div>Panel is open!</div>
    : <div>Panel is closed</div>
}
```

### **Pattern: Array Filter**
```jounce
let items = signal([...]);
let query = signal("");

let filtered = computed(() => {
    return items.value.filter(item =>
        item.name.contains(query.value)
    );
});

<div>Found: {filtered.value.length} items</div>
```

### **Pattern: Persistent State**
```jounce
let theme = persistentSignal("theme", "light");
// Auto-saves to localStorage!
// Persists across page reloads!
```

---

## 🐛 Debugging Tips

### **Issue: UI not updating**

**Check**:
1. Are you using `.value`? (not just `count`)
2. Is it inside JSX? (between `<...>` tags)
3. Did it compile? (check for errors)

**Debug**:
```bash
# Check if reactive wrappers were generated
grep "__reactive" dist/client.js
```

### **Issue: Compilation error**

**Common fixes**:
- Add semicolon at end of statement: `let x = 5;`
- Match braces: `{ ... }`
- Don't use reserved keywords: `theme` → `themeValue`

### **Issue: Can't connect to localhost:3000**

**Fix**:
```bash
# Kill existing server
lsof -i :3000
kill -9 <PID>

# Or use different port
# Edit dist/server-runtime.js: const PORT = 3001;
```

---

## 📖 Learning Path

**Day 1**: Start here
1. Run the automated test: `./test_all_examples.sh`
2. Try the counter example
3. Modify it - add a "Decrement" button
4. Read `examples/reactivity/README.md`

**Day 2**: Build something
1. Try the shopping cart example
2. Modify it - add/remove items
3. Try the form validation example
4. Build your own form

**Day 3**: Go full-stack
1. Try the todo app example
2. Add a new feature (delete all todos?)
3. Read `FEATURES.md` for all available features
4. Build your own full-stack app

**Week 2**: Master it
1. Read `DEEP_DIVE_ANALYSIS.md`
2. Study the compiled JavaScript
3. Learn the advanced patterns
4. Build something real!

---

## 🎯 Cheat Sheet

```bash
# COMPILATION
cargo run --release -- compile <file.jnc>

# RUN SERVER
cd dist && node server.js

# VIEW GENERATED CODE
cat dist/client.js

# CHECK REACTIVITY
grep "__reactive" dist/client.js

# TEST EVERYTHING
./test_all_examples.sh

# STOP SERVER
Ctrl+C
```

---

## 📚 Documentation

- **TESTING_GUIDE.md** - How to test everything
- **examples/reactivity/README.md** - Reactivity examples guide
- **FEATURES.md** - All available features
- **CLAUDE.md** - Current status & roadmap
- **DEEP_DIVE_ANALYSIS.md** - Technical deep dive

---

## 🚀 You're Ready!

You now know how to:
- ✅ Build and test Jounce
- ✅ Create reactive apps
- ✅ Use signals and computed values
- ✅ Build full-stack apps
- ✅ Debug common issues

**Go build something awesome!** 🎉

---

## 💡 Tips

**1. Start small** - Don't try to build everything at once

**2. Use the examples** - They're production-ready templates

**3. Check generated code** - Learn how Jounce works under the hood

**4. Test frequently** - Compile and run often to catch errors early

**5. Read the errors** - Jounce gives helpful error messages

---

## 🤝 Getting Help

**Questions?**
- Check `TESTING_GUIDE.md` for troubleshooting
- Look at example apps in `examples/reactivity/`
- Read the inline comments in examples

**Found a bug?**
- Check if it's a known issue in `DEEP_DIVE_ANALYSIS.md`
- Try the automated test: `./test_all_examples.sh`
- Check the browser console for JavaScript errors

---

**Welcome to Jounce! Let's build something amazing! 🚀**
