# jounce-security

**Runtime utilities for Jounce's security system**

Version: 0.11.0 (Phase 17)
License: MIT

---

## 📦 Installation

```bash
jnc add jounce-security
```

---

## 🎯 Overview

`jounce-security` provides runtime utilities that work with Jounce's compile-time security annotations (`@auth`, `@validate`, `@secure`, etc.). This package handles:

- Password hashing and verification (bcrypt)
- Token generation and validation (JWT)
- API key management
- Session management
- Rate limiting (Redis-backed)
- Input sanitization

---

## 🔐 Password Management

### hash_password

Hash a plaintext password using bcrypt.

```jounce
import { hash_password } from "jounce-security";

let password_hash = hash_password("user_password_123");
// Returns: "$2b$10$..."
```

**Parameters**:
- `password: String` - Plaintext password
- `cost?: i64` - Bcrypt cost factor (default: 10)

**Returns**: `String` - Bcrypt hash

---

### verify_password

Verify a password against a hash.

```jounce
import { verify_password } from "jounce-security";

let is_valid = verify_password("user_password_123", password_hash);
// Returns: true or false
```

**Parameters**:
- `password: String` - Plaintext password to check
- `hash: String` - Bcrypt hash to compare against

**Returns**: `Boolean` - True if password matches

---

### Example: Registration & Login

```jounce
import { hash_password, verify_password } from "jounce-security";

fn register(email: String, password: String) -> User {
    let password_hash = hash_password(password);

    let user = {
        id: generate_id(),
        email: email,
        password_hash: password_hash
    };

    db.insert("users", user);
    return user;
}

fn login(email: String, password: String) -> Token {
    let user = db.find_one("users", { email: email });

    if (!user || !verify_password(password, user.password_hash)) {
        throw new Error("Invalid credentials");
    }

    return generate_token(user.id);
}
```

---

## 🎟️ Token Management (JWT)

### generate_token

Generate a JWT token for a user.

```jounce
import { generate_token } from "jounce-security";

let token = generate_token(user.id, { role: "admin" });
// Returns: "eyJhbGciOiJIUzI1NiIs..."
```

**Parameters**:
- `user_id: String` - User identifier
- `claims?: Object` - Additional JWT claims
- `expires_in?: String` - Expiration time (default: "24h")

**Returns**: `String` - JWT token

---

### verify_token

Verify and decode a JWT token.

```jounce
import { verify_token } from "jounce-security";

let payload = verify_token(token);
// Returns: { user_id: "123", role: "admin", exp: 1234567890 }
```

**Parameters**:
- `token: String` - JWT token to verify

**Returns**: `Object` - Decoded token payload

**Throws**: `TokenExpiredError`, `TokenInvalidError`

---

### refresh_token

Refresh an expiring token.

```jounce
import { refresh_token } from "jounce-security";

let new_token = refresh_token(old_token);
// Returns new JWT with extended expiration
```

**Parameters**:
- `token: String` - Existing JWT token

**Returns**: `String` - New JWT token

---

## 🔑 API Key Management

### generate_api_key

Generate a new API key.

```jounce
import { generate_api_key } from "jounce-security";

let api_key = generate_api_key();
// Returns: "jnc_live_a1b2c3d4e5f6g7h8i9j0"
```

**Returns**: `String` - API key (format: `jnc_{env}_{random}`)

---

### hash_api_key

Hash an API key for storage.

```jounce
import { hash_api_key } from "jounce-security";

let api_key = generate_api_key();
let hash = hash_api_key(api_key);

// Store hash in database
db.insert("api_keys", {
    user_id: user.id,
    key_hash: hash,
    created_at: Date.now()
});

// Show key to user ONCE
console.log("Your API key:", api_key);
```

**Parameters**:
- `api_key: String` - API key to hash

**Returns**: `String` - SHA-256 hash of key

---

### verify_api_key

Verify an API key against stored hash.

```jounce
import { verify_api_key, hash_api_key } from "jounce-security";

fn authenticate_api_key(provided_key: String) -> User {
    let key_hash = hash_api_key(provided_key);
    let api_key_record = db.find_one("api_keys", { key_hash: key_hash });

    if (!api_key_record) {
        throw new Error("Invalid API key");
    }

    return db.find_one("users", { id: api_key_record.user_id });
}
```

---

## 🛡️ Input Sanitization

### escape_html

Escape HTML to prevent XSS attacks.

```jounce
import { escape_html } from "jounce-security";

let user_input = "<script>alert('XSS')</script>";
let safe_output = escape_html(user_input);
// Returns: "&lt;script&gt;alert(&#39;XSS&#39;)&lt;/script&gt;"
```

**Parameters**:
- `input: String` - HTML string to escape

**Returns**: `String` - Escaped HTML

---

### sanitize_sql

Sanitize SQL input to prevent injection.

```jounce
import { sanitize_sql } from "jounce-security";

let user_query = "'; DROP TABLE users; --";
let safe_query = sanitize_sql(user_query);
// Returns: "\\'; DROP TABLE users; --"
```

**Parameters**:
- `input: String` - SQL string to sanitize

**Returns**: `String` - Sanitized SQL

---

### sanitize_markdown

Sanitize markdown while preserving safe formatting.

```jounce
import { sanitize_markdown } from "jounce-security";

let user_markdown = "# Title\n<script>alert('XSS')</script>";
let safe_markdown = sanitize_markdown(user_markdown);
// Returns: "# Title\n" (script tag removed)
```

**Parameters**:
- `input: String` - Markdown to sanitize

**Returns**: `String` - Sanitized markdown

---

## ⏱️ Rate Limiting

### rate_limit

Check and enforce rate limits.

```jounce
import { rate_limit } from "jounce-security";

@ratelimit(max = 100, window = "1m")
fn search_products(query: String) -> Array<Product> {
    // Automatically rate limited
    return db.find("products", { name: query });
}

// Manual rate limiting
fn manual_rate_limit_example() {
    let key = "api:search:" + context.user.id;
    let allowed = rate_limit(key, { max: 100, window: 60 });

    if (!allowed) {
        throw new RateLimitError("Too many requests");
    }

    // Continue with operation
}
```

**Parameters**:
- `key: String` - Rate limit key (user ID, IP, etc.)
- `options: Object` - Rate limit configuration
  - `max: i64` - Maximum requests
  - `window: i64` - Time window in seconds

**Returns**: `Boolean` - True if allowed, false if rate limited

---

### get_rate_limit_status

Get current rate limit status.

```jounce
import { get_rate_limit_status } from "jounce-security";

let status = get_rate_limit_status("api:search:user123");
// Returns: { count: 45, limit: 100, remaining: 55, reset_at: 1234567890 }
```

**Parameters**:
- `key: String` - Rate limit key

**Returns**: `Object` - Rate limit status

---

## 🧪 Validation

### validate

Validate data against a schema.

```jounce
import { validate } from "jounce-security";

let UserSchema = {
    name: { type: "string", min_length: 2, required: true },
    email: { type: "string", format: "email", required: true },
    age: { type: "integer", min: 13, max: 120 }
};

let result = validate({ name: "Alice", email: "alice@example.com", age: 25 }, UserSchema);
// Returns: { success: true, errors: [] }

let result2 = validate({ name: "A", email: "invalid" }, UserSchema);
// Returns: { success: false, errors: [{ field: "name", message: "..." }, ...] }
```

**Parameters**:
- `data: Object` - Data to validate
- `schema: Object` - Validation schema

**Returns**: `ValidationResult` - Validation result

---

## 🔧 Middleware Helpers

These functions are used internally by the security annotation system:

### check_https

```jounce
import { check_https } from "jounce-security";

fn secure_operation() {
    check_https(request);  // Throws if not HTTPS in production
    // ...
}
```

### check_auth

```jounce
import { check_auth } from "jounce-security";

fn protected_operation() {
    check_auth(context);  // Throws if not authenticated
    // ...
}
```

### check_role

```jounce
import { check_role } from "jounce-security";

fn admin_operation() {
    check_role(context, "admin");  // Throws if not admin
    // ...
}
```

---

## 🎯 Complete Example

```jounce
import {
    hash_password,
    verify_password,
    generate_token,
    verify_token,
    generate_api_key,
    hash_api_key,
    escape_html,
    rate_limit
} from "jounce-security";

// User registration
fn register(email: String, password: String) -> User {
    let password_hash = hash_password(password);

    let user = {
        id: generate_id(),
        email: email,
        password_hash: password_hash,
        created_at: Date.now()
    };

    db.insert("users", user);
    return user;
}

// User login
fn login(email: String, password: String) -> AuthResponse {
    // Rate limit login attempts
    let rate_key = "login:" + email;
    if (!rate_limit(rate_key, { max: 5, window: 300 })) {
        throw new Error("Too many login attempts. Try again in 5 minutes.");
    }

    let user = db.find_one("users", { email: email });

    if (!user || !verify_password(password, user.password_hash)) {
        throw new Error("Invalid credentials");
    }

    let token = generate_token(user.id);

    return {
        token: token,
        user: user
    };
}

// Protected endpoint
@auth
fn get_profile() -> User {
    return db.find_one("users", { id: context.user.id });
}

// Admin endpoint
@auth(role = "admin")
fn delete_user(user_id: String) -> Boolean {
    db.delete("users", user_id);
    return true;
}

// Sanitized user input
@sanitize(html = true)
fn create_comment(content: String) -> Comment {
    let comment = {
        id: generate_id(),
        content: escape_html(content),
        author_id: context.user.id,
        created_at: Date.now()
    };

    db.insert("comments", comment);
    return comment;
}
```

---

## ⚙️ Configuration

Set environment variables:

```bash
# JWT secret (required)
export JWT_SECRET="your-secret-key-here"

# JWT expiration (default: 24h)
export JWT_EXPIRES_IN="24h"

# Bcrypt cost (default: 10)
export BCRYPT_COST=10

# Redis URL for rate limiting (default: localhost)
export REDIS_URL="redis://localhost:6379"

# Environment (affects security checks)
export NODE_ENV="production"
```

---

## 🧪 Testing

```jounce
import { test, expect } from "jounce-test";
import { hash_password, verify_password } from "jounce-security";

test("password hashing works", () => {
    let password = "test123";
    let hash = hash_password(password);

    expect(verify_password(password, hash)).to_be(true);
    expect(verify_password("wrong", hash)).to_be(false);
});

test("token generation and verification", () => {
    let token = generate_token("user123");
    let payload = verify_token(token);

    expect(payload.user_id).to_equal("user123");
});
```

---

## 📚 API Reference

### Functions

| Function | Description | Parameters | Returns |
|----------|-------------|------------|---------|
| `hash_password` | Hash a password | `password: String` | `String` |
| `verify_password` | Verify password | `password: String, hash: String` | `Boolean` |
| `generate_token` | Generate JWT | `user_id: String, claims?: Object` | `String` |
| `verify_token` | Verify JWT | `token: String` | `Object` |
| `refresh_token` | Refresh JWT | `token: String` | `String` |
| `generate_api_key` | Generate API key | - | `String` |
| `hash_api_key` | Hash API key | `key: String` | `String` |
| `escape_html` | Escape HTML | `input: String` | `String` |
| `sanitize_sql` | Sanitize SQL | `input: String` | `String` |
| `sanitize_markdown` | Sanitize Markdown | `input: String` | `String` |
| `rate_limit` | Check rate limit | `key: String, options: Object` | `Boolean` |
| `validate` | Validate data | `data: Object, schema: Object` | `ValidationResult` |

### Error Types

- `AuthenticationError` - User not authenticated
- `AuthorizationError` - User not authorized
- `ValidationError` - Input validation failed
- `RateLimitError` - Rate limit exceeded
- `TokenExpiredError` - JWT token expired
- `TokenInvalidError` - JWT token invalid

---

## 📖 Learn More

- [Security System Documentation](../../docs/SECURITY_SYSTEM.md)
- [Security Examples](../../examples/security/)
- [Security Annotations Guide](../../docs/SECURITY_ANNOTATIONS.md)

---

## 📄 License

MIT

**Last Updated**: November 1, 2025
**Version**: 0.11.0
