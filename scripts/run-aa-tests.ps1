#!/usr/bin/env pwsh

# Script to run Account Abstraction security tests

Write-Host "Running Account Abstraction Security Tests..." -ForegroundColor Green

# Set working directory to project root
Set-Location -Path "d:\DECENTRALIZED-APP"

# Run Rust tests for AA security module
Write-Host "Running Rust AA security tests..." -ForegroundColor Yellow
cargo test -p core aa_security

if ($LASTEXITCODE -ne 0) {
    Write-Host "Rust AA security tests failed!" -ForegroundColor Red
    exit 1
}

# Run Solidity tests for AA contracts
Write-Host "Running Solidity AA contract tests..." -ForegroundColor Yellow
cd contracts
forge test --match-contract AATests -vvv

if ($LASTEXITCODE -ne 0) {
    Write-Host "Solidity AA contract tests failed!" -ForegroundColor Red
    exit 1
}

Write-Host "All Account Abstraction security tests passed!" -ForegroundColor Green