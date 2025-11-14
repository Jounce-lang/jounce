# Component Props Guide

**Status**: ✅ Fully Working!  
**Version**: v0.8.2  
**Last Updated**: November 2, 2025

> **Canonical Reference**: If this document conflicts with JOUNCE_SPEC.md, the spec wins. Current spec: v0.8.3 (2025-11-07).

## Overview

Component props in Jounce are **fully functional** and support all common use cases:
- String, Number, Boolean props
- Function props (event handlers)
- Reactive expressions in props
- Type annotations
- Default parameters

## ✅ Working Examples

### Basic Props

```jounce
component Card(title: String, subtitle: String) {
    <div class="card">
        <h2>{title}</h2>
        <p>{subtitle}</p>
    </div>
}

component App() {
    <div>
        <Card title="Hello" subtitle="World" />
        <Card title="Jounce" subtitle="Is Awesome!" />
    </div>
}
```

### Function Props (Event Handlers)

```jounce
component Button(text: String, onClick: Function) {
    <button onclick={onClick} class="btn">{text}</button>
}

component Counter() {
    let count = signal(0);
    
    // ✅ Use arrow functions
    let increment = () => { count.value = count.value + 1; };
    
    <div>
        <h1>Count: {count.value}</h1>
        <Button text="Increment" onclick={increment} />
    </div>
}
```

### Multiple Props with Different Types

```jounce
component TodoItem(text: String, completed: Boolean, onToggle: Function) {
    <div class="flex items-center gap-2">
        <input 
            type="checkbox" 
            checked={completed}
            onclick={onToggle}
        />
        <span class={completed ? "line-through" : ""}>{text}</span>
    </div>
}
```

### Reactive Expressions in Props

```jounce
component StatusCard(value: Number) {
    let color = computed(() => {
        return value > 10 ? "bg-green-100" : "bg-red-100";
    });
    
    <div class="card {color.value}">
        <p>Value: {value}</p>
    </div>
}

component App() {
    let count = signal(5);
    
    <div>
        <StatusCard value={count.value} />
        <button onclick={() => count.value = count.value + 1}>
            Increase
        </button>
    </div>
}
```

## 📝 Best Practices

### ✅ DO: Use Arrow Functions

```jounce
// ✅ Good: Arrow function
let handleClick = () => { console.log("Clicked!"); };
<Button onclick={handleClick} />

// ✅ Good: Inline arrow function
<Button onclick={() => console.log("Clicked!")} />
```

### ❌ DON'T: Use `function` Keyword in Components

```jounce
// ❌ Avoid: function keyword has parser limitation
function handleClick() {  // Parser issue with JSX after this
    console.log("Clicked!");
}
<Button onclick={handleClick} />  // May cause parse error

// ✅ Instead: Use arrow functions
let handleClick = () => { console.log("Clicked!"); };
<Button onclick={handleClick} />
```

## 🎯 Component Props Features

### Type Annotations

```jounce
component MyComponent(
    name: String,
    age: Number,
    active: Boolean,
    onClick: Function
) {
    // Component body
}
```

### Optional Return Type

```jounce
component Card(title: String) -> JSX {
    <div>{title}</div>
}
```

### Destructuring in Generated Code

Generated JavaScript automatically destructures props with defaults:

```javascript
export function Card({ title, subtitle } = {}) {
  return h('div', { class: "card" }, 
    h('h2', null, title),
    h('p', null, subtitle)
  );
}
```

## 🚀 Complete Example

```jounce
component Button(text: String, onClick: Function, variant: String) {
    <button onclick={onClick} class="btn {variant}">
        {text}
    </button>
}

component Card(title: String, content: String, color: String) {
    <div class="card {color}">
        <h3>{title}</h3>
        <p>{content}</p>
    </div>
}

component Counter() {
    let count = signal(0);
    
    let increment = () => { count.value = count.value + 1; };
    let decrement = () => { count.value = count.value - 1; };
    let reset = () => { count.value = 0; };
    
    <div class="p-8">
        <h1 class="text-4xl mb-4">Count: {count.value}</h1>
        
        <div class="flex gap-4 mb-4">
            <Button text="Decrement" onclick={decrement} variant="btn-secondary" />
            <Button text="Reset" onclick={reset} variant="btn-outline" />
            <Button text="Increment" onclick={increment} variant="btn-primary" />
        </div>
        
        <Card 
            title="Statistics" 
            content={count.value > 10 ? "High" : "Low"}
            color="bg-blue-50"
        />
    </div>
}

component App() {
    <Counter />
}
```

## 🎓 How It Works

1. **Props Definition**: Declare props in component signature with types
2. **Props Passing**: Pass props as attributes in JSX
3. **Props Destructuring**: Generated code automatically destructures
4. **Type Safety**: Type annotations provide documentation

## ✅ What Works

- ✅ String, Number, Boolean props
- ✅ Function props (arrow functions)
- ✅ Inline arrow function props
- ✅ Multiple props
- ✅ Reactive expressions in prop values
- ✅ Type annotations
- ✅ Optional return types
- ✅ Default parameters in generated code
- ✅ Prop forwarding (passing props through components)

## 📚 See Also

- [README.md](README.md) - Full project documentation
- [CONTRIBUTING.md](CONTRIBUTING.md) - Development guide
- [examples/apps/](examples/apps/) - Working example applications

---

**Ready to use component props in production!** 🎉
