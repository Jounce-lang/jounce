# Jounce Syntax Limitations

**Version**: v0.8.3
**Last Updated**: November 4, 2025

## Overview

Jounce uses Rust-inspired syntax rather than JavaScript syntax for certain features. This document describes what Jounce does **NOT** support and what syntax to use instead.

---

## 1. For Loops - Rust Style Only

### ❌ NOT Supported: JavaScript-style `for (let x in array)`

```javascript
// ❌ JavaScript syntax - NOT SUPPORTED
for (let x in array) {
    console.log(x);
}
```

### ✅ Supported: Rust-style `for x in array`

```jounce
// ✅ Jounce syntax - iterate over array
for item in items {
    console.log(item);
}

// ✅ Jounce syntax - iterate over range (exclusive)
for i in 0..5 {
    console.log(i);  // 0, 1, 2, 3, 4
}

// ✅ Jounce syntax - iterate over range (inclusive)
for i in 0..=5 {
    console.log(i);  // 0, 1, 2, 3, 4, 5
}
```

**Generated JavaScript**:
```javascript
// for item in items
for (const item of items) {
    console.log(item);
}

// for i in 0..5
for (let i = 0; i < 5; i++) {
    console.log(i);
}

// for i in 0..=5
for (let i = 0; i <= 5; i++) {
    console.log(i);
}
```

---

## 2. Await - Prefix Only

### ❌ NOT Supported: Rust-style `.await` postfix

```rust
// ❌ Rust syntax - NOT SUPPORTED
let response = fetch("https://api.example.com").await;
```

### ✅ Supported: JavaScript-style `await expr` (prefix)

```jounce
// ✅ Jounce syntax - prefix await
async fn fetchData() {
    let response = await fetch("https://api.example.com/data");
    let json = await response.json();
    return json;
}
```

**Generated JavaScript**:
```javascript
async function fetchData() {
    let response = await fetch("https://api.example.com/data");
    let json = await response.json();
    return json;
}
```

**Note**: Jounce uses JavaScript-style prefix `await` rather than Rust-style postfix `.await` for better interoperability with JavaScript async/await patterns.

---

## 3. Module Imports - Local Files Only

### ❌ NOT Supported: Package imports like `use jounce::*`

```rust
// ❌ Package imports - NOT SUPPORTED
use jounce::forms::{Input, Button};
use std::collections::HashMap;
```

### ✅ Supported: Local file imports with `./` prefix

```jounce
// ✅ Import specific items from local file
use ./components::{Button, Card};

// ✅ Import all exports from local file
use ./utils::*;

// ✅ Import from parent directory
use ../lib/helpers::{formatText};

// ✅ Import from nested path
use ./modules/api/client::{fetchData};
```

**How it works**:
1. Jounce loads the referenced `.jnc` file
2. Extracts the requested exports (functions, components, structs, etc.)
3. Inlines them into the current module's compiled output
4. No runtime JavaScript imports are generated (compile-time resolution)

**Example**:

`utils.jnc`:
```jounce
fn formatText(text: String) -> String {
    return text.trim();
}

fn calculateTotal(items: Vec<i32>) -> i32 {
    let mut total = 0;
    for item in items {
        total = total + item;
    }
    return total;
}
```

`main.jnc`:
```jounce
use ./utils::{formatText, calculateTotal};

component App() {
    let items = signal([1, 2, 3, 4, 5]);
    let text = formatText("  Hello World  ");
    let total = calculateTotal(items.value);

    <div>
        <p>{text}</p>
        <p>Total: {total}</p>
    </div>
}
```

---

## 4. Database ORM - Not Available as User Library

### ❌ NOT Supported: `use jounce::db::*`

```jounce
// ❌ Database ORM - NOT SUPPORTED
use jounce::db::*;
use jounce::db::{Database, Query, Model};
```

**Why it looks real but isn't**:
- Files exist at `src/stdlib/db.rs` in the compiler codebase
- BUT these are **Rust compiler internals**, not user-facing libraries!
- They're used by the compiler for type checking and code analysis
- No runtime database library is generated

### ✅ Workaround: Use Node.js Libraries in Server Functions

```jounce
// Define your data types
struct User {
    id: i64,
    email: String,
    name: String,
}

// Use Node.js libraries inside server functions
server fn getUsers() -> Result<Vec<User>, String> {
    // Inside the generated server.js, you can manually add:
    // const { Pool } = require('pg');
    // const pool = new Pool({ connectionString: process.env.DATABASE_URL });
    // const result = await pool.query('SELECT * FROM users');
    // return { variant: 'Ok', data: result.rows };

    // For now, return mock data or implement after compilation
    return Ok([]);
}

// Client calls server function via auto-generated RPC
component App() {
    let users = signal([]);

    async fn loadUsers() {
        let result = await getUsers();
        users.value = result.unwrap_or([]);
    }

    onMount(() => {
        loadUsers();
    });

    <div>
        <h1>Users: {users.value.len()}</h1>
    </div>
}
```

**Status**: ❌ **NOT IMPLEMENTED** - Use Node.js database libraries (pg, mysql2, mongodb) directly in generated server.js

---

## 5. Auth Module - Not Available as User Library

### ❌ NOT Supported: `use jounce::auth::*`

```jounce
// ❌ Auth module - NOT SUPPORTED
use jounce::auth::*;
use jounce::auth::{hash_password, verify_password, generate_token};
```

**Same issue as database module**:
- Files exist at `src/stdlib/auth.rs` but are compiler internals only
- No runtime authentication library is available

### ✅ Workaround: Use Node.js Auth Libraries in Server Functions

```jounce
struct LoginRequest {
    email: String,
    password: String,
}

struct LoginResponse {
    token: String,
    user: User,
}

server fn login(req: LoginRequest) -> Result<LoginResponse, String> {
    // Inside generated server.js, manually add:
    // const bcrypt = require('bcrypt');
    // const jwt = require('jsonwebtoken');
    //
    // // Verify password
    // const user = await db.query('SELECT * FROM users WHERE email = $1', [req.email]);
    // const valid = await bcrypt.compare(req.password, user.password_hash);
    // if (!valid) return { variant: 'Err', data: 'Invalid credentials' };
    //
    // // Generate JWT
    // const token = jwt.sign({ userId: user.id }, process.env.JWT_SECRET, { expiresIn: '7d' });
    // return { variant: 'Ok', data: { token, user } };

    // For now, return mock response
    return Err("Not implemented - add bcrypt/jwt to server.js");
}
```

**Status**: ❌ **NOT IMPLEMENTED** - Use Node.js libraries (bcrypt, jsonwebtoken, passport) in generated server.js

---

## 6. Annotations - Limited Support

### ❌ NOT Supported: Security Annotations

```jounce
// ❌ These annotations PARSE but DON'T GENERATE CODE
@auth(role="admin")
server fn deleteUser(id: i64) -> Result<(), String> {
    // No middleware is generated!
    // No token validation happens!
    // No role checking happens!
    return Ok(());
}

@validate(schema="UserSchema")
@rate_limit(requests=10, window="1m")
server fn createUser(data: User) -> Result<User, String> {
    // These annotations are parsed into AST but ignored
    return Ok(data);
}
```

**What happens**:
- ✅ Parser successfully extracts annotations from code
- ✅ Annotations are stored in the AST
- ❌ No middleware is generated in server.js
- ❌ No security checks are performed
- ❌ Feature is planned but not implemented (Phase 17)

### ✅ Supported: `@persist` Annotation

```jounce
// ✅ This annotation WORKS!
component App() {
    @persist("localStorage")
    let theme = signal("dark");

    // Automatically saves to localStorage on changes
    // Automatically loads from localStorage on mount
}
```

**Generated JavaScript**:
```javascript
let theme = signal("dark");

// Load from localStorage
const __stored_theme = localStorage.getItem('theme');
if (__stored_theme !== null) {
    theme.value = JSON.parse(__stored_theme);
}

// Save to localStorage on changes
effect(() => {
    localStorage.setItem('theme', JSON.stringify(theme.value));
});
```

**Status**:
- ❌ **@auth, @validate, @rate_limit** - Parse but don't work
- ✅ **@persist("localStorage")** - Fully functional
- ⏳ **Security annotations** - Planned for Phase 17

---

## 7. Type Checker Issue - Result Methods in If Statements

### ❌ NOT Supported: `result.is_ok()` in if conditions

```jounce
server fn getUserData() -> Result<User, String> {
    // ...
}

component App() {
    async fn loadUser() {
        let result = await getUserData();

        // ❌ TYPE ERROR: "If condition must be bool or integer, got 'unknown'"
        if (result.is_ok()) {
            let user = result.unwrap();
            console.log(user);
        }
    }
}
```

**Root cause**:
- Result type exists and works ✅
- `.is_ok()` and `.is_err()` methods exist and return boolean ✅
- BUT type checker doesn't recognize the return type ❌
- This is a known bug in the type checking system

### ✅ Workarounds

**Option 1: Use `unwrap_or` (recommended)**
```jounce
let result = await getUserData();
let user = result.unwrap_or(defaultUser);
// Works - no if statement needed
```

**Option 2: Use try-catch with `unwrap`**
```jounce
try {
    let user = result.unwrap();
    console.log(user);
} catch (e) {
    console.log("Error:", e);
}
```

**Option 3: Check variant directly (advanced)**
```jounce
// Access the internal variant field
if (result.variant == "Ok") {
    let user = result.data;
    console.log(user);
}
```

**Status**: ❌ **TYPE CHECKER BUG** - Known issue, workarounds available

---

## Complete Example

Here's a complete example demonstrating the supported syntax features:

```jounce
// Local file imports (compile-time resolution)
use ./utils::{formatText};
use ./components::{Button, Card};

component App() {
    let items = signal([1, 2, 3]);
    let data = signal("");

    // Rust-style for-in loops
    fn processItems() {
        for item in items.value {
            console.log(item);
        }

        for i in 0..items.value.len() {
            console.log(i);
        }
    }

    // Prefix await (JavaScript style)
    async fn fetchData() {
        let response = await fetch("https://api.example.com/data");
        let json = await response.json();
        data.value = formatText(json.message);
    }

    <div>
        <h1>Jounce Syntax Demo</h1>
        <Button onClick={processItems}>Process Items</Button>
        <Button onClick={fetchData}>Fetch Data</Button>
        <Card title="Results">
            <p>Data: {data.value}</p>
        </Card>
    </div>
}
```

---

## Summary

| Feature | ❌ NOT Supported | ✅ Supported | Workaround |
|---------|-----------------|-------------|------------|
| **For loops** | `for (let x in array)` | `for x in array` | N/A |
| **Await** | `expr.await` | `await expr` | N/A |
| **Imports** | `use jounce::*` | `use ./local-file` | N/A |
| **Database** | `use jounce::db::*` | N/A | Use Node.js pg/mysql2 in server fn |
| **Auth** | `use jounce::auth::*` | N/A | Use bcrypt/jwt in server fn |
| **Annotations** | `@auth`, `@validate`, `@rate_limit` | `@persist("localStorage")` | Manual middleware in server.js |
| **Result checks** | `if (result.is_ok())` | `result.unwrap_or(default)` | Use unwrap_or or variant check |

---

## Detailed Explanation of Each Limitation

### 1. For Loops - `for (let x in array)` vs `for x in array`

**Why we need this documented**:
- JavaScript developers instinctively use `for (let x in array)` syntax
- This causes immediate parse errors with cryptic messages
- LLMs trained on JavaScript will generate wrong syntax

**What is impacted**:
- All array iteration code
- Range-based loops (0..10)
- Any loop that doesn't use `.map()` or `.filter()`

**How Jounce works now**:
```jounce
// Current syntax (Rust-style)
for item in items {
    console.log(item);
}

for i in 0..5 {
    console.log(i);
}
```

**Generated JavaScript**:
```javascript
for (const item of items) {
    console.log(item);
}

for (let i = 0; i < 5; i++) {
    console.log(i);
}
```

**Why it works this way**:
- Jounce is Rust-inspired and uses Rust's cleaner loop syntax
- Generates JavaScript `for...of` loops (not `for...in` which iterates keys)
- Range syntax `0..5` is more readable than `for (let i = 0; i < 5; i++)`

**If JavaScript-style were supported**:
- Would need to parse `for (let x in array)` and `for (let x of array)`
- Adds complexity distinguishing between `in` (keys) and `of` (values)
- Current Rust syntax is clearer and less error-prone

---

### 2. Await - `expr.await` vs `await expr`

**Why we need this documented**:
- Rust developers expect postfix `.await` syntax
- This is a deliberate design choice that differs from Rust

**What is impacted**:
- All async/await code
- Server function calls
- Any Promise-based operations

**How Jounce works now**:
```jounce
// Current syntax (JavaScript-style prefix)
async fn fetchData() {
    let response = await fetch(url);
    let json = await response.json();
    return json;
}
```

**Why it works this way**:
- Jounce generates JavaScript code that runs in Node.js/browser
- JavaScript async/await uses prefix `await` keyword
- No transpilation needed - direct 1:1 mapping to JS

**If Rust-style were supported**:
```jounce
// Would need to support Rust syntax
let response = fetch(url).await;  // ❌ Not supported
let json = response.json().await;  // ❌ Not supported
```

**Trade-off**:
- Chose JavaScript familiarity over Rust consistency
- Web developers already know `await expr`
- Generates cleaner, more readable JavaScript output

---

### 3. Module Imports - `use jounce::*` vs `use ./local-file`

**Why we need this documented**:
- Rust/Go/Python developers expect package imports (`use jounce::db`)
- This is the #1 source of confusion for new users
- Files exist in codebase making it look like packages work

**What is impacted**:
- All module imports
- Cannot import from "packages" or "namespaces"
- Must use relative file paths only

**How Jounce works now**:
```jounce
// Current: Local file imports only
use ./components::{Button, Card};
use ./utils::{formatText, calculateTotal};
use ../lib/api::{fetchUsers};
```

**What happens at compile time**:
1. Jounce reads the local `.jnc` file
2. Extracts exported functions/components/types
3. **Inlines them directly into compiled output**
4. No JavaScript `import` statements generated
5. Everything is bundled into single `client.js`/`server.js`

**If package imports were supported**:
```jounce
// Would work with package registry
use jounce::db::{Database, Query};
use jounce::auth::{hash_password, verify};
use std::collections::{HashMap, Vec};
```

**What would change**:
1. Package registry server (like npm/crates.io)
2. Package resolver in compiler
3. Dependency management (`jounce.toml` or similar)
4. Versioning and semver support
5. Actual runtime imports instead of inlining

**Why it doesn't work now**:
- No package registry exists yet
- No package resolution infrastructure
- Compile-time inlining is simpler for MVP
- Planned for future releases

---

### 4. Database ORM - `use jounce::db::*`

**Why we need this documented**:
- Files `src/stdlib/db.rs` exist in codebase
- LLMs see these files and think database module is available
- **CRITICAL**: These are compiler internals, not user libraries!

**What is impacted**:
- All database operations
- Type-safe query building
- Database migrations
- Connection pooling
- Transaction management

**How Jounce works now (WITHOUT database ORM)**:
```jounce
struct User {
    id: i64,
    email: String,
    name: String,
}

// Server function with manual Node.js library usage
server fn getUsers() -> Result<Vec<User>, String> {
    // Generated function body is empty
    // Developer must manually edit dist/server.js:
    return Ok([]);  // Returns empty array
}
```

**Manual workaround after compilation**:
Edit `dist/server.js` directly:
```javascript
// Inside the generated server function
export async function getUsers() {
    const { Pool } = require('pg');
    const pool = new Pool({
        connectionString: process.env.DATABASE_URL
    });

    try {
        const result = await pool.query('SELECT * FROM users');
        return { variant: 'Ok', data: result.rows };
    } catch (error) {
        return { variant: 'Err', data: error.message };
    }
}
```

**How it WOULD work with database ORM**:
```jounce
use jounce::db::{Database, Query};

struct User {
    id: i64,
    email: String,
    name: String,
}

server fn getUsers() -> Result<Vec<User>, String> {
    let db = Database::connect(env!("DATABASE_URL"))?;

    // Type-safe query builder
    let users = db.query::<User>()
        .select_all()
        .where_("active", true)
        .order_by("created_at", "DESC")
        .limit(100)
        .execute()
        .await?;

    return Ok(users);
}
```

**What would be generated**:
```javascript
import { Database } from './runtime/db.js';

export async function getUsers() {
    const db = await Database.connect(process.env.DATABASE_URL);
    const users = await db.query('User')
        .selectAll()
        .where('active', true)
        .orderBy('created_at', 'DESC')
        .limit(100)
        .execute();
    return { variant: 'Ok', data: users };
}
```

**Why it doesn't exist**:
- `src/stdlib/db.rs` is Rust code for **compiler type checking only**
- Used to validate types and relationships in source code
- No runtime JavaScript library is generated
- Would require building an entire ORM (like Prisma, TypeORM)
- Planned for Phase 17+ but significant work required

---

### 5. Auth Module - `use jounce::auth::*`

**Why we need this documented**:
- Files `src/stdlib/auth.rs` exist (compiler internals)
- Authentication is critical for production apps
- No built-in solution leads to manual implementation

**What is impacted**:
- User authentication (login/signup)
- Password hashing (bcrypt)
- JWT token generation and validation
- Session management
- Role-based access control
- OAuth/social login

**How Jounce works now (WITHOUT auth module)**:
```jounce
struct LoginRequest {
    email: String,
    password: String,
}

server fn login(req: LoginRequest) -> Result<String, String> {
    // Empty function - must manually implement
    return Err("Not implemented");
}
```

**Manual workaround after compilation**:
Edit `dist/server.js`:
```javascript
export async function login({ email, password }) {
    const bcrypt = require('bcrypt');
    const jwt = require('jsonwebtoken');
    const { Pool } = require('pg');
    const pool = new Pool({ connectionString: process.env.DATABASE_URL });

    // Query user
    const result = await pool.query(
        'SELECT * FROM users WHERE email = $1',
        [email]
    );

    if (result.rows.length === 0) {
        return { variant: 'Err', data: 'User not found' };
    }

    const user = result.rows[0];

    // Verify password
    const validPassword = await bcrypt.compare(password, user.password_hash);
    if (!validPassword) {
        return { variant: 'Err', data: 'Invalid password' };
    }

    // Generate JWT
    const token = jwt.sign(
        { userId: user.id, email: user.email },
        process.env.JWT_SECRET,
        { expiresIn: '7d' }
    );

    return { variant: 'Ok', data: token };
}
```

**How it WOULD work with auth module**:
```jounce
use jounce::auth::{hash_password, verify_password, generate_token};
use jounce::db::{Database};

struct User {
    id: i64,
    email: String,
    password_hash: String,
}

server fn signup(email: String, password: String) -> Result<User, String> {
    let db = Database::connect(env!("DATABASE_URL"))?;

    // Built-in password hashing
    let hash = hash_password(password)?;

    // Create user with automatic validation
    let user = db.create::<User>({
        email: email,
        password_hash: hash,
    }).await?;

    return Ok(user);
}

server fn login(email: String, password: String) -> Result<String, String> {
    let db = Database::connect(env!("DATABASE_URL"))?;

    // Find user
    let user = db.query::<User>()
        .where_("email", email)
        .first()
        .await?;

    // Built-in password verification
    if !verify_password(password, user.password_hash)? {
        return Err("Invalid password");
    }

    // Built-in JWT generation
    let token = generate_token(user.id, "7d")?;

    return Ok(token);
}
```

**What would be generated**:
```javascript
import { hashPassword, verifyPassword, generateToken } from './runtime/auth.js';
import { Database } from './runtime/db.js';

export async function signup(email, password) {
    const db = await Database.connect(process.env.DATABASE_URL);
    const hash = await hashPassword(password);
    const user = await db.create('User', { email, password_hash: hash });
    return { variant: 'Ok', data: user };
}

export async function login(email, password) {
    const db = await Database.connect(process.env.DATABASE_URL);
    const user = await db.query('User').where('email', email).first();

    const valid = await verifyPassword(password, user.password_hash);
    if (!valid) {
        return { variant: 'Err', data: 'Invalid password' };
    }

    const token = await generateToken(user.id, '7d');
    return { variant: 'Ok', data: token };
}
```

**Why it doesn't exist**:
- Same issue as database - `src/stdlib/auth.rs` is compiler-only code
- Building secure auth library requires extensive work
- Must handle bcrypt, JWT, sessions, OAuth, etc.
- Planned but requires database module first

---

### 6. Annotations - `@auth`, `@validate`, `@rate_limit`

**Why we need this documented**:
- Annotations parse successfully (look like they work!)
- But they generate **zero code** in output
- This is extremely confusing - silent failures

**What is impacted**:
- Security middleware (@auth)
- Input validation (@validate)
- Rate limiting (@rate_limit)
- Any custom middleware/decorators

**How Jounce works now (WITHOUT annotation middleware)**:
```jounce
@auth(role="admin")
server fn deleteUser(id: i64) -> Result<(), String> {
    // ❌ NO SECURITY CHECKS HAPPEN!
    // Anyone can call this function
    // No token validation
    // No role checking
    return Ok(());
}
```

**What gets generated**:
```javascript
// Annotation is completely ignored
export async function deleteUser(id) {
    return { variant: 'Ok', data: undefined };
}

// Server RPC handler - NO MIDDLEWARE
server.rpc('deleteUser', async (params) => {
    const [id] = params;
    return await module.exports.deleteUser(id);
});
```

**Manual workaround**:
Edit `dist/server.js` to add middleware:
```javascript
// Add middleware function
function authMiddleware(role) {
    return async (req, res, next) => {
        const token = req.headers.authorization?.replace('Bearer ', '');

        if (!token) {
            return res.status(401).json({ error: 'No token provided' });
        }

        try {
            const decoded = jwt.verify(token, process.env.JWT_SECRET);

            if (role && decoded.role !== role) {
                return res.status(403).json({ error: 'Insufficient permissions' });
            }

            req.user = decoded;
            next();
        } catch (error) {
            return res.status(401).json({ error: 'Invalid token' });
        }
    };
}

// Manually apply middleware to route
server.rpc('deleteUser', authMiddleware('admin'), async (params) => {
    const [id] = params;
    return await module.exports.deleteUser(id);
});
```

**How it WOULD work with annotation middleware**:
```jounce
@auth(role="admin")
@rate_limit(requests=10, window="1m")
@validate(schema="UserIdSchema")
server fn deleteUser(id: i64) -> Result<(), String> {
    // Annotations automatically generate middleware
    // 1. Validate JWT token
    // 2. Check user has "admin" role
    // 3. Rate limit to 10 requests per minute
    // 4. Validate id against schema
    // Only then does function execute

    return Ok(());
}
```

**What would be generated**:
```javascript
// Runtime security library auto-imported
import { authCheck, rateLimit, validate } from './runtime/security.js';

// Middleware automatically applied
server.rpc('deleteUser',
    authCheck({ role: 'admin' }),
    rateLimit({ requests: 10, window: '1m' }),
    validate({ schema: 'UserIdSchema' }),
    async (params) => {
        const [id] = params;
        return await module.exports.deleteUser(id);
    }
);
```

**Why it doesn't work**:
- Parser extracts annotations into AST ✅
- But `js_emitter.rs` doesn't generate middleware code ❌
- Requires:
  1. `runtime/security.js` library with auth/validation functions
  2. Middleware generation in emitter
  3. Integration with RPC handler system
- Planned for Phase 17 but significant work

**Exception - @persist DOES work**:
```jounce
@persist("localStorage")
let theme = signal("dark");
```

This generates:
```javascript
let theme = signal("dark");

const __stored_theme = localStorage.getItem('theme');
if (__stored_theme !== null) {
    theme.value = JSON.parse(__stored_theme);
}

effect(() => {
    localStorage.setItem('theme', JSON.stringify(theme.value));
});
```

Only `@persist` generates code because it was implemented in Phase 16.

---

### 7. Type Checker Bug - `result.is_ok()` in if statements

**Why we need this documented**:
- Result<T, E> type works perfectly everywhere else
- But `.is_ok()` in if statements causes cryptic type errors
- This is a BUG, not a design choice

**What is impacted**:
- All Result type conditional logic
- Error handling patterns
- Branching based on success/failure

**How Jounce works now (WITH the bug)**:
```jounce
server fn getUserData() -> Result<User, String> {
    // Returns Result<User, String>
}

component App() {
    async fn loadUser() {
        let result = await getUserData();

        // ❌ TYPE ERROR: "If condition must be bool or integer, got 'unknown'"
        if (result.is_ok()) {
            let user = result.unwrap();
            console.log(user);
        }
    }
}
```

**What happens at runtime**:
1. `result` is a JavaScript object: `{ variant: 'Ok', data: user }`
2. `result.is_ok()` is defined in runtime and returns `true` or `false`
3. BUT type checker in `src/type_checker.rs` doesn't recognize the return type
4. Type checker thinks `.is_ok()` returns `unknown` type
5. If condition rejects `unknown` type (only accepts bool/int)

**Current workarounds**:

**Option 1: Use unwrap_or (recommended)**
```jounce
let result = await getUserData();
let user = result.unwrap_or(defaultUser);
// No if statement needed
```

**Option 2: Try-catch**
```jounce
try {
    let user = result.unwrap();  // Throws if Err
    console.log(user);
} catch (e) {
    console.log("Error:", e);
}
```

**Option 3: Check variant directly**
```jounce
// Bypass type checker by checking internal field
if (result.variant == "Ok") {
    let user = result.data;
    console.log(user);
}
```

**How it SHOULD work (after bug fix)**:
```jounce
let result = await getUserData();

// Should work - .is_ok() returns boolean
if (result.is_ok()) {
    let user = result.unwrap();
    console.log(user);
} else {
    console.log("Error:", result.unwrap_err());
}

// Alternative - .is_err() should also work
if (result.is_err()) {
    console.log("Failed");
    return;
}
```

**What needs to be fixed**:
In `src/type_checker.rs`:
1. Recognize Result<T, E> type
2. Add `.is_ok()` method signature that returns `bool`
3. Add `.is_err()` method signature that returns `bool`
4. Type check method calls on Result instances
5. Return correct boolean type for if condition checking

**Why this is a bug**:
- Result type is fully implemented ✅
- `.is_ok()` and `.is_err()` exist in runtime ✅
- They return correct boolean values ✅
- Only the type checker has incomplete knowledge ❌

This will be fixed in a future session - it's a type system issue, not a language design issue.

---

## Summary: Why This Table Matters

**For LLMs**:
- Prevents generating broken code
- Shows exact workarounds to use
- Explains what "looks like it should work" but doesn't

**For Developers**:
- Saves hours of debugging mysterious errors
- Shows production-ready workarounds
- Sets correct expectations about current capabilities

**For Jounce Project**:
- Documents technical debt clearly
- Shows roadmap (what's coming)
- Explains architectural decisions

---

## Why These Limitations?

### Syntax Choices

1. **For loops**: Rust-style syntax is cleaner and more intuitive for iterating over collections and ranges.

2. **Await**: While Jounce is Rust-inspired, JavaScript's prefix `await` is already familiar to web developers and works seamlessly with async/await patterns.

3. **Imports**: Jounce currently uses compile-time module resolution for local files only. Package management and registry integration are planned for future releases.

### Not Yet Implemented

4. **Database ORM**: The compiler has internal database types for analysis, but no runtime library is available yet. Use Node.js database libraries directly in server functions.

5. **Auth Module**: Similar to database - compiler has internal types but no runtime library. Use Node.js auth libraries (bcrypt, jsonwebtoken) in server functions.

6. **Security Annotations**: The parser recognizes `@auth`, `@validate`, etc., but middleware generation is not implemented (Phase 17 feature). Only `@persist` annotation works currently.

7. **Result Type Checking**: This is a known bug in the type checker where `.is_ok()` and `.is_err()` return types aren't properly recognized in if conditions. Use `unwrap_or()` or check `result.variant` directly as workarounds.

---

**Last Updated**: November 4, 2025
**Jounce Version**: v0.8.3
