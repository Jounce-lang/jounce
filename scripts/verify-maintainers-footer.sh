#!/bin/bash
# Jounce Maintainer Footer Verification Script
# Ensures all golden docs have the standard maintainer footer
# Usage: ./scripts/verify-maintainers-footer.sh

echo "📝 Jounce Maintainer Footer Verification"
echo "========================================="
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

# Expected footer patterns (can be anywhere in file, but typically at end)
# Accept both: "Maintained by: **The Jounce Project**" and "**Maintained by: The Jounce Project**"
FOOTER_PATTERN="Maintained by:.*The Jounce Project"

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
PASSED=0
FAILED=0
MISSING_FILES=()
MISSING_FOOTER=()

echo "Checking for maintainer footer in golden docs..."
echo ""

for file in "${TARGET_FILES[@]}"; do
    TOTAL=$((TOTAL + 1))

    if [ ! -f "$file" ]; then
        echo -e "${YELLOW}⚠${NC} $file (FILE NOT FOUND - skipped)"
        MISSING_FILES+=("$file")
        continue
    fi

    # Check if file contains the footer (using regex pattern)
    if grep -qE "$FOOTER_PATTERN" "$file"; then
        echo -e "${GREEN}✓${NC} $file"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}✗${NC} $file (MISSING FOOTER)"
        FAILED=$((FAILED + 1))
        MISSING_FOOTER+=("$file")
    fi
done

# Summary
echo ""
echo "========================================="
echo "📊 Summary"
echo "========================================="
echo "Total files checked: $TOTAL"
echo -e "${GREEN}With footer: $PASSED${NC}"
echo -e "${RED}Missing footer: $FAILED${NC}"

if [ ${#MISSING_FILES[@]} -gt 0 ]; then
    echo ""
    echo "Files not found (skipped):"
    for file in "${MISSING_FILES[@]}"; do
        echo "  - $file"
    done
fi

if [ $FAILED -gt 0 ]; then
    echo ""
    echo "Files missing maintainer footer:"
    for file in "${MISSING_FOOTER[@]}"; do
        echo "  - $file"
    done
    echo ""
    echo "💡 To fix, run: ./scripts/fix-maintainers-footer.sh"
    echo ""
    echo -e "${RED}❌ Some docs are missing the maintainer footer!${NC}"
    exit 1
else
    echo ""
    echo -e "${GREEN}✨ All golden docs have the maintainer footer!${NC}"
    exit 0
fi
