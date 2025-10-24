#!/bin/bash

# Script to run Data Protection security tests

echo "Running Data Protection Security Tests..."

# Set working directory to project root
cd "d:\DECENTRALIZED-APP" || cd "$(dirname "$0")/.."

# Run Rust tests for data protection security module
echo "Running Rust data protection security tests..."
cargo test -p core data_protection

if [ $? -ne 0 ]; then
    echo "Rust data protection security tests failed!"
    exit 1
fi

# Run data protection simulation tests
echo "Running data protection simulation tests..."
cargo test data_protection_simulation

if [ $? -ne 0 ]; then
    echo "Data protection simulation tests failed!"
    exit 1
fi

echo "All Data Protection security tests passed!"