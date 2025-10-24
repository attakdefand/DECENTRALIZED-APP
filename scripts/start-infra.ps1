# Script to start the infrastructure components

Write-Host "Starting infrastructure components..." -ForegroundColor Green

# Change to the infra directory
Set-Location -Path "infra"

# Start all services with Docker Compose
docker-compose up -d

if ($LASTEXITCODE -eq 0) {
    Write-Host "Infrastructure components started successfully!" -ForegroundColor Green
    Write-Host "Services available at:" -ForegroundColor Yellow
    Write-Host "  PostgreSQL: localhost:5432" -ForegroundColor Yellow
    Write-Host "  ClickHouse: localhost:8123" -ForegroundColor Yellow
    Write-Host "  Redis: localhost:6379" -ForegroundColor Yellow
    Write-Host "  NATS: localhost:4222" -ForegroundColor Yellow
    Write-Host "  IPFS: localhost:5001" -ForegroundColor Yellow
    Write-Host "  Prometheus: localhost:9090" -ForegroundColor Yellow
    Write-Host "  Grafana: localhost:3001" -ForegroundColor Yellow
} else {
    Write-Host "Failed to start infrastructure components!" -ForegroundColor Red
    exit $LASTEXITCODE
}

# Change back to the root directory
Set-Location -Path ".."