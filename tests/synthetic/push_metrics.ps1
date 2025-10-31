# Script to run synthetic tests and push results to Prometheus Pushgateway
# This script should be run periodically (e.g., via Task Scheduler)

Write-Host "Running synthetic tests and pushing metrics..." -ForegroundColor Yellow

# Pushgateway endpoint
$PUSHGATEWAY_URL = "http://localhost:9091/metrics/job/synthetic_tests"

# Run health check test
Write-Host "Running health check test..." -ForegroundColor Yellow
try {
  & .\api_health_check.ps1 > $env:TEMP\health_check.log 2>&1
  $HEALTH_STATUS = 1
  Write-Host "Health check passed" -ForegroundColor Green
} catch {
  $HEALTH_STATUS = 0
  Write-Host "Health check failed" -ForegroundColor Red
}

# Run functionality test
Write-Host "Running functionality test..." -ForegroundColor Yellow
try {
  & .\api_functionality_test.ps1 > $env:TEMP\functionality_test.log 2>&1
  $FUNCTIONALITY_STATUS = 1
  Write-Host "Functionality test passed" -ForegroundColor Green
} catch {
  $FUNCTIONALITY_STATUS = 0
  Write-Host "Functionality test failed" -ForegroundColor Red
}

# Create metrics data
$metricsData = @"
# HELP synthetic_test_success Status of synthetic tests (1 = success, 0 = failure)
# TYPE synthetic_test_success gauge
synthetic_test_success{test="health_check"} $HEALTH_STATUS
synthetic_test_success{test="functionality"} $FUNCTIONALITY_STATUS
"@

# Save metrics to temporary file
$metricsData | Out-File -FilePath "$env:TEMP\synthetic_metrics.txt" -Encoding UTF8

# Push metrics to Pushgateway
Write-Host "Pushing metrics to Pushgateway..." -ForegroundColor Yellow
try {
  Invoke-WebRequest -Uri $PUSHGATEWAY_URL -Method POST -Body (Get-Content "$env:TEMP\synthetic_metrics.txt" -Raw)
  Write-Host "Metrics pushed successfully" -ForegroundColor Green
} catch {
  Write-Host "Failed to push metrics: $($_.Exception.Message)" -ForegroundColor Red
  exit 1
}

# Cleanup
Remove-Item "$env:TEMP\synthetic_metrics.txt" -ErrorAction SilentlyContinue
Remove-Item "$env:TEMP\health_check.log" -ErrorAction SilentlyContinue
Remove-Item "$env:TEMP\functionality_test.log" -ErrorAction SilentlyContinue

Write-Host "Synthetic tests completed and metrics pushed!" -ForegroundColor Green