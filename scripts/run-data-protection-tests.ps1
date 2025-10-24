#!/usr/bin/env pwsh

# Script to run Data Protection security tests

Write-Host "Running Data Protection Security Tests..." -ForegroundColor Green

# Set working directory to project root
Set-Location -Path "d:\DECENTRALIZED-APP"

# Run Rust tests for data protection security module
Write-Host "Running Rust data protection security tests..." -ForegroundColor Yellow
cargo test -p core data_protection

if ($LASTEXITCODE -ne 0) {
    Write-Host "Rust data protection security tests failed!" -ForegroundColor Red
    exit 1
}

# Run data protection simulation tests
Write-Host "Running data protection simulation tests..." -ForegroundColor Yellow
cargo test data_protection_simulation

if ($LASTEXITCODE -ne 0) {
    Write-Host "Data protection simulation tests failed!" -ForegroundColor Red
    exit 1
}

Write-Host "All Data Protection security tests passed!" -ForegroundColor Green