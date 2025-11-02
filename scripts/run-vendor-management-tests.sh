#!/bin/bash

# Vendor Management Tests Runner
# This script runs all tests related to vendor management features

echo "Running Vendor Management tests..."

# Run Rust unit tests for vendor management
echo "Running Rust unit tests for vendor management..."
cargo test vendor_management

# Run Rust integration tests
echo "Running Rust integration tests..."
cargo test test_vendor_risk_assessment
cargo test test_sla_monitoring
cargo test test_vendor_access_management
cargo test test_vendor_metrics_tracking
cargo test test_evidence_generation

# Run the validation script
echo "Running Vendor Management validation script..."
export VENDOR_SCORE_AVG=85
export OVERDUE_REVIEWS=0
./scripts/validate-vendor-management.sh

echo "All Vendor Management tests completed!"