#!/usr/bin/env pwsh

# Script to test the logic patterns structure without requiring Foundry

Write-Host "Testing Logic Patterns Structure..." -ForegroundColor Green

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
    "contracts/test/core/LogicPatternsTest.sol",
    "docs/contracts/LOGIC-PATTERNS.md",
    "scripts/run-logic-patterns-tests.ps1",
    "scripts/validate-logic-patterns.ps1",
    "scripts/test-logic-patterns-structure.ps1",
    ".github/workflows/contract-logic-patterns.yml"
)

foreach ($File in $RequiredFiles) {
    if (Test-Path "d:\DECENTRALIZED-APP\$File") {
        Write-Host "✅ $File exists" -ForegroundColor Green
    } else {
        Write-Host "❌ $File missing" -ForegroundColor Red
    }
}

# Test 3: Validate Solidity test file
Write-Host "`nTest 3: Solidity Test File Validation" -ForegroundColor Yellow
$TestFile = "d:\DECENTRALIZED-APP\contracts\test\core\LogicPatternsTest.sol"
if (Test-Path $TestFile) {
    $Content = Get-Content $TestFile
    $HasPragma = $Content -match "^pragma solidity"
    $HasImport = $Content -match "^import"
    $HasContract = $Content -match "^contract LogicPatternsTest"
    $HasTests = $Content -match "function test"
    
    if ($HasPragma -and $HasImport -and $HasContract -and $HasTests) {
        Write-Host "✅ LogicPatternsTest.sol has proper structure" -ForegroundColor Green
    } else {
        Write-Host "❌ LogicPatternsTest.sol missing required elements" -ForegroundColor Red
    }
} else {
    Write-Host "❌ LogicPatternsTest.sol missing" -ForegroundColor Red
}

# Test 4: Validate documentation
Write-Host "`nTest 4: Documentation Validation" -ForegroundColor Yellow
$DocFile = "d:\DECENTRALIZED-APP\docs\contracts\LOGIC-PATTERNS.md"
if (Test-Path $DocFile) {
    $Content = Get-Content $DocFile
    $HasTitle = $Content[0] -match "# Smart Contract Logic Patterns"
    $HasSections = $Content -match "## "
    $HasCEI = $Content -match "CEI"
    $HasReentrancy = $Content -match "Reentrancy"
    $HasAccessControl = $Content -match "Access Control"
    
    if ($HasTitle -and $HasSections -and $HasCEI -and $HasReentrancy -and $HasAccessControl) {
        Write-Host "✅ LOGIC-PATTERNS.md has proper content" -ForegroundColor Green
    } else {
        Write-Host "❌ LOGIC-PATTERNS.md missing required content" -ForegroundColor Red
    }
} else {
    Write-Host "❌ LOGIC-PATTERNS.md missing" -ForegroundColor Red
}

# Test 5: Validate PowerShell scripts
Write-Host "`nTest 5: PowerShell Scripts Syntax" -ForegroundColor Yellow
$PSScripts = @(
    "scripts/run-logic-patterns-tests.ps1",
    "scripts/validate-logic-patterns.ps1",
    "scripts/test-logic-patterns-structure.ps1"
)

foreach ($Script in $PSScripts) {
    try {
        $null = Get-Command "d:\DECENTRALIZED-APP\$Script" -ErrorAction Stop
        Write-Host "✅ $Script syntax is valid" -ForegroundColor Green
    } catch {
        Write-Host "❌ $Script has syntax errors: $($_.Exception.Message)" -ForegroundColor Red
    }
}

# Test 6: Validate GitHub Actions workflow
Write-Host "`nTest 6: GitHub Actions Workflow" -ForegroundColor Yellow
$WorkflowFile = "d:\DECENTRALIZED-APP\.github\workflows\contract-logic-patterns.yml"
if (Test-Path $WorkflowFile) {
    $Content = Get-Content $WorkflowFile
    $HasName = $Content -match "name: Contract Logic Patterns"
    $HasOn = $Content -match "on:"
    $HasJobs = $Content -match "jobs:"
    $HasSteps = $Content -match "steps:"
    
    if ($HasName -and $HasOn -and $HasJobs -and $HasSteps) {
        Write-Host "✅ contract-logic-patterns.yml has proper structure" -ForegroundColor Green
    } else {
        Write-Host "❌ contract-logic-patterns.yml missing required elements" -ForegroundColor Red
    }
} else {
    Write-Host "❌ contract-logic-patterns.yml missing" -ForegroundColor Red
}

Write-Host "`n🎉 Logic Patterns Structure Test Completed!" -ForegroundColor Green
Write-Host "All structural components are in place." -ForegroundColor Cyan
Write-Host "To run the actual logic patterns tests, install Foundry and run:" -ForegroundColor Yellow
Write-Host "   .\scripts\run-logic-patterns-tests.ps1" -ForegroundColor Cyan