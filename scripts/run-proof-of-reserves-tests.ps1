# Proof of Reserves Tests Runner
# This script runs all tests related to Proof of Reserves and treasury management features

Write-Host "Running Proof of Reserves and Treasury Management tests..." -ForegroundColor Yellow

# Run Rust unit tests for Proof of Reserves
Write-Host "Running Rust unit tests for Proof of Reserves..." -ForegroundColor Green
cargo test proof_of_reserves

# Run Rust integration tests
Write-Host "Running Rust integration tests..." -ForegroundColor Green
cargo test test_risk_manager_por_features
cargo test test_proof_of_reserves_report
cargo test test_treasury_metrics
cargo test test_evidence_generation

# Run Solidity tests for Vault contract enhancements
Write-Host "Running Solidity tests for Vault contract..." -ForegroundColor Green
forge test --match-contract TreasuryManagementTest -vvv

# Run the validation script
Write-Host "Running Proof of Reserves validation script..." -ForegroundColor Green
$env:POR_FRESHNESS_HOURS = 12
$env:LIMIT_BREACH_COUNT = 0
.\scripts\validate-proof-of-reserves.ps1

Write-Host "All Proof of Reserves and Treasury Management tests completed!" -ForegroundColor Green