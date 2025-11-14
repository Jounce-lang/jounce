# Jounce Syntax Limitations

**Version**: v0.8.3
**Last Updated**: November 7, 2025
**Status**: Authoritative Reference

---

## Purpose

This document catalogs all syntax limitations and unsupported features in Jounce. Referenced from [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md) and [LEARN_JOUNCE.md](LEARN_JOUNCE.md).

**When in doubt**: Check [JOUNCE_SPEC.md § Limitations](../../JOUNCE_SPEC.md#limitations) for authoritative behavior.

---

## Syntax Limitations

### 1. No JavaScript-Style For Loops

**NOT SUPPORTED**:
```jounce
// ❌ C-style for loops
for (let i = 0; i < 10; i++) {
    console.log(i);
}
```

**USE INSTEAD**:
```jounce
// ✅ Rust-style range iteration
for i in 0..10 {
    console.log(i);
}

// ✅ Array iteration
for item in items {
    console.log(item);
}
```

**Why**: Jounce uses Rust-style syntax for iteration.

---

### 2. No Async/Await Syntax

**NOT SUPPORTED**:
```jounce
// ❌ Prefix await (JavaScript-style)
let data = await fetch("https://api.example.com");

// ❌ Postfix await (Rust-style)
let data = fetch("https://api.example.com").await;
```

**USE INSTEAD**:
```jounce
// ✅ Server functions return Result<T, E>
@server
fn fetchData() -> Result<Data, string> {
    // Async handled internally by runtime
    return http.get("https://api.example.com/data");
}

// Client calls look synchronous via RPC
let result = fetchData();
match result {
    Ok(data) => console.log(data),
    Err(e) => console.log("Error: " + e),
}
```

**Why**: Jounce abstracts async via RPC. Server functions handle async internally; client gets synchronous-looking Result.

**See**: [JOUNCE_SPEC.md § RPC Contract](../../JOUNCE_SPEC.md#rpc-contract)

---

### 3. No Template String Literals

**NOT SUPPORTED**:
```jounce
// ❌ Template literals with ${}
let msg = `Hello ${name}!`;
let url = `https://api.example.com/${id}`;
```

**USE INSTEAD**:
```jounce
// ✅ String concatenation with +
let msg = "Hello " + name + "!";
let url = "https://api.example.com/" + id.to_string();
```

**Why**: Parser doesn't support template literal syntax.

---

### 4. No Default Function Parameters

**NOT SUPPORTED**:
```jounce
// ❌ Default parameter values
component Card(props: { title: string = "Default" }) {
    ...
}

fn greet(name: string = "World") {
    return "Hello " + name;
}
```

**USE INSTEAD**:
```jounce
// ✅ Manual defaults with || operator
component Card(props: { title: string }) {
    let title = props.title || "Default";
    ...
}

fn greet(name: string) -> string {
    let actualName = if name.len() > 0 { name } else { "World" };
    return "Hello " + actualName;
}
```

**Why**: Not yet implemented in parser.

---

### 5. No Implicit Returns

**NOT SUPPORTED**:
```jounce
// ❌ Expression-based return (Rust-style)
fn add(a: i32, b: i32) -> i32 {
    a + b  // Missing return keyword
}

fn getName() -> string {
    "Alice"  // Missing return
}
```

**USE INSTEAD**:
```jounce
// ✅ Explicit return statements
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn getName() -> string {
    return "Alice";
}
```

**Why**: Parser requires explicit `return` keyword for all return values.

---

### 6. No Destructuring in Function Parameters

**NOT SUPPORTED**:
```jounce
// ❌ Parameter destructuring
fn process({ name, age }: User) {
    console.log(name);
}

fn handleClick({ x, y }: Point) {
    console.log(x + y);
}
```

**USE INSTEAD**:
```jounce
// ✅ Manual property access
fn process(user: User) {
    let name = user.name;
    let age = user.age;
    console.log(name);
}

fn handleClick(point: Point) {
    let x = point.x;
    let y = point.y;
    console.log(x + y);
}
```

**Why**: Destructuring only supported in match expressions, not function params.

---

### 7. No Variadic Functions

**NOT SUPPORTED**:
```jounce
// ❌ Rest parameters / variadic args
fn sum(...numbers: i32[]) -> i32 {
    ...
}

fn log(level: string, ...messages: string[]) {
    ...
}
```

**USE INSTEAD**:
```jounce
// ✅ Explicit Vec parameter
fn sum(numbers: Vec<i32>) -> i32 {
    let total = 0;
    for n in numbers {
        total += n;
    }
    return total;
}

// Call with vec! macro
let result = sum(vec![1, 2, 3, 4, 5]);
```

**Why**: Not supported by type system.

---

### 8. No Optional Chaining

**NOT SUPPORTED**:
```jounce
// ❌ Optional chaining operator
let name = user?.profile?.name;
let length = data?.items?.length;
```

**USE INSTEAD**:
```jounce
// ✅ Explicit Option handling with if-let
let name = if let Some(u) = user {
    if let Some(p) = u.profile {
        p.name
    } else {
        "Unknown"
    }
} else {
    "Unknown"
};

// ✅ Or use match
let length = match data {
    Some(d) => match d.items {
        Some(items) => items.len(),
        None => 0,
    },
    None => 0,
};
```

**Why**: Jounce uses explicit Option<T> handling, not JavaScript-style optional chaining.

---

### 9. Limited Decorators/Annotations

**SUPPORTED**:
- `@server` - Server-only functions
- `@client` - Client-only functions (rarely needed)
- `@persist` - Persistent signals

**NOT SUPPORTED** (parsed but no codegen):
```jounce
// ❌ These are parsed but ignored
@memoize
fn expensive() { ... }

@deprecated
fn oldFunction() { ... }

@security("admin")
fn adminOnly() { ... }
```

**See**: [JOUNCE_SPEC.md § Supported Annotations](../../JOUNCE_SPEC.md#supported-annotations)

---

### 10. No Signal Reassignment

**NOT SUPPORTED**:
```jounce
// ❌ Reassigning signal variable
let count = signal<i32>(0);
count = signal<i32>(5);  // Type error!
```

**USE INSTEAD**:
```jounce
// ✅ Update signal value
let count = signal<i32>(0);
count.value = 5;  // Correct!
```

**Why**: Signals are immutable references; only their `.value` can be updated.

---

## Event Handler Conventions

### React-Style vs DOM-Style

**NOT SUPPORTED**:
```jounce
// ❌ React-style camelCase events
<button onClick={() => handleClick()}>
<input onChange={(e) => handleChange(e)}>
<form onSubmit={handleSubmit}>
```

**USE INSTEAD**:
```jounce
// ✅ Lowercase DOM-style events
<button onclick={() => handleClick()}>
<input onchange={(e) => handleChange(e)}>
<form onsubmit={handleSubmit}>
```

**All event handlers must be lowercase**:
- `onclick`, `ondblclick`
- `onchange`, `oninput`
- `onsubmit`, `onreset`
- `onkeydown`, `onkeyup`, `onkeypress`
- `onmousedown`, `onmouseup`, `onmousemove`, `onmouseover`, `onmouseout`
- `onfocus`, `onblur`

**Why**: Jounce follows DOM standard naming, not React conventions.

**See**: [JOUNCE_SPEC.md § JSX](../../JOUNCE_SPEC.md#8-jsx)

---

## Additional Unsupported Features

### No Nullish Coalescing
```jounce
// ❌ NOT SUPPORTED
let value = data ?? defaultValue;

// ✅ USE INSTEAD
let value = if let Some(d) = data { d } else { defaultValue };
```

### No Spread in Objects
```jounce
// ❌ NOT SUPPORTED (in object literals)
let obj = { ...baseObj, newField: value };

// ✅ USE INSTEAD
// Manually copy fields or use constructor
```

Note: Spread syntax (`...`) IS supported in `vec![]` macro:
```jounce
// ✅ SUPPORTED in arrays
let combined = vec![...array1, ...array2];
```

### No Arrow Functions in Type Positions
```jounce
// ❌ NOT SUPPORTED
type Handler = (event: Event) => void;

// ✅ USE INSTEAD
// Use fn() type
let handler: fn() = || { ... };
```

---

## Migration from JavaScript/React

Common gotchas when coming from JavaScript/React:

| JavaScript/React | Jounce | Why |
|------------------|--------|-----|
| `onClick={...}` | `onclick={...}` | DOM-style events |
| `await fetch()` | `@server fn + Result` | No await syntax |
| `` `Hello ${name}` `` | `"Hello " + name` | No templates |
| `for (;;)` | `for i in 0..10` | Rust-style loops |
| `a + b` (implicit) | `return a + b` | Explicit return |
| `user?.name` | `if let Some(u) = user { u.name }` | Explicit Option |
| `...rest` params | `params: Vec<T>` | No variadic |
| `const x = y ?? z` | `if let Some(y) = y { y } else { z }` | No nullish |

---

## Error Codes

### E_STY_001: Unsupported or Malformed Nested Style Rule

**Error Message**:
```
error[E_STY_001]: unsupported or malformed nested style rule
  help: Jounce currently supports one level of selector nesting inside `style <Component> { ... }`
  note: See docs/guides/SYNTAX_LIMITATIONS.md for current CSS syntax
```

**What it means**: You've attempted to use CSS nesting syntax that isn't currently supported.

**Supported CSS Nesting**:
```jounce
style Button {
    .container {
        padding: 20px;
    }

    button {
        padding: 10px 20px;
        background: #f0f0f0;

        // ✅ SUPPORTED: One level of nesting with &
        &:hover {
            background: #007bff;
        }

        &:active {
            background: #0056b3;
        }

        &.disabled {
            opacity: 0.5;
        }
    }
}
```

**Not Supported**:
```jounce
style Button {
    button {
        &:hover {
            // ❌ NOT SUPPORTED: Two levels deep
            &:active {
                background: red;
            }
        }
    }
}
```

**Current Limits**:
- **One level of nesting**: `button { &:hover { ... } }` ✅
- **No deep nesting**: `button { &:hover { &:active { ... } } }` ❌
- **Supported selectors**: `&:hover`, `&:active`, `&:focus`, `&.class`

---

### E_STY_002: Media Query Must Appear at Top Level

**Error Message**:
```
error[E_STY_002]: @media rules must appear at the top level of a style block
  help: Move @media outside of nested selectors
  note: @media rules cannot be nested inside element or class selectors
```

**What it means**: You've placed an `@media` query inside a nested selector, which is not supported.

**Supported Media Query Placement**:
```jounce
style Panel {
    .container {
        padding: 20px;
    }

    // ✅ SUPPORTED: @media at top level
    @media (max-width: 600px) {
        .container {
            padding: 10px;
        }
    }
}
```

**Not Supported**:
```jounce
style Panel {
    .container {
        padding: 20px;

        // ❌ NOT SUPPORTED: @media nested inside selector
        @media (max-width: 600px) {
            padding: 10px;
        }
    }
}
```

**Current Limits**:
- **Top-level only**: `@media` must be direct child of `style` block ✅
- **No nesting in selectors**: Cannot nest `@media` inside `.class` or `element` ❌
- **No nested media**: Cannot nest `@media` inside `@media` ❌

**Fix**:
Move `@media` to the top level and repeat selectors inside:
```jounce
style Panel {
    .container {
        padding: 20px;
    }

    @media (max-width: 600px) {
        .container {
            padding: 10px;
        }
    }
}
```

---

## See Also

- **[JOUNCE_SPEC.md](../../JOUNCE_SPEC.md)** - Complete language specification
- **[LEARN_JOUNCE.md § Common Mistakes](LEARN_JOUNCE.md#common-mistakes)** - Tutorial gotchas
- **[ERROR_MESSAGES.md](ERROR_MESSAGES.md)** - Understanding error codes

---

**Last Updated**: November 7, 2025 (v0.8.3)
