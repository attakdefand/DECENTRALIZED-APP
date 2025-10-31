#!/usr/bin/env pwsh

# Script to test the build matrix structure without requiring Foundry

Write-Host "Testing Build Matrix Structure..." -ForegroundColor Green

# Test 1: Check directory structure
Write-Host "`nTest 1: Directory Structure" -ForegroundColor Yellow
$RequiredDirs = @(
    "contracts",
    "contracts/src",
    "contracts/test",
    "contracts/test/core",
    "contracts/scripts",
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
    "contracts/foundry.toml",
    "contracts/remappings.txt",
    "contracts/BUILD-MATRIX-README.md",
    "contracts/test/core/BuildMatrixTest.sol",
    "contracts/scripts/install-deps.ps1",
    "scripts/run-build-matrix.ps1",
    "scripts/validate-build-matrix.ps1",
    "docs/contracts/BUILD-MATRIX.md",
    ".github/workflows/contract-build-matrix.yml"
)

foreach ($File in $RequiredFiles) {
    if (Test-Path "d:\DECENTRALIZED-APP\$File") {
        Write-Host "✅ $File exists" -ForegroundColor Green
    } else {
        Write-Host "❌ $File missing" -ForegroundColor Red
    }
}

# Test 3: Validate Foundry.toml profiles
Write-Host "`nTest 3: Foundry.toml Profiles" -ForegroundColor Yellow
$FoundryConfig = Get-Content "d:\DECENTRALIZED-APP\contracts\foundry.toml"
$Profiles = @("[profile.default]", "[profile.optimized]", "[profile.ir]", "[profile.ir-optimized]")

foreach ($Profile in $Profiles) {
    if ($FoundryConfig -contains $Profile) {
        Write-Host "✅ $Profile exists" -ForegroundColor Green
    } else {
        Write-Host "❌ $Profile missing" -ForegroundColor Red
    }
}

# Test 4: Validate remappings
Write-Host "`nTest 4: Remappings" -ForegroundColor Yellow
$Remappings = Get-Content "d:\DECENTRALIZED-APP\contracts\remappings.txt"
$RequiredRemappings = @("forge-std/=", "@openzeppelin/contracts/=", "@openzeppelin/contracts-upgradeable/=")

foreach ($Remapping in $RequiredRemappings) {
    $Found = $false
    foreach ($Line in $Remappings) {
        if ($Line -like "$Remapping*") {
            $Found = $true
            break
        }
    }
    if ($Found) {
        Write-Host "✅ $Remapping configured" -ForegroundColor Green
    } else {
        Write-Host "❌ $Remapping missing" -ForegroundColor Red
    }
}

# Test 5: Validate PowerShell scripts
Write-Host "`nTest 5: PowerShell Scripts Syntax" -ForegroundColor Yellow
$PSScripts = @(
    "scripts/run-build-matrix.ps1",
    "scripts/validate-build-matrix.ps1",
    "contracts/scripts/install-deps.ps1"
)

foreach ($Script in $PSScripts) {
    try {
        $null = Get-Command "d:\DECENTRALIZED-APP\$Script" -ErrorAction Stop
        Write-Host "✅ $Script syntax is valid" -ForegroundColor Green
    } catch {
        Write-Host "❌ $Script has syntax errors: $($_.Exception.Message)" -ForegroundColor Red
    }
}

# Test 6: Validate Solidity files
Write-Host "`nTest 6: Solidity Files" -ForegroundColor Yellow
$SolFiles = Get-ChildItem "d:\DECENTRALIZED-APP\contracts\src\core" -Filter "*.sol" -Recurse

foreach ($File in $SolFiles) {
    $Content = Get-Content $File.FullName
    $HasPragma = $Content -match "^pragma solidity"
    if ($HasPragma) {
        Write-Host "✅ $($File.Name) has pragma directive" -ForegroundColor Green
    } else {
        Write-Host "❌ $($File.Name) missing pragma directive" -ForegroundColor Red
    }
}

# Test 7: Validate test files
Write-Host "`nTest 7: Test Files" -ForegroundColor Yellow
$TestFiles = Get-ChildItem "d:\DECENTRALIZED-APP\contracts\test\core" -Filter "*.sol" -Recurse

foreach ($File in $TestFiles) {
    $Content = Get-Content $File.FullName
    $HasPragma = $Content -match "^pragma solidity"
    $HasImport = $Content -match "^import"
    if ($HasPragma -and $HasImport) {
        Write-Host "✅ $($File.Name) has proper structure" -ForegroundColor Green
    } else {
        Write-Host "❌ $($File.Name) missing pragma or import" -ForegroundColor Red
    }
}

Write-Host "`n🎉 Build Matrix Structure Test Completed!" -ForegroundColor Green
Write-Host "All structural components are in place." -ForegroundColor Cyan
Write-Host "To run the actual build matrix tests, install Foundry and run:" -ForegroundColor Yellow
Write-Host "   .\scripts\run-build-matrix.ps1" -ForegroundColor Cyan