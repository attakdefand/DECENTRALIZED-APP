# LOB (Limit Order Book) Validation Script
# This script validates the complete LOB implementation and testing

param(
    [Parameter(Mandatory=$false)]
    [string]$FoundryProfile = "default"
)

# Function to check required files
function Test-RequiredFiles {
    Write-Host "Checking required files..." -ForegroundColor Cyan
    
    $requiredFiles = @(
        "contracts/src/core/Orderbook.sol",
        "contracts/test/core/OrderbookTest.sol",
        "docs/tests/LOB-TESTS.md",
        "scripts/run-lob-tests.ps1",
        ".github/workflows/contract-lob.yml"
    )
    
    $allExist = $true
    foreach ($file in $requiredFiles) {
        if (Test-Path $file) {
            Write-Host "  [OK] $file" -ForegroundColor Green
        } else {
            Write-Host "  [MISSING] $file" -ForegroundColor Red
            $allExist = $false
        }
    }
    
    return $allExist
}

# Function to validate Solidity compilation
function Test-SolidityCompilation {
    Write-Host "Validating Solidity compilation..." -ForegroundColor Cyan
    
    # Change to contracts directory
    Set-Location -Path "contracts"
    
    try {
        $result = forge build --skip test
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  [OK] Solidity contracts compiled successfully" -ForegroundColor Green
            return $true
        } else {
            Write-Host "  [ERROR] Solidity compilation failed" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "  [ERROR] Failed to compile Solidity contracts: $_" -ForegroundColor Red
        return $false
    } finally {
        # Return to original directory
        Set-Location -Path ".."
    }
}

# Function to run LOB test suite
function Test-LOBTestSuite {
    Write-Host "Running LOB test suite..." -ForegroundColor Cyan
    
    # Change to contracts directory
    Set-Location -Path "contracts"
    
    try {
        $result = forge test --match-contract OrderbookTest
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  [OK] All LOB tests passed" -ForegroundColor Green
            return $true
        } else {
            Write-Host "  [ERROR] LOB tests failed" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "  [ERROR] Failed to run LOB tests: $_" -ForegroundColor Red
        return $false
    } finally {
        # Return to original directory
        Set-Location -Path ".."
    }
}

# Function to validate documentation
function Test-Documentation {
    Write-Host "Validating documentation..." -ForegroundColor Cyan
    
    $docFile = "docs/tests/LOB-TESTS.md"
    if (Test-Path $docFile) {
        $content = Get-Content $docFile -Raw
        $requiredSections = @(
            "Overview",
            "Test Categories",
            "Unit Tests",
            "Fuzz Tests",
            "Invariant Tests",
            "MEV Simulation Tests",
            "Replay Fixtures",
            "Test Execution",
            "Validation Criteria",
            "Implementation Evidence",
            "Security Considerations"
        )
        
        $allSectionsPresent = $true
        foreach ($section in $requiredSections) {
            if ($content -match "##\s*$section") {
                Write-Host "  [OK] Section: $section" -ForegroundColor Green
            } else {
                Write-Host "  [MISSING] Section: $section" -ForegroundColor Red
                $allSectionsPresent = $false
            }
        }
        
        return $allSectionsPresent
    } else {
        Write-Host "  [MISSING] Documentation file: $docFile" -ForegroundColor Red
        return $false
    }
}

# Function to validate GitHub Actions workflow
function Test-GitHubActions {
    Write-Host "Validating GitHub Actions workflow..." -ForegroundColor Cyan
    
    $workflowFile = ".github/workflows/contract-lob.yml"
    if (Test-Path $workflowFile) {
        $content = Get-Content $workflowFile -Raw
        $requiredElements = @(
            "name:.*LOB Contract Tests",
            "on:",
            "push:",
            "pull_request:",
            "jobs:",
            "test:",
            "runs-on:",
            "steps:",
            "foundry-toolchain",
            "run.*forge build",
            "run.*forge test.*OrderbookTest"
        )
        
        $allElementsPresent = $true
        foreach ($element in $requiredElements) {
            if ($content -match $element) {
                Write-Host "  [OK] Element: $element" -ForegroundColor Green
            } else {
                Write-Host "  [MISSING] Element: $element" -ForegroundColor Red
                $allElementsPresent = $false
            }
        }
        
        return $allElementsPresent
    } else {
        Write-Host "  [MISSING] Workflow file: $workflowFile" -ForegroundColor Red
        return $false
    }
}

# Function to run security checks
function Test-SecurityChecks {
    Write-Host "Running security checks..." -ForegroundColor Cyan
    
    # Check for common vulnerabilities
    $contractFile = "contracts/src/core/Orderbook.sol"
    if (Test-Path $contractFile) {
        $content = Get-Content $contractFile -Raw
        
        # Check for reentrancy guard
        if ($content -match "nonReentrant") {
            Write-Host "  [OK] Reentrancy guard present" -ForegroundColor Green
        } else {
            Write-Host "  [INFO] No explicit reentrancy guard (may be handled elsewhere)" -ForegroundColor Yellow
        }
        
        # Check for access control
        if ($content -match "onlyOwner") {
            Write-Host "  [OK] Access control modifiers present" -ForegroundColor Green
        } else {
            Write-Host "  [WARNING] No access control modifiers found" -ForegroundColor Yellow
        }
        
        # Check for proper token handling
        if ($content -match "transferFrom.*msg.sender.*address\(this\)") {
            Write-Host "  [OK] Proper token transfer pattern" -ForegroundColor Green
        } else {
            Write-Host "  [WARNING] Token transfer pattern not found" -ForegroundColor Yellow
        }
        
        return $true
    } else {
        Write-Host "  [ERROR] Contract file not found: $contractFile" -ForegroundColor Red
        return $false
    }
}

# Main validation function
function Invoke-LOBValidation {
    Write-Host "=== LOB (Limit Order Book) Validation ===" -ForegroundColor Green
    Write-Host "Profile: $FoundryProfile" -ForegroundColor Gray
    Write-Host ""
    
    $validationPassed = $true
    
    # Check required files
    if (-not (Test-RequiredFiles)) {
        $validationPassed = $false
    }
    Write-Host ""
    
    # Validate Solidity compilation
    if (-not (Test-SolidityCompilation)) {
        $validationPassed = $false
    }
    Write-Host ""
    
    # Run test suite
    if (-not (Test-LOBTestSuite)) {
        $validationPassed = $false
    }
    Write-Host ""
    
    # Validate documentation
    if (-not (Test-Documentation)) {
        $validationPassed = $false
    }
    Write-Host ""
    
    # Validate GitHub Actions
    if (-not (Test-GitHubActions)) {
        $validationPassed = $false
    }
    Write-Host ""
    
    # Run security checks
    if (-not (Test-SecurityChecks)) {
        $validationPassed = $false
    }
    Write-Host ""
    
    # Final result
    if ($validationPassed) {
        Write-Host "=== LOB VALIDATION PASSED ===" -ForegroundColor Green
        Write-Host "All components validated successfully!" -ForegroundColor Green
        return 0
    } else {
        Write-Host "=== LOB VALIDATION FAILED ===" -ForegroundColor Red
        Write-Host "One or more validation steps failed. Please check the output above." -ForegroundColor Red
        return 1
    }
}

# Main execution
$env:FOUNDRY_PROFILE = $FoundryProfile
exit (Invoke-LOBValidation)