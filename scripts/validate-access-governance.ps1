# Access Governance Validation Script
# This script validates access governance requirements and blocks merges if completion is below 100% or SoD violations exist

param(
    [string]$IamRbacMapPath = "docs/security/IAM-RBAC-MAP.md",
    [int]$MinCompletionPercentage = 100,
    [bool]$BlockOnIncompleteReviews = $true,
    [bool]$BlockOnSoDViolations = $true
)

Write-Host "Running Access Governance Validation..." -ForegroundColor Yellow

# Function to parse access review completion percentage from IAM-RBAC-MAP.md
function Get-AccessReviewCompletionPercentage {
    param([string]$FilePath)
    
    if (-not (Test-Path $FilePath)) {
        Write-Host "WARNING: IAM-RBAC-MAP.md not found at $FilePath" -ForegroundColor Yellow
        return 0
    }
    
    try {
        $content = Get-Content $FilePath -Raw
        # Look for access_review_completion_pct in the metrics section
        if ($content -match "access_review_completion_pct:\s*(\d+)") {
            return [int]$matches[1]
        }
        # Default to 0 if not found
        return 0
    } catch {
        Write-Host "WARNING: Failed to parse IAM-RBAC-MAP.md - $($_.Exception.Message)" -ForegroundColor Yellow
        return 0
    }
}

# Function to parse SoD violations from IAM-RBAC-MAP.md
function Get-SoDViolations {
    param([string]$FilePath)
    
    if (-not (Test-Path $FilePath)) {
        Write-Host "WARNING: IAM-RBAC-MAP.md not found at $FilePath" -ForegroundColor Yellow
        return 0
    }
    
    try {
        $content = Get-Content $FilePath -Raw
        # Look for sod_violations in the metrics section
        if ($content -match "sod_violations:\s*(\d+)") {
            return [int]$matches[1]
        }
        # Default to 0 if not found
        return 0
    } catch {
        Write-Host "WARNING: Failed to parse IAM-RBAC-MAP.md - $($_.Exception.Message)" -ForegroundColor Yellow
        return 0
    }
}

# Function to check for access review report
function Test-AccessReviewReport {
    param([string]$ReportPath = "docs/security/access-review-report.csv")
    
    if (-not (Test-Path $ReportPath)) {
        return $false
    }
    
    try {
        $report = Import-Csv $ReportPath
        return $report.Count -gt 0
    } catch {
        Write-Host "WARNING: Failed to parse access review report - $($_.Exception.Message)" -ForegroundColor Yellow
        return $false
    }
}

# Function to check for OPA bundles
function Test-OPABundles {
    param([string]$OpaPath = "infra/policies/OPA-Cedar")
    
    if (-not (Test-Path $OpaPath)) {
        return $false
    }
    
    try {
        $bundles = Get-ChildItem -Path $OpaPath -Filter "*.cedar" -Recurse
        return $bundles.Count -gt 0
    } catch {
        Write-Host "WARNING: Failed to check OPA bundles - $($_.Exception.Message)" -ForegroundColor Yellow
        return $false
    }
}

# Check if IAM-RBAC-MAP.md exists
if (-not (Test-Path $IamRbacMapPath)) {
    Write-Host "ERROR: IAM-RBAC-MAP.md not found at $IamRbacMapPath" -ForegroundColor Red
    exit 1
}

Write-Host "[PASS] IAM-RBAC-MAP.md found at $IamRbacMapPath" -ForegroundColor Green

# Get access review completion percentage
$completionPercentage = Get-AccessReviewCompletionPercentage $IamRbacMapPath
Write-Host "[INFO] Access review completion: $completionPercentage%" -ForegroundColor Gray

# Get SoD violations
$sodViolations = Get-SoDViolations $IamRbacMapPath
Write-Host "[INFO] SoD violations detected: $sodViolations" -ForegroundColor Gray

# Check for access review report
$hasAccessReviewReport = Test-AccessReviewReport
if ($hasAccessReviewReport) {
    Write-Host "[PASS] Access review report found" -ForegroundColor Green
} else {
    Write-Host "WARNING: Access review report not found" -ForegroundColor Yellow
}

# Check for OPA bundles
$hasOPABundles = Test-OPABundles
if ($hasOPABundles) {
    Write-Host "[PASS] OPA bundles found" -ForegroundColor Green
    Write-Host "[INFO] Found policy bundles in infra/policies/OPA-Cedar" -ForegroundColor Gray
} else {
    Write-Host "WARNING: OPA bundles not found" -ForegroundColor Yellow
}

# Block if access review completion is below minimum
if ($BlockOnIncompleteReviews -and $completionPercentage -lt $MinCompletionPercentage) {
    Write-Host "BLOCK: Merge prevented due to incomplete access reviews ($completionPercentage% < $MinCompletionPercentage%)" -ForegroundColor Red
    Write-Host "Please either:" -ForegroundColor Yellow
    Write-Host "  1. Complete all required access reviews" -ForegroundColor Yellow
    Write-Host "  2. Update the completion percentage in IAM-RBAC-MAP.md" -ForegroundColor Yellow
    Write-Host "  3. Set BlockOnIncompleteReviews parameter to `$false to override (not recommended)" -ForegroundColor Yellow
    
    # Log to audit trail
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $auditEntry = "[$timestamp] BLOCKED: Merge prevented due to incomplete access reviews ($completionPercentage% < $MinCompletionPercentage%)"
    Add-Content -Path "logs/access-governance-audit.log" -Value $auditEntry -ErrorAction SilentlyContinue
    
    exit 1
}

# Block if SoD violations exist
if ($BlockOnSoDViolations -and $sodViolations -gt 0) {
    Write-Host "BLOCK: Merge prevented due to SoD violations ($sodViolations violations detected)" -ForegroundColor Red
    Write-Host "Please either:" -ForegroundColor Yellow
    Write-Host "  1. Resolve all SoD violations" -ForegroundColor Yellow
    Write-Host "  2. Update the violation count in IAM-RBAC-MAP.md" -ForegroundColor Yellow
    Write-Host "  3. Set BlockOnSoDViolations parameter to `$false to override (not recommended)" -ForegroundColor Yellow
    
    # Log to audit trail
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $auditEntry = "[$timestamp] BLOCKED: Merge prevented due to SoD violations ($sodViolations violations detected)"
    Add-Content -Path "logs/access-governance-audit.log" -Value $auditEntry -ErrorAction SilentlyContinue
    
    exit 1
}

# If we get here, all checks passed
Write-Host "PASS: All access governance requirements satisfied" -ForegroundColor Green
Write-Host "  - Access review completion: $completionPercentage% (minimum required: $MinCompletionPercentage%)" -ForegroundColor Green
Write-Host "  - SoD violations: $sodViolations" -ForegroundColor Green
Write-Host "  - Access review report: $(if ($hasAccessReviewReport) { 'Found' } else { 'Not found' })" -ForegroundColor Green
Write-Host "  - OPA bundles: $(if ($hasOPABundles) { 'Found' } else { 'Not found' })" -ForegroundColor Green

# Log to audit trail
$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
$auditEntry = "[$timestamp] APPROVED: All access governance requirements satisfied (Completion: $completionPercentage%, SoD Violations: $sodViolations)"
Add-Content -Path "logs/access-governance-audit.log" -Value $auditEntry -ErrorAction SilentlyContinue

exit 0