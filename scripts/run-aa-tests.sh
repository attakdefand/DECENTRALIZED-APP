#!/bin/bash

# Script to run Account Abstraction security tests

echo "Running Account Abstraction Security Tests..."

# Set working directory to project root
cd "d:\DECENTRALIZED-APP" || cd "$(dirname "$0")/.."

# Run Rust tests for AA security module
echo "Running Rust AA security tests..."
cargo test -p core aa_security

if [ $? -ne 0 ]; then
    echo "Rust AA security tests failed!"
    exit 1
fi

# Run Solidity tests for AA contracts
echo "Running Solidity AA contract tests..."
cd contracts
forge test --match-contract AATests -vvv

if [ $? -ne 0 ]; then
    echo "Solidity AA contract tests failed!"
    exit 1
fi

echo "All Account Abstraction security tests passed!"