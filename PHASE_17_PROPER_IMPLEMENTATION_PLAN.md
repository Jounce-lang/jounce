# Phase 17: Security & Production Features - PROPER Implementation Plan

**Version**: v0.9.0
**Created**: November 1, 2025
**Status**: Planning Phase - Following CLAUDE.md Rules Strictly

---

## 🚨 COMMITMENT TO QUALITY

**This plan follows CLAUDE.md rules WITHOUT EXCEPTION:**

✅ **Implement features completely or not at all**
✅ **Fix the architecture, not the symptoms**
✅ **Test thoroughly before marking complete**
✅ **Think through edge cases first**

❌ **NO "good enough for now" implementations**
❌ **NO stubs or placeholders**
❌ **NO documentation without working code**

---

## 📊 WHAT'S ALREADY COMPLETE

### ✅ Foundation Work (Actually Complete!)

**Annotation Parsing Infrastructure**:
- `src/ast.rs`: Complete AST nodes for `Annotation`, `AnnotationArgument`, `AnnotationValue`
- `src/parser.rs`: Complete parsing of `@auth()`, `@validate()`, `@secure()`, etc.
- `tests/annotations.rs`: 3 passing tests for annotation parsing
- `examples/security/*.jnc`: 4 working example files

**Status**: ✅ **100% Complete** - Annotations parse correctly, tests pass

**What This Means**:
- We CAN parse `@auth(role="admin")` from source code
- We CANNOT generate any security middleware yet
- Parser extracts annotations into AST correctly

---

## 🎯 PHASE 17 FEATURES TO IMPLEMENT

Based on roadmap, Phase 17 should deliver:

1. **Security Middleware Generation** - Generate actual security checks
2. **Build Optimizations** - At least one optimization (DCE recommended)
3. **Deployment Tooling** - At least one platform adapter (Vercel recommended)

**Reality Check**: These are THREE major features. Each could take 8-20 hours.

**DECISION POINT**: Should we implement all three, or focus on ONE feature completely?

---

## 🔥 RECOMMENDED APPROACH: Feature-by-Feature

### **Strategy**: Implement ONE feature completely, ship it, test it, then move to next

**Why This Works**:
- Follows "implement completely or not at all" rule
- Each feature is independently valuable
- Reduces risk of incomplete work
- Easier to test and verify
- User can use feature immediately

**Order of Implementation** (recommended):

1. **FIRST**: Security Middleware Generation (8-12 hours)
   - Most valuable feature
   - Builds on completed annotation parsing
   - Clear scope and success criteria

2. **SECOND**: Build Optimizations - Dead Code Elimination (12-16 hours)
   - Valuable for production apps
   - Well-defined scope
   - Measurable results

3. **THIRD**: Deployment Tooling - Vercel Adapter (8-12 hours)
   - Valuable for deployments
   - Single platform = clear scope
   - Can expand to more platforms later

**Total Estimated Time**: 28-40 hours for complete Phase 17

---

## 📋 FEATURE 1: Security Middleware Generation (COMPLETE Implementation)

### **Goal**: Make annotations actually DO something

**User Experience**:
```jounce
@auth(role = "admin")
@server
fn delete_user(id: i64) {
    db.delete("users", id);
}
```

**Should Generate**:
```javascript
export async function delete_user({ id } = {}) {
  // GENERATED SECURITY MIDDLEWARE
  if (!__jounce_auth_check({ role: "admin" })) {
    throw new Error("Unauthorized: requires role 'admin'");
  }

  // ACTUAL FUNCTION CODE
  db.delete("users", id);
}
```

### **Implementation Steps**

#### **Step 1: Security Runtime Library** (2-3 hours)

**Create**: `runtime/security.js`

**Must Include**:
- `__jounce_auth_check(requirements)` - Check user permissions
- `__jounce_validate(schema, data)` - Validate input against schema
- `__jounce_ratelimit(limits)` - Check rate limit
- `__jounce_sanitize(data)` - Sanitize input data

**Implementation Details**:
```javascript
// runtime/security.js

// Global context for current request
let __jounce_security_context = {
  user: null,
  session: null,
  request: null
};

// Set security context (called by framework adapter)
export function __jounce_set_security_context(context) {
  __jounce_security_context = context;
}

// Auth check
export function __jounce_auth_check(requirements) {
  const { user } = __jounce_security_context;

  if (!user) {
    return false;
  }

  if (requirements.role) {
    return user.roles && user.roles.includes(requirements.role);
  }

  if (requirements.permission) {
    return user.permissions && user.permissions.includes(requirements.permission);
  }

  return true; // Just needs to be authenticated
}

// Validation check
export function __jounce_validate(schema, data) {
  // Implement schema validation
  // For now, support basic types: string, number, boolean, array, object
  // with optional min/max/pattern constraints

  if (typeof schema !== 'object') {
    throw new Error('Schema must be an object');
  }

  // Validate each field
  for (const [key, rules] of Object.entries(schema)) {
    const value = data[key];

    // Check required
    if (rules.required && value === undefined) {
      throw new Error(`Missing required field: ${key}`);
    }

    // Check type
    if (value !== undefined && rules.type) {
      const actualType = Array.isArray(value) ? 'array' : typeof value;
      if (actualType !== rules.type) {
        throw new Error(`Invalid type for ${key}: expected ${rules.type}, got ${actualType}`);
      }
    }

    // Check string constraints
    if (typeof value === 'string') {
      if (rules.minLength && value.length < rules.minLength) {
        throw new Error(`${key} must be at least ${rules.minLength} characters`);
      }
      if (rules.maxLength && value.length > rules.maxLength) {
        throw new Error(`${key} must be at most ${rules.maxLength} characters`);
      }
      if (rules.pattern && !new RegExp(rules.pattern).test(value)) {
        throw new Error(`${key} does not match required pattern`);
      }
    }

    // Check number constraints
    if (typeof value === 'number') {
      if (rules.min !== undefined && value < rules.min) {
        throw new Error(`${key} must be at least ${rules.min}`);
      }
      if (rules.max !== undefined && value > rules.max) {
        throw new Error(`${key} must be at most ${rules.max}`);
      }
    }
  }

  return true;
}

// Rate limiting (simple in-memory implementation)
const __rateLimitStore = new Map();

export function __jounce_ratelimit(limits) {
  const { user } = __jounce_security_context;
  const key = user?.id || __jounce_security_context.request?.ip || 'anonymous';

  const now = Date.now();
  const windowMs = limits.window || 60000; // Default 1 minute
  const maxRequests = limits.max || 100;

  if (!__rateLimitStore.has(key)) {
    __rateLimitStore.set(key, []);
  }

  const requests = __rateLimitStore.get(key);

  // Remove old requests outside window
  const recentRequests = requests.filter(timestamp => now - timestamp < windowMs);

  if (recentRequests.length >= maxRequests) {
    throw new Error(`Rate limit exceeded: max ${maxRequests} requests per ${windowMs}ms`);
  }

  recentRequests.push(now);
  __rateLimitStore.set(key, recentRequests);

  return true;
}

// Sanitization
export function __jounce_sanitize(data) {
  if (typeof data === 'string') {
    // Basic HTML sanitization
    return data
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;')
      .replace(/'/g, '&#x27;')
      .replace(/\//g, '&#x2F;');
  }

  if (Array.isArray(data)) {
    return data.map(__jounce_sanitize);
  }

  if (typeof data === 'object' && data !== null) {
    const sanitized = {};
    for (const [key, value] of Object.entries(data)) {
      sanitized[key] = __jounce_sanitize(value);
    }
    return sanitized;
  }

  return data;
}
```

**Tests**: `tests/security_runtime.rs`
- Test auth check with valid/invalid roles
- Test validation with valid/invalid data
- Test rate limiting
- Test sanitization

**Verification**:
- [ ] All security runtime functions work
- [ ] Tests pass
- [ ] Can be imported in generated JS

---

#### **Step 2: Middleware Generation in Emitter** (4-6 hours)

**Modify**: `src/js_emitter.rs`

**Add Function**: `generate_security_middleware()`

**Implementation**:

```rust
// In emit_function_definition()

fn emit_function_definition(&mut self, func: &FunctionDefinition) -> String {
    let mut output = String::new();

    // Export directive
    output.push_str("export ");

    // Async if needed
    if func.is_async {
        output.push_str("async ");
    }

    output.push_str("function ");
    output.push_str(&func.name.value);
    output.push_str("(");

    // Parameters
    if func.is_server || func.is_client {
        output.push_str("{ ");
        let param_names: Vec<String> = func.parameters.iter()
            .map(|p| p.name.value.clone())
            .collect();
        output.push_str(&param_names.join(", "));
        output.push_str(" } = {}");
    } else {
        let params: Vec<String> = func.parameters.iter()
            .map(|p| p.name.value.clone())
            .collect();
        output.push_str(&params.join(", "));
    }

    output.push_str(") {\n");

    // SECURITY MIDDLEWARE GENERATION
    if !func.annotations.is_empty() {
        output.push_str(&self.generate_security_middleware(&func.annotations));
    }

    // Function body
    output.push_str(&self.emit_block_statement(&func.body));

    output.push_str("}\n");
    output
}

fn generate_security_middleware(&self, annotations: &[Annotation]) -> String {
    let mut middleware = String::new();

    for annotation in annotations {
        match annotation.name.value.as_str() {
            "auth" => {
                middleware.push_str("  // Authentication check\n");
                middleware.push_str("  if (!__jounce_auth_check(");

                // Generate requirements object
                middleware.push_str("{");
                let args: Vec<String> = annotation.arguments.iter()
                    .map(|arg| {
                        let value = self.format_annotation_value(&arg.value);
                        format!("{}: {}", arg.name, value)
                    })
                    .collect();
                middleware.push_str(&args.join(", "));
                middleware.push_str("})) {\n");
                middleware.push_str("    throw new Error(\"Unauthorized\");\n");
                middleware.push_str("  }\n");
            }

            "validate" => {
                middleware.push_str("  // Input validation\n");

                // Find schema argument
                let schema_arg = annotation.arguments.iter()
                    .find(|arg| arg.name == "schema")
                    .expect("@validate requires schema argument");

                let schema_name = if let AnnotationValue::Identifier(name) = &schema_arg.value {
                    name
                } else {
                    panic!("@validate schema must be an identifier");
                };

                middleware.push_str(&format!("  __jounce_validate({}, arguments[0]);\n", schema_name));
            }

            "ratelimit" => {
                middleware.push_str("  // Rate limiting\n");
                middleware.push_str("  __jounce_ratelimit(");

                middleware.push_str("{");
                let args: Vec<String> = annotation.arguments.iter()
                    .map(|arg| {
                        let value = self.format_annotation_value(&arg.value);
                        format!("{}: {}", arg.name, value)
                    })
                    .collect();
                middleware.push_str(&args.join(", "));
                middleware.push_str("});\n");
            }

            "sanitize" => {
                middleware.push_str("  // Input sanitization\n");

                // Find fields argument
                if let Some(fields_arg) = annotation.arguments.iter().find(|arg| arg.name == "fields") {
                    if let AnnotationValue::Array(field_names) = &fields_arg.value {
                        for field_name in field_names {
                            if let AnnotationValue::String(name) = field_name {
                                middleware.push_str(&format!("  {} = __jounce_sanitize({});\n", name, name));
                            }
                        }
                    }
                } else {
                    // Sanitize all parameters
                    middleware.push_str("  const __args = __jounce_sanitize(arguments[0]);\n");
                    middleware.push_str("  Object.assign(arguments[0], __args);\n");
                }
            }

            "secure" => {
                middleware.push_str("  // HTTPS enforcement\n");
                middleware.push_str("  if (__jounce_security_context.request && __jounce_security_context.request.protocol !== 'https') {\n");
                middleware.push_str("    throw new Error(\"This endpoint requires HTTPS\");\n");
                middleware.push_str("  }\n");
            }

            _ => {
                // Unknown annotation, skip
            }
        }
    }

    middleware
}

fn format_annotation_value(&self, value: &AnnotationValue) -> String {
    match value {
        AnnotationValue::String(s) => format!("\"{}\"", s),
        AnnotationValue::Integer(n) => n.to_string(),
        AnnotationValue::Identifier(id) => id.clone(),
        AnnotationValue::Array(values) => {
            let formatted: Vec<String> = values.iter()
                .map(|v| self.format_annotation_value(v))
                .collect();
            format!("[{}]", formatted.join(", "))
        }
    }
}
```

**Tests**: `tests/security_middleware.rs`
- Test `@auth` generates correct middleware
- Test `@validate` generates correct middleware
- Test `@ratelimit` generates correct middleware
- Test `@sanitize` generates correct middleware
- Test `@secure` generates correct middleware
- Test multiple annotations on same function

**Verification**:
- [ ] Generated JS includes security checks
- [ ] Security checks are in correct order
- [ ] All annotation types supported
- [ ] Tests pass

---

#### **Step 3: Runtime Import Generation** (1-2 hours)

**Modify**: `src/js_emitter.rs` - `emit_program()`

**Add**: Import security runtime at top of generated file

```rust
fn emit_program(&mut self, program: &Program) -> String {
    let mut output = String::new();

    // Runtime imports
    output.push_str("import { h, signal, computed, effect } from './runtime/index.js';\n");

    // Check if any security annotations are used
    let uses_security = program.statements.iter().any(|stmt| {
        if let Statement::Function(func) = stmt {
            !func.annotations.is_empty()
        } else {
            false
        }
    });

    if uses_security {
        output.push_str("import { __jounce_auth_check, __jounce_validate, __jounce_ratelimit, __jounce_sanitize, __jounce_set_security_context } from './runtime/security.js';\n");
    }

    output.push_str("\n");

    // Rest of program...
    for statement in &program.statements {
        output.push_str(&self.emit_statement(statement));
        output.push_str("\n");
    }

    output
}
```

**Tests**: Integration test that compiles .jnc file with annotations

**Verification**:
- [ ] Security imports only added when needed
- [ ] Imports are valid
- [ ] Generated JS runs without import errors

---

#### **Step 4: Integration Testing** (2-3 hours)

**Create**: `examples/apps/03-secure-admin/main.jnc`

```jounce
// User schema for validation
const UserSchema = {
    username: { type: "string", required: true, minLength: 3, maxLength: 20 },
    email: { type: "string", required: true, pattern: "^[^@]+@[^@]+\\.[^@]+$" },
    age: { type: "number", min: 0, max: 150 }
};

@auth(role = "admin")
@server
fn get_users() {
    return db.query("SELECT * FROM users");
}

@auth(role = "admin")
@validate(schema = UserSchema)
@server
fn create_user(username: String, email: String, age: i64) {
    return db.insert("users", { username, email, age });
}

@auth(role = "admin")
@ratelimit(max = 10, window = 60000)
@server
fn delete_user(id: i64) {
    return db.delete("users", id);
}

@auth(role = "user")
@sanitize(fields = ["bio", "comment"])
@server
fn update_profile(bio: String, comment: String) {
    return db.update("profile", { bio, comment });
}

component AdminPanel() {
    let users = signal([]);

    fn load_users() {
        get_users().then((data) => {
            users.value = data;
        });
    }

    <div>
        <h1>Admin Panel</h1>
        <button onClick={load_users}>Load Users</button>
        <ul>
            {users.value.map((user) => {
                <li>{user.username} - {user.email}</li>
            })}
        </ul>
    </div>
}
```

**Test Process**:
1. Compile: `cargo run -- compile examples/apps/03-secure-admin/main.jnc`
2. Check generated JS includes security middleware
3. Run in browser with mock security context
4. Verify auth checks work
5. Verify validation works
6. Verify rate limiting works
7. Verify sanitization works

**Success Criteria**:
- [ ] Compiles without errors
- [ ] Generated JS includes all middleware
- [ ] Security checks execute correctly
- [ ] Unauthorized calls throw errors
- [ ] Invalid data throws errors
- [ ] Rate limits enforce correctly

---

### **Feature 1 Total Estimate**: 8-12 hours

### **Feature 1 Deliverables**:
- ✅ `runtime/security.js` - Complete security runtime
- ✅ Modified `src/js_emitter.rs` - Middleware generation
- ✅ Tests passing for all annotation types
- ✅ Example app demonstrating all features
- ✅ Documentation in `docs/SECURITY_FEATURES.md`

---

## 📋 FEATURE 2: Dead Code Elimination (COMPLETE Implementation)

### **Goal**: Remove unused functions/variables from production builds

**User Experience**:
```bash
cargo run -- compile app.jnc --optimize
```

**Result**: Generated JS only includes functions actually used

### **Implementation Steps**

#### **Step 1: Usage Analysis** (4-6 hours)

**Create**: `src/analyzer/usage.rs`

**Implement**:
- Walk AST and build call graph
- Track which functions call which functions
- Track which variables are read/written
- Mark entry points (components, exported functions)
- Mark all code reachable from entry points

**Algorithm**:
1. Find all entry points (components, `main()`, exported functions)
2. For each entry point, traverse all reachable code
3. Mark all functions/variables encountered
4. Anything not marked = dead code

**Tests**: `tests/usage_analysis.rs`
- Test simple function call chain
- Test unused functions detected
- Test recursive functions
- Test cross-component references

---

#### **Step 2: Dead Code Removal** (3-4 hours)

**Create**: `src/optimizer/dce.rs`

**Implement**:
- Take usage analysis results
- Filter AST to remove unmarked statements
- Preserve all entry points

**Tests**: `tests/dead_code_elimination.rs`
- Test unused function removed
- Test used function kept
- Test components always kept
- Test transitive dependencies kept

---

#### **Step 3: CLI Integration** (2-3 hours)

**Modify**: `src/main.rs`

**Add**: `--optimize` flag

```rust
#[derive(Parser)]
struct CompileArgs {
    /// Input .jnc file
    input: PathBuf,

    /// Enable optimizations
    #[arg(long)]
    optimize: bool,
}

fn compile_command(args: CompileArgs) {
    // ... existing compilation ...

    if args.optimize {
        // Run usage analysis
        let usage_info = analyze_usage(&ast);

        // Remove dead code
        let optimized_ast = eliminate_dead_code(ast, &usage_info);

        // Continue with optimized AST
        ast = optimized_ast;
    }

    // ... generate JS ...
}
```

**Tests**: Integration test with `--optimize` flag

---

#### **Step 4: Metrics & Verification** (1-2 hours)

**Output**: Show optimization results

```
Optimization Results:
  Functions removed: 15
  Variables removed: 8
  Original size: 12.5 KB
  Optimized size: 8.2 KB
  Size reduction: 34.4%
```

**Tests**: Verify metrics are accurate

---

### **Feature 2 Total Estimate**: 12-16 hours

### **Feature 2 Deliverables**:
- ✅ `src/analyzer/usage.rs` - Usage analysis
- ✅ `src/optimizer/dce.rs` - Dead code elimination
- ✅ CLI `--optimize` flag working
- ✅ Tests passing
- ✅ Metrics output
- ✅ Documentation in `docs/OPTIMIZATION.md`

---

## 📋 FEATURE 3: Vercel Deployment Adapter (COMPLETE Implementation)

### **Goal**: One command to deploy to Vercel

**User Experience**:
```bash
jnc deploy vercel
```

**Result**: App deployed to Vercel, URL printed

### **Implementation Steps**

#### **Step 1: Vercel Adapter** (3-4 hours)

**Create**: `src/deploy/vercel.rs`

**Implement**:
- Generate `vercel.json` config
- Create `api/` directory for serverless functions
- Generate Vercel serverless function wrappers
- Handle static assets

**Output Structure**:
```
dist/
  ├── vercel.json
  ├── public/
  │   ├── index.html
  │   └── app.js
  └── api/
      └── [function].js  (for each @server function)
```

---

#### **Step 2: CLI Deploy Command** (2-3 hours)

**Modify**: `src/main.rs`

**Add**: `jnc deploy <platform>` subcommand

```rust
#[derive(Parser)]
enum Commands {
    Compile(CompileArgs),
    Deploy(DeployArgs),
}

#[derive(Parser)]
struct DeployArgs {
    /// Platform to deploy to
    platform: String,
}
```

**Implement**: Call Vercel CLI via `std::process::Command`

---

#### **Step 3: Integration Testing** (2-3 hours)

**Tests**:
- Mock Vercel deployment
- Verify correct files generated
- Verify `vercel.json` is valid
- Test serverless function wrappers

---

#### **Step 4: Documentation** (1-2 hours)

**Create**: `docs/DEPLOYMENT.md`

**Include**:
- Prerequisites (Vercel CLI installed)
- Step-by-step deployment guide
- Troubleshooting
- Environment variables
- Custom domains

---

### **Feature 3 Total Estimate**: 8-12 hours

### **Feature 3 Deliverables**:
- ✅ `src/deploy/vercel.rs` - Vercel adapter
- ✅ CLI `deploy` subcommand working
- ✅ Tests passing
- ✅ Example deployment working
- ✅ Documentation complete

---

## ✅ DEFINITION OF DONE

### **For Each Feature**:

1. **Code Complete**:
   - [ ] All functions implemented (no TODOs, no stubs)
   - [ ] All edge cases handled
   - [ ] Error handling complete
   - [ ] Code compiles without warnings

2. **Tests Complete**:
   - [ ] Unit tests written and passing
   - [ ] Integration tests written and passing
   - [ ] Test coverage >80% for new code
   - [ ] Manual testing completed

3. **Documentation Complete**:
   - [ ] User-facing docs written
   - [ ] Code comments for complex logic
   - [ ] Example code created and tested
   - [ ] CHANGELOG.md updated

4. **Verification Complete**:
   - [ ] Feature works end-to-end
   - [ ] No regressions in existing features
   - [ ] All tests pass (`cargo test --lib`)
   - [ ] Example apps still work

---

## 🎯 RECOMMENDED EXECUTION PLAN

### **Session 1: Security Middleware (8-12 hours)**

**Steps**:
1. Create `runtime/security.js` (2-3 hours)
2. Implement middleware generation (4-6 hours)
3. Add runtime imports (1-2 hours)
4. Integration testing (2-3 hours)
5. Documentation (1 hour)

**Deliverable**: Working security annotations

---

### **Session 2: Dead Code Elimination (12-16 hours)**

**Steps**:
1. Usage analysis (4-6 hours)
2. DCE implementation (3-4 hours)
3. CLI integration (2-3 hours)
4. Metrics & verification (1-2 hours)
5. Documentation (2 hours)

**Deliverable**: Working `--optimize` flag

---

### **Session 3: Vercel Deployment (8-12 hours)**

**Steps**:
1. Vercel adapter (3-4 hours)
2. CLI deploy command (2-3 hours)
3. Integration testing (2-3 hours)
4. Documentation (1-2 hours)

**Deliverable**: Working `jnc deploy vercel`

---

## 🚀 PHASE 17 COMPLETION CRITERIA

**Phase 17 is DONE when**:

✅ Security middleware generation works end-to-end
✅ Dead code elimination works with `--optimize`
✅ Vercel deployment works with `jnc deploy vercel`
✅ All tests pass (including new tests)
✅ All example apps still work
✅ Documentation complete for all features
✅ CHANGELOG.md updated
✅ Version bumped to v0.9.0

**NO SHORTCUTS. NO COMPROMISES.**

---

## 📝 QUESTIONS FOR USER

Before starting implementation:

1. **Scope**: Should we implement all 3 features, or focus on 1-2?
2. **Priority**: Which feature is most important to you?
3. **Timeline**: Are you okay with 28-40 hours total for complete Phase 17?
4. **Security Runtime**: The in-memory implementation is simple but not production-ready. Should we plan for database-backed auth/rate-limiting later?

---

**This plan follows CLAUDE.md rules. Every feature will be implemented COMPLETELY or not at all.**
