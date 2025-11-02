#!/bin/bash

# Script to run automation and defense tests
# This script validates the implementation of automated remediation and policy enforcement features

set -e

echo "Running Automation & Defense Tests..."

# Check that required modules exist
echo "Checking for required modules..."

REQUIRED_FILES=(
    "crates/core/src/automated_remediation.rs"
    "crates/core/src/policy_enforcement.rs"
    "crates/core/tests/automation_defense_tests.rs"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo "ERROR: Required file missing: $file"
        exit 1
    fi
    echo "[PASS] $file exists"
done

# Run the tests
echo "Running automation defense tests..."

# Set environment for tests
export RUST_BACKTRACE=1

# Run tests with cargo
cargo test --package core --test automation_defense_tests

if [ $? -eq 0 ]; then
    echo "✅ All automation defense tests passed!"
else
    echo "❌ Some automation defense tests failed!"
    exit 1
fi

echo "Automation & Defense validation complete!"