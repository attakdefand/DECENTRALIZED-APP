# Vendor Management Tests Runner
# This script runs all tests related to vendor management features

Write-Host "Running Vendor Management tests..." -ForegroundColor Yellow

# Run Rust unit tests for vendor management
Write-Host "Running Rust unit tests for vendor management..." -ForegroundColor Green
cargo test vendor_management

# Run Rust integration tests
Write-Host "Running Rust integration tests..." -ForegroundColor Green
cargo test test_vendor_risk_assessment
cargo test test_sla_monitoring
cargo test test_vendor_access_management
cargo test test_vendor_metrics_tracking
cargo test test_evidence_generation

# Run the validation script
Write-Host "Running Vendor Management validation script..." -ForegroundColor Green
$env:VENDOR_SCORE_AVG = 85
$env:OVERDUE_REVIEWS = 0
.\scripts\validate-vendor-management.ps1

Write-Host "All Vendor Management tests completed!" -ForegroundColor Green