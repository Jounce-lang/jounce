# Session 20 Ready - Build Real-World Example Apps! 🚀

**Date**: October 27, 2025
**Current Version**: v0.21.0 "Session 19 Complete"
**Tests**: ✅ 638/638 passing (100%)
**Status**: Ready to build real-world apps!

---

## 📍 Where We Are

### Session 19 COMPLETE ✅

**Delivered:**
- ✅ ErrorBoundary component with onError hook
- ✅ Suspense component for async loading
- ✅ 638/638 tests passing
- ✅ 98% CLIENT, 94% FULL-STACK complete
- ✅ Zero critical bugs, zero important issues
- ✅ Production-ready error handling

**Build Artifact Status:**
- ✅ Phase 1 at 95% (JavaScript everywhere)
- ✅ All runtime features working
- ⏸️ Missing manifest.json (2-3 hours)
- ⏸️ Missing rpc.schema.json (1-2 hours)

---

## 🎯 Session 20 Goals

**Primary Goal:** Build 3 real-world example apps to prove Jounce in production

**Time Estimate:** 8-12 hours

**Apps to Build:**

### 1. Todo App with Database (3-4 hours)
**Features:**
- Full CRUD with SQLite
- Server functions for persistence
- Reactive UI with signals
- Form validation
- Add/edit/delete/toggle todos
- Persistent storage

**Why:**
- Demonstrates full-stack capabilities
- Shows database integration
- Tests reactivity system
- Proves form handling

**File:** `examples/apps/todo-with-db/main.jnc`

---

### 2. User Management App (3-4 hours)
**Features:**
- User registration/login forms
- User list with edit/delete
- Search and filtering
- Session management (or simple auth)
- CRUD operations

**Why:**
- Demonstrates auth patterns
- Shows complex forms
- Tests jounce-forms package
- Proves routing integration

**File:** `examples/apps/user-management/main.jnc`

---

### 3. Real-Time Chat App (2-4 hours)
**Features:**
- WebSocket for real-time messaging
- Chat rooms
- User presence
- Message history with database
- Live updates

**Why:**
- Demonstrates WebSocket auto-setup
- Shows real-time reactivity
- Tests database + WebSocket together
- Proves complex state management

**File:** `examples/apps/realtime-chat/main.jnc`

---

## ⚠️ CRITICAL REMINDERS

### **DO NOT FORGET THESE PRINCIPLES:**

1. **ONE .jnc FILE → WORKING APP**
   - Each app must be a SINGLE .jnc file
   - NO multiple files
   - NO separate CSS files
   - NO manual post-compilation steps

2. **NO QUICK FIXES**
   - Do it the RIGHT way even if it takes longer
   - Fix architecture, not symptoms
   - Test thoroughly before marking complete

3. **BUILD ARTIFACT ARCHITECTURE**
   - Phase 1: JavaScript everywhere (95% complete)
   - See ROADMAP.md for full details
   - No "single .wasm file only" - always need HTML + loader

---

## 🔧 What Works RIGHT NOW

### Compiler & Language
- ✅ Lexer, Parser, Type Checker, Codegen
- ✅ Generic types `<T>`
- ✅ Enums, structs, traits, impl blocks
- ✅ Lambda expressions
- ✅ Match expressions
- ✅ JSX with full component support
- ✅ Glob imports `use foo::*;`

### Reactivity
- ✅ `signal<T>(value)` - Reactive state
- ✅ `computed<T>(() => expr)` - Derived state
- ✅ `effect(() => {})` - Side effects
- ✅ `batch(() => {})` - Batched updates
- ✅ `persistentSignal<T>(key, value)` - localStorage persistence

### Components
- ✅ Component functions with props
- ✅ JSX rendering with h()
- ✅ `onMount(() => {})` - After DOM insertion
- ✅ `onUnmount(() => {})` - Before removal
- ✅ `onUpdate(() => {})` - On updates
- ✅ `onError((error) => {})` - Error handling
- ✅ `<ErrorBoundary>` - Catch component errors
- ✅ `<Suspense>` - Loading states

### Full-Stack
- ✅ `@server` functions - Backend logic
- ✅ `@client` components - Frontend UI
- ✅ RPC auto-generated between server/client
- ✅ Database access with SQLite
- ✅ Routing with navigate() and getRouter()
- ✅ WebSocket auto-setup (just import jounce-websocket)
- ✅ Environment variables (.env support)

### Packages Available
- ✅ jounce-forms - Form validation
- ✅ jounce-db - Database operations
- ✅ jounce-websocket - Real-time communication
- ✅ jounce-auth - Authentication (if needed)
- ✅ 35 total packages in ecosystem

---

## 📝 Example App Template

Here's the basic structure for each app:

```jounce
// Example: Todo App with Database

// Server function for backend logic
@server
fn getTodos() -> Array<Todo> {
    script {
        const db = getDB();
        const todos = db.query('SELECT * FROM todos');
        return todos;
    }
}

@server
fn addTodo(text: String) -> Todo {
    script {
        const db = getDB();
        const result = db.execute('INSERT INTO todos (text, done) VALUES (?, ?)', [text, false]);
        return { id: result.lastInsertRowid, text, done: false };
    }
}

// Client components
component TodoItem(props) {
    let { todo, onToggle, onDelete } = props;

    return <div class="todo-item">
        <input
            type="checkbox"
            checked={todo.done}
            onchange={() => onToggle(todo.id)}
        />
        <span>{todo.text}</span>
        <button onclick={() => onDelete(todo.id)}>Delete</button>
    </div>;
}

component App() {
    let todos = signal<Array<Todo>>([]);
    let newTodoText = signal("");

    onMount(() => {
        // Load todos from server
        getTodos().then(data => todos.set(data));
    });

    let handleAdd = () => {
        if newTodoText.get() != "" {
            addTodo(newTodoText.get()).then(todo => {
                todos.set([...todos.get(), todo]);
                newTodoText.set("");
            });
        }
    };

    return <div>
        <h1>Todo App</h1>

        <div>
            <input
                value={newTodoText.get()}
                oninput={(e) => newTodoText.set(e.target.value)}
                placeholder="New todo..."
            />
            <button onclick={handleAdd}>Add</button>
        </div>

        <div>
            {todos.get().map(todo =>
                <TodoItem
                    todo={todo}
                    onToggle={(id) => {/* handle toggle */}}
                    onDelete={(id) => {/* handle delete */}}
                />
            )}
        </div>
    </div>;
}
```

---

## 🚀 How to Build and Test

### Compile an App
```bash
cargo run -- compile examples/apps/todo-with-db/main.jnc
```

### Run the Server
```bash
cd dist && node server.js
```

### Test in Browser
```
Open http://localhost:3000
```

### Verify Tests
```bash
cargo test --lib
```

---

## ✅ Success Criteria for Session 20

**Each app must:**
1. ✅ Compile from a SINGLE .jnc file
2. ✅ Run without manual setup steps
3. ✅ Demonstrate at least 3 major features
4. ✅ Have working UI with reactivity
5. ✅ Use database or WebSocket or both
6. ✅ Be documented with comments
7. ✅ Work in browser without errors

**Session 20 complete when:**
- ✅ 3 real-world apps built and working
- ✅ All apps in `examples/apps/` directory
- ✅ Each app is a single .jnc file
- ✅ All 638 tests still passing
- ✅ Documentation updated
- ✅ Ready to add manifest.json and rpc.schema.json in Session 21

---

## 📖 Key Documentation Files

**Read before starting:**
1. `CLAUDE.md` - Current status and warnings
2. `ROADMAP.md` - Build artifact architecture (Phase 1 at 95%)
3. `SESSION_19_COMPLETE.md` - What we just completed
4. `FEATURES.md` - All available language features

**Update after Session 20:**
1. `CLAUDE.md` - Update with Session 20 results
2. `SESSION_20_COMPLETE.md` - Create summary
3. `examples/apps/README.md` - Document the apps

---

## 🎯 What to Build First

**Recommended Order:**

1. **Start with Todo App** (3-4 hours)
   - Most straightforward
   - Proves full-stack + database
   - Good warm-up

2. **Then User Management** (3-4 hours)
   - More complex forms
   - Tests jounce-forms
   - Routing patterns

3. **Finally Chat App** (2-4 hours)
   - Most complex
   - Real-time features
   - WebSocket + Database together

---

## 🎉 After Session 20

**Next Steps (Session 21):**
1. Add manifest.json generation (2-3 hours)
2. Add rpc.schema.json generation (1-2 hours)
3. Phase 1 at 100%!

**Then (Sessions 22+):**
- Build 7+ more example apps
- Prove Phase 1 in diverse scenarios
- Gather requirements for Phase 2 (WASM)

---

## 💡 Tips for Success

1. **Keep it simple** - Each app should be focused and clear
2. **Use TodoWrite** - Track progress with the todo tool
3. **Test as you go** - Run cargo test --lib frequently
4. **Commit often** - After each app, commit with good message
5. **Follow examples** - Look at existing test files for patterns
6. **Remember the principle** - ONE .jnc FILE → WORKING APP

---

**Session 20: READY TO START! 🚀**
**Goal:** Build 3 real-world example apps
**Time:** 8-12 hours
**Result:** Prove Jounce works in production!
