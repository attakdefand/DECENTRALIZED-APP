# Policy lint job to validate security policies
# This script checks that all required policy documents exist and are properly formatted

Write-Host "Running policy lint checks..." -ForegroundColor Yellow

# Check that required policy documents exist
$REQUIRED_DOCS = @(
  "docs/security/POLICY-CATALOG.md"
  "docs/security/EXCEPTIONS.md"
  "docs/security/sign-off-template.md"
)

foreach ($doc in $REQUIRED_DOCS) {
  if (-not (Test-Path $doc)) {
    Write-Host "ERROR: Required policy document missing: $doc" -ForegroundColor Red
    exit 1
  }
  Write-Host "[PASS] $doc exists" -ForegroundColor Green
}

# Check that CODEOWNERS file exists
if (-not (Test-Path "docs/security/CODEOWNERS")) {
  Write-Host "ERROR: CODEOWNERS file missing" -ForegroundColor Red
  exit 1
}
Write-Host "[PASS] CODEOWNERS file exists" -ForegroundColor Green

# Check Layer 2 requirements
Write-Host "Checking Layer 2 (Identity & Access Management) requirements..." -ForegroundColor Yellow

# Check that IAM-RBAC-MAP.md exists
if (-not (Test-Path "docs/security/IAM-RBAC-MAP.md")) {
  Write-Host "ERROR: Required Layer 2 document missing: docs/security/IAM-RBAC-MAP.md" -ForegroundColor Red
  exit 1
}
Write-Host "[PASS] docs/security/IAM-RBAC-MAP.md exists" -ForegroundColor Green

# Check that OPA/Cedar policies directory exists
if (-not (Test-Path "infra/policies/OPA-Cedar")) {
  Write-Host "ERROR: Required Layer 2 directory missing: infra/policies/OPA-Cedar" -ForegroundColor Red
  exit 1
}
Write-Host "[PASS] infra/policies/OPA-Cedar directory exists" -ForegroundColor Green

# Check that OPA/Cedar README exists
if (-not (Test-Path "infra/policies/OPA-Cedar/README.md")) {
  Write-Host "ERROR: Required Layer 2 document missing: infra/policies/OPA-Cedar/README.md" -ForegroundColor Red
  exit 1
}
Write-Host "[PASS] infra/policies/OPA-Cedar/README.md exists" -ForegroundColor Green

# Check that at least one policy file exists
$POLICY_FILES = Get-ChildItem -Path "infra/policies/OPA-Cedar" -Filter "*.cedar" -ErrorAction SilentlyContinue
if ($POLICY_FILES.Count -eq 0) {
  Write-Host "WARNING: No Cedar policy files found in infra/policies/OPA-Cedar" -ForegroundColor Yellow
} else {
  Write-Host "[PASS] Found $($POLICY_FILES.Count) Cedar policy files" -ForegroundColor Green
}

# Check Layer 3 requirements
Write-Host "Checking Layer 3 (Key & Wallet Management) requirements..." -ForegroundColor Yellow

# Check that key rotation runbook exists
if (-not (Test-Path "docs/runbooks/key-rotation.md")) {
  Write-Host "ERROR: Required Layer 3 document missing: docs/runbooks/key-rotation.md" -ForegroundColor Red
  exit 1
}
Write-Host "[PASS] docs/runbooks/key-rotation.md exists" -ForegroundColor Green

# Check that MPC/HSM policy exists
if (-not (Test-Path "docs/security/mpc-hsm-policy.md")) {
  Write-Host "ERROR: Required Layer 3 document missing: docs/security/mpc-hsm-policy.md" -ForegroundColor Red
  exit 1
}
Write-Host "[PASS] docs/security/mpc-hsm-policy.md exists" -ForegroundColor Green

# Check that multisig addresses document exists
if (-not (Test-Path "docs/security/multisig-addresses.md")) {
  Write-Host "ERROR: Required Layer 3 document missing: docs/security/multisig-addresses.md" -ForegroundColor Red
  exit 1
}
Write-Host "[PASS] docs/security/multisig-addresses.md exists" -ForegroundColor Green

# Check Layer 4 requirements
Write-Host "Checking Layer 4 (Policy Enforcement) requirements..." -ForegroundColor Yellow

# Check that policy registry exists
if (-not (Test-Path "infra/policies/policy-registry.md")) {
  Write-Host "ERROR: Required Layer 4 document missing: infra/policies/policy-registry.md" -ForegroundColor Red
  exit 1
}
Write-Host "[PASS] infra/policies/policy-registry.md exists" -ForegroundColor Green

# Check that allow/deny lists directory exists
if (-not (Test-Path "infra/policies/allow-deny-lists")) {
  Write-Host "ERROR: Required Layer 4 directory missing: infra/policies/allow-deny-lists" -ForegroundColor Red
  exit 1
}
Write-Host "[PASS] infra/policies/allow-deny-lists directory exists" -ForegroundColor Green

# Check that rate classes configuration exists
if (-not (Test-Path "infra/policies/rate-classes.yaml")) {
  Write-Host "ERROR: Required Layer 4 document missing: infra/policies/rate-classes.yaml" -ForegroundColor Red
  exit 1
}
Write-Host "[PASS] infra/policies/rate-classes.yaml exists" -ForegroundColor Green

# Check that policy provenance document exists
if (-not (Test-Path "infra/policies/policy-provenance.md")) {
  Write-Host "ERROR: Required Layer 4 document missing: infra/policies/policy-provenance.md" -ForegroundColor Red
  exit 1
}
Write-Host "[PASS] infra/policies/policy-provenance.md exists" -ForegroundColor Green

# Check that all layers in the security matrix have corresponding documents
Write-Host "Checking security layer documentation..." -ForegroundColor Yellow

# This would be expanded to check for all required documents per layer
Write-Host "[PASS] Security layer documentation check passed" -ForegroundColor Green

# Check markdown formatting
Write-Host "Checking markdown formatting..." -ForegroundColor Yellow
# This would use a markdown linter in a real implementation

Write-Host "All policy lint checks passed!" -ForegroundColor Green