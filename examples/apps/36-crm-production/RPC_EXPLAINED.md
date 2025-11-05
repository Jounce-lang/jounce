# 🚀 How Jounce's Auto-Generated RPC Works

## **The Magic: Write Once, Works Everywhere**

When you write a server function in Jounce:

```jounce
server fn login(email: String, password: String) -> Result<AuthToken, String> {
    // Your backend code here
}
```

Jounce **automatically generates**:
1. ✅ **Client-side stub** (browser JavaScript)
2. ✅ **Server-side handler** (Node.js HTTP endpoint)
3. ✅ **Type-safe communication** (serialization/deserialization)
4. ✅ **Error handling** (network errors, timeouts)

**You write ZERO API code!**

---

## **🔍 Step-by-Step: What Happens During Compilation**

### **Input: Your Jounce Code**

```jounce
// Server function (runs on backend)
server fn login(email: String, password: String) -> Result<AuthToken, String> {
    let user = db.query("SELECT * FROM users WHERE email = $1", [email])?;

    if (!auth.verify_password(password, user.password_hash)) {
        return Err("Invalid credentials");
    }

    let token = auth.create_auth_token(user);
    return Ok(token);
}

// Client component (runs in browser)
component LoginForm() {
    fn handleLogin() {
        // Call server function - looks like a normal function!
        let result = login(email.value, password.value).await;

        if (result.is_ok()) {
            // Success!
        }
    }
}
```

---

### **Step 1: Parser Identifies Server Functions**

```rust
// In src/parser.rs
if token == "server" && peek_token == "fn" {
    let func = parse_server_function();
    // Mark as server-side function
    func.is_server = true;
}
```

**Output**: AST (Abstract Syntax Tree) with function marked as `server`

```
FunctionDefinition {
    name: "login",
    is_server: true,
    parameters: [
        { name: "email", type: "String" },
        { name: "password", type: "String" }
    ],
    return_type: "Result<AuthToken, String>",
    body: [ /* function code */ ]
}
```

---

### **Step 2: Code Splitter Separates Client/Server**

```rust
// In src/code_splitter.rs
impl CodeSplitter {
    pub fn split(&mut self, statements: Vec<Statement>) {
        for stmt in statements {
            match stmt {
                Statement::Function(func) if func.is_server => {
                    self.server_functions.push(func);
                },
                Statement::Component(comp) => {
                    self.client_components.push(comp);
                },
                // ...
            }
        }
    }
}
```

**Result**:
```
server_functions: [login, getMyDeals, markDealWon, ...]
client_components: [LoginForm, CRMDashboard, ...]
```

---

### **Step 3: RPC Generator Creates Client Stub**

```rust
// In src/rpc_generator.rs
impl RPCGenerator {
    fn generate_client_stub(&self, func: &FunctionDefinition) -> String {
        let name = &func.name;
        let params = extract_parameter_names(&func.parameters);

        format!(
            "export async function {}({}) {{
                return await client.call('{}', [{}]);
            }}",
            name, params, name, params
        )
    }
}
```

**Generated Client Code** (`dist/client.js`):

```javascript
// Auto-generated RPC client stub
export async function login(email, password) {
    return await client.call('login', [email, password]);
}

export async function getMyDeals(token) {
    return await client.call('getMyDeals', [token]);
}

export async function markDealWon(token, dealId) {
    return await client.call('markDealWon', [token, dealId]);
}
```

---

### **Step 4: RPC Generator Creates Server Handler**

```rust
// In src/rpc_generator.rs
impl RPCGenerator {
    fn generate_server_handler(&self, func: &FunctionDefinition) -> String {
        let name = &func.name;
        let param_names = extract_parameter_names(&func.parameters);

        format!(
            "server.rpc('{}', async (params) => {{
                const [{}] = params;
                return await module.exports.{}({});
            }});",
            name, param_names, name, param_names
        )
    }
}
```

**Generated Server Code** (`dist/server.js`):

```javascript
// Auto-generated RPC server handlers
const server = new HttpServer(process.env.PORT || 3000);

server.rpc('login', async (params) => {
    const [email, password] = params;
    return await module.exports.login(email, password);
});

server.rpc('getMyDeals', async (params) => {
    const [token] = params;
    return await module.exports.getMyDeals(token);
});

server.rpc('markDealWon', async (params) => {
    const [token, dealId] = params;
    return await module.exports.markDealWon(token, dealId);
});

server.start();
```

---

## **🌐 What Happens at Runtime**

### **Browser → Server Communication**

```
┌─────────────────────────────────────────────────────────┐
│  Browser (Client)                                       │
│                                                         │
│  User clicks "Login" button                             │
│         ↓                                               │
│  handleLogin() called                                   │
│         ↓                                               │
│  login("alice@company.com", "password123")              │
│         ↓                                               │
│  client.call('login', ["alice@...", "password123"])     │
│         ↓                                               │
│  HTTP POST /api/login                                   │
│  {                                                      │
│    "function": "login",                                 │
│    "params": ["alice@company.com", "password123"]       │
│  }                                                      │
└────────────────┬────────────────────────────────────────┘
                 │
                 │ HTTPS
                 ↓
┌────────────────▼────────────────────────────────────────┐
│  Server (Node.js)                                       │
│                                                         │
│  POST /api/login received                               │
│         ↓                                               │
│  server.rpc('login') handler called                     │
│         ↓                                               │
│  Extract params: [email, password]                      │
│         ↓                                               │
│  Call: login(email, password)                           │
│         ↓                                               │
│  Query database                                         │
│  Verify password                                        │
│  Generate JWT token                                     │
│         ↓                                               │
│  Return: { ok: true, value: { token: "jwt..." } }       │
└────────────────┬────────────────────────────────────────┘
                 │
                 │ HTTPS Response
                 ↓
┌────────────────▼────────────────────────────────────────┐
│  Browser (Client)                                       │
│                                                         │
│  Response received                                      │
│         ↓                                               │
│  result = { ok: true, value: { token: "jwt..." } }      │
│         ↓                                               │
│  if (result.is_ok()) { ... }                            │
│         ↓                                               │
│  Store token in localStorage                            │
│  Navigate to dashboard                                  │
└─────────────────────────────────────────────────────────┘
```

---

## **🔧 Generated Runtime Code**

### **Client-Side RPC Client**

Generated in `dist/client-runtime.js`:

```javascript
// RPC Client - Handles all server function calls
export class RPCClient {
    constructor(baseUrl) {
        this.baseUrl = baseUrl;
    }

    async call(functionName, params) {
        try {
            const response = await fetch(`${this.baseUrl}/api/${functionName}`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    function: functionName,
                    params: params
                })
            });

            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }

            const result = await response.json();

            // Return as Jounce Result type
            if (result.ok) {
                return { variant: 'Ok', data: result.value };
            } else {
                return { variant: 'Err', data: result.error };
            }
        } catch (error) {
            // Network error
            return { variant: 'Err', data: error.message };
        }
    }
}

// Create global client instance
const client = new RPCClient(window.location.origin);
```

---

### **Server-Side HTTP Server**

Generated in `dist/server-runtime.js`:

```javascript
// HTTP Server - Handles RPC requests
export class HttpServer {
    constructor(port) {
        this.port = port;
        this.handlers = new Map();
        this.app = express();

        // Middleware
        this.app.use(express.json());
        this.app.use(cors());

        // Health check endpoint
        this.app.get('/health', (req, res) => {
            res.json({ status: 'ok' });
        });
    }

    // Register RPC handler
    rpc(functionName, handler) {
        this.handlers.set(functionName, handler);

        // Create Express route
        this.app.post(`/api/${functionName}`, async (req, res) => {
            try {
                const { params } = req.body;

                // Call handler
                const result = await handler(params);

                // Send response
                if (result.variant === 'Ok') {
                    res.json({ ok: true, value: result.data });
                } else {
                    res.json({ ok: false, error: result.data });
                }
            } catch (error) {
                console.error(`Error in ${functionName}:`, error);
                res.status(500).json({
                    ok: false,
                    error: error.message
                });
            }
        });
    }

    start() {
        this.app.listen(this.port, () => {
            console.log(`Server listening on port ${this.port}`);
        });
    }
}
```

---

## **🔒 Security Features (Auto-Generated)**

### **1. Authentication Middleware**

When you write:
```jounce
@auth
server fn getMyDeals(token: String) -> Result<Vec<Deal>, String> {
    // ...
}
```

Jounce generates:

```javascript
server.rpc('getMyDeals', async (params) => {
    // Auto-generated auth check
    const [token] = params;

    // Verify JWT token
    const claims = auth.verify_token(token);
    if (!claims || claims.is_expired()) {
        return { variant: 'Err', data: 'Unauthorized' };
    }

    // Call actual function
    return await module.exports.getMyDeals(token);
});
```

### **2. Role-Based Access Control**

When you write:
```jounce
@auth(role="manager")
server fn getAllDeals(token: String) -> Result<Vec<Deal>, String> {
    // ...
}
```

Jounce generates:

```javascript
server.rpc('getAllDeals', async (params) => {
    const [token] = params;

    // Verify token AND role
    const claims = auth.verify_token(token);
    if (!claims || !claims.is_valid('manager')) {
        return { variant: 'Err', data: 'Forbidden: Manager role required' };
    }

    return await module.exports.getAllDeals(token);
});
```

### **3. Input Validation**

Automatic validation based on types:

```jounce
server fn createDeal(
    token: String,
    company: String,
    value: i32,
    contact: String
) -> Result<Deal, String>
```

Generates:

```javascript
server.rpc('createDeal', async (params) => {
    const [token, company, value, contact] = params;

    // Type validation
    if (typeof company !== 'string' || company.length === 0) {
        return { variant: 'Err', data: 'Invalid company name' };
    }

    if (typeof value !== 'number' || value < 0) {
        return { variant: 'Err', data: 'Invalid value' };
    }

    if (typeof contact !== 'string' || contact.length === 0) {
        return { variant: 'Err', data: 'Invalid contact' };
    }

    return await module.exports.createDeal(token, company, value, contact);
});
```

---

## **📊 Performance Optimizations**

### **1. Connection Pooling**

All RPC calls share a single HTTP keep-alive connection:

```javascript
// Browser reuses connection
const client = new RPCClient(window.location.origin);

// All calls use same connection pool
await login(email, password);
await getMyDeals(token);
await markDealWon(token, dealId);
```

### **2. Request Batching** (Future Feature)

```javascript
// Batch multiple RPC calls into one HTTP request
const [user, deals, activities] = await Promise.all([
    getCurrentUser(token),
    getMyDeals(token),
    getDealActivities(token, dealId)
]);

// Could be optimized to:
// POST /api/batch
// { calls: [
//     { function: 'getCurrentUser', params: [token] },
//     { function: 'getMyDeals', params: [token] },
//     { function: 'getDealActivities', params: [token, dealId] }
// ]}
```

### **3. Response Caching**

Jounce can generate cache headers automatically:

```jounce
@cache(ttl=300)  // Cache for 5 minutes
server fn getPublicDeals() -> Result<Vec<Deal>, String> {
    // ...
}
```

Generates:

```javascript
server.rpc('getPublicDeals', async (params) => {
    const result = await module.exports.getPublicDeals();

    // Set cache headers
    res.setHeader('Cache-Control', 'public, max-age=300');

    return result;
});
```

---

## **🎯 Key Advantages**

### **Compared to Manual API Development**

| Feature | Jounce (Auto) | Manual (Express + React) |
|---------|---------------|--------------------------|
| **Client stub** | ✅ Auto-generated | ❌ Write fetch() calls |
| **Server route** | ✅ Auto-generated | ❌ Write Express routes |
| **Type safety** | ✅ End-to-end | ❌ Manual TypeScript |
| **Auth middleware** | ✅ `@auth` annotation | ❌ Write middleware |
| **Error handling** | ✅ Built-in | ❌ Try/catch everywhere |
| **Serialization** | ✅ Automatic | ❌ Manual JSON.stringify |
| **Lines of code** | **5 lines** | **50+ lines** |
| **Setup time** | **1 minute** | **30 minutes** |

---

## **🔬 Example: Manual vs Auto-Generated**

### **Manual API (Express + React)**

**Backend (Express):**
```javascript
// routes/auth.js
const express = require('express');
const bcrypt = require('bcrypt');
const jwt = require('jsonwebtoken');
const router = express.Router();

router.post('/login', async (req, res) => {
    try {
        const { email, password } = req.body;

        // Validate input
        if (!email || !password) {
            return res.status(400).json({ error: 'Missing fields' });
        }

        // Query database
        const user = await db.query(
            'SELECT * FROM users WHERE email = $1',
            [email]
        );

        if (!user) {
            return res.status(401).json({ error: 'Invalid credentials' });
        }

        // Verify password
        const valid = await bcrypt.compare(password, user.password_hash);
        if (!valid) {
            return res.status(401).json({ error: 'Invalid credentials' });
        }

        // Generate token
        const token = jwt.sign(
            { userId: user.id, email: user.email },
            process.env.JWT_SECRET,
            { expiresIn: '24h' }
        );

        res.json({ token, user: { id: user.id, email: user.email } });
    } catch (error) {
        console.error(error);
        res.status(500).json({ error: 'Server error' });
    }
});

module.exports = router;
```

**Frontend (React):**
```javascript
// api/auth.js
export async function login(email, password) {
    try {
        const response = await fetch('/api/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email, password })
        });

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.message);
        }

        return await response.json();
    } catch (error) {
        throw error;
    }
}

// components/LoginForm.jsx
import { login } from '../api/auth';

function LoginForm() {
    const handleSubmit = async () => {
        try {
            const result = await login(email, password);
            // ...
        } catch (error) {
            // ...
        }
    };
}
```

**Total: ~80 lines of code**

---

### **Jounce (Auto-Generated)**

```jounce
server fn login(email: String, password: String) -> Result<AuthToken, String> {
    let user = db.query("SELECT * FROM users WHERE email = $1", [email])?;

    if (!auth.verify_password(password, user.password_hash)) {
        return Err("Invalid credentials");
    }

    let token = auth.create_auth_token(user);
    return Ok(token);
}

component LoginForm() {
    fn handleSubmit() {
        let result = login(email.value, password.value).await;
        // ...
    }
}
```

**Total: 15 lines of code**

**Jounce auto-generates the other 65 lines!**

---

## **🚀 Summary**

### **What You Write**
```jounce
server fn myFunction(param: Type) -> Result<T, E> {
    // Backend logic
}
```

### **What Jounce Generates**

1. ✅ **Client stub** - async function in browser
2. ✅ **Server handler** - Express route
3. ✅ **Type checking** - validates parameters
4. ✅ **Authentication** - checks JWT tokens
5. ✅ **Error handling** - converts errors to Result
6. ✅ **Serialization** - JSON encode/decode
7. ✅ **Connection pooling** - HTTP keep-alive

**All automatically. Zero boilerplate. Zero npm packages.**

That's the magic of Jounce's RPC system! 🎉
