# Install Foundry dependencies script

Write-Host "Installing Foundry dependencies..." -ForegroundColor Yellow

# Install Foundry
Write-Host "Installing Foundry..." -ForegroundColor Green
curl -L https://foundry.paradigm.xyz | bash

# Install dependencies
Write-Host "Installing OpenZeppelin contracts..." -ForegroundColor Green
forge install --no-commit openzeppelin/openzeppelin-contracts

Write-Host "Installing Forge standard library..." -ForegroundColor Green
forge install --no-commit foundry-rs/forge-std

Write-Host "Dependencies installed successfully!" -ForegroundColor Green