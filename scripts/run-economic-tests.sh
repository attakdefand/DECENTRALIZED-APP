#!/bin/bash

# Economic testing script
# This script runs all economic simulation tests

echo "Running economic simulation tests..."

# Run all economic simulation tests
echo "Running basic economic simulation tests..."
cargo test economic_simulation -- --nocapture

# Run specific risk parameter tests
echo "Running risk parameter validation tests..."
cargo test test_risk_parameter_validation
cargo test test_fee_distribution
cargo test test_insurance_fund_calculations

# Run liquidation scenario tests
echo "Running liquidation scenario tests..."
cargo test test_liquidation_scenarios
cargo test test_liquidation_scenarios

# Run risk monitoring tests
echo "Running risk monitoring tests..."
cargo test test_risk_monitoring
cargo test test_emergency_procedures

# Run performance tests
echo "Running performance tests..."
cargo test test_risk_calculation_performance --release

# Run integration tests
echo "Running integration tests..."
cargo test test_complete_risk_workflow

echo "All economic simulation tests completed!"