# Synthetic API Health Check Script (PowerShell)
# This script performs basic health checks on all API endpoints

Write-Host "Starting API Health Checks..." -ForegroundColor Yellow

# API endpoints to test
$API_ENDPOINTS = @(
  "http://localhost:3000/health"
  "http://localhost:3000/"
  "http://localhost:3000/api/v1/pools"
  "http://localhost:3000/api/v1/orders"
  "http://localhost:3000/api/v1/markets"
)

# Metrics endpoint
$METRICS_ENDPOINT = "http://localhost:3000/metrics"

# Counter for passed/failed tests
$PASSED = 0
$FAILED = 0

# Test each API endpoint
foreach ($endpoint in $API_ENDPOINTS) {
  Write-Host "Testing $endpoint" -ForegroundColor Yellow
  
  try {
    $response = Invoke-WebRequest -Uri $endpoint -UseBasicParsing -TimeoutSec 10
    if ($response.StatusCode -eq 200) {
      Write-Host "✓ $endpoint returned status 200" -ForegroundColor Green
      $PASSED++
    } else {
      Write-Host "✗ $endpoint returned status $($response.StatusCode)" -ForegroundColor Red
      $FAILED++
    }
  } catch {
    Write-Host "✗ $endpoint failed to respond: $($_.Exception.Message)" -ForegroundColor Red
    $FAILED++
  }
}

# Test metrics endpoint
Write-Host "Testing $METRICS_ENDPOINT" -ForegroundColor Yellow
try {
  $response = Invoke-WebRequest -Uri $METRICS_ENDPOINT -UseBasicParsing -TimeoutSec 10
  if ($response.StatusCode -eq 200) {
    Write-Host "✓ $METRICS_ENDPOINT returned status 200" -ForegroundColor Green
    $PASSED++
  } else {
    Write-Host "✗ $METRICS_ENDPOINT returned status $($response.StatusCode)" -ForegroundColor Red
    $FAILED++
  }
} catch {
  Write-Host "✗ $METRICS_ENDPOINT failed to respond: $($_.Exception.Message)" -ForegroundColor Red
  $FAILED++
}

# Summary
Write-Host "Health Check Summary:" -ForegroundColor Yellow
Write-Host "Passed: $PASSED" -ForegroundColor Green
Write-Host "Failed: $FAILED" -ForegroundColor Red

if ($FAILED -eq 0) {
  Write-Host "All health checks passed!" -ForegroundColor Green
  exit 0
} else {
  Write-Host "Some health checks failed!" -ForegroundColor Red
  exit 1
}