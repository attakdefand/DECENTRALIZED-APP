# Test script for commit signing validation

Write-Host "Testing commit signing validation..." -ForegroundColor Green

# Test 1: Check that the verification script exists
if (Test-Path "scripts/verify-signed-commits.ps1") {
    Write-Host "✓ Commit signing verification script exists" -ForegroundColor Green
} else {
    Write-Host "✗ Commit signing verification script missing" -ForegroundColor Red
    exit 1
}

# Test 2: Check that the GitHub Actions workflow exists
if (Test-Path ".github/workflows/commit-signing-enforcement.yml") {
    Write-Host "✓ Commit signing enforcement workflow exists" -ForegroundColor Green
} else {
    Write-Host "✗ Commit signing enforcement workflow missing" -ForegroundColor Red
    exit 1
}

# Test 3: Check that the governance documentation exists
if (Test-Path "docs/governance/commit-signing-governance.md") {
    Write-Host "✓ Commit signing governance documentation exists" -ForegroundColor Green
} else {
    Write-Host "✗ Commit signing governance documentation missing" -ForegroundColor Red
    exit 1
}

# Test 4: Run the policy lint script to verify all checks pass
Write-Host "Running policy lint checks..." -ForegroundColor Yellow
$policyLintResult = & "scripts/policy-lint.ps1" 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Policy lint checks passed" -ForegroundColor Green
} else {
    Write-Host "✗ Policy lint checks failed" -ForegroundColor Red
    Write-Host $policyLintResult -ForegroundColor Red
    exit 1
}

Write-Host "All commit signing validation tests passed!" -ForegroundColor Green