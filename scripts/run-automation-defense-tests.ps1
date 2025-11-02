# Script to run automation and defense tests
# This script validates the implementation of automated remediation and policy enforcement features

Write-Host "Running Automation & Defense Tests..." -ForegroundColor Yellow

# Check that required modules exist
Write-Host "Checking for required modules..." -ForegroundColor Yellow

$RequiredFiles = @(
    "crates/core/src/automated_remediation.rs",
    "crates/core/src/policy_enforcement.rs",
    "crates/core/tests/automation_defense_tests.rs"
)

foreach ($file in $RequiredFiles) {
    if (-not (Test-Path $file)) {
        Write-Host "ERROR: Required file missing: $file" -ForegroundColor Red
        exit 1
    }
    Write-Host "[PASS] $file exists" -ForegroundColor Green
}

# Run the tests
Write-Host "Running automation defense tests..." -ForegroundColor Yellow

# Set environment for tests
$env:RUST_BACKTRACE = "1"

# Run tests with cargo
cargo test --package core --test automation_defense_tests

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ All automation defense tests passed!" -ForegroundColor Green
} else {
    Write-Host "❌ Some automation defense tests failed!" -ForegroundColor Red
    exit 1
}

Write-Host "Automation & Defense validation complete!" -ForegroundColor Yellow