# Jounce Runtime

**Client and server runtime libraries for Jounce applications**

---

## 📦 Runtime Files

### Core Runtime
- **`jounce.js`** - Main runtime entry point
- **`client-runtime.js`** - Client-side runtime (browser)
- **`server-runtime.js`** - Server-side runtime (Node.js)

### Reactivity System
- **`reactivity.js`** - Fine-grained reactivity implementation
  - `signal()` - Reactive state containers
  - `computed()` - Derived values
  - `effect()` - Side effects
  - `batch()` - Batch updates

### Hydration
- **`hydration.js`** - Client-side hydration for SSR apps
  - Attaches event listeners to server-rendered HTML
  - Restores reactive state
  - Minimal DOM manipulation

### Development
- **`debug_test.js`** - Runtime debugging utilities
- **`test_reactivity.js`** - Reactivity system tests
- **`serve.sh`** - Local development server script
- **`index.html`** - Test harness HTML

---

## 🚀 Usage

Runtime files are **automatically included** when you compile a Jounce app:

```bash
jnc compile app.jnc
# Outputs:
#   dist/client.js (includes client-runtime.js)
#   dist/server.js (includes server-runtime.js)
#   dist/reactivity.js (reactivity system)
```

---

## 🔧 Client Runtime API

### Creating Reactive State
```javascript
// Create signal
const count = signal(0);
console.log(count.value); // 0
count.value = 1; // Update triggers re-renders

// Computed value
const doubled = computed(() => count.value * 2);
console.log(doubled.value); // 2

// Side effect
effect(() => {
    console.log("Count:", count.value);
}); // Logs: Count: 1
```

### JSX Rendering
```javascript
// h() function for JSX
h('div', { class: 'container' },
    h('h1', null, 'Hello'),
    h('p', null, 'World')
);

// Fragment
h(Fragment, null, child1, child2, child3);
```

### Event Handling
```javascript
// Event listeners automatically bound
h('button', {
    onclick: () => count.value++
}, 'Increment');
```

---

## 🖥️ Server Runtime API

### SSR Rendering
```javascript
// Render to HTML string
const html = renderToString(App);

// Render with data
const html = renderToString(App, { initialData });
```

### Request Handling
```javascript
// Server-side request context
const ctx = getRequestContext();
console.log(ctx.url, ctx.method, ctx.headers);
```

---

## ⚡ Reactivity System

### How It Works
1. **Signals** store reactive values
2. **Computed** automatically track signal dependencies
3. **Effects** run when dependencies change
4. **Batching** coalesces multiple updates

### Fine-Grained Updates
Only affected DOM nodes update, not entire components:
```javascript
const count = signal(0);
// Only the text node updates, not the whole div
<div class="static">
    Count: {count.value}
</div>
```

### Method Call Tracking
Even method calls are reactive:
```javascript
const price = signal(19.99);
// Reactive!
<div>{price.value.toFixed(2)}</div>
```

---

## 🧪 Testing

**Run reactivity tests:**
```bash
node runtime/test_reactivity.js
```

**Debug mode:**
```bash
node runtime/debug_test.js
```

**Local server:**
```bash
./runtime/serve.sh
```

---

## 📝 Development

### Adding Runtime Features

1. **Update runtime file:**
```javascript
// runtime/client-runtime.js
export function newFeature() {
    // Implementation
}
```

2. **Test it:**
```javascript
// runtime/test_reactivity.js
console.log('Testing new feature...');
assert(newFeature() === expected);
```

3. **Update compiler:**
```rust
// src/js_emitter.rs
// Emit code using new feature
```

### Debugging

Enable debug mode:
```javascript
// In your app
window.__JOUNCE_DEBUG__ = true;
```

---

## 🔗 Resources

- [Reactivity Guide](../docs/REACTIVITY.md)
- [SSR Guide](../docs/SSR.md)
- [Client-Side Routing](../docs/ROUTING.md)

---

**Last Updated**: October 31, 2025 (v0.8.1)
