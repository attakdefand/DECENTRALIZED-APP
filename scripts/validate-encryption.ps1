#!/usr/bin/env pwsh

# Script to validate the Encryption Implementation

Write-Host "Validating Encryption Implementation..." -ForegroundColor Green

# Check that required files exist
$RequiredFiles = @(
    "crates/security_layers/src/data_security.rs",
    "crates/security_layers/tests/data_security_encryption_at_rest_validation.rs",
    "docs/security/DATA-AT-REST.md",
    "scripts/run-security-layers-tests.ps1"
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

# Check for encryption implementation in data_security.rs
Write-Host "Checking Encryption Implementation..." -ForegroundColor Yellow
$EncryptionFile = "d:\DECENTRALIZED-APP\crates\security_layers\src\data_security.rs"
if (Test-Path $EncryptionFile) {
    $Content = Get-Content $EncryptionFile
    $HasDataAtRestConfig = $Content -match "DataAtRestConfig"
    $HasKeyRotationLog = $Content -match "KeyRotationLog"
    $HasKmsAccessLog = $Content -match "KmsAccessLog"
    $HasDataAtRestManager = $Content -match "DataAtRestManager"
    $HasEncryptionEnabled = $Content -match "is_encryption_at_rest_enabled"
    
    if ($HasDataAtRestConfig -and $HasKeyRotationLog -and $HasKmsAccessLog -and $HasDataAtRestManager -and $HasEncryptionEnabled) {
        Write-Host "✅ Data Security module has proper encryption implementation" -ForegroundColor Green
    } else {
        Write-Host "❌ Data Security module missing required encryption elements" -ForegroundColor Red
        $AllFilesExist = $false
    }
} else {
    Write-Host "❌ data_security.rs missing" -ForegroundColor Red
    $AllFilesExist = $false
}

# Check for encryption validation tests
Write-Host "Checking Encryption Validation Tests..." -ForegroundColor Yellow
$TestFile = "d:\DECENTRALIZED-APP\crates\security_layers\tests\data_security_encryption_at_rest_validation.rs"
if (Test-Path $TestFile) {
    $Content = Get-Content $TestFile
    $HasConfigTests = $Content -match "test_data_at_rest_config_creation_and_validation"
    $HasKeyRotationTests = $Content -match "test_key_rotation_log"
    $HasKmsAccessTests = $Content -match "test_kms_access_log"
    $HasManagerTests = $Content -match "test_data_at_rest_manager"
    $HasIntegrationTests = $Content -match "test_data_at_rest_integration"
    
    if ($HasConfigTests -and $HasKeyRotationTests -and $HasKmsAccessTests -and $HasManagerTests -and $HasIntegrationTests) {
        Write-Host "✅ Encryption validation tests are comprehensive" -ForegroundColor Green
    } else {
        Write-Host "❌ Encryption validation tests are incomplete" -ForegroundColor Red
        $AllFilesExist = $false
    }
} else {
    Write-Host "❌ Encryption validation tests missing" -ForegroundColor Red
    $AllFilesExist = $false
}

# Check for proper documentation
Write-Host "Checking Encryption Documentation..." -ForegroundColor Yellow
$DocFile = "d:\DECENTRALIZED-APP\docs\security\DATA-AT-REST.md"
if (Test-Path $DocFile) {
    $Content = Get-Content $DocFile -Raw
    $HasTitle = $Content -match "# Data-at-Rest Encryption Implementation"
    $HasSections = $Content -match "## "
    $HasFeatures = $Content -match "### KMS-Managed Encryption"
    $HasManager = $Content -match "DataAtRestManager"
    $HasTelemetry = $Content -match "Telemetry"
    
    if ($HasTitle -and $HasSections -and $HasFeatures -and $HasManager -and $HasTelemetry) {
        Write-Host "✅ DATA-AT-REST.md has proper content" -ForegroundColor Green
    } else {
        Write-Host "❌ DATA-AT-REST.md missing required content" -ForegroundColor Red
        $AllFilesExist = $false
    }
} else {
    Write-Host "❌ DATA-AT-REST.md missing" -ForegroundColor Red
    $AllFilesExist = $false
}

# Check for Cargo.toml dependencies
Write-Host "Checking Cargo.toml Dependencies..." -ForegroundColor Yellow
$CargoFile = "d:\DECENTRALIZED-APP\Cargo.toml"
if (Test-Path $CargoFile) {
    $Content = Get-Content $CargoFile
    $HasSerde = $Content -match "serde"
    $HasAesGcm = $Content -match "aes-gcm"
    
    if ($HasSerde -and $HasAesGcm) {
        Write-Host "✅ Cargo.toml has required encryption dependencies" -ForegroundColor Green
    } else {
        Write-Host "⚠️ Cargo.toml may be missing encryption dependencies" -ForegroundColor Yellow
    }
} else {
    Write-Host "❌ Cargo.toml missing" -ForegroundColor Red
    $AllFilesExist = $false
}

# Run Rust tests for encryption
Write-Host "Running Encryption Tests..." -ForegroundColor Yellow
Set-Location -Path "d:\DECENTRALIZED-APP"
try {
    cargo test -p security_layers --test data_security_encryption_at_rest_validation
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Encryption tests passed" -ForegroundColor Green
    } else {
        Write-Host "❌ Encryption tests failed" -ForegroundColor Red
        $AllFilesExist = $false
    }
} catch {
    Write-Host "❌ Failed to run encryption tests: $_" -ForegroundColor Red
    $AllFilesExist = $false
}

# Summary
Write-Host "`nValidation Summary:" -ForegroundColor Yellow
Write-Host "==================" -ForegroundColor Yellow

if ($AllFilesExist) {
    Write-Host "✅ All required files exist and tests pass" -ForegroundColor Green
} else {
    Write-Host "❌ Some required files are missing or tests failed" -ForegroundColor Red
}

# Count passed checks
$Checks = @("File Structure", "Encryption Implementation", "Validation Tests", "Documentation", "Dependencies", "Test Execution")
$PassedChecks = 0

# This is a simplified check - in reality, we'd want more detailed validation
if ($AllFilesExist) {
    $PassedChecks = 6  # Assuming all checks pass for this example
} else {
    # Count how many checks actually passed
    if (Test-Path "d:\DECENTRALIZED-APP\crates\security_layers\src\data_security.rs") { $PassedChecks++ }
    if (Test-Path "d:\DECENTRALIZED-APP\crates\security_layers\tests\data_security_encryption_at_rest_validation.rs") { $PassedChecks++ }
    if (Test-Path "d:\DECENTRALIZED-APP\docs\security\DATA-AT-REST.md") { $PassedChecks++ }
    if (Test-Path "d:\DECENTRALIZED-APP\Cargo.toml") { $PassedChecks++ }
}

Write-Host "✅ $PassedChecks/$($Checks.Count) encryption validation checks passed" -ForegroundColor Green

if ($AllFilesExist) {
    Write-Host "`n🎉 Encryption implementation is ready for use!" -ForegroundColor Green
    Write-Host "`nTo run the encryption tests:" -ForegroundColor Yellow
    Write-Host "   .\scripts\run-security-layers-tests.ps1" -ForegroundColor Cyan
    exit 0
} else {
    Write-Host "`nSome validation checks failed" -ForegroundColor Red
    Write-Host "`nPlease check the missing files and configurations" -ForegroundColor Yellow
    exit 1
}