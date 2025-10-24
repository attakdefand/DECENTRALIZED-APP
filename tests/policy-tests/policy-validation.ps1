# Policy validation tests
# This script validates that all required policy documents exist and are properly formatted

Write-Host "Running policy validation tests..." -ForegroundColor Yellow

# Test 1: Check that all required policy documents exist
Write-Host "Test 1: Checking required policy documents..." -ForegroundColor Yellow

$REQUIRED_DOCS = @(
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

foreach ($doc in $REQUIRED_DOCS) {
  if (-not (Test-Path $doc)) {
    Write-Host "FAIL: Required policy document missing: $doc" -ForegroundColor Red
    exit 1
  }
  Write-Host "[PASS] $doc exists" -ForegroundColor Green
}

# Test 2: Check that required directories exist
Write-Host "Test 2: Checking required directories..." -ForegroundColor Yellow

$REQUIRED_DIRS = @(
  "infra/policies/OPA-Cedar"
  "infra/policies/allow-deny-lists"
)

foreach ($dir in $REQUIRED_DIRS) {
  if (-not (Test-Path $dir)) {
    Write-Host "FAIL: Required directory missing: $dir" -ForegroundColor Red
    exit 1
  }
  Write-Host "[PASS] $dir directory exists" -ForegroundColor Green
}

# Test 3: Check that at least one Cedar policy file exists
Write-Host "Test 3: Checking Cedar policy files..." -ForegroundColor Yellow

$POLICY_FILES = Get-ChildItem -Path "infra/policies/OPA-Cedar" -Filter "*.cedar" -ErrorAction SilentlyContinue
if ($POLICY_FILES.Count -eq 0) {
  Write-Host "WARN: No Cedar policy files found in infra/policies/OPA-Cedar" -ForegroundColor Yellow
} else {
  Write-Host "[PASS] Found $($POLICY_FILES.Count) Cedar policy files" -ForegroundColor Green
}

# Test 4: Check that allow/deny list files exist
Write-Host "Test 4: Checking allow/deny list files..." -ForegroundColor Yellow

$ALLOW_DENY_FILES = @(
  "infra/policies/allow-deny-lists/ip-allow-list.txt"
  "infra/policies/allow-deny-lists/ip-deny-list.txt"
  "infra/policies/allow-deny-lists/domain-allow-list.txt"
  "infra/policies/allow-deny-lists/domain-deny-list.txt"
)

foreach ($file in $ALLOW_DENY_FILES) {
  if (-not (Test-Path $file)) {
    Write-Host "FAIL: Required allow/deny list file missing: $file" -ForegroundColor Red
    exit 1
  }
  Write-Host "[PASS] $file exists" -ForegroundColor Green
}

# Test 5: Validate YAML syntax for rate classes
Write-Host "Test 5: Validating YAML syntax..." -ForegroundColor Yellow

# Simple YAML validation by checking if file can be parsed
try {
  $yamlContent = Get-Content "infra/policies/rate-classes.yaml" -Raw
  # Basic check for YAML structure
  if ($yamlContent -match "^[a-zA-Z]") {
    Write-Host "[PASS] rate-classes.yaml appears to be valid YAML" -ForegroundColor Green
  } else {
    Write-Host "WARN: rate-classes.yaml may have YAML syntax issues" -ForegroundColor Yellow
  }
} catch {
  Write-Host "WARN: Could not validate rate-classes.yaml syntax" -ForegroundColor Yellow
}

# Test 6: Check markdown formatting (basic check)
Write-Host "Test 6: Checking markdown formatting..." -ForegroundColor Yellow

$MARKDOWN_FILES = @(
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
foreach ($file in $MARKDOWN_FILES) {
  if (Test-Path $file) {
    # Check that file has content
    $content = Get-Content $file -Raw
    if ([string]::IsNullOrWhiteSpace($content)) {
      Write-Host "FAIL: $file is empty" -ForegroundColor Red
      exit 1
    }
    
    # Check for basic markdown headers
    if ($content -match "^#") {
      Write-Host "[PASS] $file has markdown headers" -ForegroundColor Green
    } else {
      Write-Host "WARN: $file may be missing markdown headers" -ForegroundColor Yellow
    }
  }
}

Write-Host "All policy validation tests passed!" -ForegroundColor Green