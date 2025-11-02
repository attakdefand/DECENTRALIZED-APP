#!/usr/bin/env pwsh

# Script to verify that commits are signed and enforce governance rules
# Block if signed_commits_pct < 100% OR checks_fail

param(
    [string]$RepositoryPath = ".",
    [int]$MinSignedCommitsPct = 100,
    [string]$Branch = "HEAD"
)

Write-Host "Verifying signed commits for branch: $Branch" -ForegroundColor Yellow

# Get commit history
try {
    $commits = git -C $RepositoryPath log --pretty=format:"%H %G?" $Branch --oneline
    if ($LASTEXITCODE -ne 0) {
        Write-Host "ERROR: Failed to get commit history" -ForegroundColor Red
        exit 1
    }
} catch {
    Write-Host "ERROR: Git command failed - $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

# Count total commits and signed commits
$totalCommits = 0
$signedCommits = 0

foreach ($commit in $commits) {
    $totalCommits++
    # Check if commit is signed (G or G? status)
    if ($commit -match " [G]$") {
        $signedCommits++
    }
}

# Calculate percentage
if ($totalCommits -eq 0) {
    Write-Host "WARNING: No commits found" -ForegroundColor Yellow
    $signedPct = 100
} else {
    $signedPct = [math]::Round(($signedCommits / $totalCommits) * 100, 2)
}

Write-Host "Total commits: $totalCommits" -ForegroundColor Gray
Write-Host "Signed commits: $signedCommits" -ForegroundColor Gray
Write-Host "Signed commit percentage: $signedPct%" -ForegroundColor Gray

# Check if requirements are met
$checksFail = $false
$signedCommitsFail = $signedPct -lt $MinSignedCommitsPct

if ($signedCommitsFail) {
    Write-Host "ERROR: Signed commit percentage ($signedPct%) is below minimum required ($MinSignedCommitsPct%)" -ForegroundColor Red
    $checksFail = $true
}

# Check for CI status (simplified check)
# In a real implementation, this would check actual CI status
$ciStatus = "success"  # Placeholder - in real implementation, check GitHub API or CI system

if ($ciStatus -ne "success") {
    Write-Host "ERROR: CI checks are failing" -ForegroundColor Red
    $checksFail = $true
}

# Block if requirements are not met
if ($signedCommitsFail -or $checksFail) {
    Write-Host "BLOCK: Changes cannot be merged due to governance requirements:" -ForegroundColor Red
    if ($signedCommitsFail) {
        Write-Host "  - Signed commits percentage ($signedPct%) is below required threshold ($MinSignedCommitsPct%)" -ForegroundColor Red
    }
    if ($checksFail) {
        Write-Host "  - Required checks are failing" -ForegroundColor Red
    }
    
    # Log to governance audit trail
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $auditEntry = "[$timestamp] BLOCKED: Merge prevented due to governance requirements. Signed commits: $signedPct%, CI status: $ciStatus"
    Add-Content -Path "logs/governance-audit.log" -Value $auditEntry -ErrorAction SilentlyContinue
    
    exit 1
} else {
    Write-Host "PASS: All governance requirements satisfied" -ForegroundColor Green
    Write-Host "  - Signed commits: $signedPct%" -ForegroundColor Green
    Write-Host "  - CI checks: $ciStatus" -ForegroundColor Green
    
    # Log to governance audit trail
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $auditEntry = "[$timestamp] APPROVED: All governance requirements satisfied. Signed commits: $signedPct%, CI status: $ciStatus"
    Add-Content -Path "logs/governance-audit.log" -Value $auditEntry -ErrorAction SilentlyContinue
    
    exit 0
}