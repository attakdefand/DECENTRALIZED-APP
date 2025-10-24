#!/bin/bash

# Script to run Rate Limiting security tests

echo "Running Rate Limiting Security Tests..."

# Set working directory to project root
cd "d:\DECENTRALIZED-APP" || cd "$(dirname "$0")/.."

# Run Rust tests for rate limiting security module
echo "Running Rust rate limiting security tests..."
cargo test -p core rate_limiting

if [ $? -ne 0 ]; then
    echo "Rust rate limiting security tests failed!"
    exit 1
fi

# Run rate limiting simulation tests
echo "Running rate limiting simulation tests..."
cargo test rate_limiting_simulation

if [ $? -ne 0 ]; then
    echo "Rate limiting simulation tests failed!"
    exit 1
fi

echo "All Rate Limiting security tests passed!"