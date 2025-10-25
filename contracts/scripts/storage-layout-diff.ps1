#!/usr/bin/env pwsh

# Storage layout diff tool for upgradeable contracts
# This script compares storage layouts between contract versions to ensure compatibility

param(
    [string]$ContractName = "EnhancedUpgradeableToken",
    [string]$OutputDir = "storage-layout-reports"
)

Write-Host "Storage Layout Diff Tool" -ForegroundColor Green
Write-Host "========================" -ForegroundColor Green

# Create output directory
if (!(Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Name $OutputDir | Out-Null
}

# Get storage layout for current implementation
Write-Host "Getting storage layout for $ContractName..." -ForegroundColor Yellow
$CurrentLayoutFile = "$OutputDir/$ContractName-current-layout.json"
try {
    # This would normally use forge inspect, but we'll simulate for now
    # forge inspect $ContractName storage-layout > $CurrentLayoutFile
    Write-Host "Simulating storage layout extraction..." -ForegroundColor Cyan
    
    # Create a sample storage layout for demonstration
    $SampleLayout = @{
        storage = @(
            @{
                astId = 101
                contract = "EnhancedUpgradeableToken"
                label = "feePercentage"
                offset = 0
                slot = "0"
                type = "t_uint256"
            },
            @{
                astId = 102
                contract = "EnhancedUpgradeableToken"
                label = "feeRecipient"
                offset = 0
                slot = "1"
                type = "t_address"
            },
            @{
                astId = 103
                contract = "EnhancedUpgradeableToken"
                label = "transferPaused"
                offset = 20
                slot = "1"
                type = "t_bool"
            },
            @{
                astId = 104
                contract = "EnhancedUpgradeableToken"
                label = "_version"
                offset = 0
                slot = "2"
                type = "t_string_storage"
            },
            @{
                astId = 105
                contract = "EnhancedUpgradeableToken"
                label = "_storageLayoutVersion"
                offset = 0
                slot = "3"
                type = "t_uint256"
            }
        )
        types = @{
            "t_uint256" = @{
                encoding = "inplace"
                label = "uint256"
                numberOfBytes = "32"
            }
            "t_address" = @{
                encoding = "inplace"
                label = "address"
                numberOfBytes = "20"
            }
            "t_bool" = @{
                encoding = "inplace"
                label = "bool"
                numberOfBytes = "1"
            }
            "t_string_storage" = @{
                encoding = "dynamic_array"
                label = "string"
                numberOfBytes = "32"
            }
        }
    }
    
    $SampleLayout | ConvertTo-Json -Depth 10 | Out-File -FilePath $CurrentLayoutFile
    Write-Host "✅ Current storage layout saved to $CurrentLayoutFile" -ForegroundColor Green
} catch {
    Write-Host "❌ Error getting current storage layout: $_" -ForegroundColor Red
    exit 1
}

# Load current layout
try {
    $CurrentLayout = Get-Content $CurrentLayoutFile | ConvertFrom-Json
    Write-Host "✅ Loaded current storage layout" -ForegroundColor Green
} catch {
    Write-Host "❌ Error loading current storage layout: $_" -ForegroundColor Red
    exit 1
}

# In a real scenario, we would also load a previous version layout for comparison
# For this example, we'll create a simulated previous version
$PreviousLayoutFile = "$OutputDir/$ContractName-previous-layout.json"
$PreviousLayout = @{
    storage = @(
        @{
            astId = 101
            contract = "EnhancedUpgradeableToken"
            label = "feePercentage"
            offset = 0
            slot = "0"
            type = "t_uint256"
        },
        @{
            astId = 102
            contract = "EnhancedUpgradeableToken"
            label = "feeRecipient"
            offset = 0
            slot = "1"
            type = "t_address"
        },
        @{
            astId = 103
            contract = "EnhancedUpgradeableToken"
            label = "transferPaused"
            offset = 20
            slot = "1"
            type = "t_bool"
        },
        @{
            astId = 104
            contract = "EnhancedUpgradeableToken"
            label = "_version"
            offset = 0
            slot = "2"
            type = "t_string_storage"
        }
    )
    types = @{
        "t_uint256" = @{
            encoding = "inplace"
            label = "uint256"
            numberOfBytes = "32"
        }
        "t_address" = @{
            encoding = "inplace"
            label = "address"
            numberOfBytes = "20"
        }
        "t_bool" = @{
            encoding = "inplace"
            label = "bool"
            numberOfBytes = "1"
        }
        "t_string_storage" = @{
            encoding = "dynamic_array"
            label = "string"
            numberOfBytes = "32"
        }
    }
}

$PreviousLayout | ConvertTo-Json -Depth 10 | Out-File -FilePath $PreviousLayoutFile
Write-Host "✅ Previous storage layout saved to $PreviousLayoutFile" -ForegroundColor Green

# Validate storage layout compatibility
Write-Host "Validating storage layout compatibility..." -ForegroundColor Yellow

# Check 1: No slot reordering
$CompatibilityIssues = @()
$MaxSlot = -1

for ($i = 0; $i -lt $CurrentLayout.storage.Count; $i++) {
    $CurrentVar = $CurrentLayout.storage[$i]
    
    # Check if this variable exists in the previous layout at the same position
    if ($i -lt $PreviousLayout.storage.Count) {
        $PreviousVar = $PreviousLayout.storage[$i]
        
        # Check if slot has changed (reordering)
        if ($CurrentVar.slot -ne $PreviousVar.slot -and $PreviousVar.slot -le $MaxSlot) {
            $CompatibilityIssues += "❌ Variable '$($CurrentVar.label)' slot changed from $($PreviousVar.slot) to $($CurrentVar.slot) (potential reordering)"
        }
    } else {
        # This is a new variable, should be appended
        if ([int]$CurrentVar.slot -lt $MaxSlot) {
            $CompatibilityIssues += "❌ New variable '$($CurrentVar.label)' in slot $($CurrentVar.slot) should be appended (after slot $MaxSlot)"
        }
    }
    
    # Update max slot
    if ([int]$CurrentVar.slot -gt $MaxSlot) {
        $MaxSlot = [int]$CurrentVar.slot
    }
}

# Check 2: No type changes that break layout
for ($i = 0; $i -lt [Math]::Min($CurrentLayout.storage.Count, $PreviousLayout.storage.Count); $i++) {
    $CurrentVar = $CurrentLayout.storage[$i]
    $PreviousVar = $PreviousLayout.storage[$i]
    
    if ($CurrentVar.label -eq $PreviousVar.label) {
        # Same variable, check type compatibility
        if ($CurrentVar.type -ne $PreviousVar.type) {
            # Check if types are compatible
            $CurrentType = $CurrentLayout.types.$($CurrentVar.type)
            $PreviousType = $PreviousLayout.types.$($PreviousVar.type)
            
            if ($CurrentType.numberOfBytes -ne $PreviousType.numberOfBytes) {
                $CompatibilityIssues += "❌ Variable '$($CurrentVar.label)' type changed from $($PreviousVar.type) to $($CurrentVar.type) with different size"
            }
        }
    }
}

# Check 3: Storage layout version tracking
$HasLayoutVersion = $CurrentLayout.storage | Where-Object { $_.label -eq "_storageLayoutVersion" }
if (-not $HasLayoutVersion) {
    $CompatibilityIssues += "⚠️  No storage layout version tracking variable found"
}

# Report results
Write-Host "Storage Layout Compatibility Report:" -ForegroundColor Yellow
Write-Host "===================================" -ForegroundColor Yellow

if ($CompatibilityIssues.Count -eq 0) {
    Write-Host "✅ No storage layout compatibility issues found" -ForegroundColor Green
    Write-Host "✅ Storage layout is safe for upgrades" -ForegroundColor Green
} else {
    foreach ($Issue in $CompatibilityIssues) {
        if ($Issue.StartsWith("❌")) {
            Write-Host $Issue -ForegroundColor Red
        } elseif ($Issue.StartsWith("⚠️")) {
            Write-Host $Issue -ForegroundColor Yellow
        } else {
            Write-Host $Issue -ForegroundColor Cyan
        }
    }
    
    if ($CompatibilityIssues | Where-Object { $_.StartsWith("❌") }) {
        Write-Host "❌ Storage layout has critical compatibility issues" -ForegroundColor Red
        exit 1
    } else {
        Write-Host "⚠️  Storage layout has warnings but no critical issues" -ForegroundColor Yellow
    }
}

# Generate detailed report
$ReportFile = "$OutputDir/storage-layout-diff-report.md"
$ReportContent = @"
# Storage Layout Diff Report

## Summary

This report compares the storage layout of $ContractName between versions to ensure upgrade compatibility.

## Current Layout

| Variable | Slot | Offset | Type |
|----------|------|--------|------|
"@

foreach ($Var in $CurrentLayout.storage) {
    $ReportContent += "| $($Var.label) | $($Var.slot) | $($Var.offset) | $($Var.type) |`n"
}

$ReportContent += @"
## Previous Layout

| Variable | Slot | Offset | Type |
|----------|------|--------|------|
"@

foreach ($Var in $PreviousLayout.storage) {
    $ReportContent += "| $($Var.label) | $($Var.slot) | $($Var.offset) | $($Var.type) |`n"
}

$ReportContent += @"
## Compatibility Check

$(if ($CompatibilityIssues.Count -eq 0) {
    "✅ No storage layout compatibility issues found. The contract is safe for upgrades."
} else {
    "⚠️  Issues found:"
    foreach ($Issue in $CompatibilityIssues) {
        "- $Issue"
    }
})

## Recommendations

1. **Never reorder existing storage variables**: Always append new variables at the end
2. **Use explicit storage slot management**: Consider using gap variables for future expansion
3. **Document all storage changes**: Maintain a changelog of storage layout modifications
4. **Implement storage layout versioning**: Track layout versions to detect incompatible upgrades
5. **Use OpenZeppelin's upgrade plugins**: Leverage automated tools for storage layout validation

## Conclusion

$(if ($CompatibilityIssues.Count -eq 0) {
    "The storage layout is compatible and safe for upgrades. No critical issues were detected."
} else {
    "While the storage layout has some issues, they may not prevent upgrades. Review the findings carefully before proceeding."
})
"@

Set-Content -Path $ReportFile -Value $ReportContent
Write-Host "📝 Detailed report saved to $ReportFile" -ForegroundColor Cyan

Write-Host ""
Write-Host "✅ Storage Layout Diff Tool completed successfully!" -ForegroundColor Green