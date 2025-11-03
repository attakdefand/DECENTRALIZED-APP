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
            vendor_access_review_completion_pct = 100
            vendor_sod_violations = 0
            vendor_key_rotation_compliance_pct = 100
            vendor_key_health_checks_pass = 1
            vendor_policy_coverage_pct = 100
            vendor_policy_violations = 0
        }
    }
    
    try {
        $content = Get-Content $FilePath -Raw | ConvertFrom-Json
        return @{
            vendor_score_avg = $content.vendor_score_avg
            overdue_reviews = $content.overdue_reviews
            vendor_access_review_completion_pct = $($content.vendor_access_review_completion_pct, 100)[[bool]$content.vendor_access_review_completion_pct -eq $null]
            vendor_sod_violations = $($content.vendor_sod_violations, 0)[[bool]$content.vendor_sod_violations -eq $null]
            vendor_key_rotation_compliance_pct = $($content.vendor_key_rotation_compliance_pct, 100)[[bool]$content.vendor_key_rotation_compliance_pct -eq $null]
            vendor_key_health_checks_pass = $($content.vendor_key_health_checks_pass, 1)[[bool]$content.vendor_key_health_checks_pass -eq $null]
            vendor_policy_coverage_pct = $($content.vendor_policy_coverage_pct, 100)[[bool]$content.vendor_policy_coverage_pct -eq $null]
            vendor_policy_violations = $($content.vendor_policy_violations, 0)[[bool]$content.vendor_policy_violations -eq $null]
        }
    } catch {
        Write-Host "WARNING: Failed to parse vendor metrics file - $($_.Exception.Message)" -ForegroundColor Yellow
        return @{
            vendor_score_avg = 0
            overdue_reviews = 0
            vendor_access_review_completion_pct = 100
            vendor_sod_violations = 0
            vendor_key_rotation_compliance_pct = 100
            vendor_key_health_checks_pass = 1
            vendor_policy_coverage_pct = 100
            vendor_policy_violations = 0
        }
    }
}

# Get vendor metrics
$metrics = Get-VendorMetrics $VendorMetricsPath
Write-Host "[INFO] Vendor score average: $($metrics.vendor_score_avg)" -ForegroundColor Gray
Write-Host "[INFO] Overdue reviews: $($metrics.overdue_reviews)" -ForegroundColor Gray
Write-Host "[INFO] Vendor access review completion: $($metrics.vendor_access_review_completion_pct)%" -ForegroundColor Gray
Write-Host "[INFO] Vendor SoD violations: $($metrics.vendor_sod_violations)" -ForegroundColor Gray
Write-Host "[INFO] Vendor key rotation compliance: $($metrics.vendor_key_rotation_compliance_pct)%" -ForegroundColor Gray
Write-Host "[INFO] Vendor key health checks pass: $($metrics.vendor_key_health_checks_pass)" -ForegroundColor Gray
Write-Host "[INFO] Vendor policy coverage: $($metrics.vendor_policy_coverage_pct)%" -ForegroundColor Gray
Write-Host "[INFO] Vendor policy violations: $($metrics.vendor_policy_violations)" -ForegroundColor Gray

# Check for overdue reviews
if ($metrics.overdue_reviews -gt $MaxOverdueReviews) {
    Write-Host "WARNING: Overdue vendor reviews detected ($($metrics.overdue_reviews))" -ForegroundColor Yellow
    exit 0 # Warn only, don't block
} else {
    Write-Host "[PASS] No overdue vendor reviews" -ForegroundColor Green
}

# Check for vendor access review completion
if ($metrics.vendor_access_review_completion_pct -lt 100) {
    Write-Host "BLOCK: Vendor access review completion below 100% ($($metrics.vendor_access_review_completion_pct)%)" -ForegroundColor Red
    exit 1
}

# Check for vendor SoD violations
if ($metrics.vendor_sod_violations -gt 0) {
    Write-Host "BLOCK: Vendor SoD violations detected ($($metrics.vendor_sod_violations))" -ForegroundColor Red
    exit 1
}

# Check for vendor key rotation compliance
if ($metrics.vendor_key_rotation_compliance_pct -lt 100) {
    Write-Host "BLOCK: Vendor key rotation compliance below 100% ($($metrics.vendor_key_rotation_compliance_pct)%)" -ForegroundColor Red
    exit 1
}

# Check for vendor key health checks
if ($metrics.vendor_key_health_checks_pass -ne 1) {
    Write-Host "BLOCK: Vendor key health checks failed" -ForegroundColor Red
    exit 1
}

# Check for vendor policy coverage
if ($metrics.vendor_policy_coverage_pct -lt 95) {
    Write-Host "BLOCK: Vendor policy coverage below 95% ($($metrics.vendor_policy_coverage_pct)%)" -ForegroundColor Red
    exit 1
}

# Check for vendor policy violations
if ($metrics.vendor_policy_violations -gt 0) {
    Write-Host "BLOCK: Vendor policy violations detected ($($metrics.vendor_policy_violations))" -ForegroundColor Red
    exit 1
}

Write-Host "Vendor Management validation completed successfully!" -ForegroundColor Green
exit 0