#!/usr/bin/env pwsh

# Run Resilience and Availability Tests Script
# This script runs all resilience and availability tests for the decentralized application

Write-Host "Running Resilience and Availability Tests" -ForegroundColor Green

# Set error action preference to stop on error
$ErrorActionPreference = "Stop"

try {
    # Run unit tests
    Write-Host "Running unit tests..." -ForegroundColor Yellow
    cargo test --package core --test resilience_availability_integration

    # Run simulation tests
    Write-Host "Running simulation tests..." -ForegroundColor Yellow
    cargo run --package core --bin resilience_availability_simulation

    # Run integration tests
    Write-Host "Running integration tests..." -ForegroundColor Yellow
    cargo test --test resilience_availability_simulation

    Write-Host "All Resilience and Availability Tests Passed!" -ForegroundColor Green
}
catch {
    Write-Host "Resilience and Availability Tests Failed!" -ForegroundColor Red
    Write-Host "Error: $_" -ForegroundColor Red
    exit 1
}