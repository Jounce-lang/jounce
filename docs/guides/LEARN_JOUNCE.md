# Learn Jounce - Complete Language Guide

**Last Updated**: November 5, 2025
**Version**: v0.8.2 "Runtime Safety Nets"
**Target Audience**: LLMs, AI assistants, and developers learning Jounce
**Prerequisite Knowledge**: Basic understanding of JavaScript/TypeScript and Rust

---

## Table of Contents

1. [What is Jounce?](#what-is-jounce)
2. [Core Syntax](#core-syntax)
3. [Components](#components)
4. [Reactivity System](#reactivity-system)
5. [JSX and UI](#jsx-and-ui)
6. [Styling](#styling)
7. [Functions and Logic](#functions-and-logic)
8. [Module System](#module-system)
9. [What Works vs What Doesn't](#what-works-vs-what-doesnt)
10. [Complete Examples](#complete-examples)

---

## What is Jounce?

Jounce is a **single-file, full-stack web framework** that compiles `.jnc` files into optimized server and client JavaScript.

**Key Principle**: Write **ONE `.jnc` file** → Get a complete web application

```bash
jnc compile app.jnc
# Generates: server.js, client.js, styles.css, index.html
```

**Language Design**:
- Syntax: Rust-inspired with JavaScript interop
- Paradigm: Component-based with fine-grained reactivity
- Type hints: Optional (Rust-style)
- Output: JavaScript (Node.js + Browser)

---

## Core Syntax

### 1. Variables

```jounce
// Immutable by default
let name = "Alice";
let age = 30;

// Mutable variables
let mut count = 0;
count = count + 1;

// Constants
const MAX_ITEMS = 100;
```

### 2. Types (Optional)

```jounce
let name: String = "Alice";
let age: i32 = 30;
let items: Vec<String> = ["a", "b", "c"];
let user: User = User { name: "Alice", age: 30 };
```

### 3. Functions

```jounce
// Function definition
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// Arrow functions (inside components)
let multiply = (a, b) => a * b;

// Async functions
async fn fetchData() {
    let response = await fetch("/api/data");
    return await response.json();
}
```

### 4. Control Flow

```jounce
// If statements
if count > 10 {
    console.log("Large");
} else if count > 5 {
    console.log("Medium");
} else {
    console.log("Small");
}

// Ternary expressions
let status = age >= 18 ? "Adult" : "Minor";

// While loops
while count < 10 {
    count = count + 1;
}

// For loops (RUST STYLE - NOT JavaScript!)
for item in items {
    console.log(item);
}

for i in 0..10 {
    console.log(i);  // 0 to 9
}

for i in 0..=10 {
    console.log(i);  // 0 to 10 (inclusive)
}
```

**⚠️ IMPORTANT**: Jounce uses **Rust-style for loops**, NOT JavaScript's `for (let x in array)`.

---

## Components

Components are the building blocks of Jounce applications.

### Basic Component

```jounce
component HelloWorld() {
    <div>
        <h1>Hello, World!</h1>
    </div>
}
```

### Component with Props

```jounce
component Greeting(name: String, age: i32) {
    <div>
        <h1>Hello, {name}!</h1>
        <p>You are {age} years old.</p>
    </div>
}

// Usage
<Greeting name="Alice" age={30} />
```

### Component with Return Type

```jounce
component Card(title: String, subtitle: String) -> JSX {
    <div class="card">
        <h2>{title}</h2>
        <p>{subtitle}</p>
    </div>
}
```

---

## Reactivity System

Jounce uses **fine-grained reactivity** with signals, computed values, and effects.

### 1. Signals (Reactive State)

```jounce
component Counter() {
    let count = signal(0);  // Create reactive signal

    let increment = () => {
        count.value = count.value + 1;  // Update via .value
    };

    <div>
        <p>Count: {count.value}</p>  // Read via .value
        <button onClick={increment}>Increment</button>
    </div>
}
```

**Key Points**:
- Use `signal(initialValue)` to create reactive state
- Read with `.value`
- Write with `.value = newValue`
- UI automatically updates when `.value` is accessed in JSX

### 2. Computed Values

```jounce
component TodoApp() {
    let todos = signal([
        { text: "Buy milk", done: false },
        { text: "Walk dog", done: true }
    ]);

    let remaining = computed(() => {
        return todos.value.filter((t) => !t.done).length();
    });

    <div>
        <p>Remaining: {remaining.value}</p>
        <p>Total: {todos.value.length()}</p>
    </div>
}
```

**Key Points**:
- Use `computed(() => ...)` for derived state
- Automatically re-runs when dependencies change
- Cached - only recomputes when needed

### 3. Effects (Side Effects)

```jounce
component Logger() {
    let count = signal(0);

    effect(() => {
        console.log("Count changed:", count.value);
    });

    // Effect runs whenever count.value changes
}
```

### 4. Persistent Signals (Local Storage)

```jounce
component App() {
    @persist("localStorage")
    let theme = signal("dark");

    // Automatically saves to localStorage on changes
    // Automatically loads from localStorage on mount
}
```

### 5. Lifecycle Hooks

```jounce
component MyComponent() {
    onMount(() => {
        console.log("Component mounted");
        return () => {
            console.log("Component unmounting");
        };
    });

    onDestroy(() => {
        console.log("Component destroyed");
    });
}
```

### 6. Runtime Safety (NEW in v0.8.2!) 🛡️

Jounce provides **3 layers of defense-in-depth protection** against common gotchas:

#### Layer 1: Type Checker (Compile-Time Errors)

```jounce
let count = signal(0);
count = 5;  // ❌ COMPILE ERROR: Cannot reassign signal variable
```

```jounce
let size = items.length();  // ❌ COMPILE ERROR: .length is a property
```

```jounce
<div>{await fetchData()}</div>  // ❌ COMPILE ERROR: await not allowed in JSX
```

#### Layer 2: Static Analyzer (Warnings)

```jounce
let count = signal(0);

fn inner() {
    let count = 5;  // ⚠️ WARNING: Shadows signal 'count'
}
```

```jounce
onMount(() => {
    setInterval(() => {
        count.value += 1;
    }, 1000);
    // ⚠️ WARNING: Missing cleanup function
});
```

#### Layer 3: Runtime Safety (Dev Mode)

```jounce
// Frozen signals prevent accidental reassignment
let count = signal(0);
count = 5;  // TypeError at runtime (if type checker missed it)
```

```jounce
// Dev-mode side effect detection
let doubled = computed(() => {
    console.log("Computing...");  // ❌ ERROR in dev mode!
    return count.value * 2;
});
```

**Key Principle**: `computed()` must be pure - no logging, network requests, or storage mutations.

**Correct Pattern**:
```jounce
// Use effect() for side effects
effect(() => {
    console.log("Count:", count.value);
});

// Keep computed() pure
let doubled = computed(() => count.value * 2);
```

---

## JSX and UI

Jounce supports full JSX syntax for building UIs.

### 1. Basic JSX

```jounce
<div>
    <h1>Title</h1>
    <p>Paragraph text</p>
</div>
```

### 2. Interpolation

```jounce
let name = "Alice";
let age = 30;

<div>
    <p>Name: {name}</p>
    <p>Age: {age}</p>
    <p>Sum: {10 + 20}</p>
</div>
```

### 3. Conditional Rendering

```jounce
let isLoggedIn = signal(false);

<div>
    {isLoggedIn.value ? (
        <p>Welcome back!</p>
    ) : (
        <p>Please log in</p>
    )}
</div>
```

### 4. List Rendering

```jounce
let items = signal(["Apple", "Banana", "Cherry"]);

<ul>
    {items.value.map((item) => {
        <li>{item}</li>
    })}
</ul>
```

**With index**:
```jounce
<ul>
    {items.value.map((item, idx) => {
        <li>{idx}: {item}</li>
    })}
</ul>
```

### 5. Event Handlers

```jounce
let count = signal(0);

let handleClick = () => {
    count.value = count.value + 1;
};

let handleInput = (e) => {
    console.log(e.target.value);
};

<div>
    <button onClick={handleClick}>Click Me</button>
    <input onInput={handleInput} />
</div>
```

### 6. String Interpolation in Attributes

```jounce
let active = signal(true);
let size = signal("large");

<button class="btn {active.value ? 'active' : ''} {size.value}">
    Click Me
</button>

// Generates reactive class updates
```

### 7. Self-Closing Tags

```jounce
<img src="photo.jpg" alt="Photo" />
<input type="text" placeholder="Enter name" />
<br />
```

---

## Styling

Jounce provides component-scoped styles with automatic scoping.

### 1. Component Styles

```jounce
component Card() {
    <div class="card">
        <h2>Title</h2>
    </div>
}

style Card {
    .card {
        padding: 20px;
        border-radius: 8px;
        background-color: white;
    }

    h2 {
        color: #333;
        font-size: 24px;
    }
}
```

**Key Points**:
- Styles are automatically scoped to the component
- No CSS-in-JS runtime overhead
- Compiled to pure CSS with hash-based class names

### 2. Pseudo-Classes

```jounce
style Button {
    .btn:hover {
        background-color: #0056b3;
    }

    .btn:active {
        transform: scale(0.95);
    }

    .btn:disabled {
        opacity: 0.5;
    }
}
```

### 3. Themes

```jounce
theme Light {
    primary: #007bff;
    secondary: #6c757d;
    background: #ffffff;
    text: #212529;
}

theme Dark {
    primary: #0d6efd;
    secondary: #6c757d;
    background: #212529;
    text: #ffffff;
}

component ThemedButton() {
    <button class="btn">Click Me</button>
}

style ThemedButton {
    .btn {
        background-color: var(--primary);
        color: var(--text);
    }
}
```

---

## Functions and Logic

### 1. Function Definitions

```jounce
// Inside components
component App() {
    fn processData(items: Vec<String>) -> String {
        let mut result = "";
        for item in items {
            result = result + item + ", ";
        }
        return result;
    }

    let data = processData(["a", "b", "c"]);
    <p>{data}</p>
}
```

### 2. Arrow Functions

```jounce
let add = (a, b) => a + b;
let greet = (name) => {
    return "Hello, " + name;
};
```

### 3. Async/Await

```jounce
async fn fetchUser(id: i32) {
    let response = await fetch("/api/users/" + id);
    let user = await response.json();
    return user;
}
```

**⚠️ IMPORTANT**: Jounce uses **prefix `await`**, NOT Rust's postfix `.await`.

---

## Module System

Jounce supports **local file imports only** (no package registry yet).

### 1. Importing from Local Files

```jounce
// Import specific items
use ./components::{Button, Card, Input};

// Import all exports
use ./utils::*;

// Import from parent directory
use ../lib/helpers::{formatText};

// Import from nested path
use ./modules/api/client::{fetchData};
```

### 2. Exporting (Automatic)

All top-level items are automatically exported:

```jounce
// components.jnc
component Button(text: String) {
    <button>{text}</button>
}

component Card(title: String) {
    <div class="card">
        <h3>{title}</h3>
    </div>
}

// Both Button and Card are automatically exported
```

**⚠️ IMPORTANT**: Jounce does NOT support package imports like `use jounce::*` yet.

---

## What Works vs What Doesn't

### ✅ What Works

**Syntax**:
- ✅ Rust-style for loops: `for x in array`
- ✅ Prefix await: `await fetch(url)`
- ✅ Local file imports: `use ./file::{Item}`
- ✅ Optional type annotations: `let x: i32 = 5`
- ✅ Mutable variables: `let mut x = 0`
- ✅ Arrow functions: `(x) => x + 1`

**Reactivity**:
- ✅ Signals with `.value` access
- ✅ Computed values with automatic tracking
- ✅ Effects for side effects
- ✅ Persistent signals (localStorage)
- ✅ Lifecycle hooks (onMount, onDestroy)

**JSX**:
- ✅ Components with props
- ✅ Conditional rendering (ternary)
- ✅ List rendering with `.map()`
- ✅ Event handlers (onClick, onInput, etc.)
- ✅ String interpolation in attributes
- ✅ Self-closing tags
- ✅ JSX in lambda expressions
- ✅ JSX in function returns

**Styling**:
- ✅ Component-scoped styles
- ✅ Pseudo-classes (:hover, :active, etc.)
- ✅ Theme system with CSS variables
- ✅ Nested selectors

### ❌ What Doesn't Work

**Syntax Limitations**:
- ❌ JavaScript for loops: `for (let x in array)` - Use `for x in array`
- ❌ Rust-style postfix await: `expr.await` - Use `await expr`
- ❌ Package imports: `use jounce::*` - Use `use ./local-file`
- ❌ Classes (OOP) - Use structs and impl blocks
- ❌ `this` keyword - Components are functions

**Module System Limitations**:
- ❌ `use jounce::db::*;` - Database ORM doesn't exist as user library (only Rust compiler internals)
- ❌ `use jounce::auth::*;` - Auth module doesn't exist as user library
- ⚠️ Workaround: Use Node.js libraries (pg, mysql2, bcrypt, jsonwebtoken) inside `server fn` blocks

**Type System Issues**:
- ❌ `result.is_ok()` in if statements - Type checker doesn't recognize boolean return
  ```jounce
  // ❌ Doesn't work
  if (result.is_ok()) {
      let value = result.unwrap();
  }

  // ✅ Use instead
  let value = result.unwrap_or(defaultValue);
  ```

**Annotations**:
- ❌ `@auth`, `@validate`, `@rate_limit` - Annotations parse but don't generate middleware yet
  ```jounce
  @auth  // ❌ Parses but does nothing
  server fn getData() -> Result<String, String> {
      return Ok("data");
  }
  ```
- ✅ `@persist("localStorage")` - Only this annotation works (for signals)

**Not Yet Implemented**:
- ⏳ Security middleware generation (Phase 17)
- ⏳ Package registry and remote imports
- ⏳ WebAssembly compilation (in progress)
- ⏳ Hot module replacement (HMR)
- ⏳ Server-side rendering (SSR)

---

## Complete Examples

### Example 1: Counter App

```jounce
component Counter() {
    let count = signal(0);

    let increment = () => {
        count.value = count.value + 1;
    };

    let decrement = () => {
        count.value = count.value - 1;
    };

    let reset = () => {
        count.value = 0;
    };

    <div class="counter">
        <h1>Count: {count.value}</h1>
        <div class="buttons">
            <button onClick={decrement}>-</button>
            <button onClick={reset}>Reset</button>
            <button onClick={increment}>+</button>
        </div>
    </div>
}

style Counter {
    .counter {
        text-align: center;
        padding: 40px;
    }

    .buttons {
        display: flex;
        gap: 10px;
        justify-content: center;
    }

    button {
        padding: 10px 20px;
        font-size: 18px;
        cursor: pointer;
    }
}
```

### Example 2: Todo App

```jounce
component TodoApp() {
    let todos = signal([
        { id: 1, text: "Buy milk", done: false },
        { id: 2, text: "Walk dog", done: false }
    ]);
    let newTodo = signal("");
    let nextId = signal(3);

    let addTodo = () => {
        if (newTodo.value.length() > 0) {
            let todo = {
                id: nextId.value,
                text: newTodo.value,
                done: false
            };
            todos.value = [...todos.value, todo];
            newTodo.value = "";
            nextId.value = nextId.value + 1;
        }
    };

    let toggleTodo = (id) => {
        todos.value = todos.value.map((todo) => {
            if (todo.id == id) {
                return { ...todo, done: !todo.done };
            }
            return todo;
        });
    };

    let remaining = computed(() => {
        return todos.value.filter((t) => !t.done).length();
    });

    <div class="todo-app">
        <h1>Todo List</h1>
        <p>Remaining: {remaining.value} / {todos.value.length()}</p>

        <div class="input-row">
            <input
                type="text"
                value={newTodo.value}
                onInput={(e) => newTodo.value = e.target.value}
                placeholder="Add todo..."
            />
            <button onClick={addTodo}>Add</button>
        </div>

        <ul class="todo-list">
            {todos.value.map((todo) => {
                <li class="{todo.done ? 'done' : ''}">
                    <input
                        type="checkbox"
                        checked={todo.done}
                        onChange={() => toggleTodo(todo.id)}
                    />
                    <span>{todo.text}</span>
                </li>
            })}
        </ul>
    </div>
}

style TodoApp {
    .todo-app {
        max-width: 600px;
        margin: 0 auto;
        padding: 20px;
    }

    .input-row {
        display: flex;
        gap: 10px;
        margin-bottom: 20px;
    }

    .input-row input {
        flex: 1;
        padding: 10px;
        font-size: 16px;
    }

    .todo-list {
        list-style: none;
        padding: 0;
    }

    .todo-list li {
        display: flex;
        align-items: center;
        gap: 10px;
        padding: 10px;
        border-bottom: 1px solid #eee;
    }

    .todo-list li.done span {
        text-decoration: line-through;
        opacity: 0.5;
    }
}
```

### Example 3: Fetch Data

```jounce
component UserProfile(userId: i32) {
    let user = signal(null);
    let loading = signal(true);
    let error = signal("");

    async fn fetchUser() {
        loading.value = true;
        error.value = "";

        let response = await fetch("/api/users/" + userId);

        if (response.ok) {
            let data = await response.json();
            user.value = data;
        } else {
            error.value = "Failed to load user";
        }

        loading.value = false;
    }

    onMount(() => {
        fetchUser();
    });

    <div class="profile">
        {loading.value ? (
            <p>Loading...</p>
        ) : error.value.length() > 0 ? (
            <p class="error">{error.value}</p>
        ) : user.value != null ? (
            <div>
                <h2>{user.value.name}</h2>
                <p>Email: {user.value.email}</p>
                <p>Age: {user.value.age}</p>
            </div>
        ) : (
            <p>No user found</p>
        )}
    </div>
}

style UserProfile {
    .profile {
        padding: 20px;
        border: 1px solid #ddd;
        border-radius: 8px;
    }

    .error {
        color: red;
    }
}
```

---

## Quick Reference

### Create Signal
```jounce
let count = signal(0);
```

### Read Signal
```jounce
count.value
```

### Write Signal
```jounce
count.value = 10;
```

### Computed Value
```jounce
let doubled = computed(() => count.value * 2);
```

### Effect
```jounce
effect(() => {
    console.log(count.value);
});
```

### Component
```jounce
component MyComponent(prop: Type) {
    <div>{prop}</div>
}
```

### Event Handler
```jounce
<button onClick={handleClick}>Click</button>
```

### Conditional
```jounce
{condition ? <div>Yes</div> : <div>No</div>}
```

### List
```jounce
{items.value.map((item) => <li>{item}</li>)}
```

### Async Function
```jounce
async fn fetch() {
    let data = await fetch(url);
}
```

### Import
```jounce
use ./module::{Item1, Item2};
```

---

## Next Steps

1. **Try the examples** - Copy and compile them with `jnc compile app.jnc`
2. **Read SYNTAX_LIMITATIONS.md** - Understand what doesn't work
3. **Check tutorial starters** - See real examples in `templates/tutorial-starters/`
4. **Experiment** - Build your own components and apps

---

**For Development/Maintenance**: See `CLAUDE.md` for compiler internals and development workflow.
