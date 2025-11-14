# Reactive Automation Design

> **Note**: For authoritative language rules, see [JOUNCE_SPEC.md](../../JOUNCE_SPEC.md).

**Feature**: Automatic generation of reactive setup code
**Status**: Design phase
**Target**: v0.9.0

---

## Problem Statement

Currently, developers must manually write reactive setup code in `dist/client.js` after compilation. This is repetitive and error-prone.

**Current Manual Pattern:**
```javascript
window.addEventListener('DOMContentLoaded', () => {
    mountComponent(Counter);

    const count = signal(0);
    const display = document.getElementById('count');
    const btn = document.getElementById('inc-btn');

    btn.addEventListener('click', () => {
        count.value = count.value + 1;
    });

    effect(() => {
        if (display) {
            display.textContent = count.value;
        }
    });
});
```

**Goal**: Compiler generates this code automatically from Jounce source.

---

## Proposed Syntax

### 1. Reactive Declarations in Components

```jounce
fn Counter() -> JSX {
    // Declare reactive state inside component
    let count = signal(0);
    let doubled = computed(|| count.value * 2);

    return (
        <div>
            <div id="count">{count}</div>
            <button id="inc-btn" onClick={() => count.value++}>+</button>
        </div>
    );
}
```

**Key Features:**
- `signal(initialValue)` creates reactive state
- `computed(|| expr)` creates derived values
- Available only inside component functions
- Scoped to component instance

### 2. Event Handlers in JSX

```jounce
<button onClick={() => count.value++}>Click Me</button>
<input onChange={(e) => name.value = e.target.value} />
<input onInput={(e) => search.value = e.target.value} />
<form onSubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
```

**Supported Events:**
- `onClick` - Button/element clicks
- `onChange` - Select, checkbox, radio changes
- `onInput` - Text input (real-time)
- `onSubmit` - Form submission
- `onKeyDown`, `onKeyUp` - Keyboard events
- `onFocus`, `onBlur` - Focus events

**Handler Syntax:**
- Inline arrow function: `onClick={() => doSomething()}`
- Inline with event: `onChange={(e) => name.value = e.target.value}`
- Function reference: `onClick={handleClick}` (function must be defined)

### 3. Text Interpolation

```jounce
<div>{count}</div>                    // Signal value
<div>{doubled}</div>                  // Computed value
<div>Count: {count}, Double: {doubled}</div>  // Multiple interpolations
<span class="count-{count}">Text</span>       // Attribute interpolation
```

**Auto-Generated Effects:**
```javascript
// For: <div id="elem-123">{count}</div>
effect(() => {
    const elem = document.getElementById('elem-123');
    if (elem) {
        elem.textContent = count.value;
    }
});
```

### 4. Two-Way Binding (Future)

```jounce
<input type="text" bind:value={name} />
<input type="checkbox" bind:checked={isActive} />
<select bind:value={selectedOption}>
```

**Generates:**
```javascript
// Get + Set
nameInput.value = name.value;
nameInput.addEventListener('input', (e) => {
    name.value = e.target.value;
});

// Effect for updates
effect(() => {
    nameInput.value = name.value;
});
```

---

## Implementation Plan

### Phase 1: Lexer Changes
**File**: `src/lexer.rs`

Add keywords:
- `signal` - Create reactive state
- `computed` - Create derived value
- `effect` - Create side effect (explicit)
- `batch` - Batch updates (explicit)

No changes to existing tokens needed for event handlers (use existing identifiers).

### Phase 2: Parser Changes
**File**: `src/parser.rs`

**2.1 Parse Reactive Declarations**
```rust
// In parse_variable_declaration()
if self.check(&TokenKind::Identifier("signal".to_string())) {
    self.advance(); // consume 'signal'
    self.consume(TokenKind::LParen, "Expected '(' after 'signal'")?;
    let initial_value = self.parse_expression()?;
    self.consume(TokenKind::RParen, "Expected ')' after signal value")?;

    // Create SignalDeclaration node
    return Ok(Statement::SignalDeclaration {
        name: var_name,
        initial_value,
    });
}
```

**2.2 Parse Event Handlers in JSX**
```rust
// In parse_jsx_attribute()
if attr_name.starts_with("on") && attr_name.len() > 2 {
    // onClick, onChange, etc.
    self.consume(TokenKind::Equal, "Expected '=' after event handler name")?;
    self.consume(TokenKind::LBrace, "Expected '{' for event handler")?;

    // Parse handler expression (arrow function or reference)
    let handler = self.parse_expression()?;

    self.consume(TokenKind::RBrace, "Expected '}' after handler")?;

    return Ok(JSXAttribute::EventHandler {
        event: attr_name[2..].to_lowercase(), // onClick -> click
        handler,
    });
}
```

**2.3 Parse Text Interpolation**
```rust
// In parse_jsx_children()
if self.check(&TokenKind::LBrace) {
    self.advance();
    let expression = self.parse_expression()?;
    self.consume(TokenKind::RBrace, "Expected '}' after interpolation")?;

    return Ok(JSXChild::Interpolation {
        expression,
    });
}
```

### Phase 3: AST Changes
**File**: `src/ast.rs`

Add new node types:
```rust
pub enum Statement {
    // Existing variants...

    SignalDeclaration {
        name: String,
        initial_value: Expression,
    },
    ComputedDeclaration {
        name: String,
        computation: Expression, // Arrow function
    },
    EffectDeclaration {
        body: Expression, // Arrow function
    },
}

pub enum JSXAttribute {
    // Existing variants...

    EventHandler {
        event: String,        // "click", "change", "input"
        handler: Expression,  // Arrow function or identifier
    },
}

pub enum JSXChild {
    // Existing variants...

    Interpolation {
        expression: Expression,
    },
}
```

### Phase 4: Code Generation
**File**: `src/js_emitter.rs`

**4.1 Track Component State**
```rust
struct ComponentContext {
    signals: Vec<SignalInfo>,
    computed_values: Vec<ComputedInfo>,
    event_handlers: Vec<EventHandlerInfo>,
    interpolations: Vec<InterpolationInfo>,
    next_element_id: usize,
}

struct SignalInfo {
    name: String,
    initial_value: String,
}

struct EventHandlerInfo {
    element_id: String,
    event: String,      // "click", "change"
    handler: String,    // JavaScript code
}

struct InterpolationInfo {
    element_id: String,
    signal_name: String,
}
```

**4.2 Generate DOMContentLoaded Wrapper**
```rust
fn emit_component_with_reactivity(
    &mut self,
    component: &ComponentDeclaration,
    ctx: &ComponentContext,
) -> String {
    let mut code = String::new();

    // Generate DOMContentLoaded wrapper
    code.push_str("window.addEventListener('DOMContentLoaded', () => {\n");
    code.push_str("  console.log('Jounce client initialized');\n\n");

    // 1. Mount component
    code.push_str(&format!("  mountComponent({});\n\n", component.name));

    // 2. Create signals
    code.push_str("  // Reactive state\n");
    for signal in &ctx.signals {
        code.push_str(&format!(
            "  const {} = signal({});\n",
            signal.name, signal.initial_value
        ));
    }
    code.push_str("\n");

    // 3. Get DOM elements
    code.push_str("  // DOM elements\n");
    let mut element_ids = std::collections::HashSet::new();

    for handler in &ctx.event_handlers {
        element_ids.insert(&handler.element_id);
    }
    for interp in &ctx.interpolations {
        element_ids.insert(&interp.element_id);
    }

    for elem_id in element_ids {
        code.push_str(&format!(
            "  const elem_{} = document.getElementById('{}');\n",
            elem_id.replace("-", "_"), elem_id
        ));
    }
    code.push_str("\n");

    // 4. Event listeners
    code.push_str("  // Event handlers\n");
    for handler in &ctx.event_handlers {
        code.push_str(&format!(
            "  elem_{}.addEventListener('{}', {});\n",
            handler.element_id.replace("-", "_"),
            handler.event,
            handler.handler
        ));
    }
    code.push_str("\n");

    // 5. Effects
    code.push_str("  // Effects\n");
    for interp in &ctx.interpolations {
        code.push_str(&format!(
            "  effect(() => {{\n\
             "    if (elem_{}) {{\n\
             "      elem_{}.textContent = {}.value;\n\
             "    }}\n\
             "  }});\n",
            interp.element_id.replace("-", "_"),
            interp.element_id.replace("-", "_"),
            interp.signal_name
        ));
    }

    code.push_str("\n  console.log('✅ App initialized!');\n");
    code.push_str("});\n");

    code
}
```

**4.3 Auto-Generate Element IDs**
```rust
fn ensure_element_id(&mut self, element: &mut JSXElement, ctx: &mut ComponentContext) -> String {
    // If element already has an id, use it
    if let Some(id) = element.get_attribute("id") {
        return id;
    }

    // Otherwise, generate one
    let generated_id = format!("elem-{}", ctx.next_element_id);
    ctx.next_element_id += 1;

    element.add_attribute("id", &generated_id);
    generated_id
}
```

### Phase 5: Testing
**File**: `tests/reactive_automation_tests.rs`

```rust
#[test]
fn test_signal_declaration() {
    let source = r#"
        fn Counter() -> JSX {
            let count = signal(0);
            return <div>{count}</div>;
        }
    "#;

    let output = compile(source);
    assert!(output.contains("const count = signal(0)"));
    assert!(output.contains("effect(() =>"));
}

#[test]
fn test_event_handler() {
    let source = r#"
        fn App() -> JSX {
            let count = signal(0);
            return <button onClick={() => count.value++}>+</button>;
        }
    "#;

    let output = compile(source);
    assert!(output.contains("addEventListener('click'"));
    assert!(output.contains("count.value++"));
}

#[test]
fn test_text_interpolation() {
    let source = r#"
        fn App() -> JSX {
            let name = signal("World");
            return <h1>Hello, {name}!</h1>;
        }
    "#;

    let output = compile(source);
    assert!(output.contains("effect(() =>"));
    assert!(output.contains("name.value"));
}
```

---

## Example Transformations

### Example 1: Simple Counter

**Input (Jounce):**
```jounce
fn Counter() -> JSX {
    let count = signal(0);

    return (
        <div>
            <div>{count}</div>
            <button onClick={() => count.value++}>+</button>
            <button onClick={() => count.value--}>-</button>
        </div>
    );
}

fn main() {
    console.log("Counter starting");
}
```

**Output (JavaScript):**
```javascript
function Counter() {
    return h('div', {},
        h('div', { id: 'elem-0' }, '0'),
        h('button', { id: 'elem-1' }, '+'),
        h('button', { id: 'elem-2' }, '-')
    );
}

window.addEventListener('DOMContentLoaded', () => {
    console.log('Jounce client initialized');

    // Mount component
    mountComponent(Counter);

    // Reactive state
    const count = signal(0);

    // DOM elements
    const elem_0 = document.getElementById('elem-0');
    const elem_1 = document.getElementById('elem-1');
    const elem_2 = document.getElementById('elem-2');

    // Event handlers
    elem_1.addEventListener('click', () => count.value++);
    elem_2.addEventListener('click', () => count.value--);

    // Effects
    effect(() => {
        if (elem_0) {
            elem_0.textContent = count.value;
        }
    });

    console.log('Counter starting');
    console.log('✅ App initialized!');
});
```

### Example 2: Form Input

**Input (Jounce):**
```jounce
fn Greeter() -> JSX {
    let name = signal("Guest");

    return (
        <div>
            <input type="text" onInput={(e) => name.value = e.target.value} />
            <h1>Hello, {name}!</h1>
        </div>
    );
}
```

**Output (JavaScript):**
```javascript
function Greeter() {
    return h('div', {},
        h('input', { type: 'text', id: 'elem-0' }),
        h('h1', { id: 'elem-1' }, 'Hello, Guest!')
    );
}

window.addEventListener('DOMContentLoaded', () => {
    mountComponent(Greeter);

    const name = signal("Guest");

    const elem_0 = document.getElementById('elem-0');
    const elem_1 = document.getElementById('elem-1');

    elem_0.addEventListener('input', (e) => name.value = e.target.value);

    effect(() => {
        if (elem_1) {
            elem_1.textContent = `Hello, ${name.value}!`;
        }
    });
});
```

### Example 3: Computed Values

**Input (Jounce):**
```jounce
fn Calculator() -> JSX {
    let a = signal(5);
    let b = signal(10);
    let sum = computed(|| a.value + b.value);

    return (
        <div>
            <div>A: {a}, B: {b}</div>
            <div>Sum: {sum}</div>
        </div>
    );
}
```

**Output (JavaScript):**
```javascript
window.addEventListener('DOMContentLoaded', () => {
    mountComponent(Calculator);

    const a = signal(5);
    const b = signal(10);
    const sum = computed(() => a.value + b.value);

    const elem_0 = document.getElementById('elem-0');
    const elem_1 = document.getElementById('elem-1');

    effect(() => {
        if (elem_0) {
            elem_0.textContent = `A: ${a.value}, B: ${b.value}`;
        }
    });

    effect(() => {
        if (elem_1) {
            elem_1.textContent = `Sum: ${sum.value}`;
        }
    });
});
```

---

## Edge Cases

### 1. Multiple Components
```jounce
fn Counter() -> JSX {
    let count = signal(0);
    return <div>{count}</div>;
}

fn Timer() -> JSX {
    let elapsed = signal(0);
    return <div>{elapsed}</div>;
}
```

**Issue**: Each component needs its own scope.
**Solution**: Generate separate initialization for each component, or only support one main component per file.

### 2. Signal Dependencies
```jounce
fn App() -> JSX {
    let a = signal(1);
    let b = signal(a.value * 2);  // Depends on 'a'

    return <div>{b}</div>;
}
```

**Issue**: `b` is a signal but should be computed.
**Solution**: Require `computed()` for derived values.

### 3. Conditional Rendering
```jounce
fn App() -> JSX {
    let show = signal(true);

    return (
        <div>
            {show ? <p>Visible</p> : null}
        </div>
    );
}
```

**Issue**: Conditional JSX is complex to handle.
**Solution**: Phase 2 feature (after basic interpolation works).

---

## Migration Path

### Stage 1: Opt-In (v0.9.0)
- Manual pattern still works
- New syntax is optional
- Can mix both approaches

### Stage 2: Recommended (v0.10.0)
- Docs show new syntax first
- Examples updated
- Manual pattern documented as "legacy"

### Stage 3: Default (v1.0.0)
- New syntax is standard
- Manual pattern still supported
- Tool to auto-migrate code

---

## Success Criteria

✅ Counter app works with new syntax (no manual setup)
✅ Stopwatch app works (complex state)
✅ Form validation works (input events)
✅ All existing apps can be migrated
✅ Documentation updated
✅ Tests pass (50+ new tests)

---

## Timeline

**Week 1**: Lexer + Parser changes (signal, event handlers, interpolation)
**Week 2**: Code generation for basic cases (signals + effects)
**Week 3**: Event handler generation (onClick, onChange, onInput)
**Week 4**: Testing + migration of example apps
**Week 5**: Documentation + edge cases

**Target Release**: v0.9.0 (5 weeks)
