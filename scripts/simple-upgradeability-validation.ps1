#!/usr/bin/env pwsh

# Simple script to validate the Upgradeability Security Layer implementation

Write-Host "Validating Upgradeability Security Layer Implementation..." -ForegroundColor Green

# Check that required files exist
$RequiredFiles = @(
    "contracts/src/core/EnhancedUpgradeableToken.sol",
    "contracts/src/core/TokenProxy.sol",
    "contracts/src/core/AppTimelock.sol",
    "contracts/src/core/GuardianMultisig.sol",
    "contracts/test/core/ComprehensiveUpgradeabilityTest.sol",
    "contracts/test/core/StorageLayoutDiffTest.sol",
    "contracts/script/UPGRADE-PLAN.md",
    "contracts/scripts/storage-layout-diff.ps1",
    "contracts/scripts/shadow-fork-dry-run.ps1",
    "scripts/run-upgradeability-tests.ps1",
    ".github/workflows/contract-upgradeability.yml",
    "docs/security/UPGRADEABILITY-SECURITY-LAYER-COMPLETED.md"
)

Write-Host "`nChecking Required Files:" -ForegroundColor Yellow
$AllFilesExist = $true
foreach ($File in $RequiredFiles) {
    if (Test-Path "d:\DECENTRALIZED-APP\$File") {
        Write-Host "✅ $File" -ForegroundColor Green
    } else {
        Write-Host "❌ $File" -ForegroundColor Red
        $AllFilesExist = $false
    }
}

if ($AllFilesExist) {
    Write-Host "`n🎉 All required files for Upgradeability Security Layer are present!" -ForegroundColor Green
    exit 0
} else {
    Write-Host "`n⚠️  Some required files are missing" -ForegroundColor Yellow
    exit 1
}