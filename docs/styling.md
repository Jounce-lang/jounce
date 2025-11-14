# Jounce Styling Guide

This is a quick reference for Jounce's styling system. For comprehensive documentation, see [STYLE_SYSTEM_USER_GUIDE.md](./guides/STYLE_SYSTEM_USER_GUIDE.md).

## Quick Start

```jounce
style Button {
    padding: 10px 20px;
    background: blue;
    color: white;

    &:hover {
        background: darkblue;
    }
}
```

## Features

- **Component-scoped styles** - Automatic hash-based class names prevent conflicts
- **Nested selectors** - Element, class, pseudo-class, and pseudo-element selectors
- **Pseudo-elements** - Support for `::before` and `::after`
- **Theme system** - CSS custom properties for runtime theming
- **Media queries** - Responsive design support
- **Zero runtime** - All CSS generated at build time

## Pseudo-Selectors

Jounce supports both pseudo-classes and pseudo-elements:

### Pseudo-Classes

```jounce
style Link {
    color: blue;

    &:hover {
        text-decoration: underline;
    }

    &:active {
        color: darkblue;
    }

    &:focus {
        outline: 2px solid blue;
    }
}
```

### Pseudo-Elements

Pseudo-elements `::before` and `::after` are fully supported:

```jounce
style Card {
    padding: 20px;

    &::before {
        content: "→";
        color: gray;
        margin-right: 5px;
    }

    &::after {
        content: "←";
        color: gray;
        margin-left: 5px;
    }
}
```

Generated CSS:
```css
.Card_a1b2c3 {
  padding: 20px;
}

.Card_a1b2c3::before {
  content: "→";
  color: gray;
  margin-right: 5px;
}

.Card_a1b2c3::after {
  content: "←";
  color: gray;
  margin-left: 5px;
}
```

### Combined Usage

You can combine pseudo-classes and pseudo-elements:

```jounce
style Button {
    &:hover {
        background: lightblue;
    }

    &::before {
        content: "▶";
    }
}
```

## Nested Selectors

Jounce supports multi-level nested selectors with a maximum depth of 3:

```jounce
style Card {
    padding: 20px;

    .header {
        font-weight: bold;

        .title {
            font-size: 24px;

            &:hover {
                color: blue;
            }
        }
    }
}
```

**Depth Counting:**
- Depth 0: Top level inside `style Component { ... }`
- Depth 1: First level of nesting (`.header`)
- Depth 2: Second level (`.title`)
- Depth 3: Third level (`&:hover` - maximum allowed)

**Important:** `@media` and `@keyframes` do NOT count toward nesting depth.

## Animations with @keyframes

Jounce supports CSS animations with scoped `@keyframes`:

```jounce
style Box {
    animation: slideIn 0.3s ease-in;

    @keyframes slideIn {
        from {
            transform: translateX(-100%);
            opacity: 0;
        }
        to {
            transform: translateX(0);
            opacity: 1;
        }
    }
}
```

### Keyframe Syntax

Keyframes use standard CSS syntax with `from`, `to`, or percentage selectors:

```jounce
style Spinner {
    animation: rotate 2s linear infinite;

    @keyframes rotate {
        0% {
            transform: rotate(0deg);
        }
        50% {
            transform: rotate(180deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }
}
```

### Automatic Scoping

Animation names are automatically scoped to prevent conflicts:
- Jounce keyframe: `@keyframes slideIn { ... }`
- Generated CSS: `@keyframes jnc-Box_a1b2c3-slideIn { ... }`
- Animation property automatically rewritten: `animation: jnc-Box_a1b2c3-slideIn 0.3s`

This scoping is automatic and transparent - you write `slideIn`, Jounce handles the rest!

## Media Queries

Jounce supports responsive design with `@media` queries:

```jounce
style Panel {
    .container {
        padding: 20px;
    }

    @media (max-width: 600px) {
        .container {
            padding: 10px;
        }
    }
}
```

### Limitations

- Media queries must be at the top level of a style block (not inside selectors).
- Nested media rules (`@media` inside `@media`) are not supported.
- Media queries cannot contain other `@media` blocks.
- Media queries do NOT count toward the 3-level nesting depth limit.

## Examples

### Responsive Layout

```jounce
style Dashboard {
    .sidebar {
        width: 250px;
    }

    .content {
        margin-left: 250px;
    }

    @media (max-width: 768px) {
        .sidebar {
            width: 100%;
        }

        .content {
            margin-left: 0;
        }
    }
}
```

### With Theme System and Animations

```jounce
theme DarkMode {
    primary: #3b82f6;
    background: #1a1a1a;
    text: #ffffff;
}

style Card {
    background: theme.DarkMode.background;
    color: theme.DarkMode.text;
    padding: 20px;
    animation: fadeIn 0.3s ease-in;

    &:hover {
        border-color: theme.DarkMode.primary;
    }

    @media (max-width: 600px) {
        padding: 10px;
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
            transform: translateY(20px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }
}
```

## Documentation

- **Full Guide**: [STYLE_SYSTEM_USER_GUIDE.md](./guides/STYLE_SYSTEM_USER_GUIDE.md)
- **Specification**: [JOUNCE_SPEC.md](../JOUNCE_SPEC.md#3-styling)
- **Migration Guide**: [STYLE_SYSTEM_MIGRATION.md](./guides/STYLE_SYSTEM_MIGRATION.md)
- **Errors**: See error codes below

## Common Errors

See [Error Codes](#error-codes) for styling-related compilation errors.


Maintained by: **The Jounce Project**
