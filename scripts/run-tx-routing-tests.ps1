#!/usr/bin/env pwsh

# Script to run Transaction Routing security tests

Write-Host "Running Transaction Routing Security Tests..." -ForegroundColor Green

# Set working directory to project root
Set-Location -Path "d:\DECENTRALIZED-APP"

# Run Rust tests for TX routing security module
Write-Host "Running Rust TX routing security tests..." -ForegroundColor Yellow
cargo test -p core tx_routing

if ($LASTEXITCODE -ne 0) {
    Write-Host "Rust TX routing security tests failed!" -ForegroundColor Red
    exit 1
}

# Run Solidity tests for TX routing contracts
Write-Host "Running Solidity TX routing contract tests..." -ForegroundColor Yellow
cd contracts
forge test --match-contract TxRoutingTests -vvv

if ($LASTEXITCODE -ne 0) {
    Write-Host "Solidity TX routing contract tests failed!" -ForegroundColor Red
    exit 1
}

Write-Host "All Transaction Routing security tests passed!" -ForegroundColor Green