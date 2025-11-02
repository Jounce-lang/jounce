# Lesson 6: Reusable Components

**Time**: 5 minutes
**Difficulty**: Beginner

## 🎯 Goal

Learn how to create and use reusable components to organize your code.

## 📝 Instructions

1. **Use the Button component** by adding this inside the `<div>`:
   ```jounce
   <Button />
   ```

2. **Add multiple buttons** - try adding 2-3 Button components
3. **See how components make code reusable!**

## 💡 What You'll Learn

- How to define components with the `component` keyword
- How to use components with self-closing syntax
- Why components make code more organized and reusable
- Component composition patterns

## ✅ Expected Output

You should see multiple blue buttons, all created from the same component!

## 🎓 Concept: Components

### What is a Component?

A component is a reusable piece of UI. Instead of copying code, you define it once and use it many times.

### Defining a Component

```jounce
component Button() {
    <button class="btn btn-primary">
        Click Me
    </button>
}
```

### Using a Component

```jounce
<Button />  // Self-closing syntax
```

### Why Components?

**Without Components** (repetitive):
```jounce
<button class="btn btn-primary">Click</button>
<button class="btn btn-primary">Click</button>
<button class="btn btn-primary">Click</button>
```

**With Components** (reusable):
```jounce
<Button />
<Button />
<Button />
```

Change the component once, all instances update!

## 🎨 Component Best Practices

### 1. One Component Per Concern
```jounce
component Header() { ... }
component Footer() { ... }
component Sidebar() { ... }
```

### 2. Descriptive Names
```jounce
component UserProfile() { ... }     // Good
component Thing() { ... }            // Bad
```

### 3. Small and Focused
Each component should do one thing well.

## 🚀 Try It Yourself

After completing this lesson, try creating your own components:

1. **Create a Card component**:
```jounce
component Card() {
    <div class="card p-6 rounded shadow">
        <h2 class="text-2xl">Card Title</h2>
        <p>Card content goes here</p>
    </div>
}
```

2. **Create a Badge component**:
```jounce
component Badge() {
    <span class="badge bg-blue-500 text-white px-3 py-1 rounded">
        New
    </span>
}
```

3. **Use them together**:
```jounce
<Card />
<Badge />
```

## ❓ Troubleshooting

**Component doesn't show up**
→ Make sure you're using `<Button />` (capital B)

**Error: Button is not defined**
→ Check that the component is defined before you use it

**Nothing renders**
→ Make sure the component has a return value (JSX)

## 🎯 Challenge

Can you create an Icon component that displays an emoji?

```jounce
component Icon() {
    <span class="text-4xl">🚀</span>
}

// Use it
<Icon />
```

## ➡️ Next Lesson

Fantastic! 🎉 You can now create reusable components. In the next lesson, you'll learn how to pass **props** to components to make them even more flexible.

[Continue to Lesson 7: Props & Composition →]
