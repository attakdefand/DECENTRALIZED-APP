#!/bin/bash

# Run safety tests script

echo "Running safety tests..."

# Run all tests
forge test -vvv

# Run specific safety tests with gas reports
forge test --match-contract SafetyTests --gas-report

# Run invariant tests if any
forge test --match-test invariant

echo "Safety tests completed!"