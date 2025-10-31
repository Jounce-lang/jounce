# Phase 3: Visual Builder (Drag-and-Drop UI)

**Timeline:** 3-4 months
**Effort:** Very High
**Impact:** Revolutionary
**Dependencies:** Phase 2 (Hot Reload)
**Status:** Planning

---

## 🎯 Goal

Build a Figma-like visual interface where non-technical users can drag-and-drop components to build Jounce apps, which generate clean `.jnc` code.

**Problem solved:** Non-coders can't build apps. Even coders waste time writing boilerplate.

**Solution:** Visual canvas + property panels + code generation → anyone can build apps.

---

## 📋 What We're Building

### **The Visual Builder:**

```
┌──────────────────────────────────────────────────────────────┐
│  File  Edit  View  Components  Deploy                        │
├────────────┬─────────────────────────────────┬───────────────┤
│            │                                  │               │
│ Components │         Canvas                  │  Properties   │
│            │                                  │               │
│ ┌────────┐ │  ┌─────────────────────────┐   │ ┌───────────┐ │
│ │ Button │ │  │                         │   │ │ Text:     │ │
│ └────────┘ │  │  [Click Me]  ←─ Selected│   │ │ Click Me  │ │
│ ┌────────┐ │  │                         │   │ ├───────────┤ │
│ │  Input │ │  │                         │   │ │ Color:    │ │
│ └────────┘ │  │                         │   │ │ [🎨 Blue] │ │
│ ┌────────┐ │  │                         │   │ ├───────────┤ │
│ │  Card  │ │  │                         │   │ │ Size:     │ │
│ └────────┘ │  │                         │   │ │ Large ▼   │ │
│            │  └─────────────────────────┘   │ └───────────┘ │
│            │                                  │               │
├────────────┴─────────────────────────────────┴───────────────┤
│ [💾 Save]  [▶ Preview]  [📱 Responsive]  [</> View Code]   │
└──────────────────────────────────────────────────────────────┘
```

---

## 🏗️ Architecture

### **High-Level System:**

```
┌─────────────────────────────────────────────────────┐
│  Visual Builder (React/Svelte Frontend)            │
│  - Canvas for drag-and-drop                        │
│  - Component tree view                             │
│  - Property panels                                 │
│  - Live preview iframe                             │
└───────────────────┬─────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────────────────┐
│  Internal Representation (JSON Tree)                │
│  {                                                  │
│    type: "component",                               │
│    name: "App",                                     │
│    children: [                                      │
│      { type: "button", props: {...}, children: [...] }
│    ]                                                │
│  }                                                  │
└───────────────────┬─────────────────────────────────┘
                    │
                    ├──────────────────┬───────────────┐
                    ▼                  ▼               ▼
┌──────────────────────┐  ┌──────────────────┐  ┌───────────┐
│  Code Generator      │  │  Visual Renderer │  │  Jounce   │
│  (JSON → .jnc)       │  │  (JSON → DOM)    │  │  Compiler │
│  Generates clean     │  │  Shows preview   │  │  .jnc →   │
│  .jnc code           │  │  in iframe       │  │  working  │
└──────────────────────┘  └──────────────────┘  │  app      │
                                                 └───────────┘
```

---

## 🎨 Core Features

### **1. Component Palette**

**Categories:**
```
📦 Primitives
  - Button (6 variants)
  - Input (5 types)
  - Text (h1-p)
  - Image
  - Icon

📊 Layout
  - Container
  - Grid (2-12 columns)
  - Flex (row/column)
  - Stack
  - Spacer

🎴 Patterns
  - Card
  - Modal
  - Navbar
  - Form
  - Table

📱 Templates
  - Login Page
  - Dashboard
  - Landing
  - (from Phase 1)
```

**Drag-and-drop:**
```
User drags "Button" from palette
  → Cursor shows preview
  → Hover over canvas (shows drop zones)
  → Drop → Button appears
  → Automatically generated code updates
```

---

### **2. Properties Panel**

**Context-sensitive based on selection:**

**Button selected:**
```
┌──────────────────────┐
│ Properties           │
├──────────────────────┤
│ Text:                │
│ [Click Me________]   │
│                      │
│ Variant:             │
│ ( ) Primary          │
│ (•) Secondary        │
│ ( ) Danger           │
│                      │
│ Size:                │
│ [Small] [Med] [Lrg]  │
│                      │
│ Disabled:            │
│ [ ] Disable button   │
│                      │
│ On Click:            │
│ [handleClick____▼]   │
│                      │
│ Advanced ▼           │
│  - Custom CSS        │
│  - Animation         │
│  - Accessibility     │
└──────────────────────┘
```

**Input selected:**
```
┌──────────────────────┐
│ Properties           │
├──────────────────────┤
│ Label:               │
│ [Email Address___]   │
│                      │
│ Type:                │
│ [Email ▼]            │
│                      │
│ Placeholder:         │
│ [you@example.com_]   │
│                      │
│ Required:            │
│ [✓] Required field   │
│                      │
│ Validation:          │
│ [Email format____▼]  │
│                      │
│ Error Message:       │
│ [Invalid email___]   │
└──────────────────────┘
```

---

### **3. Canvas (Editing Surface)**

**Features:**
- Click to select elements
- Drag to reorder
- Resize with handles
- Alignment guides
- Responsive breakpoints (mobile/tablet/desktop views)
- Zoom in/out
- Grid/snap-to-grid
- Keyboard shortcuts (Cmd+C, Cmd+V, etc.)

**Visual feedback:**
```
Selected element:
┌─────────────────┐
│ [Click Me]      │  ← Blue border
└─────────────────┘
     ⊕  ⊖  ×         ← Resize handles

Hover state:
┌─────────────────┐
│ [Click Me]      │  ← Dashed border
└─────────────────┘
```

---

### **4. Layer Tree**

**Shows component hierarchy:**
```
┌──────────────────────┐
│ Layers               │
├──────────────────────┤
│ ▼ App                │
│   ├─ Header          │
│   │  ├─ Logo         │
│   │  └─ Nav          │
│   │     ├─ Link 1    │
│   │     ├─ Link 2    │
│   │     └─ Link 3    │
│   ├─ Main            │
│   │  ├─ Card ← Selected
│   │  │  ├─ Title    │
│   │  │  └─ Button   │
│   │  └─ ...         │
│   └─ Footer          │
└──────────────────────┘
```

**Interactions:**
- Click to select
- Drag to reorder
- Toggle visibility (👁️)
- Lock/unlock (🔒)
- Rename components

---

### **5. Code View**

**Toggle between visual and code:**
```
┌──────────────────────────────────┐
│ [🎨 Visual] [</> Code]          │
├──────────────────────────────────┤
│ component App() {                │
│     return <div class="app">     │
│         <Header />               │
│         <Main>                   │
│             <Card>               │
│                 <h2>Title</h2>   │
│                 <Button          │
│                     variant="pri...│
│                     onclick={...}│
│                 >                │
│                     Click Me     │
│                 </Button>        │
│             </Card>              │
│         </Main>                  │
│     </div>;                      │
│ }                                │
└──────────────────────────────────┘
```

**Two-way sync:**
- Edit visually → Code updates
- Edit code → Visual updates

---

## 🔧 Technical Implementation

### **Tech Stack Options:**

#### **Option A: React (Most Common)**

**Pros:**
- Huge ecosystem
- Drag-and-drop libraries (react-dnd, dnd-kit)
- Many UI builders use React (Webflow, Framer)

**Cons:**
- Larger bundle size
- Steeper learning curve

**Key Libraries:**
```json
{
  "dependencies": {
    "react": "^18.0.0",
    "react-dnd": "^16.0.0",      // Drag-and-drop
    "monaco-editor": "^0.44.0",   // Code editor
    "zustand": "^4.0.0",          // State management
    "react-resizable": "^3.0.0"   // Resize handles
  }
}
```

---

#### **Option B: Svelte (Recommended)**

**Pros:**
- Smaller bundle (~30KB vs 150KB)
- Simpler syntax
- Better performance
- Built-in reactivity (similar to Jounce!)

**Cons:**
- Smaller ecosystem
- Fewer drag-and-drop libs

**Key Libraries:**
```json
{
  "dependencies": {
    "svelte": "^4.0.0",
    "svelte-dnd-action": "^0.9.0",
    "monaco-editor": "^0.44.0",
    "svelte/store": "built-in"
  }
}
```

---

#### **Option C: Vanilla JS + Web Components**

**Pros:**
- No framework dependency
- Lightweight
- Portable

**Cons:**
- More manual work
- Reinventing wheels

**Recommendation:** Svelte (best balance)

---

### **Data Model:**

**Internal JSON Tree:**
```json
{
  "version": "1.0.0",
  "components": [
    {
      "id": "comp-1",
      "type": "App",
      "children": [
        {
          "id": "elem-1",
          "type": "button",
          "props": {
            "variant": "primary",
            "size": "large",
            "disabled": false,
            "onClick": "handleClick"
          },
          "children": [
            {
              "id": "text-1",
              "type": "text",
              "value": "Click Me"
            }
          ],
          "position": { "x": 100, "y": 200 },
          "style": {
            "width": "auto",
            "height": "auto"
          }
        }
      ]
    }
  ],
  "signals": [
    {
      "name": "count",
      "initialValue": 0
    }
  ],
  "functions": [
    {
      "name": "handleClick",
      "body": "count.value = count.value + 1;"
    }
  ]
}
```

---

### **Code Generator:**

**JSON → .jnc code:**

```typescript
// src/codegen/jounce-generator.ts
export function generateJounceCode(tree: ComponentTree): string {
    const signals = generateSignals(tree.signals);
    const functions = generateFunctions(tree.functions);
    const jsx = generateJSX(tree.components[0]);

    return `
component App() {
    ${signals}

    ${functions}

    return ${jsx};
}
    `.trim();
}

function generateSignals(signals: Signal[]): string {
    return signals.map(s =>
        `let ${s.name} = signal(${JSON.stringify(s.initialValue)});`
    ).join('\n    ');
}

function generateJSX(node: ComponentNode, indent = 0): string {
    const spaces = '    '.repeat(indent);

    if (node.type === 'text') {
        return node.value;
    }

    const props = Object.entries(node.props || {})
        .map(([key, value]) => {
            if (typeof value === 'string') {
                return `${key}="${value}"`;
            } else if (typeof value === 'boolean') {
                return value ? key : '';
            } else {
                return `${key}={${value}}`;
            }
        })
        .filter(Boolean)
        .join(' ');

    const children = node.children
        .map(child => generateJSX(child, indent + 1))
        .join('\n');

    if (children) {
        return `
${spaces}<${node.type}${props ? ' ' + props : ''}>
${children}
${spaces}</${node.type}>
        `.trim();
    } else {
        return `<${node.type}${props ? ' ' + props : ''} />`;
    }
}
```

**Example output:**
```jounce
component App() {
    let count = signal(0);

    let handleClick = || {
        count.value = count.value + 1;
    };

    return <div class="app">
        <button variant="primary" onclick={handleClick}>
            Count: {count.value}
        </button>
    </div>;
}
```

---

### **Reverse: Code → JSON Parser**

**Parse .jnc → JSON tree for editing:**

```typescript
// src/parser/jounce-parser.ts
export function parseJounceToTree(code: string): ComponentTree {
    // Reuse Jounce compiler parser (Rust)
    // Call via WASM or child process

    const ast = parseJounceCode(code); // From Rust compiler

    return {
        components: astToComponents(ast),
        signals: extractSignals(ast),
        functions: extractFunctions(ast)
    };
}
```

**This enables two-way sync:**
- User edits visually → generates code
- User edits code → updates visual

---

## 🎨 UI/UX Design

### **Design System:**

**Colors:**
```css
:root {
    --primary: #2d5a2d;
    --secondary: #d4af37;
    --bg-canvas: #f5f5f5;
    --bg-panel: #ffffff;
    --border: #e0e0e0;
    --selected: #3b82f6;
    --hover: rgba(59, 130, 246, 0.1);
}
```

**Typography:**
```css
--font-ui: -apple-system, sans-serif;
--font-code: 'SF Mono', Consolas, monospace;
```

**Spacing:**
```css
--space-xs: 4px;
--space-sm: 8px;
--space-md: 16px;
--space-lg: 24px;
--space-xl: 32px;
```

---

### **Keyboard Shortcuts:**

```
Cmd/Ctrl + S     → Save
Cmd/Ctrl + Z     → Undo
Cmd/Ctrl + Y     → Redo
Cmd/Ctrl + C     → Copy
Cmd/Ctrl + V     → Paste
Cmd/Ctrl + D     → Duplicate
Delete           → Delete selected
Cmd/Ctrl + G     → Group elements
Cmd/Ctrl + /     → Toggle code view
Space + Drag     → Pan canvas
Cmd/Ctrl + +/-   → Zoom in/out
```

---

## 📦 Deliverables

### **Month 1: Foundation**
- [ ] Svelte app setup
- [ ] Canvas component (basic)
- [ ] Component palette (10 primitives)
- [ ] Drag-and-drop (basic)
- [ ] Property panel (buttons only)
- [ ] JSON tree data structure
- [ ] Code generator (basic)

### **Month 2: Core Features**
- [ ] Layer tree view
- [ ] All primitives from Phase 1
- [ ] Resize handles
- [ ] Alignment guides
- [ ] Copy/paste/duplicate
- [ ] Undo/redo system
- [ ] Code view (Monaco editor)
- [ ] Two-way sync (visual ↔ code)

### **Month 3: Polish**
- [ ] Responsive breakpoints
- [ ] Keyboard shortcuts
- [ ] Templates integration
- [ ] Save/load projects
- [ ] Export to .jnc file
- [ ] Live preview (hot reload)

### **Month 4: Production**
- [ ] Performance optimization
- [ ] Error handling
- [ ] User testing
- [ ] Documentation
- [ ] Launch!

---

## 🧪 Technical Challenges

### **Challenge 1: State Management**

**Problem:** Visual editor has complex state (selected elements, undo/redo, etc.)

**Solution:** Zustand or Svelte stores

```typescript
// stores/editor.ts
import { writable, derived } from 'svelte/store';

export const componentTree = writable<ComponentTree>({
    components: [],
    signals: [],
    functions: []
});

export const selectedElement = writable<string | null>(null);

export const history = writable<ComponentTree[]>([]);
export const historyIndex = writable<number>(0);

// Undo
export function undo() {
    historyIndex.update(i => Math.max(0, i - 1));
    componentTree.set(history[historyIndex]);
}

// Redo
export function redo() {
    historyIndex.update(i => Math.min(history.length - 1, i + 1));
    componentTree.set(history[historyIndex]);
}
```

---

### **Challenge 2: Drag-and-Drop**

**Problem:** Complex nested drag-and-drop with drop zones

**Solution:** Use `svelte-dnd-action` or `react-dnd`

```svelte
<!-- Canvas.svelte -->
<script>
import { dndzone } from 'svelte-dnd-action';

let components = [...];

function handleDrop(event) {
    components = event.detail.items;
    // Update JSON tree
    updateComponentTree(components);
}
</script>

<div use:dndzone={{items: components}} on:consider={handleDrop}>
    {#each components as component}
        <Component data={component} />
    {/each}
</div>
```

---

### **Challenge 3: Code Synchronization**

**Problem:** Keep visual and code in sync

**Solution:** Single source of truth (JSON tree)

```
User edits visually:
  → Update JSON tree
  → Generate .jnc code from JSON
  → Update code editor

User edits code:
  → Parse .jnc code
  → Update JSON tree
  → Re-render visual canvas
```

---

### **Challenge 4: Performance**

**Problem:** Large component trees slow to render

**Solution:**
- Virtualize layer tree (only render visible items)
- Debounce code generation
- Use React.memo() or Svelte's reactive statements
- Offload code generation to Web Worker

---

## 📊 Success Metrics

**Month 1 goals:**
- Drag 10 primitives onto canvas
- Edit button text/color
- Generate basic .jnc code

**Month 2 goals:**
- Build simple app (login page) visually
- Edit code and see visual update
- Undo/redo works

**Month 3 goals:**
- Responsive design preview
- Save/load projects
- Export production-ready code

**Month 4 goals:**
- Non-technical users build apps successfully
- 90% of UI built visually
- 10+ beta testers using daily

---

## 🔗 Next Steps

After Phase 3 completes:
1. Gather user feedback (watch them use it!)
2. Identify most-used features
3. Start Phase 4 (AI Assistant)

---

## 📝 Open Questions

1. **Web-based or desktop app?**
   - Web: Easier to deploy, works everywhere
   - Desktop (Electron): Better performance, offline

2. **Free or paid?**
   - Free with limits (5 projects)?
   - Paid pro version ($10/month)?
   - Always free?

3. **Cloud save or local files?**
   - Cloud: Projects stored in database
   - Local: Save .jnc + .json to disk

4. **Multi-user collaboration?**
   - Like Figma (real-time editing)?
   - Or single-user only?

**Decision needed before starting!**

---

## 🎯 Ready to Start?

**Prerequisites:**
- Phase 2 (Hot Reload) complete
- Phase 1 (Component Library) for templates
- Decision on tech stack (Svelte recommended)

**First month focus:** Get basic drag-and-drop working!

See [TECHNICAL_ARCHITECTURE.md](./TECHNICAL_ARCHITECTURE.md) for system design details!
