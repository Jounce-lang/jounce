#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧱 Jounce Release Guard — Pre-Release Validation${NC}"
echo "=============================================="
echo ""

# Track overall status
OVERALL_STATUS=0
CHECKS_PASSED=0
CHECKS_FAILED=0

# Function to run a check
run_check() {
    local name="$1"
    local command="$2"

    echo -e "${YELLOW}▶ Running: ${name}${NC}"
    echo "---"

    if eval "$command"; then
        echo -e "${GREEN}✅ PASSED: ${name}${NC}"
        echo ""
        CHECKS_PASSED=$((CHECKS_PASSED + 1))
        return 0
    else
        echo -e "${RED}❌ FAILED: ${name}${NC}"
        echo ""
        CHECKS_FAILED=$((CHECKS_FAILED + 1))
        OVERALL_STATUS=1
        return 1
    fi
}

# Run all verification checks
run_check "Example Verification" "./scripts/verify-examples.sh"
run_check "Documentation Links" "./scripts/verify-doc-links.sh"
run_check "Maintainer Footers" "./scripts/verify-maintainers-footer.sh"
run_check "Golden Docs Consistency" "./scripts/verify-golden-consistency.sh"
run_check "Rust Test Suite" "cargo test --tests"

# Print summary
echo "=============================================="
echo -e "${BLUE}📊 Release Guard Summary${NC}"
echo "=============================================="
echo -e "Total checks: $((CHECKS_PASSED + CHECKS_FAILED))"
echo -e "${GREEN}Passed: ${CHECKS_PASSED}${NC}"
echo -e "${RED}Failed: ${CHECKS_FAILED}${NC}"
echo ""

if [ $OVERALL_STATUS -eq 0 ]; then
    echo -e "${GREEN}✨ All checks passed! Release is ready.${NC}"
    exit 0
else
    echo -e "${RED}❌ Some checks failed. Please fix issues before release.${NC}"
    exit 1
fi
