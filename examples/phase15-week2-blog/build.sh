#!/bin/bash
# Build script for Jounce Blog Platform
# Usage: Run from Jounce root directory

set -e  # Exit on error

echo "🔨 Building Jounce Blog Platform..."
echo ""

# Get script directory and navigate to Jounce root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
JOUNCE_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"
cd "$JOUNCE_ROOT"

# Step 1: Compile the .jnc file
echo "📝 Step 1: Compiling main.jnc..."
cargo run --release -- compile examples/phase15-week2-blog/main.jnc

# Step 2: Copy client-app.js to dist
echo "📦 Step 2: Copying client-app.js to dist/..."
cp examples/phase15-week2-blog/client-app.js dist/

# Step 3: Generate custom index.html (compiler overwrites it)
echo "📝 Step 3: Generating custom index.html..."
cat > dist/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Jounce Blog Platform</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div id="app">
        <div class="app-container">
            <nav class="navbar">
                <div class="nav-container">
                    <div class="nav-brand">📝 Jounce Blog</div>
                    <div class="nav-links">
                        <a class="nav-link active" id="nav-home">Home</a>
                        <a class="nav-link" id="nav-editor">New Post</a>
                        <a class="nav-link" id="nav-search">Search</a>
                    </div>
                </div>
            </nav>
            <main class="main-content">
                <div id="page-home">
                    <div class="page-header">
                        <h1 class="page-title">Recent Posts</h1>
                        <p class="page-subtitle">Exploring ideas with Jounce</p>
                    </div>
                    <div class="post-grid" id="post-list"></div>
                </div>
                <div id="page-post" class="hidden">
                    <button class="btn btn-secondary btn-sm mb-3" id="back-to-home">← Back to Home</button>
                    <div class="post-view" id="post-content"></div>
                </div>
                <div id="page-editor" class="hidden">
                    <button class="btn btn-secondary btn-sm mb-3" id="back-from-editor">← Back to Home</button>
                    <div class="page-header">
                        <h1 class="page-title">Create New Post</h1>
                        <p class="page-subtitle">Write in Markdown, preview in real-time</p>
                    </div>
                    <div class="editor-container">
                        <div class="editor-panel">
                            <div class="panel-header">Editor</div>
                            <input type="text" class="editor-input" id="editor-title" placeholder="Post title..."/>
                            <input type="text" class="editor-input" id="editor-tags" placeholder="Tags (comma-separated)"/>
                            <textarea class="editor-textarea" id="editor-content" placeholder="Write your post in Markdown..."></textarea>
                            <div class="editor-actions">
                                <button class="btn btn-success" id="publish-btn">Publish Post</button>
                                <button class="btn btn-secondary" id="save-draft-btn">Save Draft</button>
                            </div>
                        </div>
                        <div class="editor-panel preview-panel">
                            <div class="panel-header">Preview</div>
                            <div class="post-content" id="editor-preview">
                                <p class="empty-state-text">Your preview will appear here</p>
                            </div>
                        </div>
                    </div>
                </div>
                <div id="page-search" class="hidden">
                    <button class="btn btn-secondary btn-sm mb-3" id="back-from-search">← Back to Home</button>
                    <div class="search-container">
                        <div class="page-header">
                            <h1 class="page-title">Search Posts</h1>
                        </div>
                        <div class="search-input-wrapper">
                            <input type="text" class="search-input" id="search-input" placeholder="Search posts by title, content, or tags..."/>
                        </div>
                        <div class="search-results-count" id="search-count">Start typing to search...</div>
                        <div class="post-grid" id="search-results"></div>
                    </div>
                </div>
            </main>
            <footer class="footer">
                <div class="footer-content">
                    <p>Built with Jounce - Phase 15 Week 2: Blog Platform</p>
                    <p>Demonstrating: Markdown rendering, Search, Comments, Progressive enhancement</p>
                </div>
            </footer>
        </div>
    </div>
    <script type="module">
        import { signal, computed, effect, batch } from './reactivity.js';
        window.signal = signal;
        window.computed = computed;
        window.effect = effect;
        window.batch = batch;
        console.log('✅ Reactivity loaded');
        const script = document.createElement('script');
        script.src = 'client-app.js';
        document.body.appendChild(script);
    </script>
</body>
</html>
EOF

echo ""
echo "✅ Build complete!"
echo ""
echo "📂 Output directory: dist/"
echo "   - index.html (generated)"
echo "   - client.js (compiled from main.jnc)"
echo "   - styles.css (extracted from style block)"
echo "   - client-app.js (copied)"
echo "   - reactivity.js (Jounce runtime)"
echo "   - client-runtime.js (Jounce runtime)"
echo ""
echo "🚀 To run the app:"
echo "   Option 1 (Node.js): node examples/phase15-week2-blog/server.js"
echo "   Option 2 (Python):  cd dist && python3 -m http.server 3000"
echo ""
