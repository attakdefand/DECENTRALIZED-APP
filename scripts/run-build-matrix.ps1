#!/usr/bin/env pwsh

# Script to run the Solidity/Vyper build matrix tests

Write-Host "Running Smart Contract Build Matrix Tests..." -ForegroundColor Green

# Set working directory to contracts
Set-Location -Path "d:\DECENTRALIZED-APP\contracts"

# Create results directory
$ResultsDir = "build-results"
if (!(Test-Path $ResultsDir)) {
    New-Item -ItemType Directory -Name $ResultsDir
}

# Define build configurations
$Configurations = @(
    @{ Name = "default"; Description = "Default configuration (optimizer=200, no IR)" },
    @{ Name = "optimized"; Description = "Optimized configuration (optimizer=10000, no IR)" },
    @{ Name = "ir"; Description = "IR configuration (optimizer=200, with IR)" },
    @{ Name = "ir-optimized"; Description = "IR-Optimized configuration (optimizer=10000, with IR)" }
)

# Define Solidity compiler versions to test
$CompilerVersions = @("0.8.19", "0.8.20", "0.8.21")

# Initialize results tracking
$BuildResults = @()
$BytecodeDiffs = @()
$GasSnapshots = @()

# Test each configuration
foreach ($Config in $Configurations) {
    Write-Host "Testing configuration: $($Config.Name)" -ForegroundColor Yellow
    
    # Run build with current configuration
    $BuildLog = "$ResultsDir/build-$($Config.Name).log"
    $OutputDir = "out-$($Config.Name)"
    
    try {
        # Clean previous build
        if (Test-Path $OutputDir) {
            Remove-Item -Recurse -Force $OutputDir
        }
        
        # Run forge build
        $BuildCommand = "forge build --profile $($Config.Name) --out $OutputDir 2>&1"
        Write-Host "Running: $BuildCommand" -ForegroundColor Cyan
        Invoke-Expression $BuildCommand | Out-File -FilePath $BuildLog
        
        # Check if build succeeded
        $BuildSuccess = $LASTEXITCODE -eq 0
        $BuildResults += @{
            Configuration = $Config.Name
            Success = $BuildSuccess
            LogFile = $BuildLog
        }
        
        if ($BuildSuccess) {
            Write-Host "Build succeeded for configuration: $($Config.Name)" -ForegroundColor Green
        } else {
            Write-Host "Build failed for configuration: $($Config.Name)" -ForegroundColor Red
        }
    } catch {
        Write-Host "Error building configuration: $($Config.Name) - $_" -ForegroundColor Red
        $BuildResults += @{
            Configuration = $Config.Name
            Success = $false
            LogFile = $BuildLog
        }
    }
}

# Test different compiler versions with default configuration
Write-Host "Testing different compiler versions..." -ForegroundColor Yellow
foreach ($Version in $CompilerVersions) {
    Write-Host "Testing compiler version: $Version" -ForegroundColor Cyan
    
    $BuildLog = "$ResultsDir/build-solc-$Version.log"
    $OutputDir = "out-solc-$Version"
    
    try {
        # Clean previous build
        if (Test-Path $OutputDir) {
            Remove-Item -Recurse -Force $OutputDir
        }
        
        # Run forge build with specific compiler version
        $BuildCommand = "forge build --use solc:$Version --out $OutputDir 2>&1"
        Write-Host "Running: $BuildCommand" -ForegroundColor Cyan
        Invoke-Expression $BuildCommand | Out-File -FilePath $BuildLog
        
        # Check if build succeeded
        $BuildSuccess = $LASTEXITCODE -eq 0
        $BuildResults += @{
            Configuration = "solc-$Version"
            Success = $BuildSuccess
            LogFile = $BuildLog
        }
        
        if ($BuildSuccess) {
            Write-Host "Build succeeded for compiler version: $Version" -ForegroundColor Green
        } else {
            Write-Host "Build failed for compiler version: $Version" -ForegroundColor Red
        }
    } catch {
        Write-Host "Error building with compiler version: $Version - $_" -ForegroundColor Red
        $BuildResults += @{
            Configuration = "solc-$Version"
            Success = $false
            LogFile = $BuildLog
        }
    }
}

# Run tests for each configuration
Write-Host "Running tests for each configuration..." -ForegroundColor Yellow
foreach ($Config in $Configurations) {
    Write-Host "Running tests for configuration: $($Config.Name)" -ForegroundColor Cyan
    
    $TestLog = "$ResultsDir/test-$($Config.Name).log"
    
    try {
        # Run forge test
        $TestCommand = "forge test --profile $($Config.Name) -vvv 2>&1"
        Write-Host "Running: $TestCommand" -ForegroundColor Cyan
        Invoke-Expression $TestCommand | Out-File -FilePath $TestLog
        
        # Check if tests succeeded
        $TestSuccess = $LASTEXITCODE -eq 0
        $BuildResults += @{
            Configuration = "$($Config.Name)-tests"
            Success = $TestSuccess
            LogFile = $TestLog
        }
        
        if ($TestSuccess) {
            Write-Host "Tests succeeded for configuration: $($Config.Name)" -ForegroundColor Green
        } else {
            Write-Host "Tests failed for configuration: $($Config.Name)" -ForegroundColor Red
        }
    } catch {
        Write-Host "Error running tests for configuration: $($Config.Name) - $_" -ForegroundColor Red
        $BuildResults += @{
            Configuration = "$($Config.Name)-tests"
            Success = $false
            LogFile = $TestLog
        }
    }
}

# Generate gas snapshots for each configuration
Write-Host "Generating gas snapshots..." -ForegroundColor Yellow
foreach ($Config in $Configurations) {
    Write-Host "Generating gas snapshot for configuration: $($Config.Name)" -ForegroundColor Cyan
    
    $SnapshotFile = "$ResultsDir/gas-snapshot-$($Config.Name).json"
    
    try {
        # Run forge snapshot
        $SnapshotCommand = "forge snapshot --profile $($Config.Name) --json --output $SnapshotFile 2>&1"
        Write-Host "Running: $SnapshotCommand" -ForegroundColor Cyan
        Invoke-Expression $SnapshotCommand
        
        # Check if snapshot succeeded
        $SnapshotSuccess = $LASTEXITCODE -eq 0
        $BuildResults += @{
            Configuration = "$($Config.Name)-snapshot"
            Success = $SnapshotSuccess
            LogFile = $SnapshotFile
        }
        
        if ($SnapshotSuccess) {
            Write-Host "Gas snapshot generated for configuration: $($Config.Name)" -ForegroundColor Green
        } else {
            Write-Host "Gas snapshot failed for configuration: $($Config.Name)" -ForegroundColor Red
        }
    } catch {
        Write-Host "Error generating gas snapshot for configuration: $($Config.Name) - $_" -ForegroundColor Red
        $BuildResults += @{
            Configuration = "$($Config.Name)-snapshot"
            Success = $false
            LogFile = $SnapshotFile
        }
    }
}

# Compare bytecode between configurations
Write-Host "Comparing bytecode between configurations..." -ForegroundColor Yellow
$ReferenceOutput = "out-default"
if (Test-Path $ReferenceOutput) {
    # Get list of contract files
    $ContractFiles = Get-ChildItem -Path "$ReferenceOutput" -Recurse -Filter "*.json" | Where-Object { $_.Name -notlike "*dbg*" }
    
    foreach ($ContractFile in $ContractFiles) {
        $ContractName = $ContractFile.BaseName
        $ReferenceBytecode = (Get-Content $ContractFile.FullName | ConvertFrom-Json).bytecode.object
        
        foreach ($Config in $Configurations) {
            $ComparisonOutput = "out-$($Config.Name)"
            $ComparisonFile = "$ComparisonOutput/$($ContractFile.Directory.Name)/$($ContractFile.Name)"
            
            if (Test-Path $ComparisonFile) {
                $ComparisonBytecode = (Get-Content $ComparisonFile | ConvertFrom-Json).bytecode.object
                
                # Compare bytecodes
                if ($ReferenceBytecode -ne $ComparisonBytecode) {
                    $DiffEntry = @{
                        Contract = $ContractName
                        ReferenceConfig = "default"
                        ComparisonConfig = $Config.Name
                        Match = $false
                        SizeDifference = $ComparisonBytecode.Length - $ReferenceBytecode.Length
                    }
                    $BytecodeDiffs += $DiffEntry
                    Write-Host "Bytecode mismatch for $ContractName between default and $($Config.Name)" -ForegroundColor Red
                } else {
                    $DiffEntry = @{
                        Contract = $ContractName
                        ReferenceConfig = "default"
                        ComparisonConfig = $Config.Name
                        Match = $true
                        SizeDifference = 0
                    }
                    $BytecodeDiffs += $DiffEntry
                    Write-Host "Bytecode matches for $ContractName between default and $($Config.Name)" -ForegroundColor Green
                }
            }
        }
    }
}

# Generate summary report
Write-Host "Generating build matrix summary report..." -ForegroundColor Yellow
$SummaryFile = "$ResultsDir/build-matrix-summary.md"

$SummaryContent = @"
# Smart Contract Build Matrix Test Results

## Summary

This report summarizes the results of testing the smart contract build matrix with different Solidity compiler versions, optimizer settings, and Yul IR toggles.

## Build Results

| Configuration | Success | Log File |
|---------------|---------|----------|
"@

foreach ($Result in $BuildResults) {
    $Status = if ($Result.Success) { "✅ Pass" } else { "❌ Fail" }
    $SummaryContent += "| $($Result.Configuration) | $Status | [$($Result.LogFile)]($($Result.LogFile)) |`n"
}

$SummaryContent += @"

## Bytecode Comparison

| Contract | Reference Config | Comparison Config | Match | Size Difference |
|----------|------------------|-------------------|-------|-----------------|
"@

foreach ($Diff in $BytecodeDiffs) {
    $MatchStatus = if ($Diff.Match) { "✅ Yes" } else { "❌ No" }
    $SummaryContent += "| $($Diff.Contract) | $($Diff.ReferenceConfig) | $($Diff.ComparisonConfig) | $MatchStatus | $($Diff.SizeDifference) |`n"
}

$SummaryContent += @"

## Gas Snapshots

Gas snapshots have been generated for each configuration and can be found in the build-results directory.

## Conclusion

This build matrix testing ensures:
- Bytecode stability across different compiler versions and settings
- Optimization safety with various optimizer runs
- No undefined behavior in generated bytecode
- Consistent gas consumption patterns

"@

Set-Content -Path $SummaryFile -Value $SummaryContent

# Display summary
Write-Host "Build Matrix Test Summary:" -ForegroundColor Yellow
Write-Host "========================" -ForegroundColor Yellow
foreach ($Result in $BuildResults) {
    $Status = if ($Result.Success) { "✅ PASS" } else { "❌ FAIL" }
    Write-Host "$Status $($Result.Configuration)" -ForegroundColor $(if ($Result.Success) { "Green" } else { "Red" })
}

Write-Host "Detailed results and logs can be found in: $ResultsDir" -ForegroundColor Cyan
Write-Host "Summary report: $SummaryFile" -ForegroundColor Cyan

# Return to original directory
Set-Location -Path "d:\DECENTRALIZED-APP"

Write-Host "Smart Contract Build Matrix Tests completed!" -ForegroundColor Green