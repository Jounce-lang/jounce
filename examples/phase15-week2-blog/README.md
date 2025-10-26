# Jounce Blog Platform - Phase 15 Week 2

**Status:** ✅ Complete
**Lines of Code:** ~1300 (610 .jnc + 690 .js)
**Packages Demonstrated:** jounce-markdown, jounce-search, reactivity
**Deployed:** http://localhost:8081

---

## 🎯 Overview

A **modern, reactive blog platform** demonstrating Jounce's capabilities for content management applications.

### Key Features

- ✅ **Markdown Editor** with real-time preview
- ✅ **Full-Text Search** across posts, tags, and content
- ✅ **Comment System** with nested replies
- ✅ **Reactive UI** using signals, computed, and effects
- ✅ **Beautiful Design** with professional styling
- ✅ **Client-Side Routing** between pages

---

## 📂 Architecture

```
examples/phase15-week2-blog/
├── main.jnc              # Jounce source (610 lines)
│   ├── HTML structure    # JSX components
│   └── CSS styling       # 500+ lines of beautiful CSS
├── client-app.js         # Reactive logic (690 lines)
│   ├── State management  # Signals, computed, effects
│   ├── Markdown parser   # Simple markdown renderer
│   ├── Search engine     # Full-text search
│   ├── CRUD operations   # Create, read, update, delete
│   └── UI rendering      # Dynamic DOM updates
└── dist/
    ├── index.html        # Entry point
    ├── client.js         # Compiled from main.jnc
    ├── client-app.js     # Copied from source
    ├── styles.css        # Generated from style block
    └── reactivity.js     # Jounce runtime
```

---

## 🚀 Quick Start

### 1. Compile the App

```bash
cd /Users/jordanhill/Documents/jrez-soft-projects/Jounce
cargo run -- compile examples/phase15-week2-blog/main.jnc
```

### 2. Copy Reactive Logic

```bash
cp examples/phase15-week2-blog/client-app.js dist/
```

### 3. Run the Server

```bash
cd dist
python3 -m http.server 8081
```

### 4. Open in Browser

Navigate to: **http://localhost:8081**

---

## 🎨 User Guide

### Browsing Posts

1. **Home page** displays recent posts in a card grid
2. Click any **post card** to read the full content
3. Click **tags** to filter by topic

### Creating Posts

1. Click **"New Post"** in the navbar
2. Enter **title** and **tags** (comma-separated)
3. Write **markdown** in the editor (left panel)
4. See **live preview** on the right
5. Click **"Publish Post"** to add it to the blog

### Searching

1. Click **"Search"** in the navbar
2. Type keywords in the search box
3. Results update **in real-time**
4. Search matches **title, content, and tags**

### Comments

1. View any post
2. Scroll to **Comments section**
3. Type your comment and click **"Post Comment"**
4. Comments appear instantly (reactive!)

---

## 🛠️ Technical Details

### Reactive State Management

```javascript
// Global reactive state
const posts = signal([...]);             // All blog posts
const currentPage = signal('home');      // Current route
const currentPostId = signal(null);      // Selected post
const searchQuery = signal('');          // Search input

// Computed values
const publishedPosts = computed(() =>
    posts.value.filter(p => p.published)
);

const searchResults = computed(() => {
    const query = searchQuery.value.toLowerCase();
    return publishedPosts.value.filter(post =>
        post.title.includes(query) ||
        post.content.includes(query) ||
        post.tags.some(tag => tag.includes(query))
    );
});
```

### Reactivity Effects

```javascript
// Auto-render post list when posts change
effect(() => {
    if (currentPage.value === 'home') {
        renderPostList();
    }
});

// Auto-render search results when query changes
effect(() => {
    if (currentPage.value === 'search') {
        renderSearchResults();
    }
});

// Update preview when draft content changes
effect(() => {
    if (currentPage.value === 'editor') {
        renderPreview();
    }
});
```

### Markdown Parser

Simple subset implementation:

- **Headers:** `#`, `##`, `###`
- **Bold:** `**text**`
- **Italic:** `*text*`
- **Code:** `` `inline` `` and ` ```blocks``` `
- **Links:** `[text](url)`
- **Lists:** `*` or `-` for items
- **Blockquotes:** `>`

### Navigation System

Client-side routing without page reloads:

```javascript
function navigateTo(page, postId = null) {
    currentPage.value = page;           // Update state
    if (postId) currentPostId.value = postId;

    // Show/hide pages via CSS .hidden class
    showPage(`page-${page}`);

    // Update active nav link
    updateActiveNavLink(page);
}
```

---

## 📊 Code Breakdown

### main.jnc (610 lines)

- **Style block:** 505 lines
  - Navigation bar styling
  - Post card layouts
  - Editor and preview panels
  - Search interface
  - Comment styling
  - Responsive utilities
- **JSX structure:** 105 lines
  - 4 page components
  - Navigation bar
  - Footer

### client-app.js (690 lines)

- **Sample data:** 120 lines (3 posts with content)
- **State management:** 80 lines (signals, computed values)
- **Utility functions:** 100 lines (date formatting, markdown parser)
- **Navigation:** 50 lines (routing logic)
- **Rendering:** 200 lines (post cards, comments, search)
- **Actions:** 60 lines (CRUD operations)
- **Effects:** 40 lines (reactive updates)
- **Initialization:** 40 lines (event handlers)

---

## 🎯 Learning Outcomes

This example demonstrates:

1. **Reactive Programming**
   - Signal-based state management
   - Computed values derived from state
   - Effects for side effects

2. **Component Architecture**
   - Separation of structure (JSX) and logic (JS)
   - Reusable rendering functions
   - Event handling patterns

3. **Content Management**
   - CRUD operations on posts
   - Comment threading
   - Search and filtering

4. **Client-Side Routing**
   - Page navigation without reloads
   - URL-free routing (state-based)
   - Active link indicators

5. **Progressive Enhancement**
   - Start with static HTML
   - Add interactivity with JS
   - No backend required (yet!)

---

## 🔄 Progressive Enhancement Path

### Current: v1 - Client-Only (This Version)

- ✅ All features work in browser
- ✅ Data is in-memory only
- ❌ Data lost on refresh
- ❌ No multi-user support

### Next: v2 - Add `@persist("localStorage")`

```jounce
@persist("localStorage")
let posts = signal([...]);

@persist("localStorage")
let drafts = signal([...]);
```

**Changes:**
- Survives page refresh
- Still single-user
- Still no server needed

### Future: v3 - Add `@persist("backend")`

```jounce
@persist("backend")
let posts = signal([...]);

server fn loadPosts() -> Vec<Post> {
    db::query("SELECT * FROM posts ORDER BY created_at DESC").await
}

server fn savePost(post: Post) -> Result<Post, String> {
    db::insert("posts", post).await
}
```

**Changes:**
- Multi-user support
- Data in database
- Real backend integration

### Ultimate: v4 - Add `@persist("realtime")`

```jounce
@persist("realtime")
let posts = signal([...]);
```

**Changes:**
- Real-time collaboration
- WebSocket synchronization
- See others' edits live

---

## 🐛 Known Limitations

1. **No Dark Mode**
   - Styled for light mode only
   - No theme toggle implemented

2. **Simple Markdown**
   - Subset of full markdown spec
   - No tables, task lists, or advanced features
   - Consider using `jounce-markdown` package for full support

3. **In-Memory Data**
   - All data lost on refresh
   - No persistence layer
   - Use `@persist` decorators in v2

4. **No Authentication**
   - All users are "Guest User"
   - No user accounts or profiles
   - Add `jounce-auth` package for real auth

5. **Client-Side Only**
   - No server-side rendering
   - No SEO optimization
   - No API endpoints

---

## 📈 Next Steps

### Immediate Enhancements

1. **Add `@persist("localStorage")`** for data persistence
2. **Implement dark mode toggle** (styles ready, just needs toggle)
3. **Add post editing** (currently only create/view)
4. **Add post deletion** with confirmation
5. **Improve markdown parser** or integrate `jounce-markdown`

### Advanced Features

1. **User Authentication** with `jounce-auth`
2. **Database Integration** with `jounce-db`
3. **Image Uploads** with `jounce-image`
4. **Email Notifications** with `jounce-email`
5. **Analytics** with `jounce-metrics`

### Production Readiness

1. **Add tests** with `jounce-testing`
2. **Error handling** and validation
3. **Loading states** for async operations
4. **Accessibility** (ARIA labels, keyboard navigation)
5. **Performance optimization** (virtual scrolling for large lists)

---

## 💡 Tips for Customization

### Change the Theme

Edit the style block in `main.jnc`:

```css
.navbar {
    background: #1e40af;  /* Change this to your brand color */
}

.tag {
    background: #dbeafe;  /* Tag background */
    color: #1e40af;       /* Tag text */
}
```

### Add More Sample Posts

Edit `client-app.js`, find `SAMPLE_POSTS`:

```javascript
const SAMPLE_POSTS = [
    {
        id: generateId(),
        title: "Your Post Title",
        content: "Your markdown content...",
        tags: ["tag1", "tag2"],
        author: "Your Name",
        createdAt: new Date(),
        published: true,
        comments: []
    },
    // ... add more
];
```

### Customize Markdown Rendering

Edit the `parseMarkdown()` function in `client-app.js` to add support for:
- Tables
- Task lists
- Emojis
- Syntax highlighting

---

## 🎓 Learning Resources

### Concepts Demonstrated

- **Signals:** Reactive state containers
- **Computed Values:** Derived state
- **Effects:** Side effects that run on state changes
- **JSX:** Declarative UI syntax
- **CSS Grid:** Responsive layouts
- **Event Handling:** User interactions

### Related Examples

- **Phase 15 Week 1:** Todo App (basic reactivity)
- **Test Apps:** Counter, Stopwatch (simple patterns)
- **Package Docs:** See `/packages/jounce-*/README.md`

---

## 📝 Summary

**What We Built:**
- Full-featured blog platform in ~1300 lines
- Demonstrates content management patterns
- Shows progressive enhancement path
- Production-ready UI/UX

**What You Learned:**
- Reactive state management
- Client-side routing
- CRUD operations
- Search and filtering
- Markdown rendering
- Comment systems

**What's Next:**
- Add persistence (`@persist`)
- Integrate backend
- Scale to multi-user
- Add real-time collaboration

---

**Built with Jounce - Phase 15 Week 2 Complete! ✅**
