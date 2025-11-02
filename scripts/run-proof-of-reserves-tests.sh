#!/bin/bash

# Proof of Reserves Tests Runner
# This script runs all tests related to Proof of Reserves and treasury management features

echo "Running Proof of Reserves and Treasury Management tests..."

# Run Rust unit tests for Proof of Reserves
echo "Running Rust unit tests for Proof of Reserves..."
cargo test proof_of_reserves

# Run Rust integration tests
echo "Running Rust integration tests..."
cargo test test_risk_manager_por_features
cargo test test_proof_of_reserves_report
cargo test test_treasury_metrics
cargo test test_evidence_generation

# Run Solidity tests for Vault contract enhancements
echo "Running Solidity tests for Vault contract..."
forge test --match-contract TreasuryManagementTest -vvv

# Run the validation script
echo "Running Proof of Reserves validation script..."
export POR_FRESHNESS_HOURS=12
export LIMIT_BREACH_COUNT=0
./scripts/validate-proof-of-reserves.sh

echo "All Proof of Reserves and Treasury Management tests completed!"