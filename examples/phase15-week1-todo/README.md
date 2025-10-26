# Phase 15, Week 1: Full-Stack Todo App

**Version**: v0.8.3
**Status**: In Development
**Goal**: Showcase progressive enhancement with @persist decorator

---

## 🎯 What This App Demonstrates

### Core Features
- ✅ **Progressive Enhancement** - localStorage → backend with ONE WORD
- ✅ **Reactive State** - signals, computed, effects
- ✅ **User Authentication** - jounce-auth package
- ✅ **Database Persistence** - jounce-db package
- ✅ **Beautiful UI** - jounce-ui components + custom styling
- ✅ **Multi-user Support** - Each user sees their own todos

### Packages Used
- `jounce-auth` - JWT authentication, sessions
- `jounce-db` - PostgreSQL/SQLite storage
- `jounce-ui` - Button, Input, Card components
- `jounce-theme` - Dark/light mode support

---

## 📁 File Structure

```
phase15-week1-todo/
├── README.md (this file)
├── v1_basic.jnc          # Basic reactivity (no persistence)
├── v2_localStorage.jnc   # Add @persist("localStorage")
├── v3_backend.jnc        # Add @persist("backend") + auth
└── screenshots/
    ├── login.png
    ├── todo-list.png
    └── dark-mode.png
```

---

## 🔄 Progressive Enhancement Journey

### Version 1: Basic Reactivity (50 lines)
```jounce
component TodoApp() {
    let todos = signal([]);  // No persistence - lost on refresh

    // Basic add/delete/toggle
}
```

**What works**: Adding, deleting, toggling todos
**What doesn't**: Data lost on page refresh

---

### Version 2: localStorage Persistence (52 lines = +2 lines!)
```jounce
component TodoApp() {
    @persist("localStorage")  // Add ONE LINE!
    let todos = signal([]);   // Now survives refresh!

    // Same code as v1
}
```

**What works**: Everything from v1 + data persists in browser
**What doesn't**: Can't sync across devices, no multi-user

---

### Version 3: Full-Stack Backend (150 lines = +100 lines)
```jounce
// Server-side data functions
server fn loadTodos(userId: String) -> Vec<Todo> {
    db.query("SELECT * FROM todos WHERE user_id = ?", [userId])
}

server fn saveTodo(userId: String, todo: Todo) {
    db.query("INSERT INTO todos VALUES (?, ?, ?)", [userId, todo.text, todo.done])
}

component TodoApp() {
    @persist("backend")  // Change ONE WORD!
    let todos = signal([]);

    // Add authentication
    let user = useAuth();

    // Same UI code as v1
}
```

**What works**: Everything + multi-user + cross-device sync
**What doesn't**: No real-time collaboration (that's v4 - realtime)

---

## 🏗️ Architecture

### Frontend (Client-side)
```
TodoApp Component
├── Auth State (user, token)
├── Todo State (todos signal with @persist)
├── UI Components
│   ├── LoginForm
│   ├── TodoInput
│   ├── TodoList
│   ├── TodoItem
│   └── TodoStats
└── Effects
    ├── Auto-save (via @persist)
    └── Update stats (computed)
```

### Backend (Server-side)
```
Server Functions
├── Authentication
│   ├── login(email, password) -> Token
│   ├── signup(email, password) -> Token
│   └── validateToken(token) -> User
└── Data Operations
    ├── loadTodos(userId) -> Vec<Todo>
    ├── saveTodo(userId, todo) -> Todo
    ├── updateTodo(userId, id, updates) -> Todo
    └── deleteTodo(userId, id) -> Bool
```

### Database Schema
```sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE todos (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    text TEXT NOT NULL,
    completed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

---

## 📊 Feature Comparison

| Feature | v1 Basic | v2 localStorage | v3 Backend |
|---------|----------|----------------|------------|
| Add/Edit/Delete | ✅ | ✅ | ✅ |
| Toggle Complete | ✅ | ✅ | ✅ |
| Stats Display | ✅ | ✅ | ✅ |
| Survives Refresh | ❌ | ✅ | ✅ |
| Multi-device Sync | ❌ | ❌ | ✅ |
| Multi-user | ❌ | ❌ | ✅ |
| Authentication | ❌ | ❌ | ✅ |
| Lines of Code | 50 | 52 | 150 |

---

## 🚀 How to Run

### Version 1 (Basic)
```bash
cd examples/phase15-week1-todo
jnc compile v1_basic.jnc
cd dist && python3 -m http.server 8080
# Open http://localhost:8080
```

### Version 2 (localStorage)
```bash
jnc compile v2_localStorage.jnc
cd dist && python3 -m http.server 8080
# Try refreshing the page - todos persist!
```

### Version 3 (Backend)
```bash
jnc compile v3_backend.jnc
cd dist && node server.js
# Full-stack server with authentication
```

---

## 🎓 Learning Objectives

1. **Understand Progressive Enhancement**
   - Start simple (v1)
   - Add persistence incrementally (v2)
   - Scale to full-stack (v3)

2. **See @persist in Action**
   - ONE LINE for localStorage
   - ONE WORD change for backend
   - Automatic code generation

3. **Learn Package Integration**
   - jounce-auth for users
   - jounce-db for data
   - jounce-ui for components

4. **Experience Full-Stack Development**
   - Client components
   - Server functions
   - Database operations

---

## 📝 Code Highlights

### The Power of @persist

**Before (manual localStorage - ~20 lines)**:
```javascript
const [todos, setTodos] = useState([]);

useEffect(() => {
  const stored = localStorage.getItem('todos');
  if (stored) setTodos(JSON.parse(stored));
}, []);

useEffect(() => {
  localStorage.setItem('todos', JSON.stringify(todos));
}, [todos]);
```

**After (with @persist - 1 line)**:
```jounce
@persist("localStorage")
let todos = signal([]);
// That's it! Auto-load + auto-save
```

### Computed Values

```jounce
let total = computed(() => todos.value.len());
let completed = computed(() =>
    todos.value.filter(t => t.done).len()
);
let remaining = computed(() => total.value - completed.value);
```

### Reactive UI Updates

```jounce
// When todos changes, stats update automatically
effect(() => {
    console.log("Total: " + total.value);
    console.log("Completed: " + completed.value);
    console.log("Remaining: " + remaining.value);
});
```

---

## 🎨 UI/UX Features

- **Beautiful Gradient Background** - Purple to pink
- **Smooth Animations** - Fade in/out, slide
- **Responsive Design** - Works on mobile/tablet/desktop
- **Dark Mode Support** - Toggle with one click
- **Accessible** - Keyboard navigation, ARIA labels
- **Loading States** - Spinners while fetching data
- **Error Handling** - User-friendly error messages

---

## 🧪 Testing Checklist

### v1 Basic
- [ ] Can add new todo
- [ ] Can toggle todo completion
- [ ] Can delete todo
- [ ] Stats update correctly
- [ ] Data lost on refresh (expected)

### v2 localStorage
- [ ] All v1 features work
- [ ] Data persists on refresh
- [ ] Works offline
- [ ] Different tabs share data

### v3 Backend
- [ ] All v2 features work
- [ ] Login/signup works
- [ ] Each user sees only their todos
- [ ] Data syncs across devices
- [ ] Logout clears session

---

## 📈 Success Metrics

- ✅ ~500 lines total (150 for v3)
- ✅ 4 packages integrated
- ✅ Full authentication flow
- ✅ Database CRUD operations
- ✅ Progressive enhancement demonstrated
- ✅ Beautiful, polished UI

---

**Status**: In Development
**Next**: Build v1_basic.jnc
