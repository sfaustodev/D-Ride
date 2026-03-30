#!/bin/bash

# Security Audit Script for dRide Leptos Landing Page
# Runs comprehensive security checks and generates a report

set -e

echo "🔒 Running Security Audit for dRide Leptos Landing Page..."
echo "================================================="

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if in correct directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Cargo.toml not found. Run this script from project root.${NC}"
    exit 1
fi

# Test Results
PASSED=0
FAILED=0
WARNINGS=0

# Function to check and report result
check_result() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ PASS${NC}: $1"
        ((PASSED++))
    else
        echo -e "${RED}❌ FAIL${NC}: $1"
        ((FAILED++))
    fi
}

echo ""
echo "1️⃣  Rust Dependency Vulnerability Scan"
echo "----------------------------------------------"

# Check for known vulnerabilities
echo "Running cargo audit..."
if command -v cargo-audit &> /dev/null; then
    cargo audit
    RESULT=$?
else
    echo "Installing cargo-audit..."
    cargo install cargo-audit
    cargo audit
    RESULT=$?
fi
check_result "cargo audit - no known vulnerabilities"

echo ""
echo "2️⃣  Security Lints with Clippy"
echo "----------------------------------------------"

echo "Running clippy with security lints..."
cargo clippy --all-targets --all-features -- \
    -W clippy::missing_errors_doc \
    -W clippy::missing_panics_doc \
    -W clippy::unwrap_used \
    -W clippy::expect_used \
    -W clippy::panic
check_result "clippy security lints - no critical issues"

echo ""
echo "3️⃣  Outdated Dependencies Check"
echo "----------------------------------------------"

echo "Checking for outdated dependencies..."
if command -v cargo-outdated &> /dev/null; then
    cargo-outdated
else
    echo "cargo-outdated not installed - skipping"
    ((WARNINGS++))
fi

echo ""
echo "4️⃣  Unused Dependencies Check"
echo "----------------------------------------------"

echo "Checking for unused dependencies..."
if command -v cargo-machete &> /dev/null; then
    cargo-machete
else
    echo "cargo-machete not installed - skipping"
    ((WARNINGS++))
fi

echo ""
echo "5️⃣  Code Quality Checks"
echo "----------------------------------------------"

echo "Running cargo test..."
cargo test --no-fail-fast
check_result "cargo test - all tests pass"

echo ""
echo "6️⃣  Build Verification"
echo "----------------------------------------------"

echo "Running cargo check..."
cargo check
check_result "cargo check - no compilation errors"

echo ""
echo "7️⃣  Security Best Practices Check"
echo "----------------------------------------------"

# Check for hardcoded secrets
echo "Checking for hardcoded secrets..."
if grep -r "secret\|password\|private_key\|api_key\|token" --include="*.rs" src/; then
    echo -e "${YELLOW}⚠️  WARNING${NC}: Potential hardcoded secrets found"
    ((WARNINGS++))
fi

# Check for debug prints
echo "Checking for debug println! usage..."
if grep -r "println!\|dbg!\|eprintln!" --include="*.rs" src/; then
    echo -e "${YELLOW}⚠️  WARNING${NC}: Debug print statements found (should use tracing!)"
    ((WARNINGS++))
fi

# Check for unsafe blocks
echo "Checking for unsafe blocks..."
if grep -r "unsafe" --include="*.rs" src/; then
    echo -e "${YELLOW}⚠️  INFO${NC}: Unsafe blocks found - review for safety"
fi

echo ""
echo "8️⃣  Environment Security Check"
echo "----------------------------------------------"

# Check if .env is in git
echo "Checking for committed secrets..."
if git ls-files .env &> /dev/null; then
    echo -e "${RED}❌ FAIL${NC}: .env file is tracked by git"
    ((FAILED++))
else
    check_result ".env file not tracked by git"
fi

echo ""
echo "================================================="
echo -e "${GREEN}✅ Security Audit Complete${NC}"
echo ""
echo "Results:"
echo "  Passed:  $PASSED"
echo "  Failed: $FAILED"
echo "  Warnings: $WARNINGS"
echo ""

if [ $FAILED -gt 0 ]; then
    echo -e "${RED}❌ Security issues found - review and fix before deployment${NC}"
    exit 1
elif [ $WARNINGS -gt 0 ]; then
    echo -e "${YELLOW}⚠️  Warnings found - review recommended${NC}"
    exit 0
else
    echo -e "${GREEN}✅ No critical security issues found${NC}"
    exit 0
fi
