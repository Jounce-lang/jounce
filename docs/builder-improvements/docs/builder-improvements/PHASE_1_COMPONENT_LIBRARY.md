# Phase 1: Component Library

**Timeline:** 3-4 weeks
**Effort:** Medium
**Impact:** High
**Dependencies:** None
**Status:** Not started

---

## 🎯 Goal

Build a comprehensive library of 50+ pre-built, production-ready Jounce components that users can copy-paste or import into their apps.

**Problem solved:** Users spend hours describing basic UI patterns ("make a button", "add a form", etc.)

**Solution:** Pre-built components they can customize instead of building from scratch.

---

## 📋 What We're Building

### **Three-Tier System:**

#### **Tier 1: Primitives** (Building blocks)
- Buttons (6 variants)
- Inputs (text, email, password, number, textarea)
- Select dropdowns
- Checkboxes & radios
- Labels & typography
- Icons
- Spinners/loaders

#### **Tier 2: Patterns** (Common combinations)
- Form groups (label + input + error)
- Card layouts
- Navigation bars
- Tabs
- Modals/dialogs
- Dropdowns/menus
- Alerts/notifications
- Badges & tags

#### **Tier 3: Templates** (Full apps)
- Login page
- Dashboard
- Landing page
- E-commerce product list
- Todo app
- Blog layout
- Settings page

---

## 🏗️ Architecture

### **Option A: Copy-Paste Snippets** (Recommended for MVP)

**Structure:**
```
components/
├── buttons/
│   ├── primary.jnc
│   ├── secondary.jnc
│   ├── danger.jnc
│   ├── outline.jnc
│   ├── ghost.jnc
│   └── link.jnc
├── inputs/
│   ├── text-input.jnc
│   ├── email-input.jnc
│   └── ...
└── templates/
    ├── login-page.jnc
    └── ...
```

**Usage:**
```bash
# User browses components on website
# Clicks "Copy Code"
# Pastes into their main.jnc
```

**Pros:**
- No package system needed
- Works immediately
- Easy to customize
- Inspired by shadcn/ui (very popular)

**Cons:**
- Not importable (can't update)
- Users must copy manually
- Harder to version

---

### **Option B: NPM Package** (Future enhancement)

**Structure:**
```bash
npm install @jounce/ui
```

**Usage:**
```jounce
import { PrimaryButton, TextInput, Card } from "@jounce/ui";

component App() {
    return <div>
        <Card>
            <TextInput label="Name" />
            <PrimaryButton>Submit</PrimaryButton>
        </Card>
    </div>;
}
```

**Pros:**
- Importable and version-controlled
- Easy to update
- Standard workflow

**Cons:**
- Requires package system in Jounce compiler
- Harder to customize
- Lock-in to library versions

---

### **Option C: Built-in to Compiler** (Advanced)

**Usage:**
```jounce
// Compiler recognizes built-in components
<button variant="primary">Click Me</button>
<input type="email" />
```

**Pros:**
- Zero setup
- Always available
- Optimized by compiler

**Cons:**
- Bloats compiler
- Hard to extend
- Less flexible

---

## 🎨 Component Design System

### **Core Principles:**

1. **Headless by default** - Structure, not style
2. **Customizable** - Easy to override colors, sizes
3. **Accessible** - ARIA labels, keyboard nav
4. **Responsive** - Mobile-first
5. **Themeable** - Light/dark mode support

### **Style Approach:**

**Option 1: Inline styles** (Keep it simple)
```jounce
<button style="background: #2d5a2d; color: white; padding: 12px 24px;">
    Click Me
</button>
```

**Option 2: CSS classes** (More flexible)
```jounce
<link rel="stylesheet" href="jounce-ui.css"/>
<button class="btn btn-primary">Click Me</button>
```

**Option 3: CSS-in-JS** (Most powerful)
```jounce
<style>
    .btn-primary {
        background: var(--color-primary);
        color: white;
        padding: var(--spacing-md);
        border-radius: var(--radius-md);
    }
</style>
<button class="btn-primary">Click Me</button>
```

**Recommendation:** Start with Option 2 (CSS classes), allow users to override.

---

## 📦 Deliverables

### **Week 1: Primitives** (Foundation)

**Components to build:**

#### **1. Buttons (6 variants)**

**Primary Button:**
```jounce
component PrimaryButton(props) {
    return <button
        class="btn btn-primary"
        onclick={props.onClick}
        disabled={props.disabled}
    >
        {props.children}
    </button>;
}

// Usage
<PrimaryButton onClick={handleClick}>Submit</PrimaryButton>
```

**CSS:**
```css
.btn {
    padding: 12px 24px;
    border-radius: 8px;
    font-size: 16px;
    font-weight: 600;
    cursor: pointer;
    border: none;
    transition: all 0.2s;
}

.btn-primary {
    background: #2d5a2d;
    color: white;
}

.btn-primary:hover {
    background: #1a3a1a;
    transform: translateY(-2px);
}

.btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}
```

**Variants:**
- `btn-primary` - Main actions
- `btn-secondary` - Less important actions
- `btn-danger` - Destructive actions
- `btn-outline` - Outlined style
- `btn-ghost` - Minimal style
- `btn-link` - Text-only link style

**Sizes:**
- `btn-sm` - Small (10px 20px)
- `btn-md` - Medium (12px 24px) [default]
- `btn-lg` - Large (16px 32px)
- `btn-xl` - Extra large (20px 40px)

---

#### **2. Inputs (5 types)**

**Text Input:**
```jounce
component TextInput(props) {
    let value = signal(props.value || "");

    return <div class="input-group">
        {props.label ? <label class="input-label">{props.label}</label> : <span></span>}
        <input
            type="text"
            class="input"
            value={value.value}
            placeholder={props.placeholder}
            oninput={(e) => { value.value = e.target.value; }}
        />
        {props.error ? <span class="input-error">{props.error}</span> : <span></span>}
    </div>;
}

// Usage
<TextInput
    label="Name"
    placeholder="Enter your name"
    error={nameError.value}
/>
```

**Types:**
- Text input
- Email input (with validation)
- Password input (with show/hide toggle)
- Number input (with +/- buttons)
- Textarea (multiline)

---

#### **3. Select & Checkbox**

**Select Dropdown:**
```jounce
component Select(props) {
    let selected = signal(props.value || "");

    return <div class="input-group">
        {props.label ? <label class="input-label">{props.label}</label> : <span></span>}
        <select
            class="select"
            value={selected.value}
            onchange={(e) => { selected.value = e.target.value; }}
        >
            {props.options.map(opt =>
                <option value={opt.value}>{opt.label}</option>
            )}
        </select>
    </div>;
}

// Usage
<Select
    label="Country"
    options={[
        { value: "us", label: "United States" },
        { value: "ca", label: "Canada" }
    ]}
/>
```

**Checkbox:**
```jounce
component Checkbox(props) {
    let checked = signal(props.checked || false);

    return <label class="checkbox-label">
        <input
            type="checkbox"
            class="checkbox"
            checked={checked.value}
            onchange={(e) => { checked.value = e.target.checked; }}
        />
        <span>{props.label}</span>
    </label>;
}
```

---

### **Week 2: Patterns** (Combinations)

#### **4. Card Component**

```jounce
component Card(props) {
    return <div class="card">
        {props.title ? <div class="card-header">
            <h3 class="card-title">{props.title}</h3>
        </div> : <span></span>}
        <div class="card-body">
            {props.children}
        </div>
        {props.footer ? <div class="card-footer">
            {props.footer}
        </div> : <span></span>}
    </div>;
}

// Usage
<Card
    title="User Profile"
    footer={<PrimaryButton>Save</PrimaryButton>}
>
    <p>Card content here</p>
</Card>
```

**CSS:**
```css
.card {
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    overflow: hidden;
}

.card-header {
    padding: 20px;
    border-bottom: 1px solid #e5e5e5;
}

.card-title {
    margin: 0;
    font-size: 20px;
    font-weight: 600;
}

.card-body {
    padding: 20px;
}

.card-footer {
    padding: 20px;
    border-top: 1px solid #e5e5e5;
    background: #f9f9f9;
}
```

---

#### **5. Modal/Dialog**

```jounce
component Modal(props) {
    let isOpen = signal(props.open || false);

    let close = || {
        isOpen.value = false;
        if (props.onClose) {
            props.onClose();
        }
    };

    return isOpen.value ? <div class="modal-overlay" onclick={close}>
        <div class="modal-content" onclick={(e) => { e.stopPropagation(); }}>
            <div class="modal-header">
                <h2>{props.title}</h2>
                <button class="modal-close" onclick={close}>×</button>
            </div>
            <div class="modal-body">
                {props.children}
            </div>
            {props.footer ? <div class="modal-footer">
                {props.footer}
            </div> : <span></span>}
        </div>
    </div> : <span></span>;
}

// Usage
<Modal
    open={showModal.value}
    title="Confirm Action"
    onClose={() => { showModal.value = false; }}
    footer={
        <div>
            <SecondaryButton onclick={() => { showModal.value = false; }}>
                Cancel
            </SecondaryButton>
            <PrimaryButton onclick={handleConfirm}>
                Confirm
            </PrimaryButton>
        </div>
    }
>
    <p>Are you sure you want to delete this?</p>
</Modal>
```

---

#### **6. Navigation Bar**

```jounce
component Navbar(props) {
    return <nav class="navbar">
        <div class="navbar-container">
            <div class="navbar-brand">
                {props.logo}
            </div>
            <div class="navbar-menu">
                {props.children}
            </div>
            <div class="navbar-actions">
                {props.actions}
            </div>
        </div>
    </nav>;
}

component NavLink(props) {
    return <a href={props.href} class="nav-link">
        {props.children}
    </a>;
}

// Usage
<Navbar
    logo={<h1>MyApp</h1>}
    actions={
        <div>
            <PrimaryButton>Sign In</PrimaryButton>
        </div>
    }
>
    <NavLink href="/">Home</NavLink>
    <NavLink href="/about">About</NavLink>
    <NavLink href="/contact">Contact</NavLink>
</Navbar>
```

---

### **Week 3: Templates** (Full pages)

#### **7. Login Page**

```jounce
component LoginPage() {
    let email = signal("");
    let password = signal("");
    let error = signal("");
    let loading = signal(false);

    let handleSubmit = || {
        if (email.value == "" || password.value == "") {
            error.value = "Please fill in all fields";
            return;
        }

        loading.value = true;
        // Simulate API call
        // In real app: fetch("/api/login", { ... })
    };

    return <div class="login-page">
        <Card title="Sign In" class="login-card">
            {error.value != "" ? <Alert variant="danger">{error.value}</Alert> : <span></span>}

            <TextInput
                label="Email"
                type="email"
                value={email.value}
                placeholder="you@example.com"
            />

            <PasswordInput
                label="Password"
                value={password.value}
                placeholder="Enter your password"
            />

            <PrimaryButton
                onclick={handleSubmit}
                disabled={loading.value}
                fullWidth={true}
            >
                {loading.value ? "Signing in..." : "Sign In"}
            </PrimaryButton>

            <div class="login-footer">
                <a href="/forgot-password">Forgot password?</a>
                <span>Don't have an account? <a href="/signup">Sign up</a></span>
            </div>
        </Card>
    </div>;
}
```

---

#### **8. Dashboard Template**

```jounce
component Dashboard() {
    let stats = signal([
        { label: "Total Users", value: "1,234", change: "+12%" },
        { label: "Revenue", value: "$45,678", change: "+8%" },
        { label: "Active Sessions", value: "567", change: "-3%" }
    ]);

    return <div class="dashboard">
        <Navbar
            logo={<h1>Dashboard</h1>}
            actions={
                <div>
                    <Avatar name="John Doe" />
                </div>
            }
        />

        <div class="dashboard-content">
            <Sidebar>
                <NavLink href="/dashboard">Overview</NavLink>
                <NavLink href="/users">Users</NavLink>
                <NavLink href="/analytics">Analytics</NavLink>
                <NavLink href="/settings">Settings</NavLink>
            </Sidebar>

            <main class="dashboard-main">
                <h1>Overview</h1>

                <div class="stats-grid">
                    {stats.value.map(stat =>
                        <StatCard
                            label={stat.label}
                            value={stat.value}
                            change={stat.change}
                        />
                    )}
                </div>

                <div class="dashboard-grid">
                    <Card title="Recent Activity">
                        <ActivityList />
                    </Card>

                    <Card title="Revenue Chart">
                        <RevenueChart />
                    </Card>
                </div>
            </main>
        </div>
    </div>;
}
```

---

### **Week 4: Documentation + Examples Site**

#### **Build Examples Website:**

**Structure:**
```
jounce-components.com/
├── /                      # Homepage
├── /components            # Component browser
│   ├── /buttons          # Button variants
│   ├── /inputs           # Input types
│   ├── /cards            # Card patterns
│   └── ...
├── /templates            # Full page templates
│   ├── /login
│   ├── /dashboard
│   └── ...
└── /docs                 # Usage guides
```

**Component Page Example:**
```html
<!-- /components/buttons -->
<h1>Buttons</h1>

<section>
    <h2>Primary Button</h2>

    <!-- Live preview -->
    <div class="preview">
        <button class="btn btn-primary">Click Me</button>
    </div>

    <!-- Code to copy -->
    <pre><code>
&lt;PrimaryButton onClick={handleClick}&gt;
    Click Me
&lt;/PrimaryButton&gt;
    </code></pre>

    <button class="copy-btn">Copy Code</button>

    <!-- Props documentation -->
    <table>
        <tr>
            <th>Prop</th>
            <th>Type</th>
            <th>Default</th>
            <th>Description</th>
        </tr>
        <tr>
            <td>onClick</td>
            <td>function</td>
            <td>-</td>
            <td>Click handler</td>
        </tr>
        <tr>
            <td>disabled</td>
            <td>boolean</td>
            <td>false</td>
            <td>Disable button</td>
        </tr>
    </table>
</section>
```

---

## 🎨 Theming System

### **CSS Variables for Easy Customization:**

```css
:root {
    /* Colors */
    --color-primary: #2d5a2d;
    --color-secondary: #d4af37;
    --color-danger: #c41e3a;
    --color-success: #28a745;
    --color-warning: #ffc107;
    --color-info: #17a2b8;

    /* Grays */
    --color-gray-50: #f9f9f9;
    --color-gray-100: #f5f5f5;
    --color-gray-200: #e5e5e5;
    --color-gray-300: #d4d4d4;
    --color-gray-700: #404040;
    --color-gray-900: #171717;

    /* Spacing */
    --spacing-xs: 4px;
    --spacing-sm: 8px;
    --spacing-md: 12px;
    --spacing-lg: 16px;
    --spacing-xl: 24px;

    /* Border radius */
    --radius-sm: 4px;
    --radius-md: 8px;
    --radius-lg: 12px;
    --radius-xl: 16px;

    /* Shadows */
    --shadow-sm: 0 1px 2px rgba(0,0,0,0.05);
    --shadow-md: 0 2px 8px rgba(0,0,0,0.1);
    --shadow-lg: 0 8px 20px rgba(0,0,0,0.15);

    /* Typography */
    --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    --font-serif: Georgia, serif;
    --font-mono: "SF Mono", Consolas, monospace;
}
```

**Users can override:**
```css
/* In their own CSS */
:root {
    --color-primary: #1e40af;  /* Change to blue */
    --radius-md: 4px;          /* Less rounded */
}
```

---

## 📚 Component Checklist

### **Primitives (20 components):**
- [ ] Primary Button
- [ ] Secondary Button
- [ ] Danger Button
- [ ] Outline Button
- [ ] Ghost Button
- [ ] Link Button
- [ ] Text Input
- [ ] Email Input
- [ ] Password Input
- [ ] Number Input
- [ ] Textarea
- [ ] Select Dropdown
- [ ] Checkbox
- [ ] Radio Button
- [ ] Toggle Switch
- [ ] Label
- [ ] Heading (h1-h6)
- [ ] Paragraph
- [ ] Icon wrapper
- [ ] Spinner/Loader

### **Patterns (20 components):**
- [ ] Card
- [ ] Modal/Dialog
- [ ] Alert/Notification
- [ ] Toast
- [ ] Badge
- [ ] Tag
- [ ] Avatar
- [ ] Navbar
- [ ] Sidebar
- [ ] Footer
- [ ] Tabs
- [ ] Accordion
- [ ] Dropdown Menu
- [ ] Breadcrumbs
- [ ] Pagination
- [ ] Table
- [ ] List
- [ ] Divider
- [ ] Progress Bar
- [ ] Skeleton Loader

### **Templates (10 templates):**
- [ ] Login Page
- [ ] Signup Page
- [ ] Dashboard
- [ ] Landing Page
- [ ] Blog Post
- [ ] E-commerce Product List
- [ ] E-commerce Checkout
- [ ] Settings Page
- [ ] Profile Page
- [ ] 404 Error Page

---

## 🧪 Testing Strategy

### **Each component needs:**

1. **Visual test** - Does it look right?
2. **Responsive test** - Works on mobile?
3. **Accessibility test** - Keyboard nav, ARIA labels?
4. **Customization test** - Can users override styles?
5. **Integration test** - Works in real app?

### **Testing tools:**

```bash
# Visual regression testing
npm install @playwright/test

# Accessibility testing
npm install @axe-core/playwright

# Responsive testing
# Use browser DevTools + real devices
```

---

## 🚀 Launch Strategy

### **Phase 1A: Soft Launch** (Week 4)
- Release 20 components
- Share with 10 beta users
- Gather feedback
- Iterate

### **Phase 1B: Public Launch** (Week 5)
- Release all 50 components
- Launch examples website
- Announce on Twitter, Reddit
- Write blog post

### **Phase 1C: Community** (Week 6+)
- Accept component contributions
- Build community templates
- Create video tutorials
- Expand library based on requests

---

## 📊 Success Metrics

**Week 4 goals:**
- 50 components built
- Examples site live
- 10 beta testers using it

**Month 2 goals:**
- 100+ users using components
- 80% of common UI patterns covered
- 50% reduction in "how do I build..." questions

**Month 3 goals:**
- Community contributing components
- 500+ weekly visitors to examples site
- Components used in 90% of new Jounce apps

---

## 🔗 Next Steps

After Phase 1 completes:
1. Analyze which components are most used
2. Identify patterns users still build from scratch
3. Start Phase 2 (Hot Reload) to speed up iteration
4. Prepare for Phase 3 (Visual Builder) using component library

---

## 📝 Open Questions

1. **Naming convention?**
   - `PrimaryButton` vs `Button variant="primary"`
   - `TextInput` vs `Input type="text"`

2. **File organization?**
   - One big file vs separate files per component?
   - Monorepo or separate repos?

3. **Styling approach?**
   - Inline styles vs CSS classes vs CSS-in-JS?
   - Tailwind-like utilities?

4. **Documentation format?**
   - MDX files?
   - Storybook-style?
   - Custom site?

**Decision needed before Week 1 starts!**

---

## 🎯 Ready to Start?

**First task:** Build the 6 button variants (1-2 days)
**Then:** Get feedback before building more

See [IMPLEMENTATION_NOTES.md](./IMPLEMENTATION_NOTES.md) for coding tips!
