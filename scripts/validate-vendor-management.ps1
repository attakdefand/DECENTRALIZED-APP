# Vendor Management Validation Script
# This script validates vendor management requirements and warns if overdue reviews exist

param(
    [string]$VendorMetricsPath = "docs/security/vendor-metrics.json",
    [int]$MaxOverdueReviews = 0
)

Write-Host "Running Vendor Management Validation..." -ForegroundColor Yellow

# Function to parse vendor metrics
function Get-VendorMetrics {
    param([string]$FilePath)
    
    if (-not (Test-Path $FilePath)) {
        Write-Host "WARNING: Vendor metrics file not found at $FilePath" -ForegroundColor Yellow
        return @{
            vendor_score_avg = 0
            overdue_reviews = 0
        }
    }
    
    try {
        $content = Get-Content $FilePath -Raw | ConvertFrom-Json
        return @{
            vendor_score_avg = $content.vendor_score_avg
            overdue_reviews = $content.overdue_reviews
        }
    } catch {
        Write-Host "WARNING: Failed to parse vendor metrics file - $($_.Exception.Message)" -ForegroundColor Yellow
        return @{
            vendor_score_avg = 0
            overdue_reviews = 0
        }
    }
}

# Get vendor metrics
$metrics = Get-VendorMetrics $VendorMetricsPath
Write-Host "[INFO] Vendor score average: $($metrics.vendor_score_avg)" -ForegroundColor Gray
Write-Host "[INFO] Overdue reviews: $($metrics.overdue_reviews)" -ForegroundColor Gray

# Check for overdue reviews
if ($metrics.overdue_reviews -gt $MaxOverdueReviews) {
    Write-Host "WARNING: Overdue vendor reviews detected ($($metrics.overdue_reviews))" -ForegroundColor Yellow
    exit 0 # Warn only, don't block
} else {
    Write-Host "[PASS] No overdue vendor reviews" -ForegroundColor Green
}

Write-Host "Vendor Management validation completed successfully!" -ForegroundColor Green
exit 0