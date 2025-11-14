# Jounce Full-Stack Development Guide

**Version**: v0.8.3
**Last Updated**: November 7, 2025

> **Canonical Reference**: If this document conflicts with [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md), the spec wins. Current spec: v0.8.3 (2025-11-07).

---

## Overview

Jounce enables full-stack development in a single `.jnc` file. Using `@server` annotations, you mark functions that run only on the server, while client-side components run in the browser. The compiler automatically handles code splitting and RPC generation.

> **Technical Reference**: See [JOUNCE_SPEC.md § Execution Model](../../JOUNCE_SPEC.md#execution-rules) for complete details on server vs client execution.

---

## Current Status (v0.8.3)

### ✅ Implemented (v0.1.0+)
- **@server Functions**: Server-only function execution with `@server` annotation
- **Automatic RPC**: Client-to-server function calls (transparent HTTP POST)
- **Code Splitting**: Separate server.js and client.js bundles
- **Type-Safe Communication**: Types preserved across client/server boundary
- **Components**: Client-side reactive components with JSX
- **Styling**: Scoped CSS with `style` blocks
- **State Management**: Signals, computed values, effects
- **Module System**: Multi-file projects with imports

### 📋 Planned (v0.10.0+)
- **Streaming**: Server→Client streaming responses
- **WebSocket RPC**: Bidirectional real-time communication
- **Middleware**: Custom RPC interceptors
- **Caching**: Built-in RPC response caching

---

## Basic Architecture

### Simple Counter (Client-Only)

```jounce
component Counter() {
    let count = signal<i32>(0);

    return <div>
        <h1>Count: {count.value}</h1>
        <button onclick={() => count.value++}>+1</button>
    </div>;
}

style Counter {
    padding: 20px;
    text-align: center;

    button {
        font-size: 20px;
        padding: 10px 20px;
        cursor: pointer;
    }

    button:hover {
        background: #007bff;
        color: white;
    }
}
```

**Compile**:
```bash
jnc compile counter.jnc
cd dist && node server.js
# Open http://localhost:3000
```

---

## Server Functions (Available Now)

### Declaring Server Functions

Mark functions with `@server` to run them exclusively on the server:

```jounce
@server
fn get_users() -> Vec<User> {
    // Database query - runs only on server
    return database.query("SELECT * FROM users");
}

@server
fn create_user(name: string, email: string) -> Result<User, string> {
    // Validation and creation - server-only
    if !is_valid_email(email) {
        return Err("Invalid email");
    }

    return database.insert_user(name, email);
}
```

### Client Components Calling Server Functions

Components run in the browser and can call server functions via automatic RPC:

```jounce
component UserList() {
    let users = signal<Vec<User>>(vec![]);
    let loading = signal<bool>(true);

    onMount(() => {
        // Automatic RPC call to server
        let result = get_users();
        users.value = result;
        loading.value = false;
    });

    return <div>
        <h2>Users</h2>
        {loading.value ? (
            <p>Loading...</p>
        ) : (
            users.value.map((user) =>
                <div class="user-card">
                    <h3>{user.name}</h3>
                    <p>{user.email}</p>
                </div>
            )
        )}
    </div>;
}
```

### Automatic RPC Generation

When you call a `@server` function from client code, Jounce automatically:

1. Generates an RPC stub on the client
2. Serializes arguments to JSON
3. Makes HTTP POST request to `/rpc/<function_name>`
4. Deserializes the response
5. Returns the result

**Source**:
```jounce
@server
fn get_user(id: i32) -> Result<User, string> {
    return database.find_user(id);
}

component Profile(props: { userId: i32 }) {
    let user = get_user(props.userId);  // Looks like normal call!

    match user {
        Ok(u) => return <div>{u.name}</div>,
        Err(e) => return <div class="error">{e}</div>,
    }
}
```

**Generated client.js**:
```javascript
function get_user(id) {
    return fetch('/rpc/get_user', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id })
    }).then(r => r.json());
}
```

**Generated server.js**:
```javascript
app.post('/rpc/get_user', async (req, res) => {
    const { id } = req.body;
    const result = get_user(id);
    res.json(result);
});
```

---

## Best Practices

### 1. Separate Concerns

Keep client and server code organized even in one file:

```jounce
// ========== SERVER FUNCTIONS ==========

@server
fn get_posts() -> Vec<Post> {
    return database.query("SELECT * FROM posts");
}

@server
fn create_post(title: string, body: string) -> Result<Post, string> {
    return database.insert_post(title, body);
}

// ========== SHARED UTILITIES ==========

fn validate_post_title(title: string) -> bool {
    return title.len() > 0 && title.len() < 100;
}

// ========== CLIENT COMPONENTS ==========

component PostList() {
    let posts = get_posts();
    return <div>...</div>;
}
```

### 2. Type Safety

Use shared types for client/server communication:

```jounce
struct User {
    id: i32,
    name: string,
    email: string,
}

@server
fn get_user(id: i32) -> Result<User, string> {
    // Server implementation
    return database.find_user(id);
}

component UserProfile(props: { userId: i32 }) {
    let result = get_user(props.userId);

    match result {
        Ok(user) => return <div>{user.name}</div>,
        Err(e) => return <div class="error">{e}</div>,
    }
}
```

### 3. Error Handling

Always handle RPC errors gracefully:

```jounce
component DataLoader() {
    let data = signal<Result<Vec<Item>, string>>(Err("Not loaded"));
    let loading = signal<bool>(true);

    onMount(() => {
        loading.value = true;
        let result = fetch_data();  // @server function
        data.value = result;
        loading.value = false;
    });

    if loading.value {
        return <div>Loading...</div>;
    }

    match data.value {
        Ok(items) => return <div>{items.map((item) => <div>{item.name}</div>)}</div>,
        Err(error) => return <div class="error">{error}</div>,
    }
}
```

### 4. Validate on Both Sides

```jounce
// Shared validation
fn is_valid_email(email: string) -> bool {
    return email.contains("@");
}

// Client validates before sending
component SignupForm() {
    let email = signal<string>("");
    let error = signal<string>("");

    let handle_submit = () => {
        if !is_valid_email(email.value) {
            error.value = "Invalid email";
            return;
        }

        let result = register_user(email.value);
        match result {
            Ok(user) => console.log("Success"),
            Err(e) => error.value = e,
        }
    };

    return <form onsubmit={(e) => { e.preventDefault(); handle_submit(); }}>
        <input
            type="email"
            value={email.value}
            oninput={(e) => email.value = e.target.value}
        />
        <button type="submit">Sign Up</button>
        {error.value.len() > 0 ? <div class="error">{error.value}</div> : null}
    </form>;
}

// Server validates again (never trust client!)
@server
fn register_user(email: string) -> Result<User, string> {
    if !is_valid_email(email) {
        return Err("Invalid email");
    }
    // Create user...
    return database.create_user(email);
}
```

---

## CLI Commands

### Current (v0.8.3)

```bash
# Compile .jnc file
jnc compile main.jnc

# Initialize new project
jnc init my-app

# Package management
jnc pkg add jounce-http
jnc pkg install
jnc pkg publish
```

### Planned (v0.10.0+)

```bash
# Development server with hot reload
jnc dev main.jnc

# Production build
jnc build --release

# Test runner
jnc test
```

---

## Project Structure

### Recommended Layout

```
my-app/
├── main.jnc              # Entry point
├── components/
│   ├── header.jnc
│   ├── footer.jnc
│   └── layout.jnc
├── services/
│   └── api.jnc           # Server functions
├── types/
│   └── models.jnc        # Shared types
└── utils/
    └── validation.jnc    # Shared utilities
```

### Compile All Files

```bash
# Compile entry point
jnc compile main.jnc

# Or use a build script
for file in **/*.jnc; do
    jnc compile "$file"
done
```

---

## Full Example: Todo App with Server

```jounce
// ========== TYPES ==========

struct Todo {
    id: i32,
    text: string,
    completed: bool,
}

// ========== SERVER FUNCTIONS ==========

@server
fn get_todos() -> Vec<Todo> {
    return database.query("SELECT * FROM todos");
}

@server
fn create_todo(text: string) -> Result<Todo, string> {
    if text.len() == 0 {
        return Err("Todo text cannot be empty");
    }
    return database.insert_todo(text);
}

@server
fn toggle_todo(id: i32) -> Result<Todo, string> {
    return database.toggle_todo(id);
}

// ========== CLIENT COMPONENTS ==========

component TodoApp() {
    let todos = signal<Vec<Todo>>(vec![]);
    let input = signal<string>("");

    onMount(() => {
        todos.value = get_todos();
    });

    let add_todo = () => {
        let result = create_todo(input.value);
        match result {
            Ok(todo) => {
                todos.value = vec![...todos.value, todo];
                input.value = "";
            },
            Err(e) => console.log("Error: " + e),
        }
    };

    let toggle = (id: i32) => {
        let result = toggle_todo(id);
        match result {
            Ok(updated) => {
                todos.value = todos.value.map((t) => {
                    if t.id == id { return updated; }
                    return t;
                });
            },
            Err(e) => console.log("Error: " + e),
        }
    };

    return <div>
        <h1>My Todos</h1>
        <div>
            <input
                type="text"
                value={input.value}
                oninput={(e) => input.value = e.target.value}
                placeholder="What needs to be done?"
            />
            <button onclick={add_todo}>Add</button>
        </div>
        <ul>
            {todos.value.map((todo) =>
                <li>
                    <input
                        type="checkbox"
                        checked={todo.completed}
                        onchange={() => toggle(todo.id)}
                    />
                    <span>{todo.text}</span>
                </li>
            )}
        </ul>
    </div>;
}
```

---

## Troubleshooting

### "RPC call failed: Network error"

**Problem**: Server not running or wrong port.

**Solution**: Ensure server is running:
```bash
cd dist && node server.js
```

### "Type mismatch in RPC response"

**Problem**: Server returned different type than expected.

**Solution**: Ensure types match on both sides. Check server logs.

### "Function not found"

**Problem**: @server function not compiled or server not restarted.

**Solution**: Recompile and restart server:
```bash
jnc compile main.jnc
cd dist && node server.js
```

---

## What's Next?

- **Complete Tutorial**: See [LEARN_JOUNCE.md](./LEARN_JOUNCE.md) for practical examples
- **Technical Details**: See [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md) for execution model
- **Quick Start**: See [README.md](../../README.md) for getting started

---

**Version**: v0.8.3 "Enhanced Language Features"
**Status**: ✅ Production Ready (580/580 tests passing)
**@server/RPC**: ✅ Fully Implemented (v0.1.0+)

---

**Maintained by: The Jounce Project**
