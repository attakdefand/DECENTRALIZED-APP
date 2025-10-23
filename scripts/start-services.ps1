# Script to start all off-chain services

Write-Host "Starting off-chain services..." -ForegroundColor Green

# Start the indexer service
Write-Host "Starting indexer service..." -ForegroundColor Yellow
Start-Process -NoNewWindow -FilePath "cargo" -ArgumentList "run", "--bin", "indexer" -WorkingDirectory "services/indexer-rs"

# Start the API service
Write-Host "Starting API service..." -ForegroundColor Yellow
Start-Process -NoNewWindow -FilePath "cargo" -ArgumentList "run", "--bin", "api" -WorkingDirectory "services/api-rs"

# Start the keepers service
Write-Host "Starting keepers service..." -ForegroundColor Yellow
Start-Process -NoNewWindow -FilePath "cargo" -ArgumentList "run", "--bin", "keepers" -WorkingDirectory "services/keepers-rs"

Write-Host "All services started successfully!" -ForegroundColor Green
Write-Host "Services available at:" -ForegroundColor Yellow
Write-Host "  API Service: http://localhost:3000" -ForegroundColor Yellow