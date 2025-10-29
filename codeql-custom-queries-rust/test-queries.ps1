# Test script for CodeQL custom queries

Write-Host "Creating CodeQL database from test project..."
codeql database create test-db --language=rust --source-root=test-project

Write-Host "Running CodeQL queries against test database..."
codeql database analyze test-db codeql-suite.qls --format=csv --output=test-results.csv

Write-Host "Query results saved to test-results.csv"

Write-Host "Running queries individually for detailed output..."
Get-ChildItem -Path "queries" -Recurse -Filter "*.ql" | ForEach-Object {
    $queryName = $_.BaseName
    Write-Host "Running $($_.FullName)"
    codeql database analyze test-db $_.FullName --format=csv --output="results-$queryName.csv"
}

Write-Host "All tests completed!"