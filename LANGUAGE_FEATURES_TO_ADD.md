# Core Language Features - Potential Additions

**Date**: October 29, 2025
**Current Version**: v0.27.0
**Status**: Brainstorming next language features

---

## ✅ ALREADY IMPLEMENTED

**Modern JavaScript Features**:
- ✅ Async/await (functions and lambdas)
- ✅ Template literals with interpolation
- ✅ Object spread `{...obj}`
- ✅ Array spread `[...arr]`
- ✅ Destructuring (array and object patterns)
- ✅ Lambda expressions (both forms: `() => expr` and `|x| expr`)
- ✅ Arrow functions with async support
- ✅ Ternary operators
- ✅ Try operator (`?`) for error propagation

**Rust-Inspired Features**:
- ✅ Pattern matching (`match` expressions)
- ✅ Lifetimes (`'a`, `'b`, `'static`)
- ✅ Type parameters/generics (`<T>`, `<T: Display>`)
- ✅ Traits and impl blocks
- ✅ Borrow checker semantics
- ✅ References (`&x`, `&mut x`, `*x`)
- ✅ Ranges (`start..end`, `start..=end`)
- ✅ Loops (`loop`, `while`, `for`, `for-in`)
- ✅ Structs and enums
- ✅ Unit type `()`
- ✅ Character literals

**Jounce-Specific Features**:
- ✅ Fine-grained reactivity (signals, computed, effects, batch)
- ✅ JSX support everywhere
- ✅ Component system with props
- ✅ Style blocks with scoping
- ✅ Theme blocks
- ✅ `@server` and `@client` decorators
- ✅ Decorators on let statements (`@persist("localStorage")`)
- ✅ Module system (use/export)
- ✅ Inline JavaScript (`script { ... }`)

---

## 🆕 POTENTIAL NEW FEATURES

### Category 1: Enhanced Type System

#### 1. **Union Types** ⭐⭐⭐⭐⭐
**Priority**: High
**Effort**: Medium (2-3 hours)
**Value**: High

**Description**: Allow values to be one of several types.

**Syntax**:
```jounce
type StringOrNumber = string | number;

fn process(value: string | number) -> string {
    match value {
        string => return value,
        number => return value.to_string(),
    }
}
```

**Why**: Improves type safety for flexible functions, common in JavaScript interop.

---

#### 2. **Intersection Types** ⭐⭐⭐⭐
**Priority**: Medium
**Effort**: Medium (2-3 hours)
**Value**: Medium-High

**Description**: Combine multiple types into one.

**Syntax**:
```jounce
type NamedEntity = { name: string } & { id: number };

fn create_entity(data: NamedEntity) -> NamedEntity {
    return data;
}
```

**Why**: Useful for composing types, common in TypeScript.

---

#### 3. **Type Aliases with Generics** ⭐⭐⭐⭐
**Priority**: Medium
**Effort**: Small (1-2 hours)
**Value**: Medium

**Description**: Generic type aliases.

**Syntax**:
```jounce
type Result<T, E> = Ok(T) | Err(E);
type ApiResponse<T> = Result<T, string>;
```

**Why**: Reduces repetition, improves code clarity.

---

#### 4. **Nullable Types (Option\<T\>)** ⭐⭐⭐⭐⭐
**Priority**: High
**Effort**: Medium (3-4 hours)
**Value**: Very High

**Description**: Built-in Option type for null safety (might already be parseable as enum).

**Syntax**:
```jounce
fn find_user(id: number) -> Option<User> {
    if (exists) {
        return Some(user);
    } else {
        return None;
    }
}

match find_user(1) {
    Some(user) => console.log(user.name),
    None => console.log("Not found"),
}
```

**Why**: Eliminates null/undefined bugs, Rust's killer feature.

---

### Category 2: Syntactic Sugar

#### 5. **Optional Chaining (`?.`)** ⭐⭐⭐⭐⭐
**Priority**: Very High
**Effort**: Small (1-2 hours)
**Value**: Very High

**Description**: Safe navigation for nullable values.

**Syntax**:
```jounce
let name = user?.profile?.name;
// Equivalent to:
let name = if (user != null && user.profile != null) {
    user.profile.name
} else {
    null
};
```

**Why**: Extremely common in modern JavaScript, reduces boilerplate.

---

#### 6. **Nullish Coalescing (`??`)** ⭐⭐⭐⭐⭐
**Priority**: Very High
**Effort**: Small (30 min - 1 hour)
**Value**: High

**Description**: Default value for null/undefined.

**Syntax**:
```jounce
let name = user.name ?? "Anonymous";
// Only uses default if left is null/undefined (not "", 0, false)
```

**Why**: Pairs with optional chaining, very common pattern.

---

#### 7. **Logical Assignment Operators** ⭐⭐⭐
**Priority**: Low-Medium
**Effort**: Small (1 hour)
**Value**: Medium

**Description**: Shorthand for logical assignments.

**Syntax**:
```jounce
x ||= 10;  // x = x || 10
y &&= 5;   // y = y && 5
z ??= 0;   // z = z ?? 0
```

**Why**: Common JavaScript pattern, reduces verbosity.

---

#### 8. **Pipeline Operator (`|>`)** ⭐⭐⭐⭐
**Priority**: Medium
**Effort**: Medium (2-3 hours)
**Value**: High

**Description**: Function chaining without nesting.

**Syntax**:
```jounce
let result = data
    |> parse
    |> validate
    |> save
    |> respond;

// Instead of:
let result = respond(save(validate(parse(data))));
```

**Why**: Improves readability for function chains.

---

### Category 3: Advanced Features

#### 9. **Decorators for Functions/Components** ⭐⭐⭐⭐
**Priority**: Medium-High
**Effort**: Medium-Large (4-6 hours)
**Value**: High

**Description**: Extend decorators to functions and components.

**Syntax**:
```jounce
@cache(ttl: 60)
@server
fn get_users() -> Vec<User> {
    return db.query("SELECT * FROM users");
}

@memo
component ExpensiveComponent(data: Vec<T>) {
    // Component body
}
```

**Why**: Enable powerful metaprogramming, caching, memoization.

---

#### 10. **Generator Functions** ⭐⭐⭐
**Priority**: Low-Medium
**Effort**: Large (8-12 hours)
**Value**: Medium

**Description**: Functions that can yield values.

**Syntax**:
```jounce
fn* count() {
    let i = 0;
    loop {
        yield i;
        i = i + 1;
    }
}

for num in count() {
    if (num > 10) { break; }
    console.log(num);
}
```

**Why**: Useful for iterators, async data streams.

---

#### 11. **Async Iterators** ⭐⭐⭐
**Priority**: Low-Medium
**Effort**: Large (8-12 hours)
**Value**: Medium

**Description**: Async version of iterators.

**Syntax**:
```jounce
async fn* fetch_pages(url: string) {
    let page = 1;
    loop {
        let data = await fetch(`${url}?page=${page}`);
        if (data.items.length == 0) { break; }
        yield data.items;
        page = page + 1;
    }
}

for await (items in fetch_pages("/api/users")) {
    process(items);
}
```

**Why**: Great for paginated APIs, streaming data.

---

### Category 4: Error Handling

#### 12. **Try/Catch Blocks** ⭐⭐⭐⭐⭐
**Priority**: Very High
**Effort**: Medium (3-4 hours)
**Value**: Very High

**Description**: Traditional exception handling (complement to Result<T, E>).

**Syntax**:
```jounce
fn risky_operation() {
    try {
        let result = might_fail();
        process(result);
    } catch (error) {
        console.log("Error:", error);
    } finally {
        cleanup();
    }
}
```

**Why**: Essential for interop with JavaScript, familiar pattern.

---

#### 13. **Custom Error Types** ⭐⭐⭐⭐
**Priority**: Medium
**Effort**: Small (1-2 hours if enums already work)
**Value**: Medium-High

**Description**: Define error types with error! macro or enum.

**Syntax**:
```jounce
enum ApiError {
    NotFound(string),
    Unauthorized,
    ServerError(number),
}

fn fetch_data() -> Result<Data, ApiError> {
    if (!authenticated) {
        return Err(ApiError::Unauthorized);
    }
    // ...
}
```

**Why**: Better error handling, type-safe error propagation.

---

### Category 5: Pattern Matching Enhancements

#### 14. **If-Let Syntax** ⭐⭐⭐⭐
**Priority**: Medium-High
**Effort**: Small (1-2 hours)
**Value**: High

**Description**: Shorthand for matching single pattern.

**Syntax**:
```jounce
if let Some(user) = find_user(id) {
    console.log(user.name);
}

// Instead of:
match find_user(id) {
    Some(user) => console.log(user.name),
    None => {},
}
```

**Why**: Common Rust pattern, improves ergonomics.

---

#### 15. **While-Let Syntax** ⭐⭐⭐
**Priority**: Low-Medium
**Effort**: Small (1-2 hours)
**Value**: Medium

**Description**: Loop while pattern matches.

**Syntax**:
```jounce
while let Some(item) = iterator.next() {
    process(item);
}
```

**Why**: Clean iterator consumption pattern.

---

### Category 6: Object-Oriented Features

#### 16. **Classes (Sugar over Structs + Impl)** ⭐⭐⭐⭐
**Priority**: Medium
**Effort**: Medium-Large (6-8 hours)
**Value**: High

**Description**: Traditional class syntax (compiles to struct + impl).

**Syntax**:
```jounce
class User {
    private id: number;
    public name: string;

    constructor(id: number, name: string) {
        this.id = id;
        this.name = name;
    }

    fn greet() -> string {
        return `Hello, ${this.name}`;
    }
}

let user = User::new(1, "Alice");
console.log(user.greet());
```

**Why**: Familiar to JavaScript/TypeScript developers, syntactic sugar.

---

#### 17. **Private Fields** ⭐⭐⭐
**Priority**: Low-Medium
**Effort**: Medium (3-4 hours)
**Value**: Medium

**Description**: Enforce private access to struct fields.

**Syntax**:
```jounce
struct User {
    pub id: number,
    private email: string,  // Only accessible within impl block
}
```

**Why**: Encapsulation, better API design.

---

### Category 7: Compilation and Metaprogramming

#### 18. **Compile-Time Constants** ⭐⭐⭐⭐
**Priority**: Medium
**Effort**: Medium (3-4 hours)
**Value**: Medium-High

**Description**: Values computed at compile time.

**Syntax**:
```jounce
const VERSION: string = env!("CARGO_PKG_VERSION");
const BUILD_TIME: string = build_timestamp!();
const IS_PROD: bool = cfg!(production);
```

**Why**: Environment configuration, build info.

---

#### 19. **Conditional Compilation** ⭐⭐⭐⭐
**Priority**: Medium
**Effort**: Medium-Large (4-6 hours)
**Value**: High

**Description**: Include/exclude code based on conditions.

**Syntax**:
```jounce
#[cfg(target = "browser")]
fn init() {
    // Browser-only code
}

#[cfg(feature = "debug")]
fn debug_log(msg: string) {
    console.log(msg);
}
```

**Why**: Feature flags, platform-specific code.

---

#### 20. **Macros (Declarative)** ⭐⭐⭐
**Priority**: Low-Medium
**Effort**: Very Large (16-24 hours)
**Value**: High (long-term)

**Description**: User-defined macros for code generation.

**Syntax**:
```jounce
macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    };
}
```

**Why**: Powerful metaprogramming, reduce boilerplate.

---

## 🎯 RECOMMENDED PRIORITIES (Session 25+)

### Quick Wins (1-2 hours each)
1. ⭐⭐⭐⭐⭐ **Nullish Coalescing (`??`)** - 30 min to 1 hour
2. ⭐⭐⭐⭐⭐ **Optional Chaining (`?.`)** - 1-2 hours
3. ⭐⭐⭐⭐ **If-Let Syntax** - 1-2 hours
4. ⭐⭐⭐ **Logical Assignment (`||=`, `&&=`, `??=`)** - 1 hour

### High-Value Features (2-6 hours)
5. ⭐⭐⭐⭐⭐ **Union Types** - 2-3 hours
6. ⭐⭐⭐⭐⭐ **Try/Catch Blocks** - 3-4 hours
7. ⭐⭐⭐⭐ **Decorators for Functions** - 4-6 hours
8. ⭐⭐⭐⭐ **Pipeline Operator** - 2-3 hours

### Long-Term Features (8+ hours)
9. ⭐⭐⭐⭐ **Classes** - 6-8 hours
10. ⭐⭐⭐ **Generator Functions** - 8-12 hours
11. ⭐⭐⭐ **Macros** - 16-24 hours

---

## 💡 SESSION 25 SUGGESTION

**Start with the "Quick Wins"** to deliver immediate value:

### Session 25 Plan: "Modern JavaScript Operators"
**Duration**: 3-4 hours
**Goal**: Add 3 highly-requested JavaScript operators

**Tasks**:
1. ✅ **Nullish Coalescing (`??`)** (30-60 min)
   - Add `TokenKind::QuestionQuestion`
   - Add to lexer
   - Add to parser (infix operator, low precedence)
   - Add to JS emitter
   - Test: `let x = y ?? 0;`

2. ✅ **Optional Chaining (`?.`)** (1-2 hours)
   - Add `TokenKind::QuestionDot`
   - Add to lexer
   - Add to parser (postfix operator like field access)
   - Add to JS emitter
   - Test: `let name = user?.profile?.name;`

3. ✅ **Logical Assignment Operators** (1 hour)
   - Add `TokenKind::PipePipeEqual`, `AmpersandAmpersandEqual`, `QuestionQuestionEqual`
   - Add to lexer
   - Add to parser (assignment statement)
   - Add to JS emitter
   - Test: `x ||= 10; y &&= 5; z ??= 0;`

**Impact**: These 3 features are extremely common in modern JavaScript and would significantly improve developer experience.

---

## 📊 FEATURE MATRIX

| Feature | Priority | Effort | Value | Status |
|---------|----------|--------|-------|--------|
| Nullish Coalescing | ⭐⭐⭐⭐⭐ | Small | Very High | ⏸️ Pending |
| Optional Chaining | ⭐⭐⭐⭐⭐ | Small | Very High | ⏸️ Pending |
| Try/Catch | ⭐⭐⭐⭐⭐ | Medium | Very High | ⏸️ Pending |
| Union Types | ⭐⭐⭐⭐⭐ | Medium | High | ⏸️ Pending |
| If-Let | ⭐⭐⭐⭐ | Small | High | ⏸️ Pending |
| Decorators | ⭐⭐⭐⭐ | Medium | High | ⏸️ Pending |
| Pipeline Operator | ⭐⭐⭐⭐ | Medium | High | ⏸️ Pending |
| Classes | ⭐⭐⭐⭐ | Large | High | ⏸️ Pending |
| Logical Assignment | ⭐⭐⭐ | Small | Medium | ⏸️ Pending |
| While-Let | ⭐⭐⭐ | Small | Medium | ⏸️ Pending |
| Generator Functions | ⭐⭐⭐ | Very Large | Medium | ⏸️ Pending |
| Macros | ⭐⭐⭐ | Very Large | High | ⏸️ Pending |

---

**Document Created**: October 29, 2025
**Purpose**: Identify potential language features for future development
**Next Step**: Choose a feature and implement it!
