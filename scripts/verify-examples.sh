#!/bin/bash
# Jounce Example Verification Script
# Compiles all active example files to ensure they work with current compiler
# Usage: ./scripts/verify-examples.sh

set -e  # Exit on first error

echo "🧪 Jounce Example Verification"
echo "================================"
echo ""

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Counters
TOTAL=0
PASSED=0
FAILED=0
SKIPPED=0

# Failed files tracking
FAILED_FILES=()

# Function to compile a single .jnc file
compile_example() {
    local file="$1"
    local display_name="$(echo "$file" | sed "s|^$PROJECT_ROOT/||")"

    TOTAL=$((TOTAL + 1))

    # Skip archived examples (too many and may use old syntax)
    if [[ "$file" == *"/archived/"* ]]; then
        echo -e "${YELLOW}⊘${NC} Skipped: $display_name (archived)"
        SKIPPED=$((SKIPPED + 1))
        return 0
    fi

    # Create a temporary directory for compilation
    local temp_dir=$(mktemp -d)
    trap "rm -rf '$temp_dir'" RETURN

    # Compile the example (suppressing output)
    if cargo run --release --quiet -- compile "$file" > "$temp_dir/output.log" 2>&1; then
        echo -e "${GREEN}✓${NC} $display_name"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}✗${NC} $display_name"
        echo "    Error: $(head -5 "$temp_dir/output.log" | tail -1)"
        FAILED=$((FAILED + 1))
        FAILED_FILES+=("$display_name")
    fi
}

# README-referenced examples (must always work!)
READMEREF_EXAMPLES=(
    "examples/apps/01-click-counter/main.jnc"
    "examples/apps/todo-app/main.jnc"
    "examples/apps/16-form-validation/main.jnc"
    "examples/apps/35-crm-dashboard/main.jnc"
)

echo "📌 README-Referenced Examples (Critical)"
echo "-----------------------------------------"
for file in "${READMEREF_EXAMPLES[@]}"; do
    if [ ! -f "$file" ]; then
        echo -e "${RED}✗${NC} $file (FILE NOT FOUND)"
        echo "    Error: README references this path but it doesn't exist!"
        FAILED=$((FAILED + 1))
        FAILED_FILES+=("$file (missing)")
        TOTAL=$((TOTAL + 1))
    else
        compile_example "$file"
    fi
done

echo ""
echo "📦 Template Files"
echo "-----------------"
for file in templates/tutorial-starters/*/main.jnc; do
    if [ -f "$file" ]; then
        compile_example "$file"
    fi
done

echo ""
echo "📦 Active Example Apps"
echo "----------------------"
for file in examples/apps/*/main.jnc; do
    if [ -f "$file" ]; then
        compile_example "$file"
    fi
done

echo ""
echo "📦 Tutorial Examples"
echo "--------------------"
for file in examples/tutorials/*/*/*.jnc; do
    if [ -f "$file" ]; then
        compile_example "$file"
    fi
done

echo ""
echo "📦 Feature Examples"
echo "-------------------"
for dir in examples/features examples/reactivity examples/security; do
    if [ -d "$dir" ]; then
        for file in $(find "$dir" -name "*.jnc" -type f); do
            compile_example "$file"
        done
    fi
done

echo ""
echo "================================"
echo "📊 Summary"
echo "================================"
echo "Total:   $TOTAL files"
echo -e "${GREEN}Passed:  $PASSED${NC}"
echo -e "${YELLOW}Skipped: $SKIPPED${NC} (archived)"
echo -e "${RED}Failed:  $FAILED${NC}"

if [ $FAILED -gt 0 ]; then
    echo ""
    echo "Failed files:"
    for file in "${FAILED_FILES[@]}"; do
        echo "  - $file"
    done
    echo ""
    exit 1
else
    echo ""
    echo -e "${GREEN}✨ All active examples compiled successfully!${NC}"
    exit 0
fi
