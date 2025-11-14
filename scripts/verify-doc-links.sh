#!/bin/bash
# Jounce Documentation Link Verification Script
# Verifies all local markdown links point to existing files/directories
# Usage: ./scripts/verify-doc-links.sh

echo "📚 Jounce Documentation Link Verification"
echo "=========================================="
echo ""

# Get project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Temp files for tracking (needed due to subshells)
TEMP_DIR=$(mktemp -d)
trap "rm -rf '$TEMP_DIR'" EXIT
TOTAL_FILE="$TEMP_DIR/total"
VALID_FILE="$TEMP_DIR/valid"
BROKEN_FILE="$TEMP_DIR/broken"
BROKEN_LIST_FILE="$TEMP_DIR/broken_list"

echo "0" > "$TOTAL_FILE"
echo "0" > "$VALID_FILE"
echo "0" > "$BROKEN_FILE"
touch "$BROKEN_LIST_FILE"

# Increment counter in file
inc_count() {
    local file="$1"
    local count=$(cat "$file")
    echo "$((count + 1))" > "$file"
}

# Process one file
process_file() {
    local mdfile="$1"
    local display=$(echo "$mdfile" | sed "s|^$PROJECT_ROOT/||")
    local line_num=0

    while IFS= read -r line; do
        line_num=$((line_num + 1))

        # Find markdown links using grep
        local links=$(echo "$line" | grep -oE '\[([^\]]+)\]\(([^)]+)\)' || true)

        if [ -n "$links" ]; then
            echo "$links" | while read -r match; do
                # Extract URL/path from (path)
                local link=$(echo "$match" | sed 's/.*(\(.*\)).*/\1/')

                # Skip external/absolute links
                [[ "$link" =~ ^https?:// ]] && continue
                [[ "$link" =~ ^mailto: ]] && continue
                [[ "$link" =~ ^# ]] && continue
                [[ "$link" =~ ^/ ]] && continue

                inc_count "$TOTAL_FILE"

                # Remove anchor
                local target="${link%%#*}"

                # Resolve path
                local file_dir="$(dirname "$mdfile")"
                local resolved="$file_dir/$target"

                # Check exists
                if [ -f "$resolved" ] || [ -d "$resolved" ]; then
                    echo -e "${GREEN}✓${NC} $display:$line_num -> $target"
                    inc_count "$VALID_FILE"
                else
                    echo -e "${RED}✗${NC} $display:$line_num -> $target (NOT FOUND)"
                    inc_count "$BROKEN_FILE"
                    echo "  - $display:$line_num: [$target]" >> "$BROKEN_LIST_FILE"
                fi
            done
        fi
    done < "$mdfile"
}

# Check README
echo "📖 Checking README.md"
echo "---------------------"
if [ -f "README.md" ]; then
    process_file "README.md"
else
    echo -e "${RED}✗${NC} README.md not found!"
    exit 1
fi

# Check docs directory
echo ""
echo "📖 Checking docs/ directory"
echo "----------------------------"
if [ -d "docs" ]; then
    find docs -name "*.md" -type f | sort | while read -r file; do
        process_file "$file"
    done
else
    echo -e "${YELLOW}⚠${NC} docs/ directory not found"
fi

# Read final counts
TOTAL=$(cat "$TOTAL_FILE")
VALID=$(cat "$VALID_FILE")
BROKEN=$(cat "$BROKEN_FILE")

# Summary
echo ""
echo "=========================================="
echo "📊 Summary"
echo "=========================================="
echo "Total links checked: $TOTAL"
echo -e "${GREEN}Valid:  $VALID${NC}"
echo -e "${RED}Broken: $BROKEN${NC}"

if [ $BROKEN -gt 0 ]; then
    echo ""
    echo "Broken links found:"
    cat "$BROKEN_LIST_FILE"
    echo ""
    echo -e "${RED}❌ Documentation has broken local links!${NC}"
    exit 1
else
    echo ""
    echo -e "${GREEN}✨ All local documentation links are valid!${NC}"
    exit 0
fi
