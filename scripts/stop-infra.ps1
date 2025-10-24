# Script to stop the infrastructure components

Write-Host "Stopping infrastructure components..." -ForegroundColor Green

# Change to the infra directory
Set-Location -Path "infra"

# Stop all services with Docker Compose
docker-compose down

if ($LASTEXITCODE -eq 0) {
    Write-Host "Infrastructure components stopped successfully!" -ForegroundColor Green
} else {
    Write-Host "Failed to stop infrastructure components!" -ForegroundColor Red
    exit $LASTEXITCODE
}

# Change back to the root directory
Set-Location -Path ".."