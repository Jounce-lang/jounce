# Lesson 3: JSX Basics

**Time**: 5 minutes
**Difficulty**: Beginner

## 🎯 Goal

Learn how to build UI with JSX - HTML-like syntax in your Jounce code.

## 📝 Instructions

1. **Add a paragraph**: Replace the first TODO comment with:
   ```jounce
   <p class="mb-4">This is a paragraph!</p>
   ```

2. **Add a button**: Replace the second TODO comment with:
   ```jounce
   <button class="btn btn-primary">Click Me</button>
   ```

3. **Run your code** and see the UI update!

## 💡 What You'll Learn

- How to write JSX (HTML-like syntax)
- How to nest elements
- How to use CSS classes
- Basic HTML elements: `<p>`, `<button>`, `<div>`, `<h1>`

## ✅ Expected Output

You should see:
- A heading: "My First App"
- A paragraph: "This is a paragraph!"
- A blue button: "Click Me"

## 🎓 Concept: JSX (JavaScript XML)

JSX looks like HTML but is actually JavaScript. It makes building UIs intuitive and declarative.

### HTML vs JSX

**HTML** (old way):
```html
<div id="app">
  <h1>Hello</h1>
</div>
```

**JSX** (Jounce way):
```jounce
<div id="app">
  <h1>Hello</h1>
</div>
```

They look the same, but JSX is embedded in your code!

### Common JSX Elements

```jounce
<h1>Heading 1</h1>
<h2>Heading 2</h2>
<p>Paragraph</p>
<button>Button</button>
<div>Container</div>
<span>Inline text</span>
<input type="text" />
<img src="image.jpg" />
```

### Self-Closing Tags

Some elements don't have content and use self-closing syntax:

```jounce
<br />          <!-- Line break -->
<hr />          <!-- Horizontal rule -->
<input />       <!-- Input field -->
<img src="..." />  <!-- Image -->
```

### Nesting Elements

Elements can be nested inside each other:

```jounce
<div>
  <h1>Title</h1>
  <p>Content</p>
</div>
```

### CSS Classes

Add styling with the `class` attribute:

```jounce
<button class="btn btn-primary">Click Me</button>
```

Jounce includes 457 utility classes out of the box!

## 🚀 Try It Yourself

After completing this lesson, try:

1. **Add an image**:
```jounce
<img src="https://via.placeholder.com/150" class="rounded" />
```

2. **Create a list**:
```jounce
<ul>
  <li>Item 1</li>
  <li>Item 2</li>
  <li>Item 3</li>
</ul>
```

3. **Add more buttons with different styles**:
```jounce
<button class="btn btn-secondary">Secondary</button>
<button class="btn btn-success">Success</button>
<button class="btn btn-danger">Danger</button>
```

## ❓ Troubleshooting

**Error: Unexpected token**
→ Make sure all tags are properly closed: `<p></p>` or `<br />`

**Nothing shows up**
→ Check that elements are inside the `<div>` container

**Styling doesn't work**
→ Make sure class names are in quotes: `class="btn"`

## 🎯 Challenge

Can you create a card layout with:
- A heading
- A paragraph
- Two buttons

```jounce
<div class="card p-6">
  <h2 class="text-2xl mb-2">Card Title</h2>
  <p class="mb-4">Card description goes here.</p>
  <button class="btn btn-primary mr-2">Action</button>
  <button class="btn btn-secondary">Cancel</button>
</div>
```

## ➡️ Next Lesson

Awesome! 🎉 You can now build UIs with JSX. In the next lesson, you'll learn how to make your UI **interactive** with event handlers.

[Continue to Lesson 4: Event Handlers →]
