#!/bin/bash

# Script to run MEV mitigation tests

echo "Running MEV Mitigation Tests..."

# Set working directory to project root
cd "d:\DECENTRALIZED-APP" || cd "$(dirname "$0")/.."

# Run Rust tests for MEV mitigation module
echo "Running Rust MEV mitigation tests..."
cargo test -p core mev_mitigation

if [ $? -ne 0 ]; then
    echo "Rust MEV mitigation tests failed!"
    exit 1
fi

# Run Solidity tests for MEV contracts
echo "Running Solidity MEV contract tests..."
cd contracts
forge test --match-contract MEVTests -vvv

if [ $? -ne 0 ]; then
    echo "Solidity MEV contract tests failed!"
    exit 1
fi

echo "All MEV mitigation tests passed!"