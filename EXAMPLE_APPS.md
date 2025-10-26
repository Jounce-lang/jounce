# Jounce Example Apps - User Showcase

**Version**: v0.8.3
**Purpose**: Show users what's possible in Jounce and how to build apps themselves
**Audience**: Users, learners, potential contributors

---

## 🎯 How to Use This Guide

This document shows you:
1. **What we built** - Real apps demonstrating Jounce features
2. **What we included** - Features showcased in each app
3. **What we left out** - Features NOT shown (and why)
4. **How YOU can build it** - Step-by-step recreation guide

---

## 🚀 Quick Start: Build Your First App

### The Jounce Workflow

```bash
# 1. Ask an LLM for code (ChatGPT, Claude, etc.)
"Build me a counter app in Jounce with localStorage persistence"

# 2. Save the code to a .jnc file
# Copy the LLM's response into app.jnc

# 3. Compile with Jounce
jnc compile app.jnc

# 4. Run it
cd dist && node server.js
# OR deploy to Vercel, Netlify, etc.
```

**That's it!** Jounce handles:
- ✅ Compiling your code to JavaScript
- ✅ Bundling client + server code
- ✅ Setting up reactivity
- ✅ Generating HTML/CSS
- ✅ Creating a dev server

---

## 📱 Example App #1: Counter (Basic Reactivity)

### What It Shows
- ✅ Reactive state with `signal()`
- ✅ Button click handlers
- ✅ Dynamic UI updates
- ✅ Basic styling with `style {}` blocks

### What We Left Out
- ❌ Persistence (no localStorage/backend)
- ❌ Components (single-file app)
- ❌ Routing (single page)
- ❌ Forms (just buttons)

**Why?** Keep it simple. This is "Hello World" for reactivity.

### Code Overview

```jounce
// test_app_counter.jnc

style {
    .counter-card {
        background: white;
        padding: 60px;
        border-radius: 20px;
    }

    .count-display {
        font-size: 120px;
        color: #667eea;
    }
}

fn Counter() -> JSX {
    return (
        <div class="counter-card">
            <h1>Simple Counter</h1>
            <div class="count-display" id="count">0</div>
            <button id="inc-btn">+ Increase</button>
        </div>
    );
}
```

### How to Recreate

**Ask LLM:**
```
Build a counter app in Jounce with:
- A number display starting at 0
- Three buttons: increment, decrement, reset
- Nice styling with gradients
- Use the 'style {}' block syntax
```

**Compile:**
```bash
jnc compile counter.jnc
cd dist && python3 -m http.server 8080
# Open http://localhost:8080
```

### File Size
- Source: ~135 lines
- Generated JS: ~10KB
- CSS: ~2KB

---

## 📝 Example App #2: Todo List (List Rendering)

### What It Shows
- ✅ Reactive arrays with `signal([])`
- ✅ List rendering (map over todos)
- ✅ Add/delete items
- ✅ Toggle completion status
- ✅ Computed values (stats)
- ✅ Multiple event handlers

### What We Left Out
- ❌ Persistence (todos lost on refresh)
- ❌ Authentication (no users)
- ❌ Backend storage (frontend only)
- ❌ Drag-and-drop reordering

**Why?** Focus on reactive list patterns. Persistence comes in App #3.

### Code Overview

```jounce
// apps/11_todo_list.jnc

style {
    .todo-container {
        background: white;
        padding: 40px;
        max-width: 600px;
    }

    .todo-item {
        display: flex;
        align-items: center;
        padding: 15px;
    }
}

fn TodoApp() -> JSX {
    return (
        <div class="todo-container">
            <input id="todo-input" placeholder="What needs to be done?" />
            <button id="add-btn">Add</button>
            <ul id="todo-list"></ul>
        </div>
    );
}
```

### How to Recreate

**Ask LLM:**
```
Build a todo list app in Jounce with:
- Input field to add new todos
- List of todos with checkboxes
- Delete button for each todo
- Stats showing total/completed/remaining
- Button to clear all completed
- Use reactive signals for the todo array
- Beautiful gradient background
```

**Compile:**
```bash
jnc compile todo.jnc
cd dist && node server.js
```

### File Size
- Source: ~170 lines
- Generated JS: ~15KB
- CSS: ~4KB

---

## 💾 Example App #3: Persistent Counter (NEW - @persist decorator!)

### What It Shows
- ✅ **Progressive enhancement** - add ONE LINE for persistence
- ✅ **localStorage** - data survives page refresh
- ✅ **@persist decorator** - automatic save/load
- ✅ **Reactive persistence** - changes auto-save

### What We Left Out
- ❌ Backend storage (coming in App #4)
- ❌ Multi-user support
- ❌ Sync across devices

**Why?** Showcase the @persist decorator's power. One line = persistence!

### Code Overview

```jounce
// test_persist_counter.jnc

component Counter() {
    // BEFORE: No persistence
    // let count = signal(0);

    // AFTER: Add ONE LINE for localStorage persistence!
    @persist("localStorage")
    let count = signal(0);

    return <div>
        <h1>Count: {count.value}</h1>
        <button onClick={() => count.value++}>Increment</button>
    </div>;
}
```

### Generated Code (What @persist does)

```javascript
// Jounce generates this automatically:
let count = signal(0);

// Load from localStorage
const __stored_count = localStorage.getItem('count');
if (__stored_count !== null) {
  count.value = JSON.parse(__stored_count);
}

// Save to localStorage on changes
effect(() => {
  localStorage.setItem('count', JSON.stringify(count.value));
});
```

### How to Recreate

**Ask LLM:**
```
Build a counter in Jounce with:
- A signal to track the count
- Increment/decrement/reset buttons
- Use @persist("localStorage") to save the count
- Show a message that says "Try refreshing - your count persists!"
```

**Test It:**
```bash
jnc compile persistent-counter.jnc
cd dist && python3 -m http.server 8080
# Increment the counter
# Refresh the page (Ctrl+Shift+R)
# Count is still there! 🎉
```

### The Magic
- **Before @persist**: 0 lines of persistence code
- **After @persist**: 1 line → 10+ lines generated automatically
- **Upgrade to backend**: Change ONE WORD: `@persist("backend")`

---

## 🛒 Example App #4: Shopping Cart (Full-Stack with Backend)

### What It Shows
- ✅ **Multi-user persistence** with `@persist("backend")`
- ✅ **Server functions** for data fetching
- ✅ **Authentication** (jounce-auth package)
- ✅ **Database** (jounce-db package)
- ✅ **Progressive enhancement** (localStorage → backend with ONE LINE)

### What We Left Out
- ❌ Payment processing (would need Stripe integration)
- ❌ Real-time sync (coming in App #5)
- ❌ Product images (keep it simple)

**Why?** Show the full stack. This is where Jounce shines.

### Code Overview

```jounce
// Coming in Phase 15, Week 3

// Server-side data fetching
server fn loadCart() -> Vec<CartItem> {
    // Talk to database
    db.query("SELECT * FROM cart WHERE user_id = ?", [user.id])
}

server fn saveCart(items: Vec<CartItem>) {
    // Save to database
    db.query("UPDATE cart SET items = ?", [items])
}

component ShoppingCart() {
    // STEP 1: Start with localStorage
    @persist("localStorage")
    let cart = signal([]);

    // STEP 2: Upgrade to backend (change ONE WORD!)
    @persist("backend")
    let cart = signal([]);

    // Jounce automatically calls loadCart() and saveCart()!

    return <div>
        <h1>Your Cart ({cart.value.len()} items)</h1>
        {cart.value.map(item =>
            <CartItem product={item} />
        )}
    </div>;
}
```

### How to Recreate

**Ask LLM:**
```
Build a shopping cart app in Jounce with:
- Product list with "Add to Cart" buttons
- Cart display showing items and total price
- Use @persist("backend") for multi-user persistence
- Include server functions: loadCart() and saveCart()
- Use jounce-auth for user authentication
- Use jounce-db for database storage
```

**Compile:**
```bash
jnc compile shopping-cart.jnc
cd dist && node server.js
# Or deploy to Vercel
```

### The Progression

```jounce
// Day 1: Prototype (frontend only)
let cart = signal([]);

// Day 2: Add localStorage (ONE LINE)
@persist("localStorage")
let cart = signal([]);

// Week 1: Multi-user backend (CHANGE ONE WORD)
@persist("backend")
let cart = signal([]);
// + Add server functions for loadCart/saveCart

// Month 1: Real-time sync (CHANGE ONE WORD)
@persist("realtime")
let cart = signal([]);
// + Add WebSocket handlers
```

**Status**: Coming in Phase 15, Week 3

---

## 📊 Example App #5: Dashboard (Real-Time Data)

### What It Shows
- ✅ **Real-time updates** with `@persist("realtime")`
- ✅ **WebSocket sync** across multiple browsers
- ✅ **Live charts** that update automatically
- ✅ **Multi-user collaboration**

### What We Left Out
- ❌ Advanced charts (would need a charting library)
- ❌ Data export (CSV, Excel)
- ❌ User permissions (admin vs viewer)

**Why?** Focus on real-time reactivity. Show the power of @persist("realtime").

### Code Overview

```jounce
// Coming in Phase 15, Week 4

component Dashboard() {
    // Real-time sync across all connected users!
    @persist("realtime")
    let metrics = signal({
        users: 0,
        revenue: 0,
        orders: 0
    });

    // When one user updates metrics, ALL browsers see it instantly!

    return <div>
        <h1>Live Dashboard</h1>
        <Metric label="Active Users" value={metrics.value.users} />
        <Metric label="Revenue" value={metrics.value.revenue} />
        <Metric label="Orders" value={metrics.value.orders} />
    </div>;
}
```

**Status**: Coming in Phase 15, Week 4

---

## 🎓 Learning Path

### Beginner (Week 1)
1. **Counter App** - Learn basic reactivity
2. **Todo List** - Learn list rendering
3. Read: `docs/guides/REACTIVITY_USER_GUIDE.md`

### Intermediate (Week 2)
4. **Persistent Counter** - Learn @persist decorator
5. Experiment with localStorage
6. Read: `FEATURES.md` (feature inventory)

### Advanced (Week 3-4)
7. **Shopping Cart** - Learn full-stack patterns
8. **Dashboard** - Learn real-time sync
9. Build your own app!

---

## 🛠️ The Jounce Advantage

### Before Jounce (Traditional Stack)
```typescript
// 50+ lines to set up localStorage persistence
const [count, setCount] = useState(0);

useEffect(() => {
  const stored = localStorage.getItem('count');
  if (stored) setCount(JSON.parse(stored));
}, []);

useEffect(() => {
  localStorage.setItem('count', JSON.stringify(count));
}, [count]);
```

### With Jounce
```jounce
// 2 lines for localStorage persistence
@persist("localStorage")
let count = signal(0);
```

**90% less code. 100% more readable.**

---

## 📦 Package Usage in Examples

| Package | App #1 | App #2 | App #3 | App #4 | App #5 |
|---------|--------|--------|--------|--------|--------|
| **Core reactivity** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **@persist decorator** | ❌ | ❌ | ✅ | ✅ | ✅ |
| **jounce-auth** | ❌ | ❌ | ❌ | ✅ | ✅ |
| **jounce-db** | ❌ | ❌ | ❌ | ✅ | ✅ |
| **jounce-websocket** | ❌ | ❌ | ❌ | ❌ | ✅ |
| **jounce-ui** | ❌ | ❌ | ❌ | ✅ | ✅ |
| **jounce-theme** | ❌ | ❌ | ❌ | ✅ | ✅ |

---

## 🎯 Coming Soon (Phase 15)

### Week 1: Full-Stack Todo App
- All features from App #2
- + Backend persistence with @persist
- + User authentication
- + Database storage

### Week 2: Blog Platform
- Markdown editor
- Post management
- Search functionality
- Comment system

### Week 3: E-Commerce Store
- Complete shopping cart (App #4)
- Product catalog
- Checkout flow
- Payment integration

### Week 4: Dashboard
- Complete real-time dashboard (App #5)
- Live charts
- WebSocket sync
- Multi-user collaboration

---

## 💬 How to Get Help

1. **Ask an LLM** (ChatGPT, Claude, etc.):
   ```
   "How do I build [feature] in Jounce?"
   "Show me an example of [pattern] in Jounce"
   "Debug this Jounce code: [paste code]"
   ```

2. **Check FEATURES.md**:
   - See what's implemented
   - Find code examples
   - Check file locations

3. **Read the guides**:
   - `docs/guides/REACTIVITY_USER_GUIDE.md`
   - `BUILDING_APPS.md`
   - `docs/guides/QUICKSTART.md`

---

## 📝 Template for Your Own App

```jounce
// my-app.jnc

// 1. Styling
style {
    .app {
        padding: 20px;
        max-width: 800px;
    }
}

// 2. Server functions (optional)
server fn loadData() -> Vec<Item> {
    // Fetch from database
}

// 3. Component
component MyApp() {
    // 4. State (with optional persistence)
    @persist("localStorage")  // or "backend" or "realtime"
    let data = signal([]);

    // 5. Event handlers
    let handleClick = || {
        data.value.push(newItem);
    };

    // 6. Render
    return <div class="app">
        <h1>My App</h1>
        <button onClick={handleClick}>Add Item</button>
    </div>;
}

// 7. Entry point
fn main() {
    console.log("App starting...");
}
```

---

**Last Updated**: October 25, 2025
**Next Update**: After Phase 15 apps are built
**Questions?** Check `FEATURES.md` or ask an LLM!
