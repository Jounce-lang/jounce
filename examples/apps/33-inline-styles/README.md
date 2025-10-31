# Inline Styles Example

Demonstrates inline styling in Jounce using object literal syntax.

## Features

- **Static Styles**: Use object literals directly in JSX
- **Reactive Styles**: Automatically update when signals change
- **Computed Styles**: Use computed values for dynamic styling
- **Conditional Styling**: Apply different styles based on state

## Syntax

```jounce
// Static styles
<div style={{ color: "red", fontSize: "16px" }}>
    Text
</div>

// Reactive styles (with signal)
<div style={{ backgroundColor: isActive.value ? "green" : "gray" }}>
    Reactive
</div>

// Using computed values
<div style={{ color: textColor.value }}>
    Computed
</div>
```

## Running

```bash
cargo run --release -- compile examples/apps/33-inline-styles/main.jnc
cd dist && node server.js
```

Open http://localhost:3000 in your browser.

## Implementation Details

- Inline styles use JavaScript object syntax
- Property names are camelCase (e.g., `backgroundColor`, `fontSize`)
- Reactive styles automatically wrap in effect() for updates
- No additional CSS files needed
