# Script to run Data Integrity security tests on Windows

Write-Host "Running Data Integrity Security Tests..." -ForegroundColor Green

# Set working directory to project root
Set-Location -Path "$PSScriptRoot\.."

# Run Rust tests for data integrity security module
Write-Host "Running Rust data integrity security tests..." -ForegroundColor Yellow
cargo test -p core data_integrity

if ($LASTEXITCODE -ne 0) {
    Write-Host "Rust data integrity security tests failed!" -ForegroundColor Red
    exit 1
}

# Run data integrity integration tests
Write-Host "Running data integrity integration tests..." -ForegroundColor Yellow
cargo test -p core --test data_integrity_integration

if ($LASTEXITCODE -ne 0) {
    Write-Host "Data integrity integration tests failed!" -ForegroundColor Red
    exit 1
}

# Run data integrity simulation tests
Write-Host "Running data integrity simulation tests..." -ForegroundColor Yellow
cargo test --test data_integrity_simulation

if ($LASTEXITCODE -ne 0) {
    Write-Host "Data integrity simulation tests failed!" -ForegroundColor Red
    exit 1
}

Write-Host "All data integrity security tests passed!" -ForegroundColor Green