# 20 More Example Apps - Comprehensive Testing Plan

**Goal**: Find 10+ more issues by building diverse, real-world example apps
**Date**: October 28, 2025
**Status**: Planning

---

## Apps 12-31: Edge Case Testing

### **App 12: Nested Components** 🔥
**Tests**: Component composition, props passing, nested JSX
```jounce
component Card(title: String, children: JSX) -> JSX {
    <div class="card">
        <h2>{title}</h2>
        <div class="content">{children}</div>
    </div>
}

component App() -> JSX {
    <Card title="Outer">
        <Card title="Inner">
            <p>Nested content</p>
        </Card>
    </Card>
}
```
**Potential Issues**: Props with JSX type, children prop handling

---

### **App 13: Conditional JSX** 🔥
**Tests**: If/else in JSX, multiple conditions, nested conditionals
```jounce
component App() -> JSX {
    let show = createSignal(true);

    <div>
        {if show.value {
            <p>Shown</p>
        } else {
            <p>Hidden</p>
        }}

        {if show.value {
            if count.value > 5 {
                <p>Greater than 5</p>
            } else {
                <p>Less than or equal to 5</p>
            }
        }}
    </div>
}
```
**Potential Issues**: If expressions in JSX, nested if blocks, else handling

---

### **App 14: Array Map with Keys** 🔥
**Tests**: Map with index, key attribute, complex items
```jounce
component App() -> JSX {
    let items = createSignal(["Apple", "Banana", "Cherry"]);

    <ul>
        {items.value.map((item, index) => {
            <li key={index}>
                {index + 1}. {item}
            </li>
        })}
    </ul>
}
```
**Potential Issues**: Lambda with two parameters, key attribute handling, index + 1

---

### **App 15: Event Handler Arguments** 🔥
**Tests**: Events with custom arguments, preventDefault, stopPropagation
```jounce
component App() -> JSX {
    fn handleClick(event: Event, msg: String) {
        event.preventDefault();
        console.log(msg);
    }

    <button onClick={(e) => handleClick(e, "Clicked!")}>
        Click Me
    </button>
}
```
**Potential Issues**: Event object, preventDefault method, multi-arg lambdas

---

### **App 16: Form Validation** 🔥
**Tests**: Multiple inputs, validation logic, error messages
```jounce
component App() -> JSX {
    let email = createSignal("");
    let password = createSignal("");
    let errors = createSignal([]);

    fn validate() {
        let errs = [];
        if email.value.length == 0 {
            errs.push("Email required");
        }
        if password.value.length < 8 {
            errs.push("Password must be 8+ chars");
        }
        errors.set(errs);
    }

    <form>
        <input type="email" value={email.value} onInput={(e) => email.set(e.target.value)} />
        <input type="password" value={password.value} onInput={(e) => password.set(e.target.value)} />
        <button type="button" onClick={validate}>Validate</button>

        {errors.value.length > 0 ? (
            <ul>
                {errors.value.map((err) => <li>{err}</li>)}
            </ul>
        ) : null}
    </form>
}
```
**Potential Issues**: Array.push method, null in JSX, e.target.value, type="button"

---

### **App 17: Computed Derived State** 🔥
**Tests**: Multiple createComputed, dependencies, chained computations
```jounce
component App() -> JSX {
    let width = createSignal(10);
    let height = createSignal(20);

    let area = createComputed(() => width.value * height.value);
    let perimeter = createComputed(() => 2 * (width.value + height.value));
    let ratio = createComputed(() => width.value / height.value);

    <div>
        <p>Area: {area.value}</p>
        <p>Perimeter: {perimeter.value}</p>
        <p>Ratio: {ratio.value.toFixed(2)}</p>
    </div>
}
```
**Potential Issues**: Multiple computed values, division, toFixed on computed

---

### **App 18: Timer/Interval** 🔥
**Tests**: setInterval, cleanup, time formatting
```jounce
component App() -> JSX {
    let seconds = createSignal(0);

    createEffect(() => {
        let timer = setInterval(() => {
            seconds.set(seconds.value + 1);
        }, 1000);

        // Cleanup
        return () => clearInterval(timer);
    });

    let minutes = createComputed(() => Math.floor(seconds.value / 60));
    let secs = createComputed(() => seconds.value % 60);

    <div>
        <h1>{minutes.value}:{secs.value.toString().padStart(2, "0")}</h1>
    </div>
}
```
**Potential Issues**: setInterval, clearInterval, cleanup functions, Math.floor, padStart

---

### **App 19: Fetch API** 🔥
**Tests**: Async/await, fetch, JSON parsing, error handling
```jounce
component App() -> JSX {
    let data = createSignal(null);
    let loading = createSignal(false);
    let error = createSignal(null);

    async fn fetchData() {
        loading.set(true);
        error.set(null);

        try {
            let response = await fetch("https://api.example.com/data");
            let json = await response.json();
            data.set(json);
        } catch (e) {
            error.set(e.message);
        } finally {
            loading.set(false);
        }
    }

    <div>
        <button onClick={fetchData}>Fetch Data</button>

        {loading.value ? <p>Loading...</p> : null}
        {error.value ? <p>Error: {error.value}</p> : null}
        {data.value ? <pre>{JSON.stringify(data.value, null, 2)}</pre> : null}
    </div>
}
```
**Potential Issues**: Try/catch/finally, await in try block, JSON.stringify, e.message

---

### **App 20: Local Storage** 🔥
**Tests**: localStorage.getItem/setItem, JSON parse/stringify, persistence
```jounce
component App() -> JSX {
    let stored = localStorage.getItem("count");
    let count = createSignal(stored ? parseInt(stored) : 0);

    createEffect(() => {
        localStorage.setItem("count", count.value.toString());
    });

    fn increment() {
        count.set(count.value + 1);
    }

    <div>
        <p>Count: {count.value} (persisted)</p>
        <button onClick={increment}>Increment</button>
    </div>
}
```
**Potential Issues**: localStorage access, parseInt, ternary with function call, toString()

---

### **App 21: Multiple Event Types** 🔥
**Tests**: onMouseEnter, onMouseLeave, onFocus, onBlur, onKeyDown
```jounce
component App() -> JSX {
    let hovering = createSignal(false);
    let focused = createSignal(false);
    let lastKey = createSignal("");

    <div>
        <div
            onMouseEnter={() => hovering.set(true)}
            onMouseLeave={() => hovering.set(false)}
            style="padding: 20px; background: {hovering.value ? 'yellow' : 'white'}"
        >
            Hover me!
        </div>

        <input
            onFocus={() => focused.set(true)}
            onBlur={() => focused.set(false)}
            onKeyDown={(e) => lastKey.set(e.key)}
            placeholder="Type here"
        />

        <p>Focused: {focused.value ? "Yes" : "No"}</p>
        <p>Last key: {lastKey.value}</p>
    </div>
}
```
**Potential Issues**: Multiple event handlers, e.key, dynamic style attribute

---

### **App 22: Class Toggle** 🔥
**Tests**: Dynamic class names, multiple classes, conditional classes
```jounce
component App() -> JSX {
    let active = createSignal(false);
    let large = createSignal(false);

    <div>
        <button
            class="btn {active.value ? 'active' : ''} {large.value ? 'large' : ''}"
            onClick={() => active.set(!active.value)}
        >
            Toggle Active
        </button>

        <button onClick={() => large.set(!large.value)}>
            Toggle Size
        </button>
    </div>
}
```
**Potential Issues**: String interpolation in class attribute, multiple dynamic classes

---

### **App 23: Refs / DOM Access** 🔥
**Tests**: createRef, ref attribute, DOM manipulation
```jounce
component App() -> JSX {
    let inputRef = createRef();

    fn focusInput() {
        if inputRef.current {
            inputRef.current.focus();
        }
    }

    <div>
        <input ref={inputRef} />
        <button onClick={focusInput}>Focus Input</button>
    </div>
}
```
**Potential Issues**: createRef, ref attribute, .current property, .focus() method

---

### **App 24: SVG Elements** 🔥
**Tests**: SVG tags, SVG attributes (viewBox, strokeWidth, etc.)
```jounce
component App() -> JSX {
    let size = createSignal(100);

    <div>
        <svg width={size.value} height={size.value} viewBox="0 0 100 100">
            <circle cx="50" cy="50" r="40" stroke="black" strokeWidth="3" fill="red" />
            <line x1="0" y1="0" x2="100" y2="100" stroke="blue" strokeWidth="2" />
        </svg>

        <input type="range" min="50" max="300" value={size.value} onInput={(e) => size.set(parseInt(e.target.value))} />
    </div>
}
```
**Potential Issues**: SVG namespace, camelCase SVG attributes (strokeWidth), viewBox attribute

---

### **App 25: Fragment Usage** 🔥
**Tests**: <>, </>, returning fragments, fragments with keys
```jounce
component App() -> JSX {
    let items = createSignal([1, 2, 3]);

    <div>
        {items.value.map((item) => (
            <>
                <h3>Item {item}</h3>
                <p>Description for item {item}</p>
            </>
        ))}
    </div>
}
```
**Potential Issues**: Fragment syntax `<>`, fragments in map, no key on fragment

---

### **App 26: Error Boundary** 🔥
**Tests**: Error catching, componentDidCatch, error recovery
```jounce
component ErrorBoundary(children: JSX) -> JSX {
    let hasError = createSignal(false);
    let error = createSignal(null);

    // This would need special error boundary support
    <div>
        {if hasError.value {
            <div class="error">
                <h2>Something went wrong</h2>
                <p>{error.value}</p>
            </div>
        } else {
            children
        }}
    </div>
}

component BuggyComponent() -> JSX {
    throw new Error("Intentional error");
    <div>Never rendered</div>
}

component App() -> JSX {
    <ErrorBoundary>
        <BuggyComponent />
    </ErrorBoundary>
}
```
**Potential Issues**: Error boundaries, throw in component, error.value rendering

---

### **App 27: Portal** 🔥
**Tests**: Rendering to different DOM node, createPortal
```jounce
component Modal(show: Bool, children: JSX) -> JSX {
    if !show {
        return null;
    }

    // Would need portal support
    createPortal(
        <div class="modal-backdrop">
            <div class="modal">{children}</div>
        </div>,
        document.body
    )
}

component App() -> JSX {
    let showModal = createSignal(false);

    <div>
        <button onClick={() => showModal.set(true)}>Open Modal</button>

        <Modal show={showModal.value}>
            <h2>Modal Title</h2>
            <button onClick={() => showModal.set(false)}>Close</button>
        </Modal>
    </div>
}
```
**Potential Issues**: createPortal, document.body, null return from component

---

### **App 28: Context API** 🔥
**Tests**: createContext, Provider, useContext
```jounce
let ThemeContext = createContext("light");

component ThemeProvider(children: JSX) -> JSX {
    let theme = createSignal("light");

    <ThemeContext.Provider value={theme}>
        {children}
    </ThemeContext.Provider>
}

component ThemedButton() -> JSX {
    let theme = useContext(ThemeContext);

    <button class="btn-{theme.value}">
        Themed Button
    </button>
}

component App() -> JSX {
    <ThemeProvider>
        <ThemedButton />
    </ThemeProvider>
}
```
**Potential Issues**: createContext, Provider component, useContext hook

---

### **App 29: Lazy Loading** 🔥
**Tests**: Dynamic imports, lazy, Suspense
```jounce
let LazyComponent = lazy(() => import("./HeavyComponent.jnc"));

component App() -> JSX {
    let showHeavy = createSignal(false);

    <div>
        <button onClick={() => showHeavy.set(true)}>Load Heavy Component</button>

        {if showHeavy.value {
            <Suspense fallback={<p>Loading...</p>}>
                <LazyComponent />
            </Suspense>
        }}
    </div>
}
```
**Potential Issues**: lazy(), import(), Suspense component, fallback prop

---

### **App 30: Lifecycle Hooks** 🔥
**Tests**: onMount, onCleanup, onUnmount
```jounce
component App() -> JSX {
    let mounted = createSignal(false);

    onMount(() => {
        console.log("Component mounted");
        mounted.set(true);
    });

    onCleanup(() => {
        console.log("Cleaning up");
    });

    createEffect(() => {
        let interval = setInterval(() => {
            console.log("Tick");
        }, 1000);

        onCleanup(() => {
            clearInterval(interval);
        });
    });

    <div>
        <p>Mounted: {mounted.value ? "Yes" : "No"}</p>
    </div>
}
```
**Potential Issues**: onMount hook, onCleanup in effect, cleanup timing

---

### **App 31: Memoization** 🔥
**Tests**: createMemo, expensive computations, dependency tracking
```jounce
component App() -> JSX {
    let numbers = createSignal([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    let expensiveSum = createMemo(() => {
        console.log("Computing sum...");
        return numbers.value.reduce((acc, n) => acc + n, 0);
    });

    let count = createSignal(0);

    <div>
        <p>Sum: {expensiveSum.value}</p>
        <p>Count: {count.value}</p>
        <button onClick={() => count.set(count.value + 1)}>Increment Count</button>
        <button onClick={() => numbers.set([...numbers.value, numbers.value.length + 1])}>Add Number</button>
    </div>
}
```
**Potential Issues**: createMemo, reduce method, spread operator in signal

---

## Testing Strategy

### Phase 1: Quick Build (Apps 12-16)
Test basic features we haven't stressed yet:
- Component props
- Conditional rendering
- Event handlers
- Form handling

### Phase 2: Advanced Features (Apps 17-21)
Test complex patterns:
- Computed values
- Timers and async
- Browser APIs
- Multiple events

### Phase 3: Framework Features (Apps 22-26)
Test framework-specific features:
- Class binding
- Refs
- SVG
- Fragments
- Error boundaries

### Phase 4: Advanced Patterns (Apps 27-31)
Test advanced framework features:
- Portals
- Context
- Lazy loading
- Lifecycle
- Memoization

---

## Expected Issue Categories

1. **JSX Edge Cases**: null, fragments, SVG, dynamic attributes
2. **Reactivity Issues**: Computed dependencies, cleanup, effects
3. **Event Handling**: Event object properties, preventDefault, multiple handlers
4. **Browser APIs**: localStorage, fetch, setInterval, DOM methods
5. **Type Issues**: Props types, JSX type, null type
6. **Advanced Features**: Refs, context, portals, lazy loading
7. **Error Handling**: Try/catch, error boundaries, error messages
8. **Performance**: Memo, lazy, optimization
9. **Syntax**: Fragments, spread, string interpolation in attributes
10. **Framework Features**: Lifecycle hooks, cleanup, context providers

---

**Next Steps**:
1. Build apps 12-16 first (basic features)
2. Document every error/issue found
3. Continue to apps 17-31 as time permits
4. Compile master issue list
5. Prioritize fixes

**Target**: Find 10-20 more issues across all apps
