# Building Interactive Jounce Apps

**Version**: v0.8.0
**Last Updated**: October 25, 2025

This guide shows you how to build interactive web applications with Jounce's reactivity system.

---

## Table of Contents

1. [App Structure](#app-structure)
2. [Manual Reactive Setup Pattern](#manual-reactive-setup-pattern)
3. [Build Workflow](#build-workflow)
4. [File Organization](#file-organization)
5. [Common Patterns](#common-patterns)
6. [Example Apps](#example-apps)
7. [Troubleshooting](#troubleshooting)

---

## App Structure

Every Jounce app follows this structure:

```jounce
// 1. Style Block (optional but recommended)
style {
    * {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }

    body {
        font-family: sans-serif;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    }

    .container {
        max-width: 600px;
        margin: 0 auto;
        padding: 20px;
    }
}

// 2. Component Function
fn MyComponent() -> JSX {
    return (
        <div class="container">
            <h1>Hello World</h1>
            <button id="my-btn">Click Me</button>
            <div id="output">0</div>
        </div>
    );
}

// 3. Main Function (entry point)
fn main() {
    console.log("🚀 App starting...");
}
```

### Key Principles

1. **Style blocks** are embedded directly in your `.jnc` file
2. **Component functions** return JSX markup
3. **ID attributes** are used to connect DOM elements to reactive logic
4. **main()** is the entry point (called automatically)

---

## Manual Reactive Setup Pattern

Currently, reactive behavior is set up manually in `dist/client.js` after compilation. This is a **5-step pattern**:

### Step 1: Mount Component

```javascript
window.addEventListener('DOMContentLoaded', () => {
    console.log('Jounce client initialized');

    // Mount your component into #app div
    mountComponent(MyComponent);
```

### Step 2: Create Reactive State

```javascript
    // Create signals for state management
    const count = signal(0);
    const name = signal("Guest");
    const isActive = signal(false);
```

**Signal API:**
- `signal(initialValue)` - Create reactive state
- `signalName.value` - Get current value
- `signalName.value = newValue` - Update (triggers effects)

### Step 3: Get DOM Elements

```javascript
    // Get references to DOM elements by ID
    const outputDiv = document.getElementById('output');
    const myBtn = document.getElementById('my-btn');
    const nameInput = document.getElementById('name-input');
```

### Step 4: Wire Event Listeners

```javascript
    // Connect user interactions to state updates
    myBtn.addEventListener('click', () => {
        count.value = count.value + 1;
    });

    nameInput.addEventListener('input', (e) => {
        name.value = e.target.value;
    });
```

**Common Events:**
- `click` - Button clicks
- `input` - Text input changes (real-time)
- `change` - Select, checkbox, radio changes
- `submit` - Form submission
- `keydown`, `keyup` - Keyboard events

### Step 5: Create Effects

```javascript
    // Effects run when signals change
    effect(() => {
        if (outputDiv) {
            outputDiv.textContent = count.value;
        }
    });

    effect(() => {
        console.log(`Name changed to: ${name.value}`);
    });

    console.log('✅ App initialized!');
});
```

**Effect API:**
- `effect(() => { ... })` - Runs immediately and whenever accessed signals change
- Auto-tracks signal dependencies
- Use for DOM updates, side effects, logging

---

## Build Workflow

### 1. Write Your App

Create `my_app.jnc`:

```jounce
style {
    body { background: #f0f0f0; }
}

fn App() -> JSX {
    return <div><h1 id="title">Hello</h1></div>;
}

fn main() {
    console.log("App starting");
}
```

### 2. Compile

```bash
cargo run -- compile my_app.jnc
```

This generates:
- `dist/styles.css` - Extracted CSS
- `dist/client.js` - Component code + JSX
- `dist/index.html` - HTML template (if not exists)

### 3. Add Reactive Setup

Edit `dist/client.js` and add your reactive logic inside `DOMContentLoaded`:

```javascript
window.addEventListener('DOMContentLoaded', () => {
    console.log('Jounce client initialized');

    mountComponent(App);

    // Your reactive setup here
    const greeting = signal("Hello");
    const titleElem = document.getElementById('title');

    effect(() => {
        if (titleElem) {
            titleElem.textContent = greeting.value;
        }
    });

    console.log('✅ App ready!');
});
```

### 4. Test in Browser

```bash
cd dist
python3 -m http.server 8080
```

Navigate to: `http://localhost:8080/index.html`

**Important**: Must use HTTP server (not `file://`) for ES6 module imports!

### 5. Verify

Open DevTools Console and check for:
```
Jounce client initialized
✅ App ready!
```

No errors = success!

---

## File Organization

```
my-project/
├── my_app.jnc              # Source file
├── dist/                   # Generated files
│   ├── index.html          # HTML template
│   ├── styles.css          # Extracted CSS
│   ├── client.js           # Your component + reactive setup
│   ├── client-runtime.js   # Jounce runtime (h, mountComponent)
│   └── reactivity.js       # Reactivity system (signal, effect)
├── Cargo.toml              # Rust project config
└── src/                    # Jounce compiler source
```

### File Roles

**Source Files (.jnc)**
- Your application code
- Contains style, components, logic
- Version controlled

**Generated Files (dist/)**
- Created by compiler
- Should NOT be manually edited (except reactive setup for now)
- Can be regenerated anytime
- `.gitignore` recommended

**Runtime Files**
- `client-runtime.js` - Auto-copied from `runtime/client-runtime.js`
- `reactivity.js` - Auto-copied from `runtime/reactivity.js`
- Required for all apps

---

## Common Patterns

### Pattern 1: Simple Counter

**Use Case**: Increment/decrement numeric value

```jounce
fn Counter() -> JSX {
    return (
        <div>
            <div id="count">0</div>
            <button id="inc-btn">+</button>
            <button id="dec-btn">-</button>
        </div>
    );
}
```

**Reactive Setup**:
```javascript
const count = signal(0);
const display = document.getElementById('count');
const incBtn = document.getElementById('inc-btn');
const decBtn = document.getElementById('dec-btn');

incBtn.addEventListener('click', () => {
    count.value = count.value + 1;
});

decBtn.addEventListener('click', () => {
    count.value = count.value - 1;
});

effect(() => {
    if (display) {
        display.textContent = count.value;
    }
});
```

### Pattern 2: Text Input

**Use Case**: Update display as user types

```jounce
fn Greeter() -> JSX {
    return (
        <div>
            <input type="text" id="name-input" placeholder="Enter name" />
            <div id="greeting">Hello, Guest!</div>
        </div>
    );
}
```

**Reactive Setup**:
```javascript
const name = signal("Guest");
const input = document.getElementById('name-input');
const greeting = document.getElementById('greeting');

input.addEventListener('input', (e) => {
    name.value = e.target.value || "Guest";
});

effect(() => {
    if (greeting) {
        greeting.textContent = `Hello, ${name.value}!`;
    }
});
```

### Pattern 3: Timer/Interval

**Use Case**: Update UI based on time

```jounce
fn Timer() -> JSX {
    return (
        <div>
            <div id="time">00:00</div>
            <button id="start-btn">Start</button>
            <button id="stop-btn">Stop</button>
        </div>
    );
}
```

**Reactive Setup**:
```javascript
const elapsed = signal(0);  // seconds
const isRunning = signal(false);
let intervalId = null;

const startBtn = document.getElementById('start-btn');
const stopBtn = document.getElementById('stop-btn');
const timeDisplay = document.getElementById('time');

startBtn.addEventListener('click', () => {
    if (!isRunning.value) {
        isRunning.value = true;
        intervalId = setInterval(() => {
            elapsed.value = elapsed.value + 1;
        }, 1000);
    }
});

stopBtn.addEventListener('click', () => {
    if (isRunning.value) {
        isRunning.value = false;
        clearInterval(intervalId);
    }
});

// Format time as MM:SS
effect(() => {
    if (timeDisplay) {
        const minutes = Math.floor(elapsed.value / 60);
        const seconds = elapsed.value % 60;
        timeDisplay.textContent = `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
    }
});

// Update button states
effect(() => {
    if (startBtn) startBtn.disabled = isRunning.value;
    if (stopBtn) stopBtn.disabled = !isRunning.value;
});
```

### Pattern 4: Computed Values

**Use Case**: Derive values from multiple signals

```jounce
fn Calculator() -> JSX {
    return (
        <div>
            <input type="number" id="a-input" value="0" />
            <span>+</span>
            <input type="number" id="b-input" value="0" />
            <div id="sum">Sum: 0</div>
        </div>
    );
}
```

**Reactive Setup**:
```javascript
const a = signal(0);
const b = signal(0);
const sum = computed(() => a.value + b.value);

const aInput = document.getElementById('a-input');
const bInput = document.getElementById('b-input');
const sumDisplay = document.getElementById('sum');

aInput.addEventListener('input', (e) => {
    a.value = parseInt(e.target.value) || 0;
});

bInput.addEventListener('input', (e) => {
    b.value = parseInt(e.target.value) || 0;
});

effect(() => {
    if (sumDisplay) {
        sumDisplay.textContent = `Sum: ${sum.value}`;
    }
});
```

### Pattern 5: Toggle/Checkbox

**Use Case**: Boolean state management

```jounce
fn Toggle() -> JSX {
    return (
        <div>
            <input type="checkbox" id="toggle" />
            <div id="status">Off</div>
        </div>
    );
}
```

**Reactive Setup**:
```javascript
const isOn = signal(false);
const toggle = document.getElementById('toggle');
const status = document.getElementById('status');

toggle.addEventListener('change', (e) => {
    isOn.value = e.target.checked;
});

effect(() => {
    if (status) {
        status.textContent = isOn.value ? "On" : "Off";
        status.style.color = isOn.value ? "green" : "red";
    }
});
```

### Pattern 6: Form Validation

**Use Case**: Validate input and show errors

```jounce
fn LoginForm() -> JSX {
    return (
        <form id="login-form">
            <input type="email" id="email" placeholder="Email" />
            <div id="email-error" class="error"></div>
            <button type="submit">Login</button>
        </form>
    );
}
```

**Reactive Setup**:
```javascript
const email = signal("");
const emailError = signal("");

const emailInput = document.getElementById('email');
const emailErrorDiv = document.getElementById('email-error');
const form = document.getElementById('login-form');

emailInput.addEventListener('input', (e) => {
    email.value = e.target.value;

    // Validate
    if (!email.value.includes('@')) {
        emailError.value = "Invalid email";
    } else {
        emailError.value = "";
    }
});

form.addEventListener('submit', (e) => {
    e.preventDefault();
    if (!emailError.value) {
        console.log('Form submitted:', email.value);
    }
});

effect(() => {
    if (emailErrorDiv) {
        emailErrorDiv.textContent = emailError.value;
        emailErrorDiv.style.display = emailError.value ? 'block' : 'none';
    }
});
```

---

## Example Apps

### 1. Counter App

**File**: `test_app_counter.jnc`
**Features**: Increment, decrement, reset
**Lines**: 106 lines with full styling
**State**: 1 signal (`count`)

**Compile & Test**:
```bash
cargo run -- compile test_app_counter.jnc
cd dist && python3 -m http.server 8080
# Open: http://localhost:8080/index.html
```

### 2. Stopwatch App

**File**: `test_app_stopwatch.jnc`
**Features**: Start, stop, reset, millisecond precision
**Lines**: 120 lines with full styling
**State**: 2 signals (`elapsed`, `isRunning`)
**Advanced**: Uses `setInterval()` for timing

**Compile & Test**:
```bash
cargo run -- compile test_app_stopwatch.jnc
cd dist && python3 -m http.server 8080
# Open: http://localhost:8080/index.html
```

### 3. Color Picker App

**File**: `test_color_picker.jnc`
**Features**: RGB sliders, live preview, hex display
**Lines**: 140+ lines
**State**: 3 signals (`red`, `green`, `blue`)
**Advanced**: Computed hex value, RGB color preview

---

## Troubleshooting

### CSS Not Loading

**Symptom**: Page has no styles
**Check**:
1. `dist/styles.css` exists
2. HTML has `<link rel="stylesheet" href="styles.css">`
3. Browser DevTools → Network → styles.css (200 OK)

### Reactivity Not Working

**Symptom**: Clicking buttons does nothing
**Check**:
1. `dist/reactivity.js` exists
2. `dist/client-runtime.js` exists
3. Console errors? (F12 → Console)
4. `signal is not defined` → reactivity.js not loaded
5. `mountComponent is not defined` → client-runtime.js not loaded

### Component Not Rendering

**Symptom**: Page shows "Loading..." forever
**Check**:
1. `mountComponent(ComponentName)` called in client.js
2. Component function exists in client.js
3. Console shows "Jounce client initialized"
4. DevTools → Elements → inspect #app div

### Invalid CSS

**Symptom**: Styles broken, weird spacing
**This was fixed in v0.8.0!**
**Update**: Recompile with latest version

**Before Fix**:
```css
max-width: 600 px;  /* ❌ Space before unit */
margin: 0auto;      /* ❌ No space between values */
```

**After Fix**:
```css
max-width: 600px;   /* ✅ Correct */
margin: 0 auto;     /* ✅ Correct */
```

### Module Import Errors

**Symptom**: `Cannot use import statement outside a module`
**Fix**: Use HTTP server, not `file://` protocol

```bash
# ❌ Wrong: file:///path/to/dist/index.html
# ✅ Right: http://localhost:8080/index.html

cd dist
python3 -m http.server 8080
```

### Event Handlers Not Firing

**Symptom**: Buttons don't respond to clicks
**Check**:
1. Element ID matches `getElementById()` call
2. Event listener added inside `DOMContentLoaded`
3. No JavaScript errors in console
4. Button is not disabled

**Debug**:
```javascript
myBtn.addEventListener('click', () => {
    console.log('Button clicked!');  // Add this
    count.value = count.value + 1;
});
```

### Effects Not Running

**Symptom**: UI doesn't update when state changes
**Check**:
1. Effect accesses `.value` property of signal
2. Effect is inside `effect(() => { ... })` wrapper
3. Signal is updated with `.value = newValue` (not reassignment)

**Wrong**:
```javascript
count = count + 1;  // ❌ Doesn't trigger effects
```

**Right**:
```javascript
count.value = count.value + 1;  // ✅ Triggers effects
```

---

## Next Steps

### Future Improvements

1. **Automatic Reactive Setup** - Compiler will generate reactive code automatically
2. **Built-in Event Handlers** - `onClick={handler}` in JSX
3. **Two-way Binding** - `<input bind:value={signal} />`
4. **Component Props** - `<MyComponent count={5} />`
5. **List Rendering** - `{items.map(item => <li>{item}</li>)}`

### Advanced Topics

- **Batching Updates** - `batch(() => { ... })` for multiple signal changes
- **Cleanup** - Stop effects, clear intervals
- **Derived State** - `computed()` for calculated values
- **Global State** - Shared signals across components
- **Routing** - URL-based navigation (jounce-router package)

---

## Resources

- **Test Guide**: See `TEST_IN_BROWSER.md` for detailed testing instructions
- **Reactivity API**: See `runtime/reactivity.js` source code
- **Examples**: All test apps in project root (`test_app_*.jnc`)
- **Issues**: Report bugs at [GitHub Issues](https://github.com/your-repo/jounce/issues)

---

**Happy Building!** 🚀
