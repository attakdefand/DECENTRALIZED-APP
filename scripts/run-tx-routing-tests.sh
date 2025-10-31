#!/bin/bash

# Script to run Transaction Routing security tests

echo "Running Transaction Routing Security Tests..."

# Set working directory to project root
cd "d:\DECENTRALIZED-APP" || cd "$(dirname "$0")/.."

# Run Rust tests for TX routing security module
echo "Running Rust TX routing security tests..."
cargo test -p core tx_routing

if [ $? -ne 0 ]; then
    echo "Rust TX routing security tests failed!"
    exit 1
fi

# Run Solidity tests for TX routing contracts
echo "Running Solidity TX routing contract tests..."
cd contracts
forge test --match-contract TxRoutingTests -vvv

if [ $? -ne 0 ]; then
    echo "Solidity TX routing contract tests failed!"
    exit 1
fi

echo "All Transaction Routing security tests passed!"