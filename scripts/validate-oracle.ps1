# Oracle Validation Script
# This script validates the complete Oracle implementation and testing

param(
    [Parameter(Mandatory=$false)]
    [string]$FoundryProfile = "default"
)

# Function to check required files
function Test-RequiredFiles {
    Write-Host "Checking required files..." -ForegroundColor Cyan
    
    $requiredFiles = @(
        "contracts/src/core/Oracle.sol",
        "contracts/test/core/OracleTest.sol",
        "docs/protocol/ORACLE-TESTS.md",
        "scripts/run-oracle-tests.ps1",
        ".github/workflows/contract-oracle.yml"
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

# Function to run Oracle test suite
function Test-OracleTestSuite {
    Write-Host "Running Oracle test suite..." -ForegroundColor Cyan
    
    # Change to contracts directory
    Set-Location -Path "contracts"
    
    try {
        $result = forge test --match-contract OracleTest
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  [OK] All Oracle tests passed" -ForegroundColor Green
            return $true
        } else {
            Write-Host "  [ERROR] Oracle tests failed" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "  [ERROR] Failed to run Oracle tests: $_" -ForegroundColor Red
        return $false
    } finally {
        # Return to original directory
        Set-Location -Path ".."
    }
}

# Function to validate documentation
function Test-Documentation {
    Write-Host "Validating documentation..." -ForegroundColor Cyan
    
    $docFile = "docs/protocol/ORACLE-TESTS.md"
    if (Test-Path $docFile) {
        $content = Get-Content $docFile -Raw
        $requiredSections = @(
            "Overview",
            "Test Categories",
            "Unit Tests",
            "Fuzz Tests",
            "Invariant Tests",
            "Manipulation Protection Tests",
            "Security Requirements Verification",
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
    
    $workflowFile = ".github/workflows/contract-oracle.yml"
    if (Test-Path $workflowFile) {
        $content = Get-Content $workflowFile -Raw
        $requiredElements = @(
            "name:.*Oracle Contract Tests",
            "on:",
            "push:",
            "pull_request:",
            "jobs:",
            "test:",
            "runs-on:",
            "steps:",
            "foundry-toolchain",
            "run.*forge build",
            "run.*forge test.*OracleTest"
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
    $contractFile = "contracts/src/core/Oracle.sol"
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
        
        # Check for proper state management
        if ($content -match "notShutdown") {
            Write-Host "  [OK] Shutdown state management present" -ForegroundColor Green
        } else {
            Write-Host "  [WARNING] No shutdown state management found" -ForegroundColor Yellow
        }
        
        return $true
    } else {
        Write-Host "  [ERROR] Contract file not found: $contractFile" -ForegroundColor Red
        return $false
    }
}

# Main validation function
function Invoke-OracleValidation {
    Write-Host "=== Oracle Security Layer Validation ===" -ForegroundColor Green
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
    if (-not (Test-OracleTestSuite)) {
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
        Write-Host "=== ORACLE VALIDATION PASSED ===" -ForegroundColor Green
        Write-Host "All components validated successfully!" -ForegroundColor Green
        return 0
    } else {
        Write-Host "=== ORACLE VALIDATION FAILED ===" -ForegroundColor Red
        Write-Host "One or more validation steps failed. Please check the output above." -ForegroundColor Red
        return 1
    }
}

# Main execution
$env:FOUNDRY_PROFILE = $FoundryProfile
exit (Invoke-OracleValidation)