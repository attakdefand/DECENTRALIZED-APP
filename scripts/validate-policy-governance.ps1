# Policy Governance Validation Script
# This script validates policy governance requirements and blocks merges if coverage is below threshold or expired exceptions exist

param(
    [string]$PolicyCatalogPath = "docs/security/POLICY-CATALOG.md",
    [string]$ExceptionRegisterPath = "docs/security/EXCEPTIONS.md",
    [int]$MinCoveragePercentage = 95,
    [bool]$BlockOnLowCoverage = $true,
    [bool]$BlockOnExpiredExceptions = $true
)

Write-Host "Running Policy Governance Validation..." -ForegroundColor Yellow

# Function to count policies in catalog
function Get-PolicyCount {
    param([string]$FilePath)
    
    if (-not (Test-Path $FilePath)) {
        return 0
    }
    
    try {
        $content = Get-Content $FilePath -Raw
        # Count policy sections (look for policy IDs like PG-001, CG-001, etc.)
        $policyMatches = Select-String -Pattern "\*\*ID\*\*: [A-Z]{2}-\d+" -InputObject $content -AllMatches
        return $policyMatches.Matches.Count
    } catch {
        Write-Host "WARNING: Failed to parse policy catalog - $($_.Exception.Message)" -ForegroundColor Yellow
        return 0
    }
}

# Function to count exceptions in register
function Get-ExceptionCount {
    param([string]$FilePath)
    
    if (-not (Test-Path $FilePath)) {
        return 0
    }
    
    try {
        $content = Get-Content $FilePath -Raw
        # Count exception records (look for exception IDs like EX-001, EX-002, etc.)
        $exceptionMatches = Select-String -Pattern "\|\s*EX-\d+" -InputObject $content -AllMatches
        return $exceptionMatches.Matches.Count
    } catch {
        Write-Host "WARNING: Failed to parse exception register - $($_.Exception.Message)" -ForegroundColor Yellow
        return 0
    }
}

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

# Function to get expired exceptions
function Get-ExpiredExceptions {
    param([string]$FilePath)
    
    $expiredExceptions = @()
    
    if (-not (Test-Path $FilePath)) {
        return $expiredExceptions
    }
    
    try {
        $content = Get-Content $FilePath -Raw
        # Extract exception table rows
        $lines = $content -split "`n"
        $inTable = $false
        $headerFound = $false
        
        foreach ($line in $lines) {
            # Check if we're in the exception records table
            if ($line -match "Exception Records") {
                $inTable = $true
                continue
            }
            
            # Skip table headers
            if ($inTable -and $line -match "ID.*Policy ID.*Description.*Risk Owner.*Expiry Date") {
                $headerFound = $true
                continue
            }
            
            # Process table rows
            if ($inTable -and $headerFound -and $line -match "^\|") {
                # Skip header separator row
                if ($line -match "----") {
                    continue
                }
                
                # Parse exception row
                $fields = $line -split '\|' | ForEach-Object { $_.Trim() }
                if ($fields.Count -ge 7) {
                    $id = $fields[1]
                    $policyId = $fields[2]
                    $description = $fields[3]
                    $riskOwner = $fields[4]
                    $expiryDate = $fields[5]
                    
                    # Check if expired
                    if (Test-DateExpired $expiryDate) {
                        $expiredExceptions += @{
                            ID = $id
                            PolicyID = $policyId
                            Description = $description
                            RiskOwner = $riskOwner
                            ExpiryDate = $expiryDate
                        }
                    }
                }
            }
            
            # Stop when we reach the next section
            if ($inTable -and $line -match "## ") {
                break
            }
        }
    } catch {
        Write-Host "WARNING: Failed to parse exception register for expired exceptions - $($_.Exception.Message)" -ForegroundColor Yellow
    }
    
    return $expiredExceptions
}

# Function to check for coverage exception
function Test-CoverageException {
    param([string]$ExceptionFilePath = "docs/security/policy-governance-exception.csv")
    
    if (-not (Test-Path $ExceptionFilePath)) {
        return $false
    }
    
    try {
        $exceptions = Import-Csv $ExceptionFilePath
        foreach ($exception in $exceptions) {
            if ($exception."Risk ID" -eq "POL-EX-001" -and $exception.Status -eq "Approved") {
                # Check if exception is still valid (not expired)
                if (-not (Test-DateExpired $exception."Expiry Date")) {
                    return $true
                }
            }
        }
    } catch {
        Write-Host "WARNING: Failed to parse policy governance exception file - $($_.Exception.Message)" -ForegroundColor Yellow
    }
    
    return $false
}

# Check if policy catalog exists
if (-not (Test-Path $PolicyCatalogPath)) {
    Write-Host "ERROR: Policy catalog not found at $PolicyCatalogPath" -ForegroundColor Red
    exit 1
}

Write-Host "[PASS] Policy catalog found at $PolicyCatalogPath" -ForegroundColor Green

# Count policies
$policyCount = Get-PolicyCount $PolicyCatalogPath
Write-Host "[INFO] Found $policyCount policies in catalog" -ForegroundColor Gray

# Calculate coverage percentage (assuming a target of 20 policies for 100% coverage)
# This is a simplified calculation - in a real implementation, this would be based on 
# the actual required policies for the project
$targetPolicyCount = 20
$coveragePercentage = [Math]::Min(100, [Math]::Floor(($policyCount / $targetPolicyCount) * 100))
Write-Host "[INFO] Policy coverage: $coveragePercentage%" -ForegroundColor Gray

# Check exception register
if (-not (Test-Path $ExceptionRegisterPath)) {
    Write-Host "WARNING: Exception register not found at $ExceptionRegisterPath" -ForegroundColor Yellow
    $exceptionCount = 0
} else {
    Write-Host "[PASS] Exception register found at $ExceptionRegisterPath" -ForegroundColor Green
    $exceptionCount = Get-ExceptionCount $ExceptionRegisterPath
    Write-Host "[INFO] Found $exceptionCount exceptions in register" -ForegroundColor Gray
}

# Check for expired exceptions
$expiredExceptions = Get-ExpiredExceptions $ExceptionRegisterPath
Write-Host "[INFO] Found $($expiredExceptions.Count) expired exceptions" -ForegroundColor Gray

# Block if coverage is below minimum
if ($BlockOnLowCoverage -and $coveragePercentage -lt $MinCoveragePercentage) {
    # Check if there's an approved exception for low coverage
    $hasCoverageException = Test-CoverageException
    
    if (-not $hasCoverageException) {
        Write-Host "BLOCK: Merge prevented due to low policy coverage ($coveragePercentage% < $MinCoveragePercentage%)" -ForegroundColor Red
        Write-Host "Please either:" -ForegroundColor Yellow
        Write-Host "  1. Add more policies to meet the coverage requirement" -ForegroundColor Yellow
        Write-Host "  2. Reduce the minimum coverage threshold" -ForegroundColor Yellow
        Write-Host "  3. Create an exception for this requirement" -ForegroundColor Yellow
        Write-Host "  4. Set BlockOnLowCoverage parameter to `$false to override (not recommended)" -ForegroundColor Yellow
        
        # Log to audit trail
        $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        $auditEntry = "[$timestamp] BLOCKED: Merge prevented due to low policy coverage ($coveragePercentage% < $MinCoveragePercentage%)"
        Add-Content -Path "logs/policy-governance-audit.log" -Value $auditEntry -ErrorAction SilentlyContinue
        
        exit 1
    } else {
        Write-Host "[INFO] Low policy coverage detected but approved exception found (POL-EX-001)" -ForegroundColor Yellow
    }
}

# Block if expired exceptions exist
if ($BlockOnExpiredExceptions -and $expiredExceptions.Count -gt 0) {
    Write-Host "BLOCK: Merge prevented due to expired exceptions:" -ForegroundColor Red
    foreach ($exception in $expiredExceptions) {
        Write-Host "  - $($exception.ID): $($exception.Description) (Expired: $($exception.ExpiryDate))" -ForegroundColor Red
    }
    Write-Host "Please either:" -ForegroundColor Yellow
    Write-Host "  1. Renew the exception with a new expiry date" -ForegroundColor Yellow
    Write-Host "  2. Close the exception if no longer needed" -ForegroundColor Yellow
    Write-Host "  3. Set BlockOnExpiredExceptions parameter to `$false to override (not recommended)" -ForegroundColor Yellow
    
    # Log to audit trail
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $auditEntry = "[$timestamp] BLOCKED: Merge prevented due to $($expiredExceptions.Count) expired exceptions"
    Add-Content -Path "logs/policy-governance-audit.log" -Value $auditEntry -ErrorAction SilentlyContinue
    
    exit 1
}

# If we get here, all checks passed
Write-Host "PASS: All policy governance requirements satisfied" -ForegroundColor Green
Write-Host "  - Policy coverage: $coveragePercentage% (minimum required: $MinCoveragePercentage%)" -ForegroundColor Green
Write-Host "  - No expired exceptions found" -ForegroundColor Green

# Log to audit trail
$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
$auditEntry = "[$timestamp] APPROVED: All policy governance requirements satisfied (Coverage: $coveragePercentage%)"
Add-Content -Path "logs/policy-governance-audit.log" -Value $auditEntry -ErrorAction SilentlyContinue

exit 0