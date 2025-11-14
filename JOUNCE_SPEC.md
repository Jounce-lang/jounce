# Jounce Language Specification

> **This is the authoritative source for the Jounce language. If any other document disagrees with this file, this file wins.**

**Version**: v0.8.3
**Release Date**: November 7, 2025
**Test Suite**: 580/580 passing (100%)
**Status**: Production Ready

**For authoring guidance**, see [docs/guides/LEARN_JOUNCE.md](docs/guides/LEARN_JOUNCE.md).
**For common mistakes**, see [docs/JOUNCE_DO_AND_DONT.md](docs/JOUNCE_DO_AND_DONT.md).

---

## Document Precedence

**This specification is authoritative.** In case of conflict between this document and other Jounce documentation (README.md, LEARN_JOUNCE.md, user guides, API references), this specification supersedes all others.

See [docs/README.md](docs/README.md) for the complete documentation hierarchy.

---

## Language Model

Jounce is a **single-file, full-stack reactive language** that compiles to multiple targets:

```
main.jnc  →  Jounce Compiler  →  dist/
                                   ├── server.js      (Node.js server)
                                   ├── client.js      (Browser client)
                                   ├── styles.css     (Scoped styles)
                                   └── index.html     (Entry point)
```

### Compilation Model

- **Input**: Single `.jnc` file with mixed server/client code
- **Output**: JavaScript (Node.js server + browser client)
- **Code Splitting**: Annotation-driven via `@server`/`@client` annotations
- **RPC**: Automatic function stubs for client→server calls
- **Reactivity**: Fine-grained signals with dependency tracking
- **Styling**: Scoped CSS with automatic hash-based class names

### Execution Environment

- **Server**: Node.js (v16+)
- **Client**: Modern browsers (ES2020+)
- **Production Target**: JavaScript-only runtime for maximum compatibility
- **Experimental Target**: WebAssembly (--target wasm, not production-ready)
- **Hot Reload**: Development server with live updates

### Targets (v0.8.x)

**Primary Output**: JavaScript
- `server.js` - Node.js backend
- `client.js` - Browser frontend
- `styles.css` - Scoped component styles
- `index.html` - HTML entry point

**Experimental Output**: WebAssembly (--target wasm)
- May fail for some language constructs
- Best-effort in v0.8.x
- If JS succeeds but WASM fails, the build is still considered successful

### RPC Contract

Client→server calls compile to synchronous-looking code that returns `Result<T, E>`:

**URL Shape**: `POST /rpc/<function_name>`
**Request Body**: JSON with function arguments
**Response**: JSON `{ "ok": value }` or `{ "err": error }`
**Client Surface**: Appears synchronous, returns Result immediately
**Server Reality**: Async HTTP under the hood, but abstracted away

**Example**:
```jounce
// Server-side
@server
fn getUser(id: i32) -> Result<User, string> {
    return db.query("SELECT * FROM users WHERE id = ?", id);
}

// Client-side call (compiled to sync-looking RPC)
let result = getUser(123);  // Actually: POST /rpc/getUser with body {"id": 123}
match result {
    Ok(user) => console.log(user.name),
    Err(e) => console.log("Error: " + e),
}
```

---

## Grammar & Constructs

### 1. Components

Components are functions that return JSX:

```jounce
component Counter() {
    let count = signal<i32>(0);

    return <div>
        <p>Count: {count.value}</p>
        <button onclick={() => count.value++}>Increment</button>
    </div>;
}
```

**Syntax**:
- `component Name(props?) { ... }`
- Must return JSX expression
- Can have typed props: `component Card(props: { title: string }) { ... }`
- Can have return type: `component Card() -> JSX { ... }`

### 2. Reactivity Primitives

#### signal<T>(initialValue)
Mutable reactive state:

```jounce
let count = signal<i32>(0);
count.value = 5;  // Triggers reactivity
```

#### computed<T>(fn)
Derived reactive values:

```jounce
let doubled = computed<i32>(() => count.value * 2);
// Auto-updates when count changes
```

#### effect(fn)
Side effects with automatic dependency tracking:

```jounce
effect(() => {
    console.log("Count: " + count.value.to_string());
});
```

#### batch(fn)
Batch multiple updates:

```jounce
batch(() => {
    count.value = 10;
    name.value = "Alice";
    // Only one re-render
});
```

### 3. Styling

#### style Blocks
Component-scoped styles:

```jounce
style Counter {
    padding: 20px;
    background: white;

    // Element selectors
    button {
        color: blue;
    }

    // Class selectors
    .title {
        font-size: 24px;
    }

    // Child combinator
    > .item {
        margin: 10px;
    }

    // Pseudo-classes
    &:hover {
        background: #f0f0f0;
    }

    // Pseudo-elements
    &::before {
        content: "→";
    }
}
```

**Features**:
- Automatic scoping (hash-based class names)
- Nested selectors (max depth: 3)
- Child combinators (`>`)
- Pseudo-classes (`:hover`, `:active`, `:focus`)
- Pseudo-elements (`::before`, `::after`)
- Modifier selectors (`&.active`)
- Media queries (responsive design)
- CSS animations with `@keyframes` (automatically scoped)

**Nested Selectors**:
- Support for multi-level nesting up to a maximum depth of 3
- Depth 0: Top level inside `style Component { ... }`
- Depth 1-3: Nested selectors (`.child`, `.child .grandchild`, etc.)
- `@media` and `@keyframes` do NOT count toward nesting depth

**Media Queries**:
- `@media (...) { ... }` is supported inside a `style <Component> { ... }` block.
- Media rules must be at the top level of the style block (not inside selectors).
- Nested media rules (`@media` inside `@media`) are not supported.

**Keyframe Animations**:
- `@keyframes` are supported inside `style <Component> { ... }` blocks
- Animation names are automatically scoped to prevent conflicts (e.g., `jnc-Component_hash-animationName`)
- Keyframe selectors: `from`, `to`, or `<number>%` (e.g., `0%`, `50%`, `100%`)
- Animation properties automatically rewritten to use scoped names

#### theme Blocks
Global theming:

```jounce
theme {
    --primary-color: #007bff;
    --background: white;
}
```

### 4. Module System

#### Imports

```jounce
// Relative imports
use ./components::{Button, Card};
use ../utils::{formatDate};

// Import aliasing (v0.8.3+)
use ./widgets::{Button as WidgetButton};
use ./types::{User as UserType};

// Wildcard imports
use ./helpers::*;
```

#### Exports

```jounce
// Explicit pub keyword (v0.8.3+)
pub fn publicFunction() -> i32 {
    return 42;
}

pub struct PublicData {
    value: i32,
}

pub const PUBLIC_CONST: i32 = 100;

// Private (no pub keyword)
fn helper() -> i32 {
    return 10;
}
```

**Visibility Rules**:
- If NO items have `pub`, all items are exported (backward compatible)
- If ANY item has `pub`, ONLY `pub` items are exported
- Applies to: functions, structs, enums, constants

### 5. Server Functions

Functions annotated with `@server` run on the server:

```jounce
@server
fn fetchUserData(userId: i32) -> Result<User, string> {
    // Server-side code (database, auth, etc.)
    return database.getUser(userId);
}

// Call from client (automatic RPC)
component UserProfile() {
    let user = fetchUserData(123);  // RPC call
    return <div>{user.name}</div>;
}
```

### 6. Pattern Matching

#### match Expressions

```jounce
let result = match value {
    Ok(data) => data,
    Err(e) => default,
};
```

#### if-let Expressions (v0.8.3+)

```jounce
// Pattern match with Option
if let Some(value) = optionValue {
    console.log(value);
} else {
    console.log("No value");
}

// Pattern match with Result
if let Ok(data) = fetchResult {
    processData(data);
}
```

### 7. Type System

**Primitive Types**:
- `i32`, `i64`, `f32`, `f64`
- `string`, `bool`
- `()` (unit type)

**Complex Types**:
- `Vec<T>`, `HashMap<K, V>`, `HashSet<T>`
- `Option<T>`, `Result<T, E>`
- Structs, Enums

**Type Annotations**:
```jounce
let count: i32 = 0;
let name: string = "Alice";
let items: Vec<i32> = vec![1, 2, 3];
```

**Type Inference**:
```jounce
let count = 0;           // Inferred as i32
let name = "Alice";      // Inferred as string
let items = vec![1, 2];  // Inferred as Vec<i32>
```

**Arrays / Indexing**:
- Array indices must be integer-typed (i32) at compile time.
- Using non-integer types (float, string, boolean) as indexes yields error E430.
- Range loop variables (`for i in 0..len { ... }`) are automatically considered integer-typed.
- Cast expressions to i32 if needed (when cast methods are available).

```jounce
let items = vec![1, 2, 3];

// ✅ Valid indexing
let x = items[0];              // Literal integer
for i in 0..items.len() {      // Range loop variable (integer-typed)
    let item = items[i];
}

// ❌ Invalid indexing
let key = "0";                 // String
let x = items[key];            // Error: Array index must be an integer [E430]
```

### 8. JSX

Full JSX support with:
- Elements: `<div>`, `<button>`, `<input>`, etc.
- Attributes: `class="..."`, `id="..."`, `onclick={handler}`
- Children: Text, expressions, nested elements
- Fragments: `<>...</>`
- Self-closing: `<img src="..." />`
- Expressions: `{count.value}`, `{items.map(...)}`

**Event Handler Convention**:
- Use lowercase DOM-style event names: `onclick`, `oninput`, `onchange`, `onsubmit`
- NOT React-style camelCase: ❌ `onClick`, ❌ `onChange`

```jounce
<div class="container">
    <h1>{title}</h1>
    {items.map((item) => <li>{item}</li>)}
    <button onclick={() => handleClick()}>Click</button>
</div>
```

### 9. Standard Library

**Available Modules**:
- `std::option` - Option<T>
- `std::result` - Result<T, E>
- `std::vec` - Vec<T> with iterators
- `std::hashmap` - HashMap<K, V>
- `std::string` - String operations
- `std::json` - JSON parsing/serialization
- `std::time` - DateTime utilities
- `std::fs` - File system operations **(server-only)**
- `std::http` - HTTP client **(server-only)**

**Note**: Modules marked "server-only" are only available in `@server` functions.

---

## Execution Rules

### Server vs Client Code

#### What Runs on Server
- Functions annotated with `@server`
- Database queries
- Authentication logic
- File system operations
- Environment variables

#### What Runs on Client
- Components
- Event handlers
- Reactive primitives (signal, computed, effect)
- DOM manipulation
- Browser APIs

#### Automatic RPC
When client calls `@server` function:
1. Compiler generates stub on client
2. Client makes HTTP POST to `/rpc/<function_name>`
3. Server executes function
4. Result serialized and returned to client

**Example**:
```jounce
// Server-side
@server
fn getUser(id: i32) -> Result<User, string> {
    return db.query("SELECT * FROM users WHERE id = ?", id);
}

// Client-side (automatic RPC)
component UserCard() {
    let user = getUser(123);  // HTTP POST to /rpc/getUser
    return <div>{user.name}</div>;
}
```

### Reactivity Rules

#### Dependency Tracking
- Reading `.value` inside `computed()` or `effect()` registers dependency
- Writing `.value` triggers all dependent computations/effects
- Synchronous execution (no async delays)

#### Update Batching
- Multiple signal updates in same tick batched automatically
- Use `batch(() => {...})` for explicit batching
- Only one re-render per batch

#### Memory Management
- Effects automatically cleaned up when component unmounts
- No manual subscription management needed
- No memory leaks from forgotten cleanup

---

## CLI Surface

### Available Commands

#### `jnc compile <file.jnc>`
Compile Jounce file to per-app output directory:
```bash
jnc compile examples/apps/my-app/main.jnc
# Creates: dist/my-app/server.js, dist/my-app/client.js,
#          dist/my-app/styles.css, dist/my-app/index.html
```

Each app is compiled to `dist/<app-folder>/` to prevent conflicts when building multiple apps.

**Options**:
- `--release` - Production build with minification
- `--target wasm` - Compile to WebAssembly (experimental)
- `--output <dir>` - Custom base output directory (default: `dist`)

#### `jnc dev <file.jnc>`
Development server with hot reload:
```bash
jnc dev main.jnc
# Starts server on http://localhost:3000
# Auto-recompiles on file changes
```

#### `jnc init <name>`
Create new project with template:
```bash
jnc init my-app
# Creates: my-app/main.jnc with starter code
```

**Templates**:
- `counter` - Minimal counter app (default)
- `todo` - Todo list with CRUD
- `form` - Form with validation
- `dashboard` - Dashboard with charts
- `blank` - Empty starter

#### `jnc pkg <command>`
Package manager commands:

```bash
# Authentication
jnc pkg register
jnc pkg login

# Package discovery
jnc pkg search "ui components"
jnc pkg info jounce-router

# Dependency management
jnc pkg add jounce-router
jnc pkg remove jounce-router
jnc pkg install
jnc pkg update

# Publishing
jnc pkg publish
jnc pkg version patch|minor|major

# Owner management
jnc pkg owner add <username>
jnc pkg owner remove <username>
jnc pkg owner list
```

#### `jnc test`
Run test suite (not yet implemented):
```bash
jnc test
```

#### `jnc build`
Alias for `jnc compile --release`:
```bash
jnc build main.jnc
```

---

## Limitations

### Current Restrictions

#### 1. No JavaScript-style for-loops
```jounce
// ❌ NOT SUPPORTED
for (let i = 0; i < 10; i++) {
    console.log(i);
}

// ✅ USE INSTEAD
for i in 0..10 {
    console.log(i);
}
```

#### 2. No async/await syntax
```jounce
// ❌ NOT SUPPORTED - No prefix await
let data = await fetch("...");

// ❌ NOT SUPPORTED - No postfix .await (Rust-style)
let data = fetch("...").await;

// ✅ USE INSTEAD - Server functions return Result<T, E>
// Async operations abstracted via RPC (see "RPC Contract" section)
@server
fn fetchData() -> Result<Data, string> {
    // Server handles async internally
    return http.get("https://api.example.com/data");
}

// Client calls look synchronous but use RPC
let result = fetchData();  // Returns Result immediately
```

#### 3. No default props
```jounce
// ❌ NOT SUPPORTED
component Card(props: { title: string = "Default" }) { ... }

// ✅ USE INSTEAD
component Card(props: { title: string }) {
    let title = props.title || "Default";
    ...
}
```

#### 4. No async client components
```jounce
// ❌ NOT SUPPORTED
component UserCard() {
    let user = await fetchUser();  // Can't await in component
    ...
}

// ✅ USE INSTEAD
@server
fn fetchUser() -> User { ... }

component UserCard() {
    let user = fetchUser();  // Synchronous RPC
    ...
}
```

#### 5. No decorators (except @server/@client)
```jounce
// ❌ NOT SUPPORTED
@memoize
fn expensive() { ... }

// ✅ ONLY SUPPORTED
@server
fn serverFn() { ... }

@client
fn clientFn() { ... }
```

#### 6. No string templates with ${}
```jounce
// ❌ NOT SUPPORTED
let msg = `Hello ${name}`;

// ✅ USE INSTEAD
let msg = "Hello " + name;
```

#### 7. No destructuring in function params
```jounce
// ❌ NOT SUPPORTED
fn process({ name, age }: User) { ... }

// ✅ USE INSTEAD
fn process(user: User) {
    let name = user.name;
    let age = user.age;
    ...
}
```

#### 8. No variadic functions
```jounce
// ❌ NOT SUPPORTED
fn sum(...numbers: i32[]) { ... }

// ✅ USE INSTEAD
fn sum(numbers: Vec<i32>) { ... }
```

#### 9. No implicit returns (must use return)
```jounce
// ❌ NOT SUPPORTED
fn add(a: i32, b: i32) -> i32 {
    a + b  // Missing return
}

// ✅ USE INSTEAD
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

#### 10. No optional chaining
```jounce
// ❌ NOT SUPPORTED
let name = user?.profile?.name;

// ✅ USE INSTEAD
let name = if let Some(u) = user {
    if let Some(p) = u.profile {
        p.name
    } else { "Unknown" }
} else { "Unknown" };
```

---

## Supported Annotations

Annotations control code placement (server vs client) and signal behavior:

| Annotation | Status | Applies To | Behavior | Notes |
|------------|--------|------------|----------|-------|
| `@server` | ✅ Implemented | Functions | Server-only execution, generates RPC stub on client | Primary annotation for server logic |
| `@client` | ✅ Implemented | Functions | Force client-side emission | Rarely needed (components default to client) |
| `@persist` | ✅ Implemented | signal() calls | Persist signal to localStorage | Used in LEARN_JOUNCE.md examples |
| `@memoize` | 📋 Planned (v0.10.0) | Functions | Cache function results | Parser accepts but no codegen |
| `@deprecated` | 📋 Planned (v0.10.0) | Any | Emit deprecation warnings | Parser accepts but no codegen |
| `@security` | 📋 Planned (v0.11.0) | Functions | Security policy enforcement | Parser accepts but no codegen |

**Usage Examples**:
```jounce
// @server - Server-only function with RPC
@server
fn fetchUserData(id: i32) -> Result<User, string> {
    return database.getUser(id);
}

// @client - Force client-side (usually not needed)
@client
fn clientOnlyHelper() -> string {
    return window.location.href;
}

// @persist - Persistent signal
let theme = @persist signal<string>("dark");
// Syncs with localStorage["theme"]
```

**Note**: All annotations are parsed. Only `@server`, `@client`, and `@persist` currently generate code.

---

## Implemented vs Planned

| Feature | Status | Version | Notes |
|---------|--------|---------|-------|
| **Core Language** |
| Lexer & Parser | ✅ Implemented | v0.1.0 | Full syntax support |
| Type Checker | ✅ Implemented | v0.1.0 | Constraint-based type inference |
| Code Generator | ✅ Implemented | v0.1.0 | JavaScript output |
| WASM Target | ⚠️ Experimental | v0.1.0 | Parses but incomplete |
| **Reactivity** |
| signal<T>() | ✅ Implemented | v0.4.0 | Production ready |
| computed<T>() | ✅ Implemented | v0.4.0 | Production ready |
| effect() | ✅ Implemented | v0.4.0 | Production ready |
| batch() | ✅ Implemented | v0.4.0 | Production ready |
| **Components** |
| Component syntax | ✅ Implemented | v0.2.0 | Full JSX support |
| Props | ✅ Implemented | v0.2.0 | Typed props |
| Return types | ✅ Implemented | v0.2.0 | JSX return type |
| Default props | ❌ Not Planned | - | Use `||` operator |
| **Styling** |
| style blocks | ✅ Implemented | v0.2.0 | Scoped CSS |
| theme blocks | ✅ Implemented | v0.2.0 | Global themes |
| Nested selectors | ✅ Implemented | v0.8.3 | Advanced CSS |
| Child combinators | ✅ Implemented | v0.8.3 | `>` selector |
| Pseudo-classes | ✅ Implemented | v0.8.3 | `:hover`, etc. |
| CSS-in-JS | ❌ Not Planned | - | Use style blocks |
| **Module System** |
| Relative imports | ✅ Implemented | v0.2.0 | `use ./module` |
| Wildcard imports | ✅ Implemented | v0.2.0 | `use ./module::*` |
| Import aliasing | ✅ Implemented | v0.8.3 | `Item as Alias` |
| Explicit pub | ✅ Implemented | v0.8.3 | Visibility control |
| Re-exports | 📋 Planned | v0.9.0 | `pub use` |
| **Pattern Matching** |
| match expressions | ✅ Implemented | v0.2.0 | Full support |
| if-let | ✅ Implemented | v0.8.3 | Result/Option |
| while-let | 📋 Planned | v0.9.0 | Loop with pattern |
| Destructuring | ⚠️ Partial | v0.2.0 | Enums only |
| **Server Functions** |
| @server annotation | ✅ Implemented | v0.1.0 | RPC generation |
| Automatic stubs | ✅ Implemented | v0.1.0 | Client calls |
| Type safety | ✅ Implemented | v0.1.0 | Full types |
| Streaming | 📋 Planned | v0.10.0 | Server → Client |
| **Package Manager** |
| Registry server | ✅ Implemented | v0.8.3 | Production ready |
| JWT auth | ✅ Implemented | v0.8.3 | Secure tokens |
| Package publish | ✅ Implemented | v0.8.3 | `jnc pkg publish` |
| Package install | ✅ Implemented | v0.8.3 | `jnc pkg add` |
| Dependency resolution | ✅ Implemented | v0.8.3 | Semver support |
| Lock files | ✅ Implemented | v0.8.3 | jounce.lock |
| **CLI Tools** |
| jnc compile | ✅ Implemented | v0.1.0 | Full compiler |
| jnc dev | ✅ Implemented | v0.3.0 | HMR server |
| jnc init | ✅ Implemented | v0.8.2 | Project scaffolding |
| jnc pkg | ✅ Implemented | v0.8.3 | Package manager |
| jnc test | 📋 Planned | v0.10.0 | Test runner |
| jnc fmt | 📋 Planned | v0.10.0 | Code formatter |
| jnc doc | 📋 Planned | v0.10.0 | Doc generator |
| jnc deploy | 📋 Planned | v0.10.0 | One-click deploy |
| **Developer Tools** |
| VSCode extension | ✅ Implemented | v0.1.0 | Basic LSP |
| Syntax highlighting | ✅ Implemented | v0.1.0 | Full support |
| Auto-completion | ⚠️ Partial | v0.1.0 | Keywords only |
| Error diagnostics | ✅ Implemented | v0.8.1 | 20+ error codes |
| Debugger | 📋 Planned | v0.10.0 | Source maps |
| **Safety Features** |
| Type checker | ✅ Implemented | v0.1.0 | Compile-time |
| Borrow checker | ⚠️ Partial | v0.1.0 | Basic checks |
| Static analyzer | ✅ Implemented | v0.8.2 | Warnings |
| Runtime guards | ✅ Implemented | v0.8.2 | Dev mode |
| Frozen signals | ✅ Implemented | v0.8.2 | Object.freeze |
| **Standard Library** |
| Vec<T> | ✅ Implemented | v0.1.0 | Full iterators |
| HashMap<K,V> | ✅ Implemented | v0.2.0 | Complete |
| HashSet<T> | ✅ Implemented | v0.2.0 | Complete |
| Option<T> | ✅ Implemented | v0.1.0 | Complete |
| Result<T,E> | ✅ Implemented | v0.1.0 | Complete |
| JSON | ✅ Implemented | v0.3.0 | Parse/stringify |
| DateTime | ✅ Implemented | v0.3.0 | Full support |
| HTTP client | ✅ Implemented | v0.3.0 | Fetch wrapper |
| File I/O | ✅ Implemented | v0.3.0 | Server-side |
| **Annotations** |
| @server | ✅ Implemented | v0.1.0 | Server functions |
| @client | ✅ Implemented | v0.1.0 | Client functions |
| @memoize | 📋 Planned | v0.10.0 | Caching |
| @deprecated | 📋 Planned | v0.10.0 | Warnings |
| @security | 📋 Planned | v0.11.0 | Security checks |

**Legend**:
- ✅ **Implemented**: Production ready, fully tested
- ⚠️ **Partial**: Works but incomplete
- 📋 **Planned**: On roadmap with target version
- ❌ **Not Planned**: Won't be implemented

---

## Version History

| Version | Date | Name | Highlights |
|---------|------|------|------------|
| v0.8.3 | Nov 7, 2025 | Enhanced Language Features | Import aliasing, advanced styles, pub keyword, if-let, package registry |
| v0.8.2 | Nov 5, 2025 | Runtime Safety Nets | Frozen signals, dev-mode checks, 3-layer protection |
| v0.8.1 | Oct 31, 2025 | Developer Experience | CSS utilities, error messages, templates |
| v0.8.0 | Oct 24, 2025 | Complete Ecosystem | 35 packages, 850+ tests |
| v0.4.0 | Oct 24, 2025 | Reactivity | signal, computed, effect, batch |
| v0.3.0 | Oct 24, 2025 | Production Ready | 638 tests, compilation cache |
| v0.2.0 | Oct 22, 2025 | Language Core Complete | Module system, operators, JSX |
| v0.1.0 | Oct 20, 2025 | Initial Release | Full compiler pipeline |

---

**Last Updated**: November 7, 2025
**Maintained By**: Jounce Core Team
**License**: MIT


Maintained by: **The Jounce Project**
