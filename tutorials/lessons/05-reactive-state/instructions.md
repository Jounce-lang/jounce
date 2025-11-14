# Lesson 5: Reactive State with Computed Values

**Time**: 5 minutes
**Difficulty**: Beginner

## 🎯 Goal

Learn how to create computed values that automatically update when their dependencies change.

## 📝 Instructions

1. **Create a computed value** after the `count` signal:
   ```jounce
   let doubled = computed(() => count.value * 2);
   ```

2. **Display the computed value** - replace `???` with:
   ```jounce
   {doubled.value}
   ```

3. **Click the button** and watch both values update automatically!

## 💡 What You'll Learn

- How to create computed values with `computed()`
- How computed values track dependencies automatically
- Why you never need to manually update computed values
- The reactive graph concept

## ✅ Expected Output

- Count starts at 0, Doubled starts at 0
- Click increment → Count: 1, Doubled: 2
- Click again → Count: 2, Doubled: 4
- Both values update automatically!

## 🎓 Concept: Computed Values

A **computed value** is a reactive value that's derived from other reactive values.

### Why Use Computed?

**Without Computed** (manual updates):
```jounce
let count = signal(0);
let doubled = signal(0);

// You'd have to manually update doubled every time!
onClick={() => {
  count.set(count.value + 1);
  doubled.set(count.value * 2);  // Manual sync 😫
}}
```

**With Computed** (automatic updates):
```jounce
let count = signal(0);
let doubled = computed(() => count.value * 2);

// Doubled updates automatically! 🎉
onClick(() => count.set(count.value + 1))
```

### How It Works

```jounce
let firstName = signal("Alice");
let lastName = signal("Smith");

// This automatically updates when either name changes!
let fullName = computed(() => firstName.value + " " + lastName.value);
```

When `firstName` or `lastName` changes, `fullName` automatically recalculates!

### The Reactive Graph

Jounce builds a dependency graph:

```
count (signal)
  └─> doubled (computed)
        └─> UI display

When count changes:
1. doubled recalculates
2. UI updates
All automatic!
```

## 🎨 More Examples

### Temperature Converter
```jounce
let celsius = signal(0);
let fahrenheit = computed(() => (celsius.value * 9/5) + 32);

// Change celsius, fahrenheit updates automatically!
```

### Shopping Cart Total
```jounce
let quantity = signal(1);
let price = signal(29.99);
let total = computed(() => quantity.value * price.value);
```

### Form Validation
```jounce
let email = signal("");
let isValid = computed(() => email.value.includes("@"));
```

## 🚀 Try It Yourself

After completing this lesson, try:

1. **Add a tripled value**:
```jounce
let tripled = computed(() => count.value * 3);
```

2. **Add a squared value**:
```jounce
let squared = computed(() => count.value * count.value);
```

3. **Create a status message**:
```jounce
let status = computed(() =>
  count.value === 0 ? "Zero" :
  count.value < 10 ? "Small" :
  "Large"
);
```

## ❓ Troubleshooting

**Computed value doesn't update**
→ Make sure you're accessing `.value` inside the computed function

**Error: computed is not defined**
→ Check your spelling - it's `computed` (lowercase)

**Value shows [object Object]**
→ You forgot `.value` when displaying: use `{doubled.value}`

## 🎯 Challenge

Can you create a computed value that shows whether the count is even or odd?

```jounce
let isEven = computed(() => count.value % 2 === 0);

<p>The count is: {isEven.value ? "Even" : "Odd"}</p>
```

## 🎓 Key Takeaway

**Computed values are lazy and cached**:
- They only recalculate when dependencies change
- Multiple reads return the same cached value
- They're optimized for performance

## ➡️ Next Lesson

Amazing! 🎉 You now understand reactive state. In the next lesson, you'll learn how to create **reusable components** to organize your code better.

[Continue to Lesson 6: Components →]
