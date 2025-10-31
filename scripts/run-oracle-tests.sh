#!/bin/bash

# Oracle testing script
# This script runs all oracle integrity tests

echo "Running oracle integrity tests..."

# Run all oracle integrity tests
echo "Running basic oracle integrity tests..."
cargo test oracle_integrity -- --nocapture

# Run specific manipulation tests
echo "Running price manipulation tests..."
cargo test test_price_manipulation_detection
cargo test test_normal_price_movement

# Run staleness tests
echo "Running data staleness tests..."
cargo test test_data_staleness
cargo test test_confidence_level

# Run complete integrity checks
echo "Running complete integrity checks..."
cargo test test_complete_integrity_check
cargo test test_integrity_check_with_failures

# Run performance tests
echo "Running performance tests..."
cargo test test_integrity_check_performance --release

# Test the oracle crate specifically
echo "Testing oracle crate..."
cd crates/oracle
cargo test --lib
cargo test --doc

echo "All oracle integrity tests completed!"