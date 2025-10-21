# RavensOne Parser Limitations

This document outlines the current state of the RavensOne parser, what works, what doesn't, and what would be needed to implement full JSX support.

## Current State: What Works ✅

The RavensOne compiler successfully parses and compiles **non-JSX RavensOne code**. This includes:

- Basic data structures (structs, enums)
- Functions and methods
- Pattern matching
- Variable declarations
- Basic expressions
- Control flow (if/else, loops)

### Example: Compiles Successfully

```rust
// examples/enum_simple.raven
enum Result {
    Ok(i32),
    Err(String),
}

fn process(value: i32) -> Result {
    if value > 0 {
        return Result::Ok(value);
    } else {
        return Result::Err("Invalid value");
    }
}

fn main() {
    let result = process(42);
    match result {
        Result::Ok(n) => println!("Success: {}", n),
        Result::Err(e) => println!("Error: {}", e),
    }
}
```

**Compilation**: `./target/release/raven compile examples/enum_simple.raven` → ✅ SUCCESS

## What Doesn't Work ❌

### 1. JSX/Component Syntax (Not Implemented)

The parser does not support JSX-like component syntax. All `.raven` files with JSX components fail to compile.

**Error**: `ParserError: No prefix parse function for Slash`

**Affected Code**:
```rust
// This does NOT compile
component Button(props: ButtonProps) {
    return <button class="btn">
        {props.label}
    </button>;
}
```

**Files that fail**:
- `examples/analytics_dashboard.raven`
- `examples/todo_app.raven`
- `examples/counter-v2.raven`
- `examples/devboard/src/main.raven`
- All other component-based examples

### 2. Lexer Limitations

#### a. Hash Symbol (`#`)
**Error**: `Lexer Error: Illegal character: '#'`

```rust
// Does NOT work
#[server]
fn get_data() -> Vec<String> { }

// Use instead
@server
fn get_data() -> Vec<String> { }
```

#### b. Emoji Characters
**Error**: `Lexer Error: Illegal character: '⭐'`

```rust
// Does NOT work
let icon = "⭐";

// Use instead
let icon = "star";
```

#### c. Template Literals (Backticks)
**Error**: `Lexer Error: Illegal character: '\`'`

```rust
// Does NOT work
let html = `<div>${name}</div>`;

// Use instead
let html = "<div>" + name + "</div>";
```

### 3. Method Chaining
**Error**: `ParserError: No prefix parse function for Dot`

```rust
// Does NOT work
if message.name.trim().is_empty() {
    return Err("Name required");
}

// Use instead
if message.name == "" {
    return Err("Name required");
}
```

### 4. String Methods
**Error**: Parser fails on method calls like `.contains()`, `.to_string()`

```rust
// Does NOT work
if email.contains("@") {
    // valid email
}

// String literals don't need .to_string()
let name = "John".to_string(); // Unnecessary

// Use instead
let name = "John"; // Already a String
```

### 5. JSX Comments
**Error**: `ParserError: No prefix parse function for Slash`

```rust
// Does NOT work
<div>
    {/* This is a comment */}
    <h1>Hello</h1>
</div>

// Use instead - remove JSX comments entirely
<div>
    <h1>Hello</h1>
</div>
```

## Technical Requirements for JSX Support

To enable JSX/component syntax in RavensOne, the following compiler work is needed:

### 1. Lexer Updates

**File**: `src/lexer.rs`

Add support for:
- JSX opening tags: `<div`
- JSX closing tags: `</div>`
- JSX self-closing tags: `<img />`
- JSX attributes: `class="foo" onclick={handler}`
- JSX children: nested elements and text
- JSX expressions: `{variable}`, `{expr + 1}`
- JSX comments: `{/* comment */}`

**New Token Types Needed**:
```rust
pub enum Token {
    // ... existing tokens

    // JSX tokens
    LessThan,           // <
    GreaterThan,        // >
    Slash,              // /
    JsxText(String),    // Text content between tags
    JsxOpen,            // < (in JSX context)
    JsxClose,           // >
    JsxSelfClose,       // />
}
```

### 2. Parser Updates

**File**: `src/parser.rs`

Implement JSX parsing functions:

```rust
// Parse JSX element
fn parse_jsx_element(&mut self) -> Result<Expression, ParserError> {
    // <TagName attributes>children</TagName>
}

// Parse JSX attributes
fn parse_jsx_attributes(&mut self) -> Result<Vec<JsxAttribute>, ParserError> {
    // class="value" onclick={handler}
}

// Parse JSX children
fn parse_jsx_children(&mut self) -> Result<Vec<JsxChild>, ParserError> {
    // Mix of elements, text, and expressions
}

// Parse JSX expressions
fn parse_jsx_expression(&mut self) -> Result<Expression, ParserError> {
    // {variable} or {expr + 1}
}
```

**New AST Node Types Needed**:
```rust
pub enum Expression {
    // ... existing variants

    JsxElement {
        tag: String,
        attributes: Vec<JsxAttribute>,
        children: Vec<JsxChild>,
    },
    JsxFragment {
        children: Vec<JsxChild>,
    },
}

pub struct JsxAttribute {
    name: String,
    value: JsxAttributeValue,
}

pub enum JsxAttributeValue {
    String(String),
    Expression(Box<Expression>),
}

pub enum JsxChild {
    Element(Box<Expression>),
    Text(String),
    Expression(Box<Expression>),
}
```

### 3. Code Generation Updates

**File**: `src/codegen.rs`

Add JSX-to-JavaScript/WASM code generation:

```rust
fn generate_jsx_element(&mut self, element: &JsxElement) -> String {
    // Generate DOM creation code
    // createElement(), setAttribute(), appendChild(), etc.
}

fn generate_jsx_to_dom(&mut self, jsx: &Expression) -> String {
    match jsx {
        Expression::JsxElement { tag, attributes, children } => {
            // Generate: document.createElement(tag)
            // For each attribute: element.setAttribute(name, value)
            // For each child: element.appendChild(child)
        }
        _ => // handle other cases
    }
}
```

### 4. Runtime Library Updates

**File**: `dist/client-runtime.js`

Add helper functions for JSX:

```javascript
export function createElement(tag, props, ...children) {
    const element = document.createElement(tag);

    // Set attributes
    if (props) {
        for (const [key, value] of Object.entries(props)) {
            if (key.startsWith('on')) {
                // Event handler
                element.addEventListener(key.slice(2).toLowerCase(), value);
            } else {
                element.setAttribute(key, value);
            }
        }
    }

    // Append children
    for (const child of children) {
        if (typeof child === 'string') {
            element.appendChild(document.createTextNode(child));
        } else {
            element.appendChild(child);
        }
    }

    return element;
}
```

## Development Roadmap

### Phase 1: Lexer Support (2-3 days)
- [ ] Add JSX token types
- [ ] Implement JSX tokenization
- [ ] Handle context switching (code vs JSX)
- [ ] Add tests for JSX lexing

### Phase 2: Parser Support (3-5 days)
- [ ] Create JSX AST node types
- [ ] Implement JSX element parsing
- [ ] Implement JSX attribute parsing
- [ ] Implement JSX children parsing
- [ ] Implement JSX expression parsing
- [ ] Add tests for JSX parsing

### Phase 3: Code Generation (2-3 days)
- [ ] Implement JSX-to-DOM generation
- [ ] Handle event handlers
- [ ] Handle reactive expressions
- [ ] Add tests for code generation

### Phase 4: Runtime Support (1-2 days)
- [ ] Create JSX runtime helpers
- [ ] Implement component mounting
- [ ] Implement reactive updates
- [ ] Add tests for runtime

### Phase 5: Integration Testing (2-3 days)
- [ ] Test with DevBoard example
- [ ] Test with Analytics Dashboard
- [ ] Test with Todo App
- [ ] Performance optimization
- [ ] Documentation

**Total Estimated Time**: 10-16 days of development

## Workarounds Until JSX is Implemented

Until JSX support is added, use these patterns:

### 1. DOM Builder Functions

```rust
fn create_button(label: String, onclick: fn()) -> Element {
    let button = document.createElement("button");
    button.setAttribute("class", "btn");
    button.textContent = label;
    button.addEventListener("click", onclick);
    return button;
}
```

### 2. Template Strings with String Concatenation

```rust
fn render_card(title: String, content: String) -> String {
    let html = "<div class=\"card\">" +
        "<h3>" + title + "</h3>" +
        "<p>" + content + "</p>" +
        "</div>";
    return html;
}
```

### 3. Direct DOM Manipulation

```rust
fn main() {
    let app = document.getElementById("app");
    let heading = document.createElement("h1");
    heading.textContent = "Welcome to RavensOne";
    app.appendChild(heading);
}
```

## References

- **Working Example**: `examples/enum_simple.raven` - Basic RavensOne syntax without JSX
- **Aspirational Examples**: All files in `examples/` with JSX syntax
- **DevBoard App**: `examples/devboard/src/main.raven` - Full application showing intended syntax

## Testing JSX Implementation

When JSX support is added, test with this minimal example:

```rust
component Hello(name: String) {
    return <div>
        <h1>Hello, {name}!</h1>
        <p>Welcome to RavensOne</p>
    </div>;
}

fn main() {
    let app = Hello("World");
    mount(app, "#app");
}
```

**Expected Output**: DOM tree with `<div><h1>Hello, World!</h1><p>Welcome to RavensOne</p></div>`

---

**Document Status**: Current as of 2025-10-20
**RavensOne Version**: Checked against commit f9a1db5
