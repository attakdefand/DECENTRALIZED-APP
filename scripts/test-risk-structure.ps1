# Risk Structure Test Script
# This script verifies the Risk implementation structure

Write-Host "=== Risk Structure Verification ===" -ForegroundColor Green

# Required files
$requiredFiles = @(
    "contracts/src/core/LendingPool.sol",
    "contracts/test/core/LendingPoolTest.sol",
    "docs/tests/RISK-SIMS.md",
    "scripts/run-risk-tests.ps1",
    "scripts/validate-risk.ps1",
    ".github/workflows/contract-risk.yml"
)

$allPresent = $true

foreach ($file in $requiredFiles) {
    if (Test-Path $file) {
        Write-Host "  [OK] $file" -ForegroundColor Green
    } else {
        Write-Host "  [MISSING] $file" -ForegroundColor Red
        $allPresent = $false
    }
}

if ($allPresent) {
    Write-Host "=== ALL RISK FILES PRESENT ===" -ForegroundColor Green
    exit 0
} else {
    Write-Host "=== RISK STRUCTURE INCOMPLETE ===" -ForegroundColor Red
    exit 1
}