# LOB Structure Test Script
# This script verifies the LOB implementation structure

Write-Host "=== LOB Structure Verification ===" -ForegroundColor Green

# Required files
$requiredFiles = @(
    "contracts/src/core/Orderbook.sol",
    "contracts/test/core/OrderbookTest.sol",
    "docs/tests/LOB-TESTS.md",
    "scripts/run-lob-tests.ps1",
    "scripts/validate-lob.ps1",
    ".github/workflows/contract-lob.yml"
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
    Write-Host "=== ALL LOB FILES PRESENT ===" -ForegroundColor Green
    exit 0
} else {
    Write-Host "=== LOB STRUCTURE INCOMPLETE ===" -ForegroundColor Red
    exit 1
}