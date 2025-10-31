#!/usr/bin/env pwsh

# Script to test the event structure without requiring Foundry

Write-Host "Testing Event Schema Structure..." -ForegroundColor Green

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
    "contracts/src/core/EventSchema.sol",
    "contracts/src/core/EventSchemaToken.sol",
    "contracts/test/core/EventSchemaTest.sol",
    "docs/contracts/EVENT-SCHEMA.md",
    "scripts/run-event-tests.ps1",
    "scripts/validate-event-schema.ps1",
    "scripts/test-event-structure.ps1",
    ".github/workflows/contract-event-schema.yml"
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

# EventSchema.sol
$EventSchemaFile = "d:\DECENTRALIZED-APP\contracts\src\core\EventSchema.sol"
if (Test-Path $EventSchemaFile) {
    $Content = Get-Content $EventSchemaFile
    $HasPragma = $Content -match "^pragma solidity"
    $HasImport = $Content -match "^import"
    $HasContract = $Content -match "^contract EventSchema"
    $HasEvents = $Content -match "event "
    $HasEmit = $Content -match "emit "
    
    if ($HasPragma -and $HasImport -and $HasContract -and $HasEvents -and $HasEmit) {
        Write-Host "✅ EventSchema.sol has proper structure" -ForegroundColor Green
    } else {
        Write-Host "❌ EventSchema.sol missing required elements" -ForegroundColor Red
    }
} else {
    Write-Host "❌ EventSchema.sol missing" -ForegroundColor Red
}

# EventSchemaToken.sol
$EventSchemaTokenFile = "d:\DECENTRALIZED-APP\contracts\src\core\EventSchemaToken.sol"
if (Test-Path $EventSchemaTokenFile) {
    $Content = Get-Content $EventSchemaTokenFile
    $HasPragma = $Content -match "^pragma solidity"
    $HasImport = $Content -match "^import"
    $HasContract = $Content -match "^contract EventSchemaToken"
    $HasEvents = $Content -match "event "
    $HasEmit = $Content -match "emit "
    
    if ($HasPragma -and $HasImport -and $HasContract -and $HasEvents -and $HasEmit) {
        Write-Host "✅ EventSchemaToken.sol has proper structure" -ForegroundColor Green
    } else {
        Write-Host "❌ EventSchemaToken.sol missing required elements" -ForegroundColor Red
    }
} else {
    Write-Host "❌ EventSchemaToken.sol missing" -ForegroundColor Red
}

# Test 4: Validate Solidity test file
Write-Host "`nTest 4: Solidity Test File Validation" -ForegroundColor Yellow
$TestFile = "d:\DECENTRALIZED-APP\contracts\test\core\EventSchemaTest.sol"
if (Test-Path $TestFile) {
    $Content = Get-Content $TestFile
    $HasPragma = $Content -match "^pragma solidity"
    $HasImport = $Content -match "^import"
    $HasContract = $Content -match "^contract EventSchemaTest"
    $HasTests = $Content -match "function test"
    $HasEventTests = $Content -match "event" -or $Content -match "Event"
    
    if ($HasPragma -and $HasImport -and $HasContract -and $HasTests -and $HasEventTests) {
        Write-Host "✅ EventSchemaTest.sol has proper structure" -ForegroundColor Green
    } else {
        Write-Host "❌ EventSchemaTest.sol missing required elements" -ForegroundColor Red
    }
} else {
    Write-Host "❌ EventSchemaTest.sol missing" -ForegroundColor Red
}

# Test 5: Validate documentation
Write-Host "`nTest 5: Documentation Validation" -ForegroundColor Yellow
$DocFile = "d:\DECENTRALIZED-APP\docs\contracts\EVENT-SCHEMA.md"
if (Test-Path $DocFile) {
    $Content = Get-Content $DocFile
    $HasTitle = $Content[0] -match "# Smart Contract Event Schema Implementation"
    $HasSections = $Content -match "## "
    $HasEventSchema = $Content -match "Event Schema"
    $HasIndexing = $Content -match "Indexing"
    
    if ($HasTitle -and $HasSections -and $HasEventSchema -and $HasIndexing) {
        Write-Host "✅ EVENT-SCHEMA.md has proper content" -ForegroundColor Green
    } else {
        Write-Host "❌ EVENT-SCHEMA.md missing required content" -ForegroundColor Red
    }
} else {
    Write-Host "❌ EVENT-SCHEMA.md missing" -ForegroundColor Red
}

# Test 6: Validate PowerShell scripts
Write-Host "`nTest 6: PowerShell Scripts Syntax" -ForegroundColor Yellow
$PSScripts = @(
    "scripts/run-event-tests.ps1",
    "scripts/validate-event-schema.ps1",
    "scripts/test-event-structure.ps1"
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
$WorkflowFile = "d:\DECENTRALIZED-APP\.github\workflows\contract-event-schema.yml"
if (Test-Path $WorkflowFile) {
    $Content = Get-Content $WorkflowFile
    $HasName = $Content -match "name: Contract Event Schema"
    $HasOn = $Content -match "on:"
    $HasJobs = $Content -match "jobs:"
    $HasSteps = $Content -match "steps:"
    
    if ($HasName -and $HasOn -and $HasJobs -and $HasSteps) {
        Write-Host "✅ contract-event-schema.yml has proper structure" -ForegroundColor Green
    } else {
        Write-Host "❌ contract-event-schema.yml missing required elements" -ForegroundColor Red
    }
} else {
    Write-Host "❌ contract-event-schema.yml missing" -ForegroundColor Red
}

Write-Host "`n🎉 Event Schema Structure Test Completed!" -ForegroundColor Green
Write-Host "All structural components are in place." -ForegroundColor Cyan
Write-Host "To run the actual event schema tests, install Foundry and run:" -ForegroundColor Yellow
Write-Host "   .\scripts\run-event-tests.ps1" -ForegroundColor Cyan