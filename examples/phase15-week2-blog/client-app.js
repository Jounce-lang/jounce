// Jounce Blog Platform - Reactive Client Application
// Phase 15 Week 2: Blog Platform with Markdown, Search, and Comments

// Import reactivity functions (will be available from compiled dist/reactivity.js)
const { signal, computed, effect } = window;

// ============================================================================
// DATA MODELS & STATE
// ============================================================================

// Sample post data
const SAMPLE_POSTS = [
    {
        id: 1,
        title: "Welcome to Jounce Blog",
        content: `# Welcome to Jounce Blog!

This is a **modern, reactive** blog platform built with Jounce.

## Features

- **Markdown Support**: Write posts in Markdown, render as beautiful HTML
- **Full-Text Search**: Find posts instantly by title, content, or tags
- **Comment System**: Engage with nested comments
- **Real-Time Preview**: See your markdown rendered as you type
- **Progressive Enhancement**: Start simple, scale up when needed

## Getting Started

Click "New Post" to create your first post, or explore the existing content.

\`\`\`javascript
// Jounce makes reactivity simple
let count = signal(0);
let doubled = computed(() => count.value * 2);
\`\`\`

> "The best way to predict the future is to build it" - Abraham Lincoln

Happy writing! 📝`,
        tags: ["announcement", "welcome", "tutorial"],
        author: "Jordan Hill",
        createdAt: new Date("2025-10-25T10:00:00Z"),
        published: true,
        comments: [
            {
                id: 1,
                author: "Alice",
                text: "This looks amazing! Can't wait to try it out.",
                createdAt: new Date("2025-10-25T11:30:00Z"),
                replies: [
                    {
                        id: 2,
                        author: "Jordan Hill",
                        text: "Thanks Alice! Let me know what you build.",
                        createdAt: new Date("2025-10-25T12:00:00Z")
                    }
                ]
            }
        ]
    },
    {
        id: 2,
        title: "Understanding Reactive Programming",
        content: `# Understanding Reactive Programming

Reactive programming is a paradigm focused on **data streams** and **automatic propagation of changes**.

## Core Concepts

### Signals
Signals hold reactive state that can be observed:

\`\`\`javascript
let name = signal("Alice");
console.log(name.value); // "Alice"
name.value = "Bob"; // Triggers reactivity
\`\`\`

### Computed Values
Computed values derive from other reactive sources:

\`\`\`javascript
let firstName = signal("Jane");
let lastName = signal("Doe");
let fullName = computed(() => \`\${firstName.value} \${lastName.value}\`);
\`\`\`

### Effects
Effects run automatically when dependencies change:

\`\`\`javascript
effect(() => {
    console.log("Name changed to:", name.value);
});
\`\`\`

## Why Reactive?

1. **Automatic Updates**: No manual DOM manipulation
2. **Declarative Code**: Describe *what*, not *how*
3. **Performance**: Only update what changed
4. **Composability**: Build complex UIs from simple pieces

Try it yourself in the next post!`,
        tags: ["tutorial", "reactivity", "programming"],
        author: "Jordan Hill",
        createdAt: new Date("2025-10-24T15:30:00Z"),
        published: true,
        comments: []
    },
    {
        id: 3,
        title: "Building Your First Jounce App",
        content: `# Building Your First Jounce App

Let's build a simple counter app step by step.

## Step 1: Create the File

Create \`counter.jnc\`:

\`\`\`jounce
style {
    .counter {
        font-size: 48px;
        text-align: center;
        padding: 20px;
    }

    button {
        padding: 10px 20px;
        font-size: 16px;
        margin: 5px;
    }
}

fn Counter() -> JSX {
    return (
        <div class="counter">
            <h1>Counter App</h1>
            <div id="count">0</div>
            <button id="increment">+</button>
            <button id="decrement">-</button>
        </div>
    );
}
\`\`\`

## Step 2: Add Reactivity

In your client code:

\`\`\`javascript
let count = signal(0);

document.getElementById('increment').onclick = () => {
    count.value++;
};

document.getElementById('decrement').onclick = () => {
    count.value--;
};

effect(() => {
    document.getElementById('count').textContent = count.value;
});
\`\`\`

## Step 3: Compile and Run

\`\`\`bash
jnc compile counter.jnc
cd dist && python3 -m http.server 8080
\`\`\`

That's it! You've built your first reactive app. 🎉`,
        tags: ["tutorial", "beginner", "example"],
        author: "Jordan Hill",
        createdAt: new Date("2025-10-23T09:15:00Z"),
        published: true,
        comments: [
            {
                id: 3,
                author: "Bob",
                text: "Great tutorial! Works perfectly.",
                createdAt: new Date("2025-10-23T14:00:00Z"),
                replies: []
            }
        ]
    }
];

// ============================================================================
// GLOBAL REACTIVE STATE
// ============================================================================

const posts = signal(SAMPLE_POSTS);
const currentPage = signal('home'); // 'home' | 'post' | 'editor' | 'search'
const currentPostId = signal(null);
const searchQuery = signal('');
const draftPost = signal({
    title: '',
    content: '',
    tags: '',
    author: 'Guest User'
});

// Computed: Current post being viewed
const currentPost = computed(() => {
    if (!currentPostId.value) return null;
    return posts.value.find(p => p.id === currentPostId.value);
});

// Computed: Published posts only
const publishedPosts = computed(() =>
    posts.value.filter(p => p.published)
);

// Computed: Search results
const searchResults = computed(() => {
    const query = searchQuery.value.toLowerCase().trim();
    if (!query) return [];

    return publishedPosts.value.filter(post => {
        const titleMatch = post.title.toLowerCase().includes(query);
        const contentMatch = post.content.toLowerCase().includes(query);
        const tagMatch = post.tags.some(tag => tag.toLowerCase().includes(query));
        return titleMatch || contentMatch || tagMatch;
    });
});

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

function formatDate(date) {
    const d = new Date(date);
    return d.toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'long',
        day: 'numeric'
    });
}

function formatTime(date) {
    const d = new Date(date);
    return d.toLocaleTimeString('en-US', {
        hour: '2-digit',
        minute: '2-digit'
    });
}

function generateId() {
    return Date.now();
}

function parseMarkdown(markdown) {
    // Simple markdown parser (subset)
    let html = markdown;

    // Headers
    html = html.replace(/^### (.*$)/gim, '<h3>$1</h3>');
    html = html.replace(/^## (.*$)/gim, '<h2>$1</h2>');
    html = html.replace(/^# (.*$)/gim, '<h1>$1</h1>');

    // Bold
    html = html.replace(/\*\*(.*?)\*\*/gim, '<strong>$1</strong>');

    // Italic
    html = html.replace(/\*(.*?)\*/gim, '<em>$1</em>');

    // Code blocks
    html = html.replace(/```(\w+)?\n([\s\S]*?)```/gim, '<pre><code>$2</code></pre>');

    // Inline code
    html = html.replace(/`([^`]+)`/gim, '<code>$1</code>');

    // Blockquotes
    html = html.replace(/^> (.*$)/gim, '<blockquote>$1</blockquote>');

    // Lists
    html = html.replace(/^\* (.*$)/gim, '<li>$1</li>');
    html = html.replace(/^- (.*$)/gim, '<li>$1</li>');
    html = html.replace(/(<li>.*<\/li>)/s, '<ul>$1</ul>');

    // Links
    html = html.replace(/\[([^\]]+)\]\(([^)]+)\)/gim, '<a href="$2">$1</a>');

    // Paragraphs
    html = html.split('\n\n').map(p => {
        if (p.trim().startsWith('<') || p.trim() === '') return p;
        return `<p>${p}</p>`;
    }).join('\n');

    return html;
}

function getExcerpt(content, maxLength = 150) {
    const text = content.replace(/[#*>`\-]/g, '').trim();
    if (text.length <= maxLength) return text;
    return text.substring(0, maxLength) + '...';
}

// ============================================================================
// NAVIGATION
// ============================================================================

function navigateTo(page, postId = null) {
    currentPage.value = page;
    if (postId) currentPostId.value = postId;

    // Update active nav link
    document.querySelectorAll('.nav-link').forEach(link => {
        link.classList.remove('active');
    });

    const activeLink = document.getElementById(`nav-${page}`);
    if (activeLink) activeLink.classList.add('active');
}

function showPage(pageId) {
    // Hide all pages
    ['page-home', 'page-post', 'page-editor', 'page-search'].forEach(id => {
        const el = document.getElementById(id);
        if (el) el.classList.add('hidden');
    });

    // Show target page
    const targetPage = document.getElementById(pageId);
    if (targetPage) targetPage.classList.remove('hidden');
}

// ============================================================================
// RENDERING FUNCTIONS
// ============================================================================

function renderPostCard(post) {
    const excerpt = getExcerpt(post.content);
    const tagsHtml = post.tags.map(tag =>
        `<span class="tag" data-tag="${tag}">#${tag}</span>`
    ).join('');

    return `
        <div class="post-card" data-post-id="${post.id}">
            <h3 class="post-card-title">${post.title}</h3>
            <div class="post-meta">
                <span>By ${post.author}</span>
                <span>${formatDate(post.createdAt)}</span>
            </div>
            <p class="post-card-excerpt">${excerpt}</p>
            <div class="post-tags">${tagsHtml}</div>
        </div>
    `;
}

function renderPostList() {
    const postListEl = document.getElementById('post-list');
    if (!postListEl) return;

    const postsHtml = publishedPosts.value.map(post => renderPostCard(post)).join('');
    postListEl.innerHTML = postsHtml;

    // Add click handlers
    postListEl.querySelectorAll('.post-card').forEach(card => {
        card.addEventListener('click', () => {
            const postId = parseInt(card.dataset.postId);
            viewPost(postId);
        });
    });

    // Add tag click handlers
    postListEl.querySelectorAll('.tag').forEach(tag => {
        tag.addEventListener('click', (e) => {
            e.stopPropagation();
            searchByTag(tag.dataset.tag);
        });
    });
}

function renderPost() {
    const post = currentPost.value;
    if (!post) return;

    const contentEl = document.getElementById('post-content');
    if (!contentEl) return;

    const contentHtml = parseMarkdown(post.content);
    const tagsHtml = post.tags.map(tag => `<span class="tag">#${tag}</span>`).join('');

    const commentsHtml = renderComments(post.comments || []);

    contentEl.innerHTML = `
        <div class="post-header">
            <h1 class="post-title">${post.title}</h1>
            <div class="post-meta">
                <span>By ${post.author}</span>
                <span>${formatDate(post.createdAt)} at ${formatTime(post.createdAt)}</span>
            </div>
            <div class="post-tags">${tagsHtml}</div>
        </div>
        <div class="post-content">
            ${contentHtml}
        </div>
        <div class="comments-section">
            <h2 class="comments-header">Comments (${(post.comments || []).length})</h2>
            <div class="comment-form">
                <textarea class="comment-textarea" id="new-comment" placeholder="Add a comment..."></textarea>
                <button class="btn btn-primary" id="submit-comment">Post Comment</button>
            </div>
            <div class="comments-list">
                ${commentsHtml}
            </div>
        </div>
    `;

    // Add comment submit handler
    const submitBtn = document.getElementById('submit-comment');
    if (submitBtn) {
        submitBtn.addEventListener('click', () => addComment(post.id));
    }
}

function renderComments(comments) {
    if (!comments || comments.length === 0) {
        return '<p class="empty-state-text">No comments yet. Be the first to comment!</p>';
    }

    return comments.map(comment => {
        const repliesHtml = comment.replies && comment.replies.length > 0
            ? `<div class="comment-reply">${renderComments(comment.replies)}</div>`
            : '';

        return `
            <div class="comment">
                <div class="comment-author">${comment.author}</div>
                <div class="comment-date">${formatDate(comment.createdAt)} at ${formatTime(comment.createdAt)}</div>
                <div class="comment-text">${comment.text}</div>
                ${repliesHtml}
            </div>
        `;
    }).join('');
}

function renderSearchResults() {
    const resultsEl = document.getElementById('search-results');
    const countEl = document.getElementById('search-count');
    if (!resultsEl || !countEl) return;

    const results = searchResults.value;

    if (searchQuery.value.trim() === '') {
        countEl.textContent = 'Start typing to search...';
        resultsEl.innerHTML = '';
        return;
    }

    if (results.length === 0) {
        countEl.textContent = 'No results found';
        resultsEl.innerHTML = '<div class="empty-state"><p class="empty-state-text">No posts match your search</p></div>';
        return;
    }

    countEl.textContent = `Found ${results.length} result${results.length === 1 ? '' : 's'}`;
    resultsEl.innerHTML = results.map(post => renderPostCard(post)).join('');

    // Add click handlers
    resultsEl.querySelectorAll('.post-card').forEach(card => {
        card.addEventListener('click', () => {
            const postId = parseInt(card.dataset.postId);
            viewPost(postId);
        });
    });
}

function renderPreview() {
    const previewEl = document.getElementById('editor-preview');
    if (!previewEl) return;

    const content = draftPost.value.content;
    if (!content.trim()) {
        previewEl.innerHTML = '<p class="empty-state-text">Your preview will appear here</p>';
        return;
    }

    previewEl.innerHTML = parseMarkdown(content);
}

// ============================================================================
// ACTIONS
// ============================================================================

function viewPost(postId) {
    currentPostId.value = postId;
    navigateTo('post', postId);
}

function searchByTag(tag) {
    searchQuery.value = tag;
    navigateTo('search');
}

function addComment(postId) {
    const textarea = document.getElementById('new-comment');
    if (!textarea || !textarea.value.trim()) return;

    const newComment = {
        id: generateId(),
        author: 'Guest User',
        text: textarea.value,
        createdAt: new Date(),
        replies: []
    };

    // Update posts
    const updatedPosts = posts.value.map(post => {
        if (post.id === postId) {
            return {
                ...post,
                comments: [...(post.comments || []), newComment]
            };
        }
        return post;
    });

    posts.value = updatedPosts;
    textarea.value = '';
}

function publishPost() {
    const draft = draftPost.value;

    if (!draft.title.trim() || !draft.content.trim()) {
        alert('Please fill in title and content');
        return;
    }

    const newPost = {
        id: generateId(),
        title: draft.title,
        content: draft.content,
        tags: draft.tags.split(',').map(t => t.trim()).filter(t => t),
        author: draft.author,
        createdAt: new Date(),
        published: true,
        comments: []
    };

    posts.value = [newPost, ...posts.value];

    // Reset draft
    draftPost.value = {
        title: '',
        content: '',
        tags: '',
        author: 'Guest User'
    };

    // Clear editor inputs
    document.getElementById('editor-title').value = '';
    document.getElementById('editor-tags').value = '';
    document.getElementById('editor-content').value = '';

    // Navigate to the new post
    viewPost(newPost.id);
}

// ============================================================================
// EFFECTS (REACTIVE UPDATES)
// ============================================================================

// Re-render post list when posts change
effect(() => {
    if (currentPage.value === 'home') {
        renderPostList();
    }
});

// Re-render current post when it changes
effect(() => {
    if (currentPage.value === 'post') {
        renderPost();
    }
});

// Re-render search results when query or results change
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

// Show/hide pages based on currentPage
effect(() => {
    const pageMap = {
        'home': 'page-home',
        'post': 'page-post',
        'editor': 'page-editor',
        'search': 'page-search'
    };

    showPage(pageMap[currentPage.value]);
});

// ============================================================================
// INITIALIZATION
// ============================================================================

function initializeApp() {
    console.log('📝 Initializing Jounce Blog Platform...');

    // Navigation handlers
    const navHome = document.getElementById('nav-home');
    const navEditor = document.getElementById('nav-editor');
    const navSearch = document.getElementById('nav-search');

    if (navHome) navHome.addEventListener('click', () => navigateTo('home'));
    if (navEditor) navEditor.addEventListener('click', () => navigateTo('editor'));
    if (navSearch) navSearch.addEventListener('click', () => navigateTo('search'));

    // Back buttons
    const backToHome = document.getElementById('back-to-home');
    const backFromEditor = document.getElementById('back-from-editor');
    const backFromSearch = document.getElementById('back-from-search');

    if (backToHome) backToHome.addEventListener('click', () => navigateTo('home'));
    if (backFromEditor) backFromEditor.addEventListener('click', () => navigateTo('home'));
    if (backFromSearch) backFromSearch.addEventListener('click', () => navigateTo('home'));

    // Search input
    const searchInput = document.getElementById('search-input');
    if (searchInput) {
        searchInput.addEventListener('input', (e) => {
            searchQuery.value = e.target.value;
        });
    }

    // Editor inputs
    const editorTitle = document.getElementById('editor-title');
    const editorTags = document.getElementById('editor-tags');
    const editorContent = document.getElementById('editor-content');

    if (editorTitle) {
        editorTitle.addEventListener('input', (e) => {
            draftPost.value = { ...draftPost.value, title: e.target.value };
        });
    }

    if (editorTags) {
        editorTags.addEventListener('input', (e) => {
            draftPost.value = { ...draftPost.value, tags: e.target.value };
        });
    }

    if (editorContent) {
        editorContent.addEventListener('input', (e) => {
            draftPost.value = { ...draftPost.value, content: e.target.value };
        });
    }

    // Publish button
    const publishBtn = document.getElementById('publish-btn');
    if (publishBtn) {
        publishBtn.addEventListener('click', publishPost);
    }

    // Save draft button (future feature)
    const saveDraftBtn = document.getElementById('save-draft-btn');
    if (saveDraftBtn) {
        saveDraftBtn.addEventListener('click', () => {
            alert('Draft saved! (localStorage integration coming soon)');
        });
    }

    // Initial render
    renderPostList();

    console.log('✅ Blog platform ready!');
    console.log(`📊 Loaded ${posts.value.length} posts`);
}

// Wait for DOM and reactivity to be ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initializeApp);
} else {
    initializeApp();
}
