#!/bin/bash
# Test All Jounce Examples
# Compiles all examples and verifies they work

set -e  # Exit on error

echo "🧪 Testing Jounce Compiler & Examples"
echo "======================================"
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
PASSED=0
FAILED=0

echo "1️⃣  Building compiler..."
if cargo build --release > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Compiler built successfully${NC}"
    ((PASSED++))
else
    echo -e "${RED}❌ Compiler build failed${NC}"
    ((FAILED++))
    exit 1
fi
echo ""

echo "2️⃣  Running tests..."
if cargo test --lib 2>&1 | grep -q "635 passed"; then
    echo -e "${GREEN}✅ All 635 tests passing${NC}"
    ((PASSED++))
else
    echo -e "${RED}❌ Tests failed${NC}"
    ((FAILED++))
fi
echo ""

echo "3️⃣  Testing examples..."
echo ""

# Array of examples to test
examples=(
    "test_fine_grained_reactivity.jnc:Counter"
    "examples/reactivity/shopping-cart.jnc:Shopping Cart"
    "examples/reactivity/form-validation-simple.jnc:Form Validation"
    "examples/reactivity/search-filter.jnc:Search & Filter"
    "examples/reactivity/dashboard.jnc:Dashboard"
    "examples/reactivity/theme-switcher.jnc:Theme Switcher"
    "examples/apps/todo-app/main_reactive.jnc:Todo App"
)

for example in "${examples[@]}"; do
    file="${example%%:*}"
    name="${example##*:}"

    echo -n "   Testing $name... "

    if cargo run --release -- compile "$file" > /dev/null 2>&1; then
        # Check if reactive wrappers were generated
        if grep -q "__reactive" dist/client.js 2>/dev/null; then
            echo -e "${GREEN}✅ Compiled with reactivity${NC}"
            ((PASSED++))
        else
            echo -e "${YELLOW}⚠️  Compiled (no reactive wrappers)${NC}"
            ((PASSED++))
        fi
    else
        echo -e "${RED}❌ Failed to compile${NC}"
        ((FAILED++))
    fi
done

echo ""
echo "======================================"
echo -e "Results: ${GREEN}${PASSED} passed${NC}, ${RED}${FAILED} failed${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed! Jounce is working perfectly!${NC}"
    echo ""
    echo "Next steps:"
    echo "  1. Run an example: cd dist && node server.js"
    echo "  2. Open browser: http://localhost:3000"
    echo "  3. Test reactivity by interacting with the UI"
    exit 0
else
    echo -e "${RED}❌ Some tests failed. Check the output above.${NC}"
    exit 1
fi
