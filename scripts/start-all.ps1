# Script to start all components of the decentralized application

Write-Host "Starting decentralized application..." -ForegroundColor Green

# Start infrastructure components
Write-Host "Starting infrastructure components..." -ForegroundColor Yellow
Set-Location -Path "infra"
docker-compose up -d
Set-Location -Path ".."

# Wait a moment for infrastructure to start
Start-Sleep -Seconds 10

# Start off-chain services
Write-Host "Starting off-chain services..." -ForegroundColor Yellow

# Start the indexer service
Write-Host "Starting indexer service..." -ForegroundColor Yellow
Start-Process -NoNewWindow -FilePath "cargo" -ArgumentList "run", "--bin", "indexer" -WorkingDirectory "services/indexer-rs"

# Start the API service
Write-Host "Starting API service..." -ForegroundColor Yellow
Start-Process -NoNewWindow -FilePath "cargo" -ArgumentList "run", "--bin", "api" -WorkingDirectory "services/api-rs"

# Start the keepers service
Write-Host "Starting keepers service..." -ForegroundColor Yellow
Start-Process -NoNewWindow -FilePath "cargo" -ArgumentList "run", "--bin", "keepers" -WorkingDirectory "services/keepers-rs"

# Start the CLI
Write-Host "Starting CLI..." -ForegroundColor Yellow
cargo run --bin dex

Write-Host "All components started successfully!" -ForegroundColor Green