# Lesson 2: Variables & Signals

**Time**: 5 minutes
**Difficulty**: Beginner

## 🎯 Goal

Learn about reactive state with signals - the foundation of Jounce's reactivity system.

## 📝 Instructions

1. **Find the line**: `let name = createSignal("World");`
2. **Change "World"** to your actual name (keep the quotes!)
3. **Save and run** to see the UI update automatically

## 💡 What You'll Learn

- How to create signals with `createSignal()`
- How to access signal values with `.value`
- How reactivity works (UI updates automatically!)
- The difference between regular variables and signals

## ✅ Expected Output

After changing the name, you should see:

```
Hello, [Your Name]!
Welcome to reactive programming!
```

## 🎓 Concept: Reactive Signals

### What is a Signal?

A **signal** is a reactive variable that tracks its value and notifies the UI when it changes.

```jounce
let count = createSignal(0);    // Create a signal
console.log(count.value);        // Read the value → 0
count.set(5);                    // Update the value
console.log(count.value);        // Read again → 5
```

### Why Use Signals?

**Regular Variable** (not reactive):
```jounce
let name = "Alice";
name = "Bob";  // UI doesn't update!
```

**Signal** (reactive):
```jounce
let name = createSignal("Alice");
name.set("Bob");  // UI updates automatically! ✨
```

## 🔍 Key Concepts

### Creating a Signal
```jounce
let mySignal = createSignal(initialValue);
```

### Reading a Signal
```jounce
let value = mySignal.value;  // Use .value to read
```

### Updating a Signal
```jounce
mySignal.set(newValue);  // Use .set() to update
```

## 🚀 Try It Yourself

After completing this lesson, experiment by:

1. **Adding another signal**:
```jounce
let age = createSignal(25);
```

2. **Using it in the UI**:
```jounce
<p>Age: {age.value}</p>
```

3. **Trying different data types**:
```jounce
let count = createSignal(0);        // Number
let name = createSignal("Alice");   // String
let active = createSignal(true);    // Boolean
```

## ❓ Troubleshooting

**UI doesn't update**
→ Make sure you're using `.value` to access the signal: `{name.value}`

**Error: createSignal is not defined**
→ Check your spelling - it's `createSignal` (camelCase)

**Value is [object Object]**
→ You forgot `.value` - use `{name.value}` not just `{name}`

## 🎯 Challenge

Can you add a second signal for your city and display it too?

```jounce
let city = createSignal("New York");

<p>From: {city.value}</p>
```

## ➡️ Next Lesson

Excellent! 🎉 Now you understand reactive state. In the next lesson, you'll learn how to build UI with **JSX** - HTML-like syntax in your code.

[Continue to Lesson 3: JSX Basics →]
