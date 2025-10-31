#!/bin/bash

# Policy validation tests
# This script validates that all required policy documents exist and are properly formatted

set -e

echo "Running policy validation tests..."

# Test 1: Check that all required policy documents exist
echo "Test 1: Checking required policy documents..."

REQUIRED_DOCS=(
  "docs/security/POLICY-CATALOG.md"
  "docs/security/EXCEPTIONS.md"
  "docs/security/sign-off-template.md"
  "docs/security/CODEOWNERS"
  "docs/security/IAM-RBAC-MAP.md"
  "docs/runbooks/key-rotation.md"
  "docs/security/mpc-hsm-policy.md"
  "docs/security/multisig-addresses.md"
  "infra/policies/policy-registry.md"
  "infra/policies/rate-classes.yaml"
  "infra/policies/policy-provenance.md"
)

for doc in "${REQUIRED_DOCS[@]}"; do
  if [ ! -f "$doc" ]; then
    echo "FAIL: Required policy document missing: $doc"
    exit 1
  fi
  echo "[PASS] $doc exists"
done

# Test 2: Check that required directories exist
echo "Test 2: Checking required directories..."

REQUIRED_DIRS=(
  "infra/policies/OPA-Cedar"
  "infra/policies/allow-deny-lists"
)

for dir in "${REQUIRED_DIRS[@]}"; do
  if [ ! -d "$dir" ]; then
    echo "FAIL: Required directory missing: $dir"
    exit 1
  fi
  echo "[PASS] $dir directory exists"
done

# Test 3: Check that at least one Cedar policy file exists
echo "Test 3: Checking Cedar policy files..."

if [ ! -n "$(ls -A infra/policies/OPA-Cedar/*.cedar 2>/dev/null)" ]; then
  echo "WARN: No Cedar policy files found in infra/policies/OPA-Cedar"
else
  count=$(ls -1 infra/policies/OPA-Cedar/*.cedar 2>/dev/null | wc -l)
  echo "[PASS] Found $count Cedar policy files"
fi

# Test 4: Check that allow/deny list files exist
echo "Test 4: Checking allow/deny list files..."

ALLOW_DENY_FILES=(
  "infra/policies/allow-deny-lists/ip-allow-list.txt"
  "infra/policies/allow-deny-lists/ip-deny-list.txt"
  "infra/policies/allow-deny-lists/domain-allow-list.txt"
  "infra/policies/allow-deny-lists/domain-deny-list.txt"
)

for file in "${ALLOW_DENY_FILES[@]}"; do
  if [ ! -f "$file" ]; then
    echo "FAIL: Required allow/deny list file missing: $file"
    exit 1
  fi
  echo "[PASS] $file exists"
done

# Test 5: Validate YAML syntax for rate classes
echo "Test 5: Validating YAML syntax..."

if command -v yamllint &> /dev/null; then
  yamllint infra/policies/rate-classes.yaml
  echo "[PASS] rate-classes.yaml syntax is valid"
else
  echo "[SKIP] yamllint not found, skipping YAML validation"
fi

# Test 6: Check markdown formatting (basic check)
echo "Test 6: Checking markdown formatting..."

MARKDOWN_FILES=(
  "docs/security/POLICY-CATALOG.md"
  "docs/security/EXCEPTIONS.md"
  "docs/security/sign-off-template.md"
  "docs/security/IAM-RBAC-MAP.md"
  "docs/runbooks/key-rotation.md"
  "docs/security/mpc-hsm-policy.md"
  "docs/security/multisig-addresses.md"
  "infra/policies/policy-registry.md"
  "infra/policies/policy-provenance.md"
)

# Simple check for basic markdown structure
for file in "${MARKDOWN_FILES[@]}"; do
  if [ -f "$file" ]; then
    # Check that file has content
    if [ ! -s "$file" ]; then
      echo "FAIL: $file is empty"
      exit 1
    fi
    
    # Check for basic markdown headers
    if ! grep -q "^#" "$file"; then
      echo "WARN: $file may be missing markdown headers"
    else
      echo "[PASS] $file has markdown headers"
    fi
  fi
done

echo "All policy validation tests passed!"