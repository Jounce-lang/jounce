# Jounce Starter Templates

**Production-ready starter templates** to kickstart your Jounce projects!

## Quick Start

```bash
# 1. Choose a template
cd templates/

# 2. Copy to your project
cp -r minimal-counter my-app

# 3. Build and run
cd my-app
jnc compile main.jnc
cd dist && node server.js

# 4. Open in browser
open http://localhost:3000
```

---

## 📚 Available Templates

### 1. Minimal Counter
**Your first Jounce app** - Learn the absolute basics!

- ✅ Reactive signals
- ✅ Event handlers
- ✅ JSX syntax
- ✅ CSS utilities

**Difficulty:** Beginner
**Time:** 5 minutes
**Lines:** 38

[View Template →](./minimal-counter/)

---

### 2. Todo App
**Full-featured todo list** - Master array operations!

- ✅ Add/Delete/Toggle todos
- ✅ Filter by status
- ✅ Computed values
- ✅ Array methods (map, filter)
- ✅ Keyboard shortcuts

**Difficulty:** Intermediate
**Time:** 15 minutes
**Lines:** 155

[View Template →](./todo-app/)

---

### 3. Form App
**Production-ready forms** - Handle validation like a pro!

- ✅ Multi-field forms
- ✅ Real-time validation
- ✅ Error messages
- ✅ Loading states
- ✅ Success confirmation

**Difficulty:** Intermediate
**Time:** 20 minutes
**Lines:** 280

[View Template →](./form-app/)

---

### 4. Dashboard
**Multi-component layout** - Build complex UIs!

- ✅ Component composition
- ✅ Grid layouts
- ✅ Responsive design
- ✅ Reusable components
- ✅ Props with types

**Difficulty:** Intermediate
**Time:** 15 minutes
**Lines:** 140

[View Template →](./dashboard/)

---

## 🎯 Choose Your Template

### I'm brand new to Jounce
→ Start with **Minimal Counter**
→ Then try **Todo App**

### I know the basics
→ Build a **Form App**
→ Or create a **Dashboard**

### I want to learn specific features
- **Reactivity & Signals** → Minimal Counter
- **Arrays & Filtering** → Todo App
- **Forms & Validation** → Form App
- **Components & Layout** → Dashboard

---

## 🚀 What You'll Learn

### All Templates Teach:
- ✅ Reactive state with `signal()`
- ✅ JSX syntax and rendering
- ✅ Event handling
- ✅ CSS utility classes
- ✅ Conditional rendering

### Advanced Templates Include:
- ✅ `computed()` for derived state
- ✅ Array methods (map, filter, reduce)
- ✅ Form validation patterns
- ✅ Component composition
- ✅ Loading and error states
- ✅ Responsive layouts

---

## 📖 Template Structure

Each template includes:

```
template-name/
├── main.jnc          # Main application code
├── README.md         # Detailed guide with examples
└── (optional)        # Additional files
```

Every README contains:
- 📝 **Quick Start** - Get running in 30 seconds
- 📚 **What You'll Learn** - Key concepts
- 💡 **Customization Ideas** - Ways to extend
- 🔧 **Advanced Patterns** - Level up your skills
- 🔗 **Learn More** - Links to docs

---

## 🛠️ Customization

All templates are **fully customizable**:

1. **Modify the code** - Change features, add new ones
2. **Update styling** - Use different colors, layouts
3. **Add more components** - Build modular UIs
4. **Connect to APIs** - Fetch real data
5. **Deploy to production** - Share your creation!

---

## 📚 Learning Path

**Week 1: Fundamentals**
1. Build Minimal Counter
2. Modify it (add +10 button, max/min values)
3. Try Todo App
4. Add search to Todo App

**Week 2: Forms & Validation**
1. Build Form App
2. Add custom validation
3. Create multi-step form
4. Add file upload

**Week 3: Layouts & Components**
1. Build Dashboard
2. Add more stats
3. Create custom components
4. Make it responsive

**Week 4: Your Own App**
1. Combine learnings
2. Build something unique
3. Add API integration
4. Deploy to production!

---

## 🎨 Styling

All templates use **Jounce CSS Utilities** (457 classes):

```jounce
<div class="container mx-auto p-8">
    <div class="card p-6 shadow-lg rounded-lg">
        <h1 class="text-3xl font-bold text-primary">
            Hello Jounce!
        </h1>
        <button class="btn btn-primary btn-lg rounded">
            Click Me
        </button>
    </div>
</div>
```

[View Full CSS Reference →](../docs/CSS_UTILITIES.md)

---

## 🤝 Contributing

Have a great template idea?

1. Create your template in `templates/your-template/`
2. Add a comprehensive README
3. Test it thoroughly
4. Submit a PR!

**Good template ideas:**
- Blog with markdown
- E-commerce cart
- Chat interface
- Data visualization
- Admin panel
- Auth flow

---

## 📦 Template Checklist

When creating templates, ensure:

- ✅ Code is well-commented
- ✅ Follows Jounce best practices
- ✅ README has clear examples
- ✅ Works out of the box
- ✅ Demonstrates specific features
- ✅ Includes customization ideas
- ✅ Has realistic use case

---

## 🔗 Resources

- [Jounce Documentation](../docs/)
- [CSS Utilities](../docs/CSS_UTILITIES.md)
- [Reactivity Guide](../docs/REACTIVITY.md)
- [Component Guide](../docs/COMPONENTS.md)
- [Example Apps](../examples/apps/)

---

## 💬 Need Help?

- 📖 Read the [Full Documentation](../docs/)
- 💡 Check [Example Apps](../examples/apps/)
- 🐛 [Report Issues](https://github.com/jounce/issues)
- 💬 [Join Community](https://discord.gg/jounce)

---

**Happy Building! 🎉**

Start with a template, make it your own, and build something amazing!
