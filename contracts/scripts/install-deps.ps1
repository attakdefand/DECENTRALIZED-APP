#!/usr/bin/env pwsh

# Script to install contract dependencies

Write-Host "Installing contract dependencies..." -ForegroundColor Green

# Set working directory to contracts
Set-Location -Path "d:\DECENTRALIZED-APP\contracts"

# Create lib directory if it doesn't exist
if (!(Test-Path "lib")) {
    New-Item -ItemType Directory -Name "lib"
}

# Install Forge dependencies
Write-Host "Installing Forge dependencies..." -ForegroundColor Yellow
forge install --no-commit

# Install OpenZeppelin Contracts
Write-Host "Installing OpenZeppelin Contracts..." -ForegroundColor Yellow
if (!(Test-Path "lib\openzeppelin-contracts")) {
    git clone https://github.com/OpenZeppelin/openzeppelin-contracts.git lib/openzeppelin-contracts
}

# Install Forge Standard Library
Write-Host "Installing Forge Standard Library..." -ForegroundColor Yellow
if (!(Test-Path "lib\forge-std")) {
    git clone https://github.com/foundry-rs/forge-std.git lib/forge-std
}

# Verify installations
Write-Host "Verifying installations..." -ForegroundColor Yellow
if (Test-Path "lib\openzeppelin-contracts") {
    Write-Host "✅ OpenZeppelin Contracts installed" -ForegroundColor Green
} else {
    Write-Host "❌ OpenZeppelin Contracts installation failed" -ForegroundColor Red
}

if (Test-Path "lib\forge-std") {
    Write-Host "✅ Forge Standard Library installed" -ForegroundColor Green
} else {
    Write-Host "❌ Forge Standard Library installation failed" -ForegroundColor Red
}

# Run a simple build to verify everything works
Write-Host "Running test build..." -ForegroundColor Yellow
forge build --profile default

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Test build successful" -ForegroundColor Green
} else {
    Write-Host "❌ Test build failed" -ForegroundColor Red
}

# Return to original directory
Set-Location -Path "d:\DECENTRALIZED-APP"

Write-Host "Contract dependencies installation completed!" -ForegroundColor Green