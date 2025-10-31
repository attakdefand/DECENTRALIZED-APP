# Economic testing script
# This script runs all economic simulation tests

Write-Host "Running economic simulation tests..." -ForegroundColor Yellow

# Run all economic simulation tests
Write-Host "Running basic economic simulation tests..." -ForegroundColor Green
cargo test economic_simulation -- --nocapture

# Run specific risk parameter tests
Write-Host "Running risk parameter validation tests..." -ForegroundColor Green
cargo test test_risk_parameter_validation
cargo test test_fee_distribution
cargo test test_insurance_fund_calculations

# Run liquidation scenario tests
Write-Host "Running liquidation scenario tests..." -ForegroundColor Green
cargo test test_liquidation_scenarios

# Run risk monitoring tests
Write-Host "Running risk monitoring tests..." -ForegroundColor Green
cargo test test_risk_monitoring
cargo test test_emergency_procedures

# Run performance tests
Write-Host "Running performance tests..." -ForegroundColor Green
cargo test test_risk_calculation_performance --release

# Run integration tests
Write-Host "Running integration tests..." -ForegroundColor Green
cargo test test_complete_risk_workflow

Write-Host "All economic simulation tests completed!" -ForegroundColor Green