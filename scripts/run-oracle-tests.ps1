# Oracle testing script
# This script runs all oracle integrity tests

Write-Host "Running oracle integrity tests..." -ForegroundColor Yellow

# Run all oracle integrity tests
Write-Host "Running basic oracle integrity tests..." -ForegroundColor Green
cargo test oracle_integrity -- --nocapture

# Run specific manipulation tests
Write-Host "Running price manipulation tests..." -ForegroundColor Green
cargo test test_price_manipulation_detection
cargo test test_normal_price_movement

# Run staleness tests
Write-Host "Running data staleness tests..." -ForegroundColor Green
cargo test test_data_staleness
cargo test test_confidence_level

# Run complete integrity checks
Write-Host "Running complete integrity checks..." -ForegroundColor Green
cargo test test_complete_integrity_check
cargo test test_integrity_check_with_failures

# Run performance tests
Write-Host "Running performance tests..." -ForegroundColor Green
cargo test test_integrity_check_performance --release

# Test the oracle crate specifically
Write-Host "Testing oracle crate..." -ForegroundColor Green
Set-Location crates/oracle
cargo test --lib
cargo test --doc

Write-Host "All oracle integrity tests completed!" -ForegroundColor Green