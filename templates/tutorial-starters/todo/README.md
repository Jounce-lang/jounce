# Todo App Template

A full-featured todo list application built with Jounce.

## What's Included

- ✅ Add new todos
- ✅ Mark todos as complete
- ✅ Delete todos
- ✅ Count remaining items
- ✅ Empty state handling
- ✅ Keyboard shortcuts (Enter to add)

## Features

**State Management**:
- Array of todos with signals
- Computed value for remaining count

**User Interactions**:
- Text input with two-way binding
- Checkbox toggling
- Delete button
- Enter key to submit

**UI Polish**:
- Conditional rendering for empty state
- Strike-through for completed items
- Responsive layout
- Clean, modern design

## Getting Started

```bash
# Compile the app
jnc compile main.jnc

# Run development server
jnc dev

# Open http://localhost:3000
```

## What You'll Learn

- Managing arrays with signals
- Array methods (map, filter)
- Object spreading and immutability
- Computed values
- Conditional rendering
- Event handling (click, input, keypress)
- Dynamic CSS classes

## Customize It!

Try these challenges:

1. **Add filtering** - Show all/active/completed todos
2. **Add local storage** - Persist todos between sessions
3. **Add editing** - Double-click to edit todo text
4. **Add priorities** - High/medium/low priority levels
5. **Add due dates** - Calendar picker for deadlines
6. **Add categories** - Group todos by project/tag

## Learn More

- [Signals Guide](https://docs.jounce.dev/guide/signals)
- [Array Methods](https://docs.jounce.dev/guide/arrays)
- [Computed Values](https://docs.jounce.dev/guide/computed)
- [Tutorial](https://tutorial.jounce.dev)

## Deploy

```bash
# Deploy to production
jnc deploy --platform vercel
```

**Happy coding!** 🚀
