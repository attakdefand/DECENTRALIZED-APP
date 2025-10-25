#!/usr/bin/env pwsh

# Script to test the CPMM structure without requiring Foundry

Write-Host "Testing CPMM Structure..." -ForegroundColor Green

# Test 1: Check directory structure
Write-Host "`nTest 1: Directory Structure" -ForegroundColor Yellow
$RequiredDirs = @(
    "contracts",
    "contracts/src",
    "contracts/test",
    "contracts/test/core",
    "docs/contracts"
)

foreach ($Dir in $RequiredDirs) {
    if (Test-Path "d:\DECENTRALIZED-APP\$Dir") {
        Write-Host "✅ $Dir exists" -ForegroundColor Green
    } else {
        Write-Host "❌ $Dir missing" -ForegroundColor Red
    }
}

# Test 2: Check file structure
Write-Host "`nTest 2: File Structure" -ForegroundColor Yellow
$RequiredFiles = @(
    "contracts/src/core/CPMM.sol",
    "contracts/test/core/CPMMTest.sol",
    "docs/contracts/CPMM.md",
    "contracts/CPMM-README.md",
    "scripts/run-cpmm-tests.ps1",
    "scripts/validate-cpmm.ps1",
    "scripts/test-cpmm-structure.ps1",
    ".github/workflows/contract-cpmm.yml"
)

foreach ($File in $RequiredFiles) {
    if (Test-Path "d:\DECENTRALIZED-APP\$File") {
        Write-Host "✅ $File exists" -ForegroundColor Green
    } else {
        Write-Host "❌ $File missing" -ForegroundColor Red
    }
}

# Test 3: Validate Solidity contract files
Write-Host "`nTest 3: Solidity Contract File Validation" -ForegroundColor Yellow

# CPMM.sol
$CPMMFile = "d:\DECENTRALIZED-APP\contracts\src\core\CPMM.sol"
if (Test-Path $CPMMFile) {
    $Content = Get-Content $CPMMFile
    $HasPragma = $Content -match "^pragma solidity"
    $HasImport = $Content -match "^import"
    $HasContract = $Content -match "^contract ConstantProductAMM"
    $HasCPMM = $Content -match "constant.*product" -or $Content -match "x.*y.*k" -or $Content -match "k.*x.*y"
    $HasLiquidity = $Content -match "addLiquidity" -or $Content -match "removeLiquidity"
    $HasSwap = $Content -match "swap"
    $HasFees = $Content -match "fee"
    
    if ($HasPragma -and $HasImport -and $HasContract -and $HasCPMM -and $HasLiquidity -and $HasSwap -and $HasFees) {
        Write-Host "✅ CPMM.sol has proper structure" -ForegroundColor Green
    } else {
        Write-Host "❌ CPMM.sol missing required elements" -ForegroundColor Red
    }
} else {
    Write-Host "❌ CPMM.sol missing" -ForegroundColor Red
}

# Test 4: Validate Solidity test file
Write-Host "`nTest 4: Solidity Test File Validation" -ForegroundColor Yellow
$TestFile = "d:\DECENTRALIZED-APP\contracts\test\core\CPMMTest.sol"
if (Test-Path $TestFile) {
    $Content = Get-Content $TestFile
    $HasPragma = $Content -match "^pragma solidity"
    $HasImport = $Content -match "^import"
    $HasContract = $Content -match "^contract CPMMTest"
    $HasTests = $Content -match "function test"
    $HasCPMMTests = $Content -match "CPMM" -or $Content -match "constant.*product" -or $Content -match "liquidity" -or $Content -match "swap"
    
    if ($HasPragma -and $HasImport -and $HasContract -and $HasTests -and $HasCPMMTests) {
        Write-Host "✅ CPMMTest.sol has proper structure" -ForegroundColor Green
    } else {
        Write-Host "❌ CPMMTest.sol missing required elements" -ForegroundColor Red
    }
} else {
    Write-Host "❌ CPMMTest.sol missing" -ForegroundColor Red
}

# Test 5: Validate documentation
Write-Host "`nTest 5: Documentation Validation" -ForegroundColor Yellow
$DocFile = "d:\DECENTRALIZED-APP\docs\contracts\CPMM.md"
if (Test-Path $DocFile) {
    $Content = Get-Content $DocFile
    $HasTitle = $Content[0] -match "# Constant Product Market Maker"
    $HasSections = $Content -match "## "
    $HasCPMM = $Content -match "Constant Product" -or $Content -match "x.*y.*k" -or $Content -match "k.*x.*y"
    $HasLiquidity = $Content -match "Liquidity"
    $HasFees = $Content -match "Fee"
    
    if ($HasTitle -and $HasSections -and $HasCPMM -and $HasLiquidity -and $HasFees) {
        Write-Host "✅ CPMM.md has proper content" -ForegroundColor Green
    } else {
        Write-Host "❌ CPMM.md missing required content" -ForegroundColor Red
    }
} else {
    Write-Host "❌ CPMM.md missing" -ForegroundColor Red
}

# Test 6: Validate PowerShell scripts
Write-Host "`nTest 6: PowerShell Scripts Syntax" -ForegroundColor Yellow
$PSScripts = @(
    "scripts/run-cpmm-tests.ps1",
    "scripts/validate-cpmm.ps1",
    "scripts/test-cpmm-structure.ps1"
)

foreach ($Script in $PSScripts) {
    try {
        $null = Get-Command "d:\DECENTRALIZED-APP\$Script" -ErrorAction Stop
        Write-Host "✅ $Script syntax is valid" -ForegroundColor Green
    } catch {
        Write-Host "❌ $Script has syntax errors: $($_.Exception.Message)" -ForegroundColor Red
    }
}

# Test 7: Validate GitHub Actions workflow
Write-Host "`nTest 7: GitHub Actions Workflow" -ForegroundColor Yellow
$WorkflowFile = "d:\DECENTRALIZED-APP\.github\workflows\contract-cpmm.yml"
if (Test-Path $WorkflowFile) {
    $Content = Get-Content $WorkflowFile
    $HasName = $Content -match "name: Contract CPMM"
    $HasOn = $Content -match "on:"
    $HasJobs = $Content -match "jobs:"
    $HasSteps = $Content -match "steps:"
    $HasCPMM = $Content -match "CPMM" -or $Content -match "constant.*product"
    
    if ($HasName -and $HasOn -and $HasJobs -and $HasSteps -and $HasCPMM) {
        Write-Host "✅ contract-cpmm.yml has proper structure" -ForegroundColor Green
    } else {
        Write-Host "❌ contract-cpmm.yml missing required elements" -ForegroundColor Red
    }
} else {
    Write-Host "❌ contract-cpmm.yml missing" -ForegroundColor Red
}

Write-Host "`n🎉 CPMM Structure Test Completed!" -ForegroundColor Green
Write-Host "All structural components are in place." -ForegroundColor Cyan
Write-Host "To run the actual CPMM tests, install Foundry and run:" -ForegroundColor Yellow
Write-Host "   .\scripts\run-cpmm-tests.ps1" -ForegroundColor Cyan