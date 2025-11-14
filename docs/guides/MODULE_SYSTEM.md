# Module System Guide

**Version**: v0.8.3
**Last Updated**: November 7, 2025

Jounce supports multi-file projects with a simple and intuitive module system. You can split your code into multiple files and import functions, structs, enums, and other definitions across files.

> **Technical Reference**: See [JOUNCE_SPEC.md § Module System](../../JOUNCE_SPEC.md) for complete grammar and execution rules.

---

## Table of Contents

- [Quick Start](#quick-start)
- [Import Syntax](#import-syntax)
- [Import Aliasing (New in v0.8.3)](#import-aliasing-new-in-v083)
- [Visibility Control](#visibility-control)
- [Module Resolution](#module-resolution)
- [Best Practices](#best-practices)
- [Examples](#examples)
- [Troubleshooting](#troubleshooting)

---

## Quick Start

### Creating a Module

Any `.jnc` file is automatically a module. Use the `pub` keyword to export definitions.

**math.jnc**:
```jounce
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

pub fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

// Private function (not exported)
fn internal_helper() -> i32 {
    return 42;
}
```

### Importing a Module

Use the `use` statement to import another file:

**main.jnc**:
```jounce
use ./math;

fn main() {
    let result = add(5, 3);
    console.log(result.to_string());
}
```

---

## Import Syntax

### Relative Path Imports

Jounce supports relative path imports using `./` (current directory) and `../` (parent directory):

```jounce
// Import from current directory
use ./math;

// Import from parent directory
use ../utils;

// Import from nested directories
use ./models/user;
use ../lib/helpers;
```

### Selective Imports

Import specific items from a module:

```jounce
use ./math::{add, multiply};
use ./types::{User, Post, Comment};

fn main() {
    let sum = add(5, 3);  // Direct access
}
```

### Wildcard Imports

Import all public items (use sparingly):

```jounce
use ./math::*;

fn main() {
    let result = add(10, 20);
}
```

---

## Import Aliasing (New in v0.8.3)

Rename imports to avoid naming conflicts:

### Basic Aliasing

```jounce
use ./widgets::{Button as WidgetButton};
use ./ui::{Button as UIButton};

component App() {
    return <div>
        <WidgetButton label="Click me" />
        <UIButton variant="primary" />
    </div>;
}
```

### Multiple Aliases

```jounce
use ./types::{
    User as UserType,
    Post as PostType,
    Comment as CommentType
};

fn process_user(u: UserType) -> PostType {
    // ...
}
```

### Aliasing with Namespaces

```jounce
use ./auth as AuthModule;
use ./database as DB;

fn main() {
    AuthModule::login("user", "pass");
    DB::connect("localhost");
}
```

---

## Visibility Control

### Public Items (v0.8.3+)

Use the `pub` keyword to export definitions:

```jounce
// Public - available to importers
pub fn public_function() -> i32 {
    return 42;
}

pub struct User {
    pub name: string,
    pub email: string,
}

// Private - only within this module
fn private_helper() -> i32 {
    return 10;
}
```

### Module Exports

Only `pub` items can be imported:

**utils.jnc**:
```jounce
pub fn format_date(timestamp: i64) -> string {
    return "2025-11-07";
}

fn internal_parse(raw: string) -> i64 {
    // Private - cannot be imported
    return 0;
}
```

**main.jnc**:
```jounce
use ./utils::{format_date};

fn main() {
    format_date(12345);  // ✅ Works
    // internal_parse("test");  // ❌ Error: not exported
}
```

---

## Module Resolution

### How Jounce Finds Modules

When you write `use ./math`, Jounce looks for:
1. `./math.jnc` in the current directory
2. If not found, returns error "Module not found"

When you write `use jounce_http::HttpClient`, Jounce searches:
1. Local packages in `jounce_modules/`
2. Registry packages (if installed via `jnc pkg add`)

### File Extensions

Always use `.jnc` extensions for source files, but **omit** the extension in import statements:

```jounce
// ✅ Correct
use ./math;
use ./models/user;

// ❌ Incorrect
use ./math.jnc;
use ./models/user.jnc;
```

---

## Best Practices

### 1. Organize by Feature

Structure your project around features or domains:

```
my-app/
├── main.jnc
├── models/
│   ├── user.jnc
│   ├── post.jnc
│   └── comment.jnc
├── services/
│   ├── auth.jnc
│   └── api.jnc
└── utils/
    ├── validation.jnc
    └── formatting.jnc
```

### 2. Use Import Aliasing for Conflicts

When two modules export the same name:

```jounce
use ./widgets::{Button as WidgetButton};
use ./components::{Button as CompButton};

component App() {
    return <div>
        <WidgetButton />
        <CompButton />
    </div>;
}
```

### 3. Shared Types File

Create a `types.jnc` file for shared structs and enums:

**types.jnc**:
```jounce
pub struct User {
    id: i32,
    name: string,
    email: string,
}

pub enum Role {
    Admin,
    User,
    Guest,
}
```

**auth.jnc**:
```jounce
use ./types::{User, Role};

pub fn create_user(name: string, email: string) -> User {
    return User {
        id: generate_id(),
        name: name,
        email: email,
    };
}
```

### 4. Avoid Circular Dependencies

Don't create circular imports:

```jounce
// ❌ AVOID THIS

// a.jnc
use ./b;

// b.jnc
use ./a;  // Circular dependency!
```

Instead, extract shared code to a third file:

```jounce
// ✅ BETTER

// shared.jnc
pub struct SharedData { /* ... */ }

// a.jnc
use ./shared;

// b.jnc
use ./shared;
```

### 5. Export Only What's Needed

Keep implementation details private:

```jounce
// Public API
pub fn calculate_total(items: Vec<Item>) -> f64 {
    return items.map(item_price).sum();
}

// Private helper
fn item_price(item: Item) -> f64 {
    return item.price * item.quantity;
}
```

---

## Examples

### Example 1: Simple Calculator

**calculator/math.jnc**:
```jounce
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

pub fn subtract(a: i32, b: i32) -> i32 {
    return a - b;
}

pub fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

pub fn divide(a: i32, b: i32) -> i32 {
    return a / b;
}
```

**calculator/main.jnc**:
```jounce
use ./math::{add, subtract, multiply, divide};

fn main() {
    console.log("10 + 5 = " + add(10, 5).to_string());
    console.log("10 - 5 = " + subtract(10, 5).to_string());
    console.log("10 * 5 = " + multiply(10, 5).to_string());
    console.log("10 / 5 = " + divide(10, 5).to_string());
}
```

### Example 2: Multi-Module Todo App

**Project Structure**:
```
todo-app/
├── main.jnc       # Entry point
├── types.jnc      # Shared types
├── utils.jnc      # Utility functions
└── storage.jnc    # Storage layer
```

**types.jnc**:
```jounce
pub struct Todo {
    id: i32,
    title: string,
    completed: bool,
}

pub enum Filter {
    All,
    Active,
    Completed,
}

pub fn create_todo(id: i32, title: string) -> Todo {
    return Todo {
        id: id,
        title: title,
        completed: false,
    };
}
```

**utils.jnc**:
```jounce
pub fn format_count(count: i32) -> string {
    if count == 1 {
        return "1 item";
    }
    return count.to_string() + " items";
}

pub fn validate_title(title: string) -> bool {
    return title.length() > 0;
}
```

**storage.jnc**:
```jounce
use ./types::{Todo, create_todo};

pub fn get_sample_todo() -> Todo {
    return create_todo(1, "Sample task");
}

pub fn format_todo(todo: Todo) -> string {
    let status = if todo.completed { "[x]" } else { "[ ]" };
    return status + " " + todo.title;
}
```

**main.jnc**:
```jounce
use ./types::{create_todo};
use ./utils::{format_count};
use ./storage::{format_todo};

fn main() {
    console.log("=== Todo App ===");

    let todo = create_todo(1, "Learn Jounce");
    console.log(format_todo(todo));

    console.log(format_count(2));
}
```

### Example 3: Import Aliasing

**main.jnc**:
```jounce
use ./widgets::{Button as WidgetButton, Card as WidgetCard};
use ./ui::{Button as UIButton};
use ./types::{User as UserType};

component App() {
    let user: UserType = get_current_user();

    return <div>
        <WidgetCard>
            <WidgetButton label="Widget" />
            <UIButton variant="primary" label="UI" />
        </WidgetCard>
    </div>;
}
```

---

## Troubleshooting

### Error: "Module not found"

**Problem**: Jounce can't find the module you're trying to import.

**Solutions**:
1. Check the file exists at the path you specified
2. Verify the file has a `.jnc` extension
3. Make sure you're using the correct relative path (`./` or `../`)
4. Check for typos in the module name
5. Ensure you omitted the `.jnc` extension in the import

### Error: "Item not exported"

**Problem**: The function/struct you're importing isn't marked `pub`.

**Solution**: Add `pub` keyword to the definition:
```jounce
// In the module file
pub fn my_function() { }  // Now exportable
```

### Error: "Circular dependency detected"

**Problem**: Two modules import each other.

**Solution**: Extract shared code to a third module that both can import.

### Name Conflicts

**Problem**: Two modules export the same name.

**Solution**: Use import aliasing:
```jounce
use ./moduleA::{Item as ItemA};
use ./moduleB::{Item as ItemB};
```

---

## What's Next?

- **Complete Tutorial**: See [LEARN_JOUNCE.md](./LEARN_JOUNCE.md) for practical examples
- **Technical Details**: See [JOUNCE_SPEC.md § Module System](../../JOUNCE_SPEC.md) for complete grammar
- **Package System**: See [PACKAGE_MANAGER_GUIDE.md](./PACKAGE_MANAGER_GUIDE.md) for using external packages

---

**Version**: v0.8.3 "Enhanced Language Features"
**Status**: ✅ Production Ready (580/580 tests passing)
**Features**: Relative imports, selective imports, import aliasing, `pub` keyword

---

**Maintained by: The Jounce Project**
