#!/usr/bin/env pwsh

# Script to run all security layers tests

Write-Host "Running Security Layers Tests..." -ForegroundColor Green

# Set working directory to project root
Set-Location -Path "d:\DECENTRALIZED-APP"

# Run Rust tests for security layers
Write-Host "Running Rust security layers tests..." -ForegroundColor Yellow
cargo test -p security_layers

if ($LASTEXITCODE -ne 0) {
    Write-Host "Rust security layers tests failed!" -ForegroundColor Red
    exit 1
}

# Run security layers validation tests
Write-Host "Running security layers validation tests..." -ForegroundColor Yellow
cargo test -p security_layers test_security_layers_from_csv

if ($LASTEXITCODE -ne 0) {
    Write-Host "Security layers validation tests failed!" -ForegroundColor Red
    exit 1
}

# Run security layers simulation
Write-Host "Running security layers simulation..." -ForegroundColor Yellow
cargo run -p security_layers

if ($LASTEXITCODE -ne 0) {
    Write-Host "Security layers simulation failed!" -ForegroundColor Red
    exit 1
}

Write-Host "All Security Layers tests passed!" -ForegroundColor Green