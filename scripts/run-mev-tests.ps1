#!/usr/bin/env pwsh

# Script to run MEV mitigation tests

Write-Host "Running MEV Mitigation Tests..." -ForegroundColor Green

# Set working directory to project root
Set-Location -Path "d:\DECENTRALIZED-APP"

# Run Rust tests for MEV mitigation module
Write-Host "Running Rust MEV mitigation tests..." -ForegroundColor Yellow
cargo test -p core mev_mitigation

if ($LASTEXITCODE -ne 0) {
    Write-Host "Rust MEV mitigation tests failed!" -ForegroundColor Red
    exit 1
}

# Run Solidity tests for MEV contracts
Write-Host "Running Solidity MEV contract tests..." -ForegroundColor Yellow
cd contracts
forge test --match-contract MEVTests -vvv

if ($LASTEXITCODE -ne 0) {
    Write-Host "Solidity MEV contract tests failed!" -ForegroundColor Red
    exit 1
}

Write-Host "All MEV mitigation tests passed!" -ForegroundColor Green