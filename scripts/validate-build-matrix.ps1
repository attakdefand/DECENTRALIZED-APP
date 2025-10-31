#!/usr/bin/env pwsh

# Script to validate the build matrix implementation

Write-Host "Validating Build Matrix Implementation..." -ForegroundColor Green

# Check that required files exist
$RequiredFiles = @(
    "contracts/foundry.toml",
    "contracts/remappings.txt",
    "scripts/run-build-matrix.ps1",
    "docs/contracts/BUILD-MATRIX.md",
    ".github/workflows/contract-build-matrix.yml"
)

$AllFilesExist = $true
foreach ($File in $RequiredFiles) {
    if (Test-Path "d:\DECENTRALIZED-APP\$File") {
        Write-Host "✅ $File exists" -ForegroundColor Green
    } else {
        Write-Host "❌ $File is missing" -ForegroundColor Red
        $AllFilesExist = $false
    }
}

# Check that test file exists
if (Test-Path "d:\DECENTRALIZED-APP\contracts\test\core\BuildMatrixTest.sol") {
    Write-Host "✅ BuildMatrixTest.sol exists" -ForegroundColor Green
} else {
    Write-Host "❌ BuildMatrixTest.sol is missing" -ForegroundColor Red
    $AllFilesExist = $false
}

# Check that dependency installation script exists
if (Test-Path "d:\DECENTRALIZED-APP\contracts\scripts\install-deps.ps1") {
    Write-Host "✅ install-deps.ps1 exists" -ForegroundColor Green
} else {
    Write-Host "❌ install-deps.ps1 is missing" -ForegroundColor Red
    $AllFilesExist = $false
}

# Validate Foundry.toml configuration
Write-Host "Validating Foundry.toml configuration..." -ForegroundColor Yellow
$FoundryConfig = Get-Content "d:\DECENTRALIZED-APP\contracts\foundry.toml"
$RequiredProfiles = @("default", "optimized", "ir", "ir-optimized")

foreach ($Profile in $RequiredProfiles) {
    if ($FoundryConfig -match "\[profile\.$Profile\]") {
        Write-Host "✅ Profile $Profile configured" -ForegroundColor Green
    } else {
        Write-Host "❌ Profile $Profile missing" -ForegroundColor Red
        $AllFilesExist = $false
    }
}

# Validate remappings
Write-Host "Validating remappings..." -ForegroundColor Yellow
$Remappings = Get-Content "d:\DECENTRALIZED-APP\contracts\remappings.txt"
$RequiredRemappings = @("@openzeppelin/contracts/", "forge-std/", "@openzeppelin/contracts-upgradeable/")

foreach ($Remapping in $RequiredRemappings) {
    if ($Remappings -match $Remapping) {
        Write-Host "✅ Remapping $Remapping configured" -ForegroundColor Green
    } else {
        Write-Host "❌ Remapping $Remapping missing" -ForegroundColor Red
        $AllFilesExist = $false
    }
}

# Check for Foundry installation
Write-Host "Checking for Foundry installation..." -ForegroundColor Yellow
$FoundryInstalled = $false
try {
    $ForgeVersion = forge --version 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Foundry is installed" -ForegroundColor Green
        Write-Host "   $ForgeVersion" -ForegroundColor Cyan
        $FoundryInstalled = $true
    } else {
        Write-Host "❌ Foundry is not installed" -ForegroundColor Red
    }
} catch {
    Write-Host "❌ Foundry is not installed" -ForegroundColor Red
}

# Summary
Write-Host "`nValidation Summary:" -ForegroundColor Yellow
Write-Host "==================" -ForegroundColor Yellow

if ($AllFilesExist -and $FoundryInstalled) {
    Write-Host "✅ All required files exist" -ForegroundColor Green
    Write-Host "✅ Foundry.toml profiles configured correctly" -ForegroundColor Green
    Write-Host "✅ Remappings configured correctly" -ForegroundColor Green
    Write-Host "✅ Foundry is installed" -ForegroundColor Green
    Write-Host "`n🎉 Build matrix implementation is ready for use!" -ForegroundColor Green
    Write-Host "`nTo run the build matrix tests:" -ForegroundColor Yellow
    Write-Host "   .\scripts\run-build-matrix.ps1" -ForegroundColor Cyan
    exit 0
} else {
    Write-Host "❌ Some validation checks failed" -ForegroundColor Red
    if (-not $FoundryInstalled) {
        Write-Host "`nTo install Foundry:" -ForegroundColor Yellow
        Write-Host "   Visit https://getfoundry.sh/ and follow installation instructions" -ForegroundColor Cyan
        Write-Host "   Or run: curl -L https://foundry.paradigm.xyz | bash" -ForegroundColor Cyan
    }
    Write-Host "`nPlease check the missing files and configurations" -ForegroundColor Yellow
    exit 1
}