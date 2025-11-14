# Jounce Do & Don't (v0.8.x)

**Maintained by: The Jounce Project**

This document provides clear guidance on common mistakes and best practices when writing Jounce code.

For authoritative language rules, see [JOUNCE_SPEC.md](../JOUNCE_SPEC.md).

---

## Do

- ✅ Do use `signal(...)`
- ✅ Do use lowercase event handlers (`onclick`, `oninput`, `onchange`)
- ✅ Do return JSX explicitly from components (`return <div>...</div>;`)
- ✅ Do keep `@media` at top-level in `style` blocks
- ✅ Do prefer JS output as the canonical build artifact in v0.8.x
- ✅ Do use Rust-style loops: `for i in 0..items.length() { ... }`
- ✅ Do use `signal<T>(initialValue)` with explicit types
- ✅ Do access signal values with `.value` property
- ✅ Do use `computed<T>(...)` for derived values
- ✅ Do use `effect(...)` for side effects
- ✅ Do use `@server` annotation for server-only functions
- ✅ Do use `pub` keyword to export functions/components from modules

---

## Don't

- ❌ Don't use `createSignal(...)` (React-style API removed in v0.8.x)
- ❌ Don't use React-style event names (`onClick`, `onChange`, `onInput`)
- ❌ Don't use JavaScript `for (...)` loops (use Rust-style `for i in 0..n`)
- ❌ Don't `await` inside JSX (async operations must be outside JSX expressions)
- ❌ Don't assume 100% WASM emission — it is best-effort right now
- ❌ Don't add author credits to LLMs or vendors; use "The Jounce Project"
- ❌ Don't nest `@media` queries inside selectors (will emit E_STY_002 error)
- ❌ Don't forget to return JSX from components (implicit returns not supported)
- ❌ Don't use `createComputed(...)` (use `computed<T>(...)` instead)
- ❌ Don't access signals without `.value` (will emit E030 error)
- ❌ Don't use non-integer array indices (will emit E430 error)

---

## Common Patterns

### ✅ Correct: Signal Usage
```jounce
let count = signal<i32>(0);
count.value = count.value + 1;
console.log(count.value);
```

### ❌ Incorrect: Missing .value
```jounce
let count = signal<i32>(0);
count = count + 1;  // Error: Can't reassign signal
console.log(count);  // Error: E030 - Missing .value
```

---

### ✅ Correct: Lowercase Event Handlers
```jounce
<button onclick={() => handleClick()}>Click Me</button>
<input oninput={(e) => handleInput(e)} />
```

### ❌ Incorrect: React-style Events
```jounce
<button onClick={() => handleClick()}>Click Me</button>  // Won't work
<input onInput={(e) => handleInput(e)} />  // Won't work
```

---

### ✅ Correct: Rust-style Loops
```jounce
for i in 0..items.length() {
    console.log(items[i]);
}
```

### ❌ Incorrect: JavaScript Loops
```jounce
for (let i = 0; i < items.length(); i++) {  // Parse error
    console.log(items[i]);
}
```

---

### ✅ Correct: Media Queries at Top-Level
```jounce
style Component {
    padding: 20px;

    @media (max-width: 600px) {
        padding: 10px;
    }
}
```

### ❌ Incorrect: Nested Media Queries
```jounce
style Component {
    .container {
        padding: 20px;

        @media (max-width: 600px) {  // Error: E_STY_002
            padding: 10px;
        }
    }
}
```

---

### ✅ Correct: Async Outside JSX
```jounce
component UserProfile() {
    let user = fetchUser();  // RPC call outside JSX

    return <div>
        <h1>{user.name}</h1>
    </div>;
}
```

### ❌ Incorrect: Await Inside JSX
```jounce
component UserProfile() {
    return <div>
        <h1>{await fetchUser().name}</h1>  // Error: await in JSX
    </div>;
}
```

---

## Migration from Legacy APIs

If you have code using old React-like APIs, here's how to migrate:

| Old (v0.7.x)              | New (v0.8.x)              |
|---------------------------|---------------------------|
| `createSignal(...)`       | `signal<T>(...)`          |
| `createComputed(...)`     | `computed<T>(...)`        |
| `onClick={...}`           | `onclick={...}`           |
| `onChange={...}`          | `onchange={...}`          |
| `for (let i=0; i<n; i++)` | `for i in 0..n { ... }`   |

---

## Error Code Quick Reference

Common errors you might encounter:

- **E030**: Accessing signal without `.value`
- **E430**: Array index must be an integer
- **E_STY_001**: Unsupported or malformed nested style rule
- **E_STY_002**: Media query must appear at top level

See [docs/guides/ERROR_MESSAGES.md](guides/ERROR_MESSAGES.md) for complete error reference.

---

**Maintained by: The Jounce Project**
