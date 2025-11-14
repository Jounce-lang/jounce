#!/bin/bash
# Jounce Golden Docs Consistency Linter
# Prevents drift between README, SPEC, LEARN, and DO_AND_DONT
# Usage: ./scripts/verify-golden-consistency.sh

echo "📋 Jounce Golden Docs Consistency Check"
echo "========================================"
echo ""

# Get project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Golden docs
GOLDEN_DOCS=(
    "README.md"
    "JOUNCE_SPEC.md"
    "docs/guides/LEARN_JOUNCE.md"
    "docs/JOUNCE_DO_AND_DONT.md"
)

# Tracking
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
FAILURES=()

# Helper: run a check
check() {
    local name="$1"
    local file="$2"
    local condition="$3"

    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))

    if eval "$condition"; then
        echo -e "  ${GREEN}✓${NC} $name"
        PASSED_CHECKS=$((PASSED_CHECKS + 1))
        return 0
    else
        echo -e "  ${RED}✗${NC} $name"
        FAILED_CHECKS=$((FAILED_CHECKS + 1))
        FAILURES+=("$file: $name")
        return 1
    fi
}

# Check 1: References JOUNCE_SPEC as authoritative
echo "${BLUE}Check 1: Authoritative source references${NC}"
echo "-------------------------------------------"
for doc in "${GOLDEN_DOCS[@]}"; do
    if [ ! -f "$doc" ]; then
        echo -e "  ${YELLOW}⚠${NC} $doc (file not found)"
        continue
    fi

    check "References JOUNCE_SPEC.md as authoritative" "$doc" \
        "grep -qiE '(authoritative|see.*JOUNCE_SPEC|For.*language rules.*JOUNCE_SPEC)' '$doc'"
done

echo ""

# Check 2: Version v0.8.x mentioned in first 100 lines
echo "${BLUE}Check 2: Version alignment (v0.8.x in first 100 lines)${NC}"
echo "-------------------------------------------------------"
for doc in "${GOLDEN_DOCS[@]}"; do
    if [ ! -f "$doc" ]; then
        continue
    fi

    check "Contains v0.8 in first 100 lines" "$doc" \
        "head -100 '$doc' | grep -q 'v0\.8'"
done

echo ""

# Check 3: No vendor credits
echo "${BLUE}Check 3: No vendor/LLM credits${NC}"
echo "--------------------------------"
VENDOR_PATTERNS=(
    "Claude"
    "Anthropic"
    "OpenAI"
    "GPT-"
    "ChatGPT"
)

for doc in "${GOLDEN_DOCS[@]}"; do
    if [ ! -f "$doc" ]; then
        continue
    fi

    found_vendors=""
    for vendor in "${VENDOR_PATTERNS[@]}"; do
        if grep -qF "$vendor" "$doc"; then
            found_vendors="$found_vendors $vendor"
        fi
    done

    if [ -z "$found_vendors" ]; then
        check "No vendor credits" "$doc" "true"
    else
        check "No vendor credits (found:$found_vendors)" "$doc" "false"
    fi
done

echo ""

# Check 4: Lowercase DOM events (no onClick/onChange in prose)
echo "${BLUE}Check 4: Lowercase event names (no onClick/onChange)${NC}"
echo "-----------------------------------------------------"
REACT_EVENTS=(
    "onClick"
    "onChange"
    "onSubmit"
    "onInput"
    "onFocus"
    "onBlur"
)

for doc in "${GOLDEN_DOCS[@]}"; do
    if [ ! -f "$doc" ]; then
        continue
    fi

    found_events=""
    for event in "${REACT_EVENTS[@]}"; do
        if grep -q "$event" "$doc"; then
            found_events="$found_events $event"
        fi
    done

    if [ -z "$found_events" ]; then
        check "Uses lowercase event names" "$doc" "true"
    else
        check "Uses lowercase event names (found:$found_events)" "$doc" "false"
    fi
done

echo ""

# Check 5: Async model constraint mentioned
echo "${BLUE}Check 5: Async constraint documented${NC}"
echo "--------------------------------------"
ASYNC_PATTERNS=(
    "No client-side async"
    "async.*await.*not.*supported"
    "client.*async.*constraint"
    "Server functions run asynchronously"
)

for doc in "${GOLDEN_DOCS[@]}"; do
    if [ ! -f "$doc" ]; then
        continue
    fi

    found_async=false
    for pattern in "${ASYNC_PATTERNS[@]}"; do
        if grep -qiE "$pattern" "$doc"; then
            found_async=true
            break
        fi
    done

    if [ "$found_async" = true ]; then
        check "Mentions async model" "$doc" "true"
    else
        # Special case: JOUNCE_SPEC.md is the source, others should reference it
        if [ "$doc" = "JOUNCE_SPEC.md" ]; then
            # Check if spec actually documents async behavior
            if grep -qiE "(async|asynchronous|await)" "$doc"; then
                check "Documents async model" "$doc" "true"
            else
                check "Documents async model" "$doc" "false"
            fi
        else
            # Other docs should either mention it or link to SPEC
            check "Mentions async model or references SPEC" "$doc" "false"
        fi
    fi
done

# Summary
echo ""
echo "========================================"
echo "📊 Summary"
echo "========================================"
echo "Total checks: $TOTAL_CHECKS"
echo -e "${GREEN}Passed: $PASSED_CHECKS${NC}"
echo -e "${RED}Failed: $FAILED_CHECKS${NC}"

if [ $FAILED_CHECKS -gt 0 ]; then
    echo ""
    echo "Failed checks:"
    for failure in "${FAILURES[@]}"; do
        echo "  - $failure"
    done
    echo ""
    echo -e "${RED}❌ Golden docs have consistency issues!${NC}"
    echo ""
    echo "💡 Common fixes:"
    echo "  - Add 'For authoritative language rules, see JOUNCE_SPEC.md' to doc headers"
    echo "  - Ensure v0.8.x is mentioned in first 100 lines"
    echo "  - Remove vendor credits (Claude, Anthropic, OpenAI, GPT)"
    echo "  - Use lowercase event names: onclick not onClick"
    echo "  - Document async constraints or reference SPEC section"
    exit 1
else
    echo ""
    echo -e "${GREEN}✨ All golden docs are consistent!${NC}"
    exit 0
fi
