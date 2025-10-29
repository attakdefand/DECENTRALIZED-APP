#!/bin/bash

# Test script for CodeQL custom queries

echo "Creating CodeQL database from test project..."
codeql database create test-db --language=rust --source-root=test-project

echo "Running CodeQL queries against test database..."
codeql database analyze test-db codeql-suite.qls --format=csv --output=test-results.csv

echo "Query results saved to test-results.csv"

echo "Running queries individually for detailed output..."
for query in queries/*/*/*.ql; do
    echo "Running $query"
    codeql database analyze test-db "$query" --format=csv --output="results-$(basename "$query" .ql).csv"
done

echo "All tests completed!"