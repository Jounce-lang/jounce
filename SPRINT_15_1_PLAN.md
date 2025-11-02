# Sprint 15.1: Interactive Tutorial System

**Duration**: November 1-7, 2025 (1 week)
**Goal**: Users go from zero to first app in under 1 hour
**Release**: Part of v0.9.0 "Super Easy Start"

---

## 🎯 Objective

Build **tutorial.jounce.dev** - an interactive, in-browser tutorial that teaches Jounce in 10 progressive lessons (50 minutes total).

**Success Metrics**:
- ✅ 10,000+ tutorial completions in first month
- ✅ 90%+ completion rate (users finish all 10 lessons)
- ✅ Average time: 50-60 minutes
- ✅ User satisfaction: 9/10 or higher

---

## 📋 Deliverables Checklist

### Core Infrastructure
- [ ] Set up tutorial.jounce.dev domain
- [ ] Build web app with Monaco editor
- [ ] Implement live preview pane (split screen)
- [ ] Add Jounce compiler (WASM or API)
- [ ] Progress tracking (local storage)
- [ ] Auto-save functionality

### 10 Progressive Lessons
- [ ] Lesson 1: Hello World (2 mins)
- [ ] Lesson 2: Variables & Signals (5 mins)
- [ ] Lesson 3: JSX Basics (5 mins)
- [ ] Lesson 4: Event Handlers (5 mins)
- [ ] Lesson 5: Reactive State (5 mins)
- [ ] Lesson 6: Components (5 mins)
- [ ] Lesson 7: Props & Composition (5 mins)
- [ ] Lesson 8: Styling (5 mins)
- [ ] Lesson 9: Forms & Validation (5 mins)
- [ ] Lesson 10: Deploy Your App (5 mins)

### Completion & Rewards
- [ ] "Jounce Certified Developer" badge
- [ ] Certificate generation (PDF)
- [ ] Share on social media (auto-generated image)
- [ ] Leaderboard (optional)

---

## 🏗️ Technical Architecture

### Frontend Stack
```
tutorial.jounce.dev
├── Framework: SolidJS (lightweight, reactive)
├── Editor: Monaco Editor (VS Code engine)
├── Styling: Tailwind CSS + Jounce utilities
├── Compiler: Jounce WASM module or API endpoint
├── Preview: Iframe with hot reload
└── State: Solid signals + localStorage
```

### Backend (Optional)
```
api.tutorial.jounce.dev
├── Framework: Node.js + Express (or serverless)
├── Database: PostgreSQL (user progress, analytics)
├── Auth: GitHub OAuth (optional, for saving progress)
└── CDN: Cloudflare (static assets)
```

### Infrastructure
- **Hosting**: Vercel or Cloudflare Pages
- **Domain**: tutorial.jounce.dev
- **SSL**: Automatic (Vercel/Cloudflare)
- **Analytics**: Plausible (privacy-friendly)

---

## 📚 Lesson Plan (Detailed)

### Lesson 1: Hello World (2 mins)

**Goal**: Write your first Jounce program

**Starting Code**:
```jounce
// Welcome to Jounce! Let's write your first program.
// Replace this comment with: console.log("Hello, Jounce!");

```

**Instructions**:
1. Delete the comment
2. Type: `console.log("Hello, Jounce!");`
3. Click "Run" to see the output

**Learning Outcomes**:
- Understand the editor
- See live output
- Feel the instant feedback loop

**Validation**: Check for `console.log("Hello, Jounce!")` in code

---

### Lesson 2: Variables & Signals (5 mins)

**Goal**: Learn reactive state with signals

**Starting Code**:
```jounce
// Signals are reactive variables that automatically update the UI
// Create a signal: let name = createSignal("World");

let name = createSignal("Your Name");

component App() {
    <div>
        <h1>Hello, {name.value}!</h1>
    </div>
}
```

**Instructions**:
1. Change "Your Name" to your actual name
2. See the UI update automatically
3. Understand that `.value` accesses the signal's value

**Learning Outcomes**:
- Create signals with `createSignal()`
- Access signal values with `.value`
- See reactivity in action

**Validation**: Check for modified signal value

---

### Lesson 3: JSX Basics (5 mins)

**Goal**: Build UI with JSX

**Starting Code**:
```jounce
component App() {
    <div>
        <h1>My First App</h1>
        <!-- TODO: Add a paragraph below -->
    </div>
}
```

**Instructions**:
1. Add `<p>This is a paragraph!</p>` below the `<h1>`
2. Try adding a `<button>Click Me</button>`
3. Experiment with nesting elements

**Learning Outcomes**:
- JSX is HTML-like syntax
- Elements can be nested
- Self-closing tags work (`<br />`)

**Validation**: Check for `<p>` tag in JSX

---

### Lesson 4: Event Handlers (5 mins)

**Goal**: Handle user interactions

**Starting Code**:
```jounce
component App() {
    let count = createSignal(0);

    <!-- TODO: Add onClick handler to button -->
    <div>
        <h1>Count: {count.value}</h1>
        <button>Increment</button>
    </div>
}
```

**Instructions**:
1. Add `onClick={() => count.set(count.value + 1)}` to the button
2. Click the button and watch the count increase
3. Try adding a "Decrement" button

**Learning Outcomes**:
- `onClick` handlers
- Update signals with `.set()`
- Lambda expressions `() => {}`

**Validation**: Check for `onClick` attribute

---

### Lesson 5: Reactive State (5 mins)

**Goal**: Master reactivity patterns

**Starting Code**:
```jounce
component App() {
    let count = createSignal(0);
    let doubled = computed(() => count.value * 2);

    <div>
        <h1>Count: {count.value}</h1>
        <p>Doubled: {doubled.value}</p>
        <button onClick={() => count.set(count.value + 1)}>+1</button>
    </div>
}
```

**Instructions**:
1. Click the button and see both values update
2. Add a `tripled` computed value
3. Understand that computed values update automatically

**Learning Outcomes**:
- `computed()` creates derived values
- Reactivity graph updates automatically
- No manual subscriptions needed

**Validation**: Check for `computed()` usage

---

### Lesson 6: Components (5 mins)

**Goal**: Create reusable components

**Starting Code**:
```jounce
component Button() {
    <button class="btn btn-primary">
        Click Me
    </button>
}

component App() {
    <div>
        <h1>My App</h1>
        <!-- TODO: Use the Button component here -->
    </div>
}
```

**Instructions**:
1. Add `<Button />` inside the `<div>`
2. Create a new `Card` component
3. Use your Card component in App

**Learning Outcomes**:
- Components are reusable
- Self-closing component syntax
- Component composition

**Validation**: Check for `<Button />` in App

---

### Lesson 7: Props & Composition (5 mins)

**Goal**: Pass data to components

**Starting Code**:
```jounce
component Card(title: String, description: String) {
    <div class="card p-4">
        <h2>{title}</h2>
        <p>{description}</p>
    </div>
}

component App() {
    <div>
        <!-- TODO: Add Card with title and description props -->
    </div>
}
```

**Instructions**:
1. Add `<Card title="Welcome" description="This is my card" />`
2. Create multiple cards with different props
3. Understand prop passing

**Learning Outcomes**:
- Components accept props
- Props are typed with `: String`
- Props are accessed by name

**Validation**: Check for `<Card` with props

---

### Lesson 8: Styling (5 mins)

**Goal**: Style components with CSS

**Starting Code**:
```jounce
component App() {
    <div class="container mx-auto p-8">
        <h1 class="text-3xl font-bold text-primary">
            Styled App
        </h1>
    </div>
}

style App {
    background-color: #f5f5f5;
    min-height: 100vh;
}
```

**Instructions**:
1. Modify the style block colors
2. Add `:hover` styles to the h1
3. Try utility classes (text-xl, p-4, rounded)

**Learning Outcomes**:
- `style` blocks are component-scoped
- Utility classes available out-of-box
- Pseudo-classes (`:hover`) work

**Validation**: Check for `style` block

---

### Lesson 9: Forms & Validation (5 mins)

**Goal**: Build interactive forms

**Starting Code**:
```jounce
component App() {
    let name = createSignal("");
    let email = createSignal("");

    let handleSubmit = () => {
        console.log("Name:", name.value);
        console.log("Email:", email.value);
    };

    <form onSubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
        <input
            type="text"
            placeholder="Name"
            value={name.value}
            onInput={(e) => name.set(e.target.value)}
        />
        <!-- TODO: Add email input -->
        <button type="submit">Submit</button>
    </form>
}
```

**Instructions**:
1. Add email input (copy pattern from name input)
2. Fill the form and submit
3. Check console for output

**Learning Outcomes**:
- Two-way binding with `value` + `onInput`
- Form submission with `e.preventDefault()`
- Console logging

**Validation**: Check for email input

---

### Lesson 10: Deploy Your App (5 mins)

**Goal**: Ship to production!

**Starting Code**:
```jounce
component App() {
    let deployed = createSignal(false);

    <div class="container mx-auto p-8 text-center">
        <h1 class="text-4xl font-bold mb-4">
            🎉 Congratulations!
        </h1>
        <p class="mb-8">
            You've completed the Jounce tutorial!
        </p>
        {deployed.value ? (
            <div class="success">
                <h2>Your app is live! 🚀</h2>
                <a href="#">View your app →</a>
            </div>
        ) : (
            <button onClick={() => deployed.set(true)} class="btn btn-primary btn-lg">
                Deploy to Production
            </button>
        )}
    </div>
}
```

**Instructions**:
1. Click "Deploy to Production"
2. Get your "Jounce Certified Developer" badge
3. Share your achievement!

**Learning Outcomes**:
- You can build full apps with Jounce
- Deployment is one click
- You're ready to build real projects

**Validation**: User clicks deploy button

---

## 🎨 UI/UX Design

### Layout
```
┌─────────────────────────────────────────────────────────┐
│  Tutorial.Jounce.dev                    [Progress: 3/10] │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  Lesson 3: JSX Basics                                    │
│  ────────────────────────────────────────────────────   │
│                                                           │
│  Build UI with JSX. JSX is HTML-like syntax...           │
│                                                           │
│  Instructions:                                            │
│  1. Add a paragraph below the h1                         │
│  2. Try adding a button                                  │
│                                                           │
├──────────────────────────┬──────────────────────────────┤
│                          │                              │
│  CODE EDITOR             │    LIVE PREVIEW              │
│  (Monaco)                │    (Iframe)                  │
│                          │                              │
│  component App() {       │    ┌──────────────────┐     │
│    <div>                 │    │ My First App     │     │
│      <h1>My First App    │    │                  │     │
│      <!-- Add <p> here   │    │ [Your output]    │     │
│    </div>                │    │                  │     │
│  }                       │    └──────────────────┘     │
│                          │                              │
└──────────────────────────┴──────────────────────────────┘
│  [Previous]  [Reset Code]  [Hint]       [Next Lesson →] │
└─────────────────────────────────────────────────────────┘
```

### Color Scheme
- **Primary**: #3b82f6 (blue)
- **Success**: #10b981 (green)
- **Background**: #ffffff (white)
- **Editor**: #1e1e1e (dark mode)
- **Preview**: #f9fafb (light gray)

### Responsive Design
- **Desktop**: Split-pane (50/50)
- **Tablet**: Tabs (Code/Preview)
- **Mobile**: Stacked (Code on top, Preview below)

---

## 🔧 Technical Implementation

### Phase 1: Setup (Day 1)
- [ ] Set up domain (tutorial.jounce.dev)
- [ ] Create SolidJS app
- [ ] Install Monaco Editor
- [ ] Configure Tailwind CSS
- [ ] Deploy to Vercel

### Phase 2: Core Features (Day 2-3)
- [ ] Implement split-pane layout
- [ ] Integrate Monaco with syntax highlighting
- [ ] Add iframe preview with hot reload
- [ ] Implement progress tracking
- [ ] Add localStorage persistence

### Phase 3: Lessons (Day 4-5)
- [ ] Write all 10 lesson content
- [ ] Create starter code for each lesson
- [ ] Write validation logic
- [ ] Add hints system
- [ ] Test each lesson flow

### Phase 4: Polish (Day 6-7)
- [ ] Add animations and transitions
- [ ] Implement certificate generation
- [ ] Add social sharing
- [ ] Performance optimization
- [ ] User testing and fixes

---

## 📊 Analytics & Tracking

Track these metrics:
- **Lesson completion rate** (per lesson)
- **Time spent per lesson**
- **Drop-off points** (where users quit)
- **Error frequency** (syntax errors)
- **Device breakdown** (desktop/tablet/mobile)
- **Completion rate** (% who finish all 10)

**Tools**: Plausible Analytics or PostHog

---

## 🚀 Deployment

### Hosting Options

**Option 1: Vercel (Recommended)**
- Automatic SSL
- Edge network (fast globally)
- Zero config
- Free tier sufficient

**Option 2: Cloudflare Pages**
- Same benefits as Vercel
- Cloudflare CDN
- R2 storage option

### Deployment Steps
```bash
# 1. Build the app
npm run build

# 2. Deploy to Vercel
vercel --prod

# 3. Configure domain
vercel domains add tutorial.jounce.dev
```

---

## ✅ Definition of Done

Sprint 15.1 is complete when:

- [ ] tutorial.jounce.dev is live
- [ ] All 10 lessons are working
- [ ] Code editor has syntax highlighting
- [ ] Live preview updates in real-time
- [ ] Progress is saved (localStorage)
- [ ] Certificate is generated on completion
- [ ] Mobile responsive (works on all devices)
- [ ] Performance: Page loads in < 2 seconds
- [ ] No critical bugs
- [ ] 5 beta testers complete successfully

---

## 🎯 Success Criteria

**Week 1 Metrics**:
- 100+ tutorial starts
- 70%+ completion rate
- 9/10 average rating
- <1 second live preview latency

**Month 1 Metrics**:
- 10,000+ tutorial completions
- 90%+ completion rate
- Featured on Hacker News, Reddit, Twitter
- 1,000+ certificates generated

---

## 📝 Next Steps After Sprint 15.1

Once interactive tutorial is live, move to Sprint 15.2:
- YouTube channel with video tutorials
- "Jounce in 100 Seconds" viral video
- 4 live coding sessions
- Target: 10,000+ views in first month

---

**Created**: November 1, 2025
**Sprint**: 15.1 (Week 1 of Phase 15)
**Owner**: Jounce Core Team
**Status**: Ready to start
