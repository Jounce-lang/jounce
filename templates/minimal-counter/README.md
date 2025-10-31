# Minimal Counter Template

**Your first Jounce app** - Learn the basics in under 40 lines!

## What's Included

✅ **Reactive signals** - `signal()` for state management
✅ **JSX syntax** - Declarative UI with HTML-like syntax
✅ **Event handlers** - `onclick` with lambda functions
✅ **CSS utilities** - Tailwind-inspired classes for styling

## Quick Start

```bash
# 1. Copy this template
cp -r templates/minimal-counter my-counter-app
cd my-counter-app

# 2. Compile and run
jnc compile main.jnc
cd dist && node server.js

# 3. Open browser
open http://localhost:3000
```

## What You'll Learn

### 1. Reactive Signals
```jounce
let count = signal(0);  // Create reactive state
{count.value}           // Display current value
count.value = count.value + 1;  // Update state
```

### 2. JSX Syntax
```jounce
return <div class="container">
    <h1>Counter App</h1>
    <p>Count: {count.value}</p>
</div>;
```

### 3. Event Handlers
```jounce
<button onclick={|| count.value = count.value + 1}>
    Increment
</button>
```

### 4. CSS Utilities
```jounce
<div class="container mx-auto p-8">       // Container with margin and padding
    <div class="card p-6 shadow-lg">      // Card component with shadow
        <h1 class="text-4xl font-bold">   // Large, bold text
```

## Next Steps

1. **Modify the counter** - Change initial value, step size, or max/min values
2. **Add more buttons** - Try +10, -10, or double/half buttons
3. **Style it** - Explore more [CSS utilities](../../docs/CSS_UTILITIES.md)
4. **Add computed values** - Use `computed()` for derived state

## Learn More

- [Reactivity Guide](../../docs/REACTIVITY.md)
- [JSX Reference](../../docs/JSX.md)
- [CSS Utilities](../../docs/CSS_UTILITIES.md)

---

**Difficulty:** Beginner
**Time:** 5 minutes
**Lines:** 38
