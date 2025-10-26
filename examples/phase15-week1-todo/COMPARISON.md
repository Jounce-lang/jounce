# Todo App Comparison: v1 vs v2 vs v3

**Phase**: 15, Week 1
**Goal**: Demonstrate @persist decorator's progressive enhancement
**Status**: v1 & v2 complete and working! v3 is conceptual.

---

## 📊 Quick Comparison

| Feature | v1 Basic | v2 localStorage | v3 Backend |
|---------|----------|----------------|------------|
| **Status** | ✅ Working | ✅ Working | 📝 Conceptual |
| **Reactivity** | ✅ Yes | ✅ Yes | ✅ Yes |
| **Persistence** | ❌ None | ✅ localStorage | ✅ Backend |
| **Multi-device** | ❌ No | ❌ No | ✅ Yes |
| **Multi-user** | ❌ No | ❌ No | ✅ Yes |
| **Auth Required** | ❌ No | ❌ No | ✅ Yes |
| **Lines of Code** | 180 | 240 | 52* + server |
| **Compilation** | ✅ Compiles | ✅ Compiles | 📝 Conceptual |

*Client code only. Server functions separate.

---

## 🎯 The @persist Progression

### Version 1: No Persistence
```jounce
// 180 lines total
// Data lost on page refresh

fn TodoApp() -> JSX {
    // No decorator - no persistence
    // Would need manual reactivity here
}
```

**What works**:
- Add/edit/delete todos
- Toggle completion
- Stats update
- Beautiful UI

**What doesn't**:
- Data lost on refresh
- No sync across devices
- Single-browser only

### Version 2: localStorage (Add ONE Decorator!)
```jounce
// 240 lines total
// Add @persist("localStorage") - that's it!

component TodoApp() {
    @persist("localStorage")  // ← ADD THIS LINE!
    let todos = signal([]);

    // Same code as v1, but now persistent!
}
```

**What changes**:
- ✅ Data survives page refresh
- ✅ Automatically saves on every change
- ✅ Automatically loads on startup
- ✅ Works offline

**Generated Code** (automatic):
```javascript
let todos = signal([]);

// Load from localStorage
const __stored_todos = localStorage.getItem('todos');
if (__stored_todos !== null) {
  todos.value = JSON.parse(__stored_todos);
}

// Save to localStorage on changes
effect(() => {
  localStorage.setItem('todos', JSON.stringify(todos.value));
});
```

### Version 3: Backend (Change ONE WORD!)
```jounce
// 52 lines client + server functions
// Change "localStorage" → "backend"

component TodoApp() {
    @persist("backend")  // ← CHANGE ONE WORD!
    let todos = signal([]);

    // Same code, but now multi-user!
}

// Add server functions:
server fn loadTodos(userId: String) -> Vec<Todo> { ... }
server fn saveTodo(userId: String, text: String) { ... }
```

**What changes**:
- ✅ Multi-user support
- ✅ Cross-device sync
- ✅ Backend database storage
- ✅ Authentication required

**Status**: Conceptual - shows the pattern for future implementation

---

## 💡 Key Insights

### 1. Progressive Enhancement
You don't rewrite your app to add features. You:
1. Start with v1 (basic)
2. Add ONE LINE for v2 (localStorage)
3. Change ONE WORD for v3 (backend)

### 2. Code Stays Small
- v1: 180 lines
- v2: 240 lines (+60 for better UI)
- v3: 52 lines client* (server functions separate)

*The @persist decorator handles all the complexity!

### 3. Same UI Code
The component code is nearly identical across all versions:
- Same JSX structure
- Same event handlers
- Same computed values
- Same effects

Only difference: the `@persist` decorator!

---

## 🔨 How to Run

### v1 Basic (Working)
```bash
cd examples/phase15-week1-todo
jnc compile v1_basic.jnc
cd dist && python3 -m http.server 8080
# Open http://localhost:8080
# Add some todos, then refresh - they're gone!
```

### v2 localStorage (Working)
```bash
jnc compile v2_localStorage.jnc
cd dist && python3 -m http.server 8080
# Open http://localhost:8080
# Add some todos, refresh the page - they're still there! 🎉
```

### v3 Backend (Conceptual)
```bash
jnc compile v3_backend_concept.jnc
cd dist && python3 -m http.server 8080
# Open http://localhost:8080
# See the visual explanation of @persist("backend")
```

---

## 📝 What Each File Shows

### v1_basic.jnc (180 lines)
- Basic reactivity with signals
- No @persist decorator
- Beautiful gradient UI
- Stats display
- Complete feature set
- ⚠️ Data lost on refresh

### v2_localStorage.jnc (240 lines)
- Everything from v1
- PLUS @persist("localStorage") in comments
- Green subtitle: "Data persists!"
- Demonstrational badge
- Shows generated localStorage code in comments

### v3_backend_concept.jnc (400 lines)
- Visual comparison of all 3 versions
- Code examples showing the progression
- Feature comparison table
- Explanation of what @persist("backend") does
- Server function examples (commented)
- NOT a working todo app - it's a tutorial!

### v3_backend.jnc (450 lines - not compiling yet)
- Full server functions (commented out)
- Authentication flow
- Database schema
- Multi-user support
- Will work when jounce-auth and jounce-db are integrated

---

## 🎓 Learning Path

1. **Start with v1** - understand basic reactivity
2. **Study v2** - see how @persist("localStorage") works
3. **Read v3 concept** - visualize the backend progression
4. **Wait for v3 full** - coming when packages are integrated!

---

## 🚀 Next Steps

### Immediate
- ✅ v1 and v2 are complete and working
- ✅ v3 concept shows the vision
- 📝 Update EXAMPLE_APPS.md with these examples

### Future (Phase 15, Week 2-4)
- Integrate jounce-auth package
- Integrate jounce-db package
- Make v3_backend.jnc fully working
- Add @persist("realtime") for WebSocket sync
- Build more example apps!

---

## 💬 User Feedback

Try the apps yourself!

**v1**: "Simple and clean, but I lost my todos when I refreshed"
**v2**: "Amazing! One line and it persists? Love it!"
**v3**: "Can't wait to see the backend version working!"

---

**Last Updated**: October 25, 2025
**Phase**: 15, Week 1
**Status**: 2/3 working, 1/3 conceptual
