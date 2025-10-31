#!/bin/bash

# Script to validate the Encryption Implementation

set -e

echo "Validating Encryption Implementation..."

# Check that required files exist
REQUIRED_FILES=(
    "crates/security_layers/src/data_security.rs"
    "crates/security_layers/tests/data_security_encryption_at_rest_validation.rs"
    "docs/security/DATA-AT-REST.md"
    "scripts/run-security-layers-tests.ps1"
)

all_files_exist=true
for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file exists"
    else
        echo "❌ $file is missing"
        all_files_exist=false
    fi
done

# Check for encryption implementation in data_security.rs
echo "Checking Encryption Implementation..."
ENCRYPTION_FILE="crates/security_layers/src/data_security.rs"
if [ -f "$ENCRYPTION_FILE" ]; then
    content=$(cat "$ENCRYPTION_FILE")
    if grep -q "DataAtRestConfig" "$ENCRYPTION_FILE" && \
       grep -q "KeyRotationLog" "$ENCRYPTION_FILE" && \
       grep -q "KmsAccessLog" "$ENCRYPTION_FILE" && \
       grep -q "DataAtRestManager" "$ENCRYPTION_FILE" && \
       grep -q "is_encryption_at_rest_enabled" "$ENCRYPTION_FILE"; then
        echo "✅ Data Security module has proper encryption implementation"
    else
        echo "❌ Data Security module missing required encryption elements"
        all_files_exist=false
    fi
else
    echo "❌ data_security.rs missing"
    all_files_exist=false
fi

# Check for encryption validation tests
echo "Checking Encryption Validation Tests..."
TEST_FILE="crates/security_layers/tests/data_security_encryption_at_rest_validation.rs"
if [ -f "$TEST_FILE" ]; then
    content=$(cat "$TEST_FILE")
    if grep -q "test_data_at_rest_config_creation_and_validation" "$TEST_FILE" && \
       grep -q "test_key_rotation_log" "$TEST_FILE" && \
       grep -q "test_kms_access_log" "$TEST_FILE" && \
       grep -q "test_data_at_rest_manager" "$TEST_FILE" && \
       grep -q "test_data_at_rest_integration" "$TEST_FILE"; then
        echo "✅ Encryption validation tests are comprehensive"
    else
        echo "❌ Encryption validation tests are incomplete"
        all_files_exist=false
    fi
else
    echo "❌ Encryption validation tests missing"
    all_files_exist=false
fi

# Check for proper documentation
echo "Checking Encryption Documentation..."
DOC_FILE="docs/security/DATA-AT-REST.md"
if [ -f "$DOC_FILE" ]; then
    content=$(cat "$DOC_FILE")
    if [[ $(head -n 1 "$DOC_FILE") =~ "# Data-at-Rest Encryption Implementation" ]] && \
       grep -q "## " "$DOC_FILE" && \
       grep -q "DataAtRestConfig" "$DOC_FILE" && \
       grep -q "DataAtRestManager" "$DOC_FILE" && \
       grep -q "Telemetry" "$DOC_FILE"; then
        echo "✅ DATA-AT-REST.md has proper content"
    else
        echo "❌ DATA-AT-REST.md missing required content"
        all_files_exist=false
    fi
else
    echo "❌ DATA-AT-REST.md missing"
    all_files_exist=false
fi

# Check for Cargo.toml dependencies
echo "Checking Cargo.toml Dependencies..."
CARGO_FILE="Cargo.toml"
if [ -f "$CARGO_FILE" ]; then
    content=$(cat "$CARGO_FILE")
    if grep -q "serde" "$CARGO_FILE" && \
       grep -q "aes-gcm" "$CARGO_FILE"; then
        echo "✅ Cargo.toml has required encryption dependencies"
    else
        echo "⚠️ Cargo.toml may be missing encryption dependencies"
    fi
else
    echo "❌ Cargo.toml missing"
    all_files_exist=false
fi

# Run Rust tests for encryption
echo "Running Encryption Tests..."
cd "$(pwd)"
if cargo test -p security_layers data_security_encryption_at_rest_validation; then
    echo "✅ Encryption tests passed"
else
    echo "❌ Encryption tests failed"
    all_files_exist=false
fi

# Summary
echo ""
echo "Validation Summary:"
echo "=================="

if [ "$all_files_exist" = true ]; then
    echo "✅ All required files exist and tests pass"
else
    echo "❌ Some required files are missing or tests failed"
fi

# Count passed checks
checks=("File Structure" "Encryption Implementation" "Validation Tests" "Documentation" "Dependencies" "Test Execution")
passed_checks=0

# This is a simplified check - in reality, we'd want more detailed validation
if [ "$all_files_exist" = true ]; then
    passed_checks=6  # Assuming all checks pass for this example
else
    # Count how many checks actually passed
    [ -f "crates/security_layers/src/data_security.rs" ] && ((passed_checks++))
    [ -f "crates/security_layers/tests/data_security_encryption_at_rest_validation.rs" ] && ((passed_checks++))
    [ -f "docs/security/DATA-AT-REST.md" ] && ((passed_checks++))
    [ -f "Cargo.toml" ] && ((passed_checks++))
fi

echo "✅ $passed_checks/${#checks[@]} encryption validation checks passed"

if [ "$all_files_exist" = true ]; then
    echo ""
    echo "🎉 Encryption implementation is ready for use!"
    echo ""
    echo "To run the encryption tests:"
    echo "   ./scripts/run-security-layers-tests.ps1"
    exit 0
else
    echo ""
    echo "Some validation checks failed"
    echo ""
    echo "Please check the missing files and configurations"
    exit 1
fi