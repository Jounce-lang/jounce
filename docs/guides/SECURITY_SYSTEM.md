# Jounce Security System

> **Note**: For authoritative language rules, see [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md).

**Version**: v0.11.0 (Phase 17)
**Status**: Design Document
**Last Updated**: November 1, 2025

---

## 🎯 Overview

Jounce's security system provides **compile-time security annotations** that generate runtime middleware checks. This enables developers to write secure code with minimal boilerplate while maintaining type safety and performance.

---

## 🔒 Security Model

### Core Principles

1. **Security by Default** - Insecure code should be harder to write
2. **Compile-Time Checks** - Catch security issues during compilation
3. **Zero Runtime Overhead** - Security checks compile to efficient code
4. **Type-Safe** - Security annotations are type-checked
5. **Composable** - Multiple security layers can be combined

### Architecture

```
Jounce Source (.jnc)
        ↓
    [Lexer] → Tokenize @ annotations
        ↓
    [Parser] → Parse security annotations
        ↓
    [AST] → Store annotations with functions
        ↓
    [Analyzer] → Validate security rules
        ↓
    [Emitter] → Generate middleware checks
        ↓
JavaScript Output (with security guards)
```

---

## 📝 Security Annotations

### 1. @secure

**Purpose**: Marks a function as security-sensitive

**Behavior**:
- Requires HTTPS in production
- Enables strict mode checks
- Logs all access attempts
- Validates all inputs

**Syntax**:
```jounce
@secure
fn sensitive_operation() {
    // This function requires HTTPS
}
```

**Generated Code**:
```javascript
export async function sensitive_operation() {
  // Security checks
  if (process.env.NODE_ENV === 'production' && !request.secure) {
    throw new Error('HTTPS required for secure operations');
  }

  // Log access
  logger.info('sensitive_operation called', {
    timestamp: Date.now(),
    user: context.user?.id
  });

  // Original function body
  // ...
}
```

---

### 2. @auth

**Purpose**: Requires user authentication

**Syntax**:
```jounce
// Basic auth check
@auth
fn get_user_profile() {
    // User must be logged in
}

// Role-based auth
@auth(role = "admin")
fn delete_user(id: i64) {
    // Only admins can delete
}

// Multiple roles
@auth(roles = ["admin", "moderator"])
fn ban_user(id: i64) {
    // Admins or moderators
}

// Custom permission
@auth(permission = "user:write")
fn update_user(id: i64, data: UserData) {
    // Check custom permission
}
```

**Generated Code**:
```javascript
export async function delete_user(id) {
  // Auth check
  if (!context.user) {
    throw new AuthenticationError('Authentication required');
  }

  // Role check
  if (!context.user.roles.includes('admin')) {
    throw new AuthorizationError('Admin role required');
  }

  // Original function body
  // ...
}
```

---

### 3. @validate

**Purpose**: Validates function inputs against a schema

**Syntax**:
```jounce
// Validate single parameter
@validate(schema = UserSchema)
fn create_user(data: UserData) {
    // data is guaranteed to match UserSchema
}

// Validate specific parameters
@validate(email = EmailSchema, password = PasswordSchema)
fn register(email: String, password: String) {
    // Both parameters validated
}

// Custom validator function
@validate(validator = is_valid_user)
fn update_user(id: i64, data: UserData) {
    // Uses custom validation function
}
```

**Generated Code**:
```javascript
export async function create_user(data) {
  // Validate input
  const validation = UserSchema.validate(data);
  if (!validation.success) {
    throw new ValidationError('Invalid user data', validation.errors);
  }

  // Original function body with validated data
  // ...
}
```

---

### 4. @ratelimit

**Purpose**: Limits how often a function can be called

**Syntax**:
```jounce
// Max 100 requests per minute per user
@ratelimit(max = 100, window = "1m")
fn search_products(query: String) {
    // Rate limited to 100/min
}

// Per IP address
@ratelimit(max = 10, window = "1h", key = "ip")
fn forgot_password(email: String) {
    // 10 attempts per hour per IP
}

// Global rate limit
@ratelimit(max = 1000, window = "1m", scope = "global")
fn expensive_computation() {
    // 1000 total requests per minute
}
```

**Generated Code**:
```javascript
export async function search_products(query) {
  // Rate limit check
  const key = `ratelimit:search_products:${context.user.id}`;
  const count = await redis.incr(key);

  if (count === 1) {
    await redis.expire(key, 60); // 1 minute
  }

  if (count > 100) {
    throw new RateLimitError('Rate limit exceeded', {
      retryAfter: await redis.ttl(key)
    });
  }

  // Original function body
  // ...
}
```

---

### 5. @sanitize

**Purpose**: Sanitizes inputs to prevent XSS/injection attacks

**Syntax**:
```jounce
// Sanitize HTML
@sanitize(html = true)
fn create_comment(content: String) {
    // content has HTML tags escaped
}

// Sanitize SQL
@sanitize(sql = true)
fn search_users(query: String) {
    // query is SQL-safe
}

// Custom sanitizer
@sanitize(sanitizer = sanitize_markdown)
fn create_post(content: String) {
    // Uses custom sanitizer function
}
```

**Generated Code**:
```javascript
export async function create_comment(content) {
  // Sanitize HTML
  const sanitized_content = escapeHtml(content);

  // Original function body with sanitized input
  // ... (uses sanitized_content instead of content)
}
```

---

## 🔗 Composing Annotations

Multiple annotations can be combined:

```jounce
@secure
@auth(role = "admin")
@validate(schema = UserSchema)
@ratelimit(max = 10, window = "1m")
fn admin_create_user(data: UserData) {
    // This function is:
    // - HTTPS-only
    // - Admin-only
    // - Validated
    // - Rate-limited
}
```

**Execution Order**:
1. @ratelimit - Check rate limit first (fail fast)
2. @secure - Check HTTPS
3. @auth - Check authentication/authorization
4. @validate - Validate inputs
5. @sanitize - Sanitize inputs
6. Function body executes

---

## 🛠️ Middleware Generation

### How It Works

1. **Parser** collects annotations on each function
2. **Analyzer** validates annotation arguments
3. **Emitter** generates middleware wrapper code
4. **Runtime** executes checks in correct order

### Example Transformation

**Input (Jounce)**:
```jounce
@auth(role = "admin")
@validate(schema = UserSchema)
fn create_user(data: UserData) -> User {
    let user = User {
        id: generate_id(),
        name: data.name,
        email: data.email,
        role: "user"
    };

    db.insert("users", user);
    return user;
}
```

**Output (JavaScript)**:
```javascript
export async function create_user(data, context) {
  // === Generated Middleware ===

  // 1. Authentication check
  if (!context.user) {
    throw new AuthenticationError('Authentication required');
  }

  // 2. Authorization check
  if (!context.user.roles.includes('admin')) {
    throw new AuthorizationError('Admin role required');
  }

  // 3. Validation check
  const validation = UserSchema.validate(data);
  if (!validation.success) {
    throw new ValidationError('Invalid user data', validation.errors);
  }

  // === Original Function Body ===

  const user = {
    id: generate_id(),
    name: data.name,
    email: data.email,
    role: "user"
  };

  db.insert("users", user);
  return user;
}
```

---

## 🎮 RPC Integration

Security annotations automatically integrate with Jounce's RPC layer:

```jounce
// Server-side
@auth(role = "admin")
@validate(schema = UserSchema)
fn create_user(data: UserData) -> User {
    // Implementation
}

// Client-side
import { create_user } from "./server";

// Security checks happen automatically on server
try {
    let user = await create_user({ name: "Alice", email: "alice@example.com" });
    console.log("User created:", user);
} catch (error) {
    if (error instanceof AuthorizationError) {
        console.log("Permission denied");
    } else if (error instanceof ValidationError) {
        console.log("Invalid data:", error.errors);
    }
}
```

**Benefits**:
- Security enforced on server (can't be bypassed)
- Type-safe error handling on client
- Zero boilerplate for common patterns

---

## 📦 Security Package (jounce-security)

The `jounce-security` package provides runtime utilities:

```jounce
import { hash_password, verify_password, generate_token } from "jounce-security";

@auth
fn login(email: String, password: String) -> Token {
    let user = db.find_user_by_email(email);

    if (!verify_password(password, user.password_hash)) {
        throw new Error("Invalid credentials");
    }

    return generate_token(user.id);
}

fn register(email: String, password: String) -> User {
    let password_hash = hash_password(password);

    let user = User {
        id: generate_id(),
        email: email,
        password_hash: password_hash
    };

    db.insert("users", user);
    return user;
}
```

**Package API**:
- `hash_password(password: String) -> String`
- `verify_password(password: String, hash: String) -> Boolean`
- `generate_token(user_id: String) -> String`
- `verify_token(token: String) -> UserId`
- `hash_api_key(key: String) -> String`
- `generate_api_key() -> String`

---

## ⚙️ Configuration

Security settings in `jounce.config.json`:

```json
{
  "security": {
    "https_required": true,
    "auth_provider": "jwt",
    "jwt_secret": "${JWT_SECRET}",
    "session_timeout": "24h",
    "password_min_length": 8,
    "ratelimit_storage": "redis",
    "sanitize_by_default": false,
    "log_security_events": true
  }
}
```

---

## 🧪 Testing Security

```jounce
import { test, expect } from "jounce-test";

test("admin-only function requires admin role", async () => {
    let context = { user: { id: "123", roles: ["user"] } };

    expect(() => {
        delete_user(456, context);
    }).to_throw(AuthorizationError);
});

test("validation rejects invalid data", async () => {
    let invalid_data = { name: "", email: "not-an-email" };

    expect(() => {
        create_user(invalid_data);
    }).to_throw(ValidationError);
});

test("rate limit enforced", async () => {
    for (let i = 0; i < 100; i++) {
        await search_products("test");
    }

    // 101st request should fail
    expect(() => {
        search_products("test");
    }).to_throw(RateLimitError);
});
```

---

## 📊 Performance Impact

Security annotations have **minimal runtime overhead**:

| Annotation | Overhead | Notes |
|------------|----------|-------|
| @secure | ~0.1ms | HTTPS check + logging |
| @auth | ~0.5ms | Session lookup + role check |
| @validate | ~1-5ms | Depends on schema complexity |
| @ratelimit | ~1ms | Redis lookup |
| @sanitize | ~0.5ms | String operations |

**Combined**: ~3-7ms total for all annotations

For comparison:
- Database query: ~10-100ms
- HTTP request: ~50-500ms
- File I/O: ~1-10ms

Security overhead is **negligible** compared to typical operations.

---

## 🔐 Best Practices

### 1. Always Use @auth on Sensitive Functions

```jounce
// ❌ Bad - No auth check
fn delete_user(id: i64) {
    db.delete("users", id);
}

// ✅ Good - Auth required
@auth(role = "admin")
fn delete_user(id: i64) {
    db.delete("users", id);
}
```

### 2. Validate All User Inputs

```jounce
// ❌ Bad - No validation
fn create_user(data: UserData) {
    db.insert("users", data);
}

// ✅ Good - Validated
@validate(schema = UserSchema)
fn create_user(data: UserData) {
    db.insert("users", data);
}
```

### 3. Use Rate Limiting on Public APIs

```jounce
// ❌ Bad - No rate limit
fn search(query: String) {
    return expensive_search(query);
}

// ✅ Good - Rate limited
@ratelimit(max = 100, window = "1m")
fn search(query: String) {
    return expensive_search(query);
}
```

### 4. Combine Security Layers

```jounce
// ✅ Defense in depth
@secure
@auth(role = "admin")
@validate(schema = CreateUserSchema)
@ratelimit(max = 10, window = "1m")
fn admin_create_user(data: UserData) -> User {
    // Multiple security layers
}
```

---

## 🚀 Migration Guide

### From Manual Checks

**Before**:
```jounce
fn delete_user(id: i64, context: Context) {
    // Manual security checks
    if (!context.user) {
        throw new Error("Not authenticated");
    }

    if (!context.user.roles.includes("admin")) {
        throw new Error("Not authorized");
    }

    db.delete("users", id);
}
```

**After**:
```jounce
@auth(role = "admin")
fn delete_user(id: i64) {
    db.delete("users", id);
}
```

**Benefits**:
- ✅ 80% less code
- ✅ Standardized error messages
- ✅ Compile-time validation
- ✅ Automatic testing

---

## 📖 Implementation Phases

### Phase 1: Parser Support (Sprint 17.1)
- [ ] Lexer: Add @ token
- [ ] Parser: Parse security annotations
- [ ] AST: Store annotations with functions
- [ ] Tests: Annotation parsing

### Phase 2: Middleware Generation (Sprint 17.1)
- [ ] Emitter: Generate @secure checks
- [ ] Emitter: Generate @auth checks
- [ ] Emitter: Generate @validate checks
- [ ] Tests: Generated code

### Phase 3: Runtime Package (Sprint 17.1)
- [ ] Create jounce-security package
- [ ] Implement auth helpers
- [ ] Implement validation helpers
- [ ] Implement rate limiting
- [ ] Tests: Package functionality

### Phase 4: Documentation (Sprint 17.1)
- [ ] Security guide
- [ ] API reference
- [ ] Migration guide
- [ ] Examples

---

## 🎯 Success Metrics

- [ ] All security annotations parse correctly
- [ ] Middleware generates correct code
- [ ] 100% test coverage on security package
- [ ] Zero security vulnerabilities in examples
- [ ] Documentation complete
- [ ] Migration guide tested

---

**Next Steps**: Implement lexer and parser support for annotations.

**Last Updated**: November 1, 2025
**Phase**: 17 - Security & Production Features
**Sprint**: 17.1 - Security Annotations

---

**Maintained by: The Jounce Project**
