# Todo App Template

**Full-featured todo list** - Learn array operations, filtering, and computed values!

## Features

✅ **Add/Delete todos** - CRUD operations
✅ **Toggle completion** - Interactive checkboxes
✅ **Filter by status** - All, Active, Completed views
✅ **Computed values** - Auto-calculated counts
✅ **Keyboard shortcuts** - Enter to add
✅ **Conditional rendering** - Dynamic UI
✅ **Array operations** - map, filter, spread operator

## Quick Start

```bash
# 1. Copy this template
cp -r templates/todo-app my-todo-app
cd my-todo-app

# 2. Compile and run
jnc compile main.jnc
cd dist && node server.js

# 3. Open browser
open http://localhost:3000
```

## What You'll Learn

### 1. Array Operations
```jounce
// Add item
todos.value = [...todos.value, newTodo];

// Update item
todos.value = todos.value.map((todo) => {
    if (todo.id == id) {
        return { ...todo, completed: !todo.completed };
    }
    return todo;
});

// Remove item
todos.value = todos.value.filter((todo) => todo.id != id);
```

### 2. Computed Values
```jounce
let activeCount = computed(() => {
    return todos.value.filter((todo) => !todo.completed).length;
});

// Use in JSX
<span>{activeCount.value} items left</span>
```

### 3. Event Handling
```jounce
// Input events
<input
    oninput={(e) => input.value = e.target.value}
    onkeydown={(e) => {
        if (e.key == "Enter") {
            addTodo();
        }
    }}
/>

// Click events with parameters
<button onclick={() => deleteTodo(todo.id)}>
    Delete
</button>
```

### 4. Conditional Rendering
```jounce
{filteredTodos.value.length == 0 && (
    <div>No todos yet!</div>
)}

{todos.value.length > 0 && (
    <div>Footer content</div>
)}
```

### 5. Dynamic Classes
```jounce
<div class={`p-3 ${todo.completed ? "bg-light" : "bg-white"}`}>
<span class={`${todo.completed ? "line-through" : ""}`}>
```

## Code Structure

- **State** (lines 5-9): All app state with signals
- **Actions** (lines 11-37): Functions to modify state
- **Computed** (lines 39-53): Derived values
- **Render** (lines 55+): JSX with event handlers

## Customization Ideas

1. **Add categories** - Tag todos with projects
2. **Add due dates** - Sort by deadline
3. **Add priority** - High/medium/low importance
4. **Persist to localStorage** - Save on page reload
5. **Add subtasks** - Nested todo lists
6. **Add search** - Filter by text
7. **Add drag & drop** - Reorder todos

## Advanced Features

Try adding these to level up:

```jounce
// Persistence
effect(() => {
    localStorage.setItem("todos", JSON.stringify(todos.value));
});

// Search
let searchQuery = signal("");
let searchedTodos = computed(() => {
    return todos.value.filter((todo) =>
        todo.text.toLowerCase().includes(searchQuery.value.toLowerCase())
    );
});

// Sorting
let sortedTodos = computed(() => {
    return [...todos.value].sort((a, b) => b.id - a.id);
});
```

## Learn More

- [Reactivity Guide](../../docs/REACTIVITY.md)
- [Array Methods](../../docs/ARRAYS.md)
- [Event Handling](../../docs/EVENTS.md)

---

**Difficulty:** Intermediate
**Time:** 15 minutes
**Lines:** 155
