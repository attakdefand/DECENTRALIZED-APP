# Risk Governance Validation Script
# This script validates risk governance requirements and blocks merges if open high risks exist without a plan

param(
    [string]$RiskRegisterPath = "docs/security/risk-register.csv",
    [string]$ExceptionRegisterPath = "docs/security/exception-records.csv",
    [bool]$BlockOnOpenHighRisks = $true
)

Write-Host "Running Risk Governance Validation..." -ForegroundColor Yellow

# Function to check if a date string is expired
function Test-DateExpired {
    param([string]$DateString)
    
    if ([string]::IsNullOrEmpty($DateString)) {
        return $false
    }
    
    try {
        $date = [DateTime]::Parse($DateString)
        return $date -lt (Get-Date)
    } catch {
        return $false
    }
}

# Check if risk register exists
if (-not (Test-Path $RiskRegisterPath)) {
    Write-Host "ERROR: Risk register not found at $RiskRegisterPath" -ForegroundColor Red
    exit 1
}

Write-Host "[PASS] Risk register found at $RiskRegisterPath" -ForegroundColor Green

# Import risk register CSV
try {
    $riskRegister = Import-Csv $RiskRegisterPath
    Write-Host "[INFO] Loaded $($riskRegister.Count) risks from register" -ForegroundColor Gray
} catch {
    Write-Host "ERROR: Failed to parse risk register CSV - $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

# Count open high severity risks
$openHighRisks = @()
$expiredExceptions = @()

foreach ($risk in $riskRegister) {
    # Check if risk is open and high/critical severity
    if ($risk.Status -eq "Open" -and ($risk.Severity -eq "High" -or $risk.Severity -eq "Critical")) {
        $openHighRisks += $risk
    }
    
    # Check if risk has an expired date
    if (Test-DateExpired $risk."Expiry Date") {
        $expiredExceptions += $risk
    }
}

Write-Host "[INFO] Found $($openHighRisks.Count) open high/critical risks" -ForegroundColor Gray
Write-Host "[INFO] Found $($expiredExceptions.Count) expired exceptions" -ForegroundColor Gray

# Check exception records if they exist
$exceptionRecords = @()
if (Test-Path $ExceptionRegisterPath) {
    try {
        $exceptionRecords = Import-Csv $ExceptionRegisterPath
        Write-Host "[PASS] Exception records found at $ExceptionRegisterPath" -ForegroundColor Green
    } catch {
        Write-Host "WARNING: Failed to parse exception records CSV - $($_.Exception.Message)" -ForegroundColor Yellow
    }
} else {
    Write-Host "WARNING: Exception records not found at $ExceptionRegisterPath" -ForegroundColor Yellow
}

# Check for exception records for open high risks
$risksWithoutPlans = @()
foreach ($risk in $openHighRisks) {
    $hasException = $false
    foreach ($exception in $exceptionRecords) {
        if ($exception."Risk ID" -eq $risk.ID -and $exception.Status -eq "Approved") {
            $hasException = $true
            break
        }
    }
    
    if (-not $hasException) {
        $risksWithoutPlans += $risk
    }
}

Write-Host "[INFO] Found $($risksWithoutPlans.Count) open high risks without approved exception plans" -ForegroundColor Gray

# Block if open high risks exist without plans
if ($BlockOnOpenHighRisks -and $risksWithoutPlans.Count -gt 0) {
    Write-Host "BLOCK: Merge prevented due to open high/critical risks without approved exception plans:" -ForegroundColor Red
    foreach ($risk in $risksWithoutPlans) {
        Write-Host "  - $($risk.ID): $($risk.Title) (Severity: $($risk.Severity))" -ForegroundColor Red
    }
    Write-Host "Please either:" -ForegroundColor Yellow
    Write-Host "  1. Remediate these risks" -ForegroundColor Yellow
    Write-Host "  2. Create exception records with approval for these risks" -ForegroundColor Yellow
    Write-Host "  3. Set BlockOnOpenHighRisks parameter to `$false to override (not recommended)" -ForegroundColor Yellow
    
    # Log to audit trail
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $auditEntry = "[$timestamp] BLOCKED: Merge prevented due to $($risksWithoutPlans.Count) open high/critical risks without approved exception plans"
    Add-Content -Path "logs/risk-governance-audit.log" -Value $auditEntry -ErrorAction SilentlyContinue
    
    exit 1
}

# Block if expired exceptions exist
if ($expiredExceptions.Count -gt 0) {
    Write-Host "BLOCK: Merge prevented due to expired exceptions:" -ForegroundColor Red
    foreach ($risk in $expiredExceptions) {
        Write-Host "  - $($risk.ID): $($risk.Title) (Expired: $($risk."Expiry Date"))" -ForegroundColor Red
    }
    Write-Host "Please either:" -ForegroundColor Yellow
    Write-Host "  1. Renew the exception with a new expiry date" -ForegroundColor Yellow
    Write-Host "  2. Remediate the underlying risk" -ForegroundColor Yellow
    
    # Log to audit trail
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $auditEntry = "[$timestamp] BLOCKED: Merge prevented due to $($expiredExceptions.Count) expired exceptions"
    Add-Content -Path "logs/risk-governance-audit.log" -Value $auditEntry -ErrorAction SilentlyContinue
    
    exit 1
}

# If we get here, all checks passed
Write-Host "PASS: All risk governance requirements satisfied" -ForegroundColor Green
Write-Host "  - No open high/critical risks without approved exception plans" -ForegroundColor Green
Write-Host "  - No expired exceptions" -ForegroundColor Green

# Log to audit trail
$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
$auditEntry = "[$timestamp] APPROVED: All risk governance requirements satisfied"
Add-Content -Path "logs/risk-governance-audit.log" -Value $auditEntry -ErrorAction SilentlyContinue

exit 0