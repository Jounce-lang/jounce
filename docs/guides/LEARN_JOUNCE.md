# Learn Jounce - Become Productive Fast

> **Note**: This is a teaching guide. If anything here conflicts with [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md), follow JOUNCE_SPEC.md.

**Summary**: Build full-stack reactive web apps in minutes, not hours.

**Last Updated**: November 7, 2025
**Version**: v0.8.3 "Enhanced Language Features"
**For**: Developers who want to ship fast
**Time to First App**: < 10 minutes

> **Note**: For technical specifications, see [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md)

---

## Philosophy

**Learn by Building**. This guide teaches Jounce by building 5 progressively complex apps:

1. **Counter** (5 mins) - Reactivity basics
2. **Todo List** (15 mins) - State management
3. **Contact Form** (15 mins) - Validation and server functions
4. **Dashboard** (20 mins) - Components and composition
5. **Blog** (30 mins) - Full-stack CRUD app

By the end, you'll be shipping production apps.

---

## Prerequisites

- Basic understanding of JavaScript/TypeScript
- Familiarity with React or similar frameworks (helpful but not required)
- Node.js v16+ installed
- Text editor (VSCode recommended)

---

## Output Directory Structure (v0.8.x+)

Starting in v0.8.x, each compiled app goes to its own subdirectory to prevent conflicts:

```bash
examples/apps/my-app/main.jnc  →  dist/my-app/
my-project/src/app.jnc         →  dist/src/
counter.jnc                    →  dist/app/  (fallback)
```

This means when you compile, you'll run:
```bash
jnc compile my-app/main.jnc
cd dist/my-app && node server.js
```

---

## How This Relates to the Spec

This guide teaches Jounce through practical examples. For authoritative language behavior, see:

- **[JOUNCE_SPEC.md](../../JOUNCE_SPEC.md)** - Complete language specification (canonical reference)
- **[docs/README.md](../README.md)** - Full documentation hierarchy

**When in doubt**: The spec wins. This guide focuses on common patterns and may simplify complex details for learning. Always check the spec for edge cases and precise behavior.

---

## LLM AUTHORING RULES (v0.8.x)

When generating Jounce code, follow these rules:

- ✅ Use `signal(...)` (not `createSignal(...)`)
- ✅ Use lowercase DOM events: `onclick`, `oninput`, `onchange` (not `onClick`, `onInput`, `onChange`)
- ✅ Components must `return <...>;` (explicit JSX return required)
- ✅ Use Rust-style loops: `for i in 0..list.length() { ... }` (not JavaScript `for (let i = 0; i < list.length(); i++)`)
- ✅ No `await` inside JSX (async operations must be outside JSX expressions)
- ✅ Style: `@media` queries only at top-level in style blocks (not nested inside selectors)

---

## Setup (2 Minutes)

```bash
# Clone and build
git clone https://github.com/Jounce-lang/jounce-pre-production.git
cd jounce-pre-production
cargo build --release

# Add to PATH
export PATH="$PWD/target/release:$PATH"

# Verify installation
jnc --version  # Should show v0.8.3
```

---

## Tutorial 1: Counter (5 Minutes)

### Goal
Build a reactive counter that updates the UI automatically.

### What You'll Learn
- Components
- Signals (reactive state)
- Event handlers
- Basic styling

### Build It

Create `counter.jnc`:

```jounce
component Counter() {
    // Reactive state
    let count = signal<i32>(0);

    return <div>
        <h1>Count: {count.value}</h1>
        <button onclick={() => count.value++}>+1</button>
        <button onclick={() => count.value--}>-1</button>
        <button onclick={() => count.value = 0}>Reset</button>
    </div>;
}

style Counter {
    padding: 40px;
    text-align: center;
    font-family: Arial, sans-serif;

    h1 {
        font-size: 48px;
        margin: 20px 0;
        color: #333;
    }

    button {
        font-size: 18px;
        padding: 10px 20px;
        margin: 5px;
        cursor: pointer;
        border: 2px solid #007bff;
        background: white;
        border-radius: 5px;
        transition: all 0.2s;
    }

    button:hover {
        background: #007bff;
        color: white;
    }
}
```

### Run It

```bash
jnc compile counter.jnc
cd dist && node server.js
# Open http://localhost:3000
```

### Key Concepts

**Signals** create reactive state:
```jounce
let count = signal<i32>(0);
```

**Reading** `.value` gets the current value:
```jounce
<h1>Count: {count.value}</h1>
```

**Writing** `.value` triggers UI updates:
```jounce
count.value++  // UI updates automatically!
```

**Scoped Styles** only apply to this component:
```jounce
style Counter {
    button { ... }  // Only affects buttons in Counter
}
```

### Next Steps
Try adding a "Double" button that multiplies the count by 2.

---

## Tutorial 2: Todo List (15 Minutes)

### Goal
Build a todo app with add/delete/toggle functionality.

### What You'll Learn
- Arrays and iteration
- Computed values
- Filtering and mapping
- Conditional rendering

### Build It

Create `todo.jnc`:

```jounce
struct Todo {
    id: i32,
    text: string,
    completed: bool,
}

component TodoApp() {
    let todos = signal<Vec<Todo>>(vec![]);
    let input = signal<string>("");
    let filter = signal<string>("all");

    // Computed: filtered todos
    let filteredTodos = computed<Vec<Todo>>(() => {
        let allTodos = todos.value;
        let currentFilter = filter.value;

        if currentFilter == "active" {
            return allTodos.filter((t) => !t.completed);
        } else if currentFilter == "completed" {
            return allTodos.filter((t) => t.completed);
        }
        return allTodos;
    });

    // Add todo
    let addTodo = () => {
        let text = input.value.trim();
        if text.len() > 0 {
            let newTodo = Todo {
                id: todos.value.len() as i32,
                text: text,
                completed: false,
            };
            todos.value = vec![...todos.value, newTodo];
            input.value = "";
        }
    };

    // Toggle todo
    let toggleTodo = (id: i32) => {
        todos.value = todos.value.map((t) => {
            if t.id == id {
                return Todo { ...t, completed: !t.completed };
            }
            return t;
        });
    };

    // Delete todo
    let deleteTodo = (id: i32) => {
        todos.value = todos.value.filter((t) => t.id != id);
    };

    return <div>
        <h1>My Todos</h1>

        <div class="input-container">
            <input
                type="text"
                value={input.value}
                oninput={(e) => input.value = e.target.value}
                onkeypress={(e) => {
                    if e.key == "Enter" {
                        addTodo();
                    }
                }}
                placeholder="What needs to be done?"
            />
            <button onclick={addTodo}>Add</button>
        </div>

        <div class="filters">
            <button
                class={filter.value == "all" ? "active" : ""}
                onclick={() => filter.value = "all"}
            >
                All
            </button>
            <button
                class={filter.value == "active" ? "active" : ""}
                onclick={() => filter.value = "active"}
            >
                Active
            </button>
            <button
                class={filter.value == "completed" ? "active" : ""}
                onclick={() => filter.value = "completed"}
            >
                Completed
            </button>
        </div>

        <ul>
            {filteredTodos.value.map((todo) =>
                <li class={todo.completed ? "completed" : ""}>
                    <input
                        type="checkbox"
                        checked={todo.completed}
                        onchange={() => toggleTodo(todo.id)}
                    />
                    <span>{todo.text}</span>
                    <button class="delete" onclick={() => deleteTodo(todo.id)}>
                        ×
                    </button>
                </li>
            )}
        </ul>
    </div>;
}

style TodoApp {
    max-width: 600px;
    margin: 40px auto;
    padding: 20px;

    h1 {
        text-align: center;
        color: #2c3e50;
    }

    .input-container {
        display: flex;
        margin-bottom: 20px;

        input {
            flex: 1;
            padding: 10px;
            font-size: 16px;
            border: 2px solid #ddd;
            border-radius: 5px 0 0 5px;
        }

        button {
            padding: 10px 20px;
            background: #007bff;
            color: white;
            border: none;
            border-radius: 0 5px 5px 0;
            cursor: pointer;
        }
    }

    .filters {
        display: flex;
        gap: 10px;
        margin-bottom: 20px;

        button {
            padding: 5px 15px;
            border: 2px solid #ddd;
            background: white;
            cursor: pointer;
            border-radius: 5px;
        }

        button.active {
            background: #007bff;
            color: white;
            border-color: #007bff;
        }
    }

    ul {
        list-style: none;
        padding: 0;

        li {
            display: flex;
            align-items: center;
            padding: 10px;
            border-bottom: 1px solid #eee;

            span {
                flex: 1;
                margin-left: 10px;
            }

            .delete {
                color: #e74c3c;
                border: none;
                background: none;
                font-size: 24px;
                cursor: pointer;
            }
        }

        li.completed span {
            text-decoration: line-through;
            color: #999;
        }
    }
}
```

### Run It

```bash
jnc compile todo.jnc
cd dist && node server.js
```

### Key Concepts

**Computed values** automatically update when dependencies change:
```jounce
let filteredTodos = computed<Vec<Todo>>(() => {
    return todos.value.filter((t) => !t.completed);
});
```

**Array operations** work like JavaScript:
```jounce
// Map
todos.value.map((t) => ...)

// Filter
todos.value.filter((t) => ...)

// Spread
vec![...todos.value, newTodo]
```

**Conditional classes**:
```jounce
<li class={todo.completed ? "completed" : ""}>
```

### Next Steps
Add a "Clear Completed" button and a counter showing how many todos are left.

---

## Tutorial 3: Contact Form with Server (15 Minutes)

### Goal
Build a form that validates input and sends data to the server.

### What You'll Learn
- Server functions (`@server`)
- Automatic RPC
- Form validation
- Error handling

### Build It

Create `contact.jnc`:

```jounce
struct FormData {
    name: string,
    email: string,
    message: string,
}

@server
fn submitContact(data: FormData) -> Result<string, string> {
    // Validate on server
    if data.name.len() < 2 {
        return Err("Name must be at least 2 characters");
    }

    if !data.email.contains("@") {
        return Err("Invalid email address");
    }

    if data.message.len() < 10 {
        return Err("Message must be at least 10 characters");
    }

    // In real app: save to database
    console.log("Received contact from: " + data.name);

    return Ok("Thanks for contacting us!");
}

component ContactForm() {
    let name = signal<string>("");
    let email = signal<string>("");
    let message = signal<string>("");
    let status = signal<string>("");
    let error = signal<string>("");
    let submitting = signal<bool>(false);

    let handleSubmit = () => {
        submitting.value = true;
        error.value = "";
        status.value = "";

        let data = FormData {
            name: name.value,
            email: email.value,
            message: message.value,
        };

        // Automatic RPC to server
        let result = submitContact(data);

        if let Ok(msg) = result {
            status.value = msg;
            name.value = "";
            email.value = "";
            message.value = "";
        } else if let Err(err) = result {
            error.value = err;
        }

        submitting.value = false;
    };

    return <div>
        <h1>Contact Us</h1>

        <form onsubmit={(e) => {
            e.preventDefault();
            handleSubmit();
        }}>
            <div class="field">
                <label>Name</label>
                <input
                    type="text"
                    value={name.value}
                    oninput={(e) => name.value = e.target.value}
                    placeholder="Your name"
                    required
                />
            </div>

            <div class="field">
                <label>Email</label>
                <input
                    type="email"
                    value={email.value}
                    oninput={(e) => email.value = e.target.value}
                    placeholder="you@example.com"
                    required
                />
            </div>

            <div class="field">
                <label>Message</label>
                <textarea
                    value={message.value}
                    oninput={(e) => message.value = e.target.value}
                    placeholder="Your message..."
                    rows="5"
                    required
                ></textarea>
            </div>

            {error.value.len() > 0 ? (
                <div class="error">{error.value}</div>
            ) : null}

            {status.value.len() > 0 ? (
                <div class="success">{status.value}</div>
            ) : null}

            <button
                type="submit"
                disabled={submitting.value}
            >
                {submitting.value ? "Sending..." : "Send Message"}
            </button>
        </form>
    </div>;
}

style ContactForm {
    max-width: 500px;
    margin: 40px auto;
    padding: 20px;

    h1 {
        text-align: center;
        color: #2c3e50;
    }

    .field {
        margin-bottom: 20px;

        label {
            display: block;
            margin-bottom: 5px;
            font-weight: bold;
            color: #555;
        }

        input, textarea {
            width: 100%;
            padding: 10px;
            font-size: 16px;
            border: 2px solid #ddd;
            border-radius: 5px;
            font-family: Arial, sans-serif;
        }

        input:focus, textarea:focus {
            outline: none;
            border-color: #007bff;
        }
    }

    .error {
        padding: 10px;
        background: #fee;
        border: 1px solid #fcc;
        color: #c00;
        border-radius: 5px;
        margin-bottom: 15px;
    }

    .success {
        padding: 10px;
        background: #efe;
        border: 1px solid #cfc;
        color: #060;
        border-radius: 5px;
        margin-bottom: 15px;
    }

    button {
        width: 100%;
        padding: 12px;
        font-size: 18px;
        background: #007bff;
        color: white;
        border: none;
        border-radius: 5px;
        cursor: pointer;
    }

    button:hover:not(:disabled) {
        background: #0056b3;
    }

    button:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }
}
```

### Run It

```bash
jnc compile contact.jnc
cd dist && node server.js
```

### Key Concepts

**Server functions** run on the backend:
```jounce
@server
fn submitContact(data: FormData) -> Result<string, string> {
    // This code runs on the server!
    console.log("Server received: " + data.name);
    return Ok("Success!");
}
```

**Automatic RPC** from client to server:
```jounce
// This looks like a normal function call...
let result = submitContact(data);

// But it's actually an HTTP request to the server!
```

**Pattern matching with if-let** (v0.8.3):
```jounce
if let Ok(msg) = result {
    status.value = msg;
} else if let Err(err) = result {
    error.value = err;
}
```

**Ternary for conditional rendering**:
```jounce
{error.value.len() > 0 ? (
    <div class="error">{error.value}</div>
) : null}
```

### Next Steps
Add client-side validation that shows errors as you type, before submitting.

---

## Common Patterns

### Pattern 1: Loading States

```jounce
let loading = signal<bool>(false);
let data = signal<Vec<Item>>(vec![]);

let fetchData = () => {
    loading.value = true;
    let result = fetchItems();  // @server function
    data.value = result;
    loading.value = false;
};

return <div>
    {loading.value ? (
        <div>Loading...</div>
    ) : (
        <ul>{data.value.map((item) => <li>{item.name}</li>)}</ul>
    )}
</div>;
```

### Pattern 2: Debounced Input

```jounce
let searchTerm = signal<string>("");
let timer = signal<i32>(0);

let handleSearch = (value: string) => {
    clearTimeout(timer.value);
    timer.value = setTimeout(() => {
        performSearch(value);
    }, 300);
};

<input
    value={searchTerm.value}
    oninput={(e) => {
        searchTerm.value = e.target.value;
        handleSearch(e.target.value);
    }}
/>
```

### Pattern 3: Reusable Components

```jounce
// Button.jnc
component Button(props: {
    label: string,
    onclick: fn(),
    variant: string,
}) {
    return <button
        class={"btn btn-" + props.variant}
        onclick={props.onclick}
    >
        {props.label}
    </button>;
}

style Button {
    .btn {
        padding: 10px 20px;
        border: none;
        border-radius: 5px;
        cursor: pointer;
    }

    .btn-primary { background: #007bff; color: white; }
    .btn-danger { background: #dc3545; color: white; }
    .btn-success { background: #28a745; color: white; }
}

// Use it
<Button
    label="Click Me"
    onclick={() => console.log("Clicked")}
    variant="primary"
/>
```

### Pattern 4: Modal Dialog

```jounce
component Modal(props: { isOpen: bool, onclose: fn(), title: string }) {
    if !props.isOpen {
        return null;
    }

    return <div class="modal-backdrop" onclick={props.onclose}>
        <div class="modal-content" onclick={(e) => e.stopPropagation()}>
            <div class="modal-header">
                <h2>{props.title}</h2>
                <button class="close" onclick={props.onclose}>×</button>
            </div>
            <div class="modal-body">
                {props.children}
            </div>
        </div>
    </div>;
}

style Modal {
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0,0,0,0.5);
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .modal-content {
        background: white;
        border-radius: 8px;
        max-width: 500px;
        width: 90%;
        max-height: 80vh;
        overflow: auto;
    }

    .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 20px;
        border-bottom: 1px solid #eee;

        h2 { margin: 0; }

        .close {
            background: none;
            border: none;
            font-size: 32px;
            cursor: pointer;
            color: #999;
        }
    }

    .modal-body {
        padding: 20px;
    }
}
```

---

## Best Practices

### 1. Component Organization

```jounce
// ✅ Good: Small, focused components
component UserCard(props: { user: User }) {
    return <div>
        <h3>{props.user.name}</h3>
        <p>{props.user.email}</p>
    </div>;
}

// ❌ Bad: Giant component doing everything
component App() {
    // 500 lines of code...
}
```

### 2. Signal Naming

```jounce
// ✅ Good: Clear names
let isLoading = signal<bool>(false);
let userCount = signal<i32>(0);
let selectedItem = signal<Option<Item>>(None);

// ❌ Bad: Vague names
let data = signal<bool>(false);
let x = signal<i32>(0);
```

### 3. Computed vs Effects

```jounce
// ✅ Good: Use computed for derived values
let filteredItems = computed<Vec<Item>>(() => {
    return items.value.filter((i) => i.active);
});

// ❌ Bad: Don't use effects for derived values
effect(() => {
    filteredItems.value = items.value.filter((i) => i.active);
});

// ✅ Good: Use effects for side effects
effect(() => {
    console.log("Count changed: " + count.value.to_string());
    saveToLocalStorage("count", count.value);
});
```

### 4. Type Safety

```jounce
// ✅ Good: Use strong types
struct User {
    id: i32,
    name: string,
    email: string,
}

let users = signal<Vec<User>>(vec![]);

// ❌ Bad: Weak types lead to bugs
let users = signal<Vec<any>>(vec![]);
```

---

## Troubleshooting

### Common Errors

#### Error: "Expected `;` after expression"

**Problem**: Forgot semicolon
```jounce
// ❌ Wrong
let count = signal<i32>(0)  // Missing ;

// ✅ Correct
let count = signal<i32>(0);
```

#### Error: "Cannot call server function from server context"

**Problem**: Trying to call `@server` function from another `@server` function
```jounce
// ❌ Wrong
@server
fn getUser() {
    return fetchFromDB();  // Can't call @server from @server
}

// ✅ Correct: Extract shared logic
fn fetchFromDB() {
    return database.query(...);
}

@server
fn getUser() {
    return fetchFromDB();  // Regular function call
}
```

#### Error: "Signal reassignment is not allowed"

**Problem**: Trying to reassign the signal itself
```jounce
// ❌ Wrong
let count = signal<i32>(0);
count = signal<i32>(5);  // Can't reassign signal

// ✅ Correct: Update the value
let count = signal<i32>(0);
count.value = 5;  // Update signal's value
```

---

## Next Steps

### Explore Examples

Check out [examples/](../../examples/) for 35+ working apps:

- **Real Apps**: Blog, Dashboard, E-commerce, Chat
- **UI Patterns**: Modals, Tabs, Dropdowns, Tooltips
- **Advanced**: Authentication, Database, WebSockets

### Read the Spec

For complete technical details, see [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md):

- Full grammar reference
- All CLI commands
- Execution model
- Limitations
- Implemented vs planned features

### Join the Community

- **GitHub Issues**: Report bugs or request features
- **Discussions**: Ask questions and share projects
- **Examples**: Contribute your own apps

---

## Common Mistakes

New Jounce developers often trip on these syntax differences from JavaScript/React:

### 1. ❌ React-Style Event Handlers
```jounce
// ❌ WRONG - React-style camelCase
<button onClick={() => ...}>

// ✅ CORRECT - Lowercase DOM-style
<button onclick={() => ...}>
```

All event handlers must be lowercase: `onclick`, `onchange`, `oninput`, `onsubmit`, etc.

### 2. ❌ Async/Await Syntax
```jounce
// ❌ WRONG - No await in Jounce
let data = await fetchData();

// ✅ CORRECT - Use Result<T, E> with @server functions
@server
fn fetchData() -> Result<Data, string> { ... }

let result = fetchData();  // Returns Result immediately via RPC
match result {
    Ok(data) => ...,
    Err(e) => ...,
}
```

### 3. ❌ Template String Literals
```jounce
// ❌ WRONG - No ${} template strings
let msg = `Hello ${name}`;

// ✅ CORRECT - String concatenation
let msg = "Hello " + name;
```

### 4. ❌ JavaScript-Style For Loops
```jounce
// ❌ WRONG - No C-style for loops
for (let i = 0; i < 10; i++) { ... }

// ✅ CORRECT - Rust-style ranges
for i in 0..10 { ... }
```

### 5. ❌ Missing Return Keyword
```jounce
// ❌ WRONG - Implicit returns not supported
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// ✅ CORRECT - Explicit return
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

### 6. ❌ Signal Reassignment
```jounce
// ❌ WRONG - Can't reassign the signal itself
let count = signal<i32>(0);
count = signal<i32>(5);  // Error!

// ✅ CORRECT - Update the value
count.value = 5;
```

For complete limitations, see [JOUNCE_SPEC.md § Limitations](../../JOUNCE_SPEC.md#limitations).

---

## Quick Reference Card

### Reactivity
```jounce
let x = signal<i32>(0);           // Mutable state
let y = computed<i32>(() => ...); // Derived value
effect(() => { ... });             // Side effect
batch(() => { ... });              // Batch updates
```

### Components
```jounce
component Name() { ... }                    // No props
component Name(props: { x: i32 }) { ... }   // With props
component Name() -> JSX { ... }             // Return type
```

### Server Functions
```jounce
@server
fn name() { ... }  // Runs on server, callable from client
```

### Imports
```jounce
use ./module::{Item};              // Import item
use ./module::{Item as Alias};     // Import with alias (v0.8.3+)
use ./module::*;                   // Import all
```

### Styling
```jounce
style Component { ... }            // Component styles
theme { --var: value; }             // Global theme
```

### Patterns
```jounce
match value { ... }                 // Pattern matching
if let Some(x) = opt { ... }        // if-let (v0.8.3+)
```

---

**Ready to build? Start with [Tutorial 1: Counter](#tutorial-1-counter-5-minutes)!**


Maintained by: **The Jounce Project**
