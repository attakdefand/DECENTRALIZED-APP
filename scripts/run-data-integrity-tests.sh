#!/bin/bash

# Script to run Data Integrity security tests

echo "Running Data Integrity Security Tests..."

# Set working directory to project root
cd "$(dirname "$0")/.." || exit 1

# Run Rust tests for data integrity security module
echo "Running Rust data integrity security tests..."
cargo test -p core data_integrity

if [ $? -ne 0 ]; then
    echo "Rust data integrity security tests failed!"
    exit 1
fi

# Run data integrity integration tests
echo "Running data integrity integration tests..."
cargo test -p core --test data_integrity_integration

if [ $? -ne 0 ]; then
    echo "Data integrity integration tests failed!"
    exit 1
fi

# Run data integrity simulation tests
echo "Running data integrity simulation tests..."
cargo test --test data_integrity_simulation

if [ $? -ne 0 ]; then
    echo "Data integrity simulation tests failed!"
    exit 1
fi

echo "All data integrity security tests passed!"