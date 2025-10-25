# Storage layout validation script
# This script validates storage layout compatibility between contract versions

Write-Host "Validating storage layout compatibility..." -ForegroundColor Yellow

# Run the comprehensive storage layout diff tool
Write-Host "Running storage layout diff tool..." -ForegroundColor Green
& ".\scripts\storage-layout-diff.ps1" -ContractName "EnhancedUpgradeableToken"

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Storage layout validation completed successfully" -ForegroundColor Green
} else {
    Write-Host "❌ Storage layout validation failed" -ForegroundColor Red
    exit 1
}

Write-Host "Storage layout validation completed!" -ForegroundColor Green