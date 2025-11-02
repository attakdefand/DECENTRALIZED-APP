# Oracle Structure Test Script
# This script verifies the Oracle implementation structure

Write-Host "=== Oracle Structure Verification ===" -ForegroundColor Green

# Required files
$requiredFiles = @(
    "contracts/src/core/Oracle.sol",
    "contracts/test/core/OracleTest.sol",
    "docs/protocol/ORACLE-TESTS.md",
    "scripts/run-oracle-tests.ps1",
    "scripts/validate-oracle.ps1",
    ".github/workflows/contract-oracle.yml"
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
    Write-Host "=== ALL ORACLE FILES PRESENT ===" -ForegroundColor Green
    exit 0
} else {
    Write-Host "=== ORACLE STRUCTURE INCOMPLETE ===" -ForegroundColor Red
    exit 1
}