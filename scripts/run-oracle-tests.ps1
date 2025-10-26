# Oracle Test Runner
# This script runs all tests for the Oracle security layer

param(
    [Parameter(Mandatory=$false)]
    [string]$FoundryProfile = "default",
    
    [Parameter(Mandatory=$false)]
    [switch]$Verbose,
    
    [Parameter(Mandatory=$false)]
    [switch]$GasReport
)

# Function to check if Foundry is installed
function Test-FoundryInstalled {
    try {
        $forgeVersion = forge --version
        Write-Host "Foundry is installed: $forgeVersion" -ForegroundColor Green
        return $true
    } catch {
        Write-Host "Error: Foundry is not installed or not in PATH" -ForegroundColor Red
        Write-Host "Please install Foundry from: https://getfoundry.sh/" -ForegroundColor Yellow
        return $false
    }
}

# Function to run unit tests
function Invoke-UnitTests {
    Write-Host "Running Oracle Unit Tests..." -ForegroundColor Cyan
    
    $cmd = "forge test --match-contract OracleTest --match-test test"
    if ($Verbose) { $cmd += " -vvv" }
    if ($GasReport) { $cmd += " --gas-report" }
    
    Invoke-Expression $cmd
    return $LASTEXITCODE
}

# Function to run fuzz tests
function Invoke-FuzzTests {
    Write-Host "Running Oracle Fuzz Tests..." -ForegroundColor Cyan
    
    $cmd = "forge test --match-contract OracleTest --match-test testFuzz"
    if ($Verbose) { $cmd += " -vvv" }
    if ($GasReport) { $cmd += " --gas-report" }
    
    Invoke-Expression $cmd
    return $LASTEXITCODE
}

# Function to run invariant tests
function Invoke-InvariantTests {
    Write-Host "Running Oracle Invariant Tests..." -ForegroundColor Cyan
    
    $cmd = "forge test --match-contract OracleTest --match-test invariant"
    if ($Verbose) { $cmd += " -vvv" }
    if ($GasReport) { $cmd += " --gas-report" }
    
    Invoke-Expression $cmd
    return $LASTEXITCODE
}

# Function to run all tests
function Invoke-AllTests {
    Write-Host "Running All Oracle Tests..." -ForegroundColor Cyan
    
    $cmd = "forge test --match-contract OracleTest"
    if ($Verbose) { $cmd += " -vvv" }
    if ($GasReport) { $cmd += " --gas-report" }
    
    Invoke-Expression $cmd
    return $LASTEXITCODE
}

# Function to check test coverage
function Invoke-Coverage {
    Write-Host "Running Oracle Test Coverage..." -ForegroundColor Cyan
    
    $cmd = "forge coverage --match-contract OracleTest"
    if ($Verbose) { $cmd += " -vvv" }
    
    Invoke-Expression $cmd
    return $LASTEXITCODE
}

# Main execution
Write-Host "=== Oracle Test Runner ===" -ForegroundColor Green
Write-Host "Foundry Profile: $FoundryProfile" -ForegroundColor Gray

# Check if Foundry is installed
if (-not (Test-FoundryInstalled)) {
    exit 1
}

# Set Foundry profile
$env:FOUNDRY_PROFILE = $FoundryProfile

# Change to contracts directory
Set-Location -Path "contracts"

try {
    # Run unit tests
    $unitResult = Invoke-UnitTests
    if ($unitResult -ne 0) {
        Write-Host "Unit tests failed with exit code: $unitResult" -ForegroundColor Red
        exit $unitResult
    }
    
    # Run fuzz tests
    $fuzzResult = Invoke-FuzzTests
    if ($fuzzResult -ne 0) {
        Write-Host "Fuzz tests failed with exit code: $fuzzResult" -ForegroundColor Red
        exit $fuzzResult
    }
    
    # Run invariant tests
    $invariantResult = Invoke-InvariantTests
    if ($invariantResult -ne 0) {
        Write-Host "Invariant tests failed with exit code: $invariantResult" -ForegroundColor Red
        exit $invariantResult
    }
    
    # Run coverage if requested
    if ($Verbose) {
        $coverageResult = Invoke-Coverage
        if ($coverageResult -ne 0) {
            Write-Host "Coverage failed with exit code: $coverageResult" -ForegroundColor Red
            exit $coverageResult
        }
    }
    
    Write-Host "All Oracle tests passed successfully!" -ForegroundColor Green
    
} catch {
    Write-Host "Error running tests: $_" -ForegroundColor Red
    exit 1
} finally {
    # Return to original directory
    Set-Location -Path ".."
}