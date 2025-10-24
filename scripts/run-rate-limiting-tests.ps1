#!/usr/bin/env pwsh

# Script to run Rate Limiting security tests

Write-Host "Running Rate Limiting Security Tests..." -ForegroundColor Green

# Set working directory to project root
Set-Location -Path "d:\DECENTRALIZED-APP"

# Run Rust tests for rate limiting security module
Write-Host "Running Rust rate limiting security tests..." -ForegroundColor Yellow
cargo test -p core rate_limiting

if ($LASTEXITCODE -ne 0) {
    Write-Host "Rust rate limiting security tests failed!" -ForegroundColor Red
    exit 1
}

# Run rate limiting simulation tests
Write-Host "Running rate limiting simulation tests..." -ForegroundColor Yellow
cargo test rate_limiting_simulation

if ($LASTEXITCODE -ne 0) {
    Write-Host "Rate limiting simulation tests failed!" -ForegroundColor Red
    exit 1
}

Write-Host "All Rate Limiting security tests passed!" -ForegroundColor Green