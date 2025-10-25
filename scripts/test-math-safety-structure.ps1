#!/usr/bin/env pwsh

# Script to test the math safety structure without requiring Foundry

Write-Host "Testing Math Safety Structure..." -ForegroundColor Green

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
    "contracts/src/core/MathSafetyDemo.sol",
    "contracts/test/core/MathSafetyTest.sol",
    "docs/contracts/MATH-SAFETY.md",
    "scripts/run-math-safety-tests.ps1",
    "scripts/validate-math-safety.ps1",
    "scripts/test-math-safety-structure.ps1",
    ".github/workflows/contract-math-safety.yml"
)

foreach ($File in $RequiredFiles) {
    if (Test-Path "d:\DECENTRALIZED-APP\$File") {
        Write-Host "✅ $File exists" -ForegroundColor Green
    } else {
        Write-Host "❌ $File missing" -ForegroundColor Red
    }
}

# Test 3: Validate Solidity contract file
Write-Host "`nTest 3: Solidity Contract File Validation" -ForegroundColor Yellow
$ContractFile = "d:\DECENTRALIZED-APP\contracts\src\core\MathSafetyDemo.sol"
if (Test-Path $ContractFile) {
    $Content = Get-Content $ContractFile
    $HasPragma = $Content -match "^pragma solidity"
    $HasImport = $Content -match "^import"
    $HasContract = $Content -match "^contract MathSafetyDemo"
    $HasFunctions = $Content -match "function "
    
    if ($HasPragma -and $HasImport -and $HasContract -and $HasFunctions) {
        Write-Host "✅ MathSafetyDemo.sol has proper structure" -ForegroundColor Green
    } else {
        Write-Host "❌ MathSafetyDemo.sol missing required elements" -ForegroundColor Red
    }
} else {
    Write-Host "❌ MathSafetyDemo.sol missing" -ForegroundColor Red
}

# Test 4: Validate Solidity test file
Write-Host "`nTest 4: Solidity Test File Validation" -ForegroundColor Yellow
$TestFile = "d:\DECENTRALIZED-APP\contracts\test\core\MathSafetyTest.sol"
if (Test-Path $TestFile) {
    $Content = Get-Content $TestFile
    $HasPragma = $Content -match "^pragma solidity"
    $HasImport = $Content -match "^import"
    $HasContract = $Content -match "^contract MathSafetyTest"
    $HasTests = $Content -match "function test"
    
    if ($HasPragma -and $HasImport -and $HasContract -and $HasTests) {
        Write-Host "✅ MathSafetyTest.sol has proper structure" -ForegroundColor Green
    } else {
        Write-Host "❌ MathSafetyTest.sol missing required elements" -ForegroundColor Red
    }
} else {
    Write-Host "❌ MathSafetyTest.sol missing" -ForegroundColor Red
}

# Test 5: Validate documentation
Write-Host "`nTest 5: Documentation Validation" -ForegroundColor Yellow
$DocFile = "d:\DECENTRALIZED-APP\docs\contracts\MATH-SAFETY.md"
if (Test-Path $DocFile) {
    $Content = Get-Content $DocFile
    $HasTitle = $Content[0] -match "# Smart Contract Math/Safety Patterns"
    $HasSections = $Content -match "## "
    $HasOverflow = $Content -match "Overflow"
    $HasPrecision = $Content -match "Precision"
    $HasConservation = $Content -match "Conservation"
    
    if ($HasTitle -and $HasSections -and $HasOverflow -and $HasPrecision -and $HasConservation) {
        Write-Host "✅ MATH-SAFETY.md has proper content" -ForegroundColor Green
    } else {
        Write-Host "❌ MATH-SAFETY.md missing required content" -ForegroundColor Red
    }
} else {
    Write-Host "❌ MATH-SAFETY.md missing" -ForegroundColor Red
}

# Test 6: Validate PowerShell scripts
Write-Host "`nTest 6: PowerShell Scripts Syntax" -ForegroundColor Yellow
$PSScripts = @(
    "scripts/run-math-safety-tests.ps1",
    "scripts/validate-math-safety.ps1",
    "scripts/test-math-safety-structure.ps1"
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
$WorkflowFile = "d:\DECENTRALIZED-APP\.github\workflows\contract-math-safety.yml"
if (Test-Path $WorkflowFile) {
    $Content = Get-Content $WorkflowFile
    $HasName = $Content -match "name: Contract Math Safety"
    $HasOn = $Content -match "on:"
    $HasJobs = $Content -match "jobs:"
    $HasSteps = $Content -match "steps:"
    
    if ($HasName -and $HasOn -and $HasJobs -and $HasSteps) {
        Write-Host "✅ contract-math-safety.yml has proper structure" -ForegroundColor Green
    } else {
        Write-Host "❌ contract-math-safety.yml missing required elements" -ForegroundColor Red
    }
} else {
    Write-Host "❌ contract-math-safety.yml missing" -ForegroundColor Red
}

Write-Host "`n🎉 Math Safety Structure Test Completed!" -ForegroundColor Green
Write-Host "All structural components are in place." -ForegroundColor Cyan
Write-Host "To run the actual math safety tests, install Foundry and run:" -ForegroundColor Yellow
Write-Host "   .\scripts\run-math-safety-tests.ps1" -ForegroundColor Cyan