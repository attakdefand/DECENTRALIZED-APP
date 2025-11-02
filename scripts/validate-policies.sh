#!/bin/bash

# Script to validate security policy documents
set -e

echo "Validating security policy documents..."

# Check that required policy documents exist
required_files=(
    "docs/security/POLICY-CATALOG.md"
    "docs/security/IAM-RBAC-MAP.md"
    "docs/runbooks/key-rotation.md"
    "docs/security/mpc-hsm-policy.md"
    "docs/security/multisig-addresses.md"
    "infra/policies/policy-registry.md"
    "infra/policies/policy-provenance.md"
)

for file in "${required_files[@]}"; do
    if [ ! -f "$file" ]; then
        echo "ERROR: Required policy file missing: $file"
        exit 1
    fi
    echo "✓ $file exists"
done

# Check for basic structure in key policy documents
echo "Checking policy document structure..."

# Check POLICY-CATALOG.md
if ! grep -q "# Policy Catalog" "docs/security/POLICY-CATALOG.md"; then
    echo "ERROR: POLICY-CATALOG.md missing title"
    exit 1
fi
echo "✓ POLICY-CATALOG.md has correct title"

# Check IAM-RBAC-MAP.md
if ! grep -q "# IAM RBAC Map" "docs/security/IAM-RBAC-MAP.md"; then
    echo "ERROR: IAM-RBAC-MAP.md missing title"
    exit 1
fi
echo "✓ IAM-RBAC-MAP.md has correct title"

# Check key-rotation.md
if ! grep -q "# Key Rotation Runbook" "docs/runbooks/key-rotation.md"; then
    echo "ERROR: key-rotation.md missing title"
    exit 1
fi
echo "✓ key-rotation.md has correct title"

echo "All policy documents validated successfully!"
exit 0