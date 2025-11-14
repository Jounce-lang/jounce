#!/bin/bash
# Jounce Maintainer Footer Fix Script
# Adds the standard maintainer footer to files that are missing it
# Usage: ./scripts/fix-maintainers-footer.sh
# Note: Idempotent - safe to run multiple times

echo "🔧 Jounce Maintainer Footer Fix"
echo "================================"
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

# Expected footer pattern
FOOTER_PATTERN="Maintained by:.*The Jounce Project"
# Standard footer to add
STANDARD_FOOTER="Maintained by: **The Jounce Project**"

# Files that must have the footer
TARGET_FILES=(
    "README.md"
    "JOUNCE_SPEC.md"
    "docs/guides/LEARN_JOUNCE.md"
    "docs/JOUNCE_DO_AND_DONT.md"
    "docs/errors.md"
    "docs/styling.md"
    "docs/cli.md"
)

# Counters
TOTAL=0
FIXED=0
ALREADY_OK=0
SKIPPED=0

echo "Checking and fixing maintainer footers..."
echo ""

for file in "${TARGET_FILES[@]}"; do
    TOTAL=$((TOTAL + 1))

    if [ ! -f "$file" ]; then
        echo -e "${YELLOW}⚠${NC} $file (FILE NOT FOUND - skipped)"
        SKIPPED=$((SKIPPED + 1))
        continue
    fi

    # Check if file already contains the footer
    if grep -qE "$FOOTER_PATTERN" "$file"; then
        echo -e "${GREEN}✓${NC} $file (already has footer)"
        ALREADY_OK=$((ALREADY_OK + 1))
    else
        # Add footer to end of file
        echo "" >> "$file"
        echo "" >> "$file"
        echo "$STANDARD_FOOTER" >> "$file"
        echo -e "${GREEN}✓${NC} $file (footer added)"
        FIXED=$((FIXED + 1))
    fi
done

# Summary
echo ""
echo "================================"
echo "📊 Summary"
echo "================================"
echo "Total files: $TOTAL"
echo -e "${GREEN}Already OK: $ALREADY_OK${NC}"
echo -e "${GREEN}Fixed: $FIXED${NC}"
echo -e "${YELLOW}Skipped: $SKIPPED${NC}"
echo ""

if [ $FIXED -gt 0 ]; then
    echo -e "${GREEN}✨ Fixed $FIXED file(s)!${NC}"
else
    echo -e "${GREEN}✨ All files already have the footer!${NC}"
fi
