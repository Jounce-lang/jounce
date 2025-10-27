#!/bin/bash
# Jounce Auto-Recompile Watcher
# Usage: ./watch.sh <file.jnc>
#
# Watches a .jnc file and automatically recompiles on changes.
# Pair with browser-sync or live-server for instant browser updates:
#   npm install -g live-server
#   live-server dist --port=8080
#
# Or use Python's http.server with manual refresh:
#   cd dist && python3 -m http.server 8080

if [ -z "$1" ]; then
    echo "Usage: ./watch.sh <file.jnc>"
    echo ""
    echo "Example: ./watch.sh examples/single-file-counter/main.jnc"
    echo ""
    echo "For auto-reload in browser, run in another terminal:"
    echo "  npm install -g live-server"
    echo "  live-server dist --port=8080"
    exit 1
fi

FILE="$1"

if [ ! -f "$FILE" ]; then
    echo "Error: File '$FILE' not found"
    exit 1
fi

echo "👀 Watching: $FILE"
echo "📦 Compiling on save..."
echo ""
echo "Tip: For live browser reload, run this in another terminal:"
echo "  live-server dist --port=8080"
echo ""
echo "Press Ctrl+C to stop"
echo "----------------------------------------"

# Initial compilation
cargo run -- compile "$FILE"

# Watch for changes (macOS/Linux compatible)
if command -v fswatch &> /dev/null; then
    # macOS (install: brew install fswatch)
    fswatch -o "$FILE" | while read; do
        echo ""
        echo "🔄 File changed, recompiling..."
        cargo run -- compile "$FILE"
        echo "✅ Recompilation complete at $(date '+%H:%M:%S')"
    done
elif command -v inotifywait &> /dev/null; then
    # Linux (install: apt-get install inotify-tools)
    while inotifywait -e modify "$FILE"; do
        echo ""
        echo "🔄 File changed, recompiling..."
        cargo run -- compile "$FILE"
        echo "✅ Recompilation complete at $(date '+%H:%M:%S')"
    done
else
    echo "⚠️  No file watcher found!"
    echo ""
    echo "Please install fswatch (macOS) or inotify-tools (Linux):"
    echo "  macOS: brew install fswatch"
    echo "  Linux: sudo apt-get install inotify-tools"
    echo ""
    echo "Falling back to polling every 2 seconds..."
    echo ""

    LAST_MOD=$(stat -f %m "$FILE" 2>/dev/null || stat -c %Y "$FILE")
    while true; do
        sleep 2
        CURRENT_MOD=$(stat -f %m "$FILE" 2>/dev/null || stat -c %Y "$FILE")
        if [ "$CURRENT_MOD" != "$LAST_MOD" ]; then
            echo ""
            echo "🔄 File changed, recompiling..."
            cargo run -- compile "$FILE"
            echo "✅ Recompilation complete at $(date '+%H:%M:%S')"
            LAST_MOD=$CURRENT_MOD
        fi
    done
fi
