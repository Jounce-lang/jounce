# Lesson 4: Event Handlers

**Time**: 5 minutes
**Difficulty**: Beginner

## 🎯 Goal

Make your UI interactive by responding to user clicks and other events.

## 📝 Instructions

1. **Find the `<button>` tag**
2. **Add an onClick handler** after the `class` attribute:
   ```jounce
   onClick={() => count.set(count.value + 1)}
   ```
3. **Click the button** and watch the count increase!

## 💡 What You'll Learn

- How to handle user clicks with `onClick`
- How to update signals with `.set()`
- How to write lambda expressions `() => {}`
- How reactivity makes the UI update automatically

## ✅ Expected Output

- You should see "Count: 0" initially
- Each button click increments the count
- The UI updates automatically (no manual refresh!)

## 🎓 Concept: Event Handlers

Event handlers let you respond to user actions like clicks, typing, hovering, etc.

### onClick Handler

The most common event handler:

```jounce
<button onClick={() => doSomething()}>
  Click Me
</button>
```

### Lambda Expressions

The `() => {}` syntax is called a lambda or arrow function:

```jounce
// Simple lambda (one line)
onClick={() => count.set(5)}

// Lambda with multiple statements
onClick={() => {
  console.log("Button clicked!");
  count.set(count.value + 1);
}}
```

### Updating Signals

To change a signal's value, use `.set()`:

```jounce
let count = signal(0);

// Read the value
let current = count.value;    // 0

// Update the value
count.set(5);                  // Now it's 5

// Increment
count.set(count.value + 1);    // Now it's 6
```

## 🎨 Common Event Handlers

```jounce
// Click events
<button onClick={() => handleClick()}>Click</button>

// Input events (typing)
<input onInput={(e) => handleInput(e.target.value)} />

// Mouse events
<div onMouseEnter={() => setHover(true)}>Hover me</div>
<div onMouseLeave={() => setHover(false)}>Leave</div>

// Form events
<form onSubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
  ...
</form>

// Keyboard events
<input onKeyDown={(e) => handleKeyPress(e)} />
```

## 🚀 Try It Yourself

After completing this lesson, try:

1. **Add a Decrement button**:
```jounce
<button onClick={() => count.set(count.value - 1)} class="btn btn-secondary ml-2">
  Decrement
</button>
```

2. **Add a Reset button**:
```jounce
<button onClick={() => count.set(0)} class="btn btn-danger ml-2">
  Reset
</button>
```

3. **Make it increment by 5**:
```jounce
<button onClick={() => count.set(count.value + 5)}>
  +5
</button>
```

## ❓ Troubleshooting

**Button doesn't do anything**
→ Make sure you added `onClick={() => ...}` to the button

**Error: count.set is not a function**
→ You need to create the signal first: `let count = signal(0);`

**Count doesn't update in UI**
→ Make sure you're using `{count.value}` in the JSX

**Syntax error**
→ Check your lambda syntax: `() =>` (arrow, not equals)

## 🎯 Challenge

Can you create a button that doubles the count each time you click it?

```jounce
<button onClick={() => count.set(count.value * 2)}>
  Double It!
</button>
```

## ➡️ Next Lesson

Fantastic! 🎉 You've made your first interactive UI. In the next lesson, you'll learn about **computed values** - reactive values that update automatically based on other values.

[Continue to Lesson 5: Reactive State →]
