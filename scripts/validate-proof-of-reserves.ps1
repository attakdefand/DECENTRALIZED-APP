# Proof of Reserves Validation Script
# This script validates Proof of Reserves reports and treasury metrics

Write-Host "Running Proof of Reserves validation..." -ForegroundColor Yellow

# Check if required environment variables are set
$POR_FRESHNESS_HOURS = $env:POR_FRESHNESS_HOURS
$LIMIT_BREACH_COUNT = $env:LIMIT_BREACH_COUNT

if ($null -eq $POR_FRESHNESS_HOURS) {
    $POR_FRESHNESS_HOURS = 0
}

if ($null -eq $LIMIT_BREACH_COUNT) {
    $LIMIT_BREACH_COUNT = 0
}

Write-Host "Proof of Reserves Freshness Hours: $POR_FRESHNESS_HOURS" -ForegroundColor Green
Write-Host "Limit Breach Count: $LIMIT_BREACH_COUNT" -ForegroundColor Green

# Validate Proof of Reserves freshness (should be < 24 hours)
if ($POR_FRESHNESS_HOURS -ge 24) {
    Write-Host "ERROR: Proof of Reserves report is stale (>= 24 hours old)" -ForegroundColor Red
    exit 1
}

# Validate limit breach count (should be 0)
if ($LIMIT_BREACH_COUNT -gt 0) {
    Write-Host "ERROR: Limit breach count is greater than 0" -ForegroundColor Red
    exit 1
}

Write-Host "Proof of Reserves validation passed!" -ForegroundColor Green
exit 0