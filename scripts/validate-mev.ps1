# MEV & Fairness Validation Script
# This script validates the complete MEV & Fairness implementation and testing

param(
    [Parameter(Mandatory=$false)]
    [string]$FoundryProfile = "default"
)

# Function to check required files
function Test-RequiredFiles {
    Write-Host "Checking required files..." -ForegroundColor Cyan
    
    $requiredFiles = @(
        "contracts/src/core/CommitReveal.sol",
        "contracts/src/core/BatchAuction.sol",
        "contracts/test/core/MEVTests.sol",
        "contracts/test/core/MEVAndFairnessTest.sol",
        "docs/protocol/MEV-MITIGATIONS.md",
        "docs/protocol/MEV-TESTS.md",
        "scripts/run-mev-tests.ps1",
        ".github/workflows/mev-mitigation.yml"
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

# Function to run MEV test suite
function Test-MEVTestSuite {
    Write-Host "Running MEV test suite..." -ForegroundColor Cyan
    
    # Change to contracts directory
    Set-Location -Path "contracts"
    
    try {
        $result = forge test --match-contract "MEVTests|MEVAndFairnessTest"
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  [OK] All MEV tests passed" -ForegroundColor Green
            return $true
        } else {
            Write-Host "  [ERROR] MEV tests failed" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "  [ERROR] Failed to run MEV tests: $_" -ForegroundColor Red
        return $false
    } finally {
        # Return to original directory
        Set-Location -Path ".."
    }
}

# Function to validate documentation
function Test-Documentation {
    Write-Host "Validating documentation..." -ForegroundColor Cyan
    
    $docFiles = @(
        @{Path="docs/protocol/MEV-MITIGATIONS.md"; RequiredSections=@("Overview", "Implemented Mitigations", "Technical Implementation", "Testing and Validation")},
        @{Path="docs/protocol/MEV-TESTS.md"; RequiredSections=@("Overview", "Test Categories", "Unit Tests", "Fuzz Tests", "Invariant Tests", "MEV Simulation Tests", "Security Requirements Verification", "Test Execution", "Validation Criteria", "Implementation Evidence", "Security Considerations")}
    )
    
    $allSectionsPresent = $true
    foreach ($doc in $docFiles) {
        $docFile = $doc.Path
        if (Test-Path $docFile) {
            $content = Get-Content $docFile -Raw
            foreach ($section in $doc.RequiredSections) {
                if ($content -match "##\s*$section") {
                    Write-Host "  [OK] Section: $section" -ForegroundColor Green
                } else {
                    Write-Host "  [MISSING] Section: $section" -ForegroundColor Red
                    $allSectionsPresent = $false
                }
            }
        } else {
            Write-Host "  [MISSING] Documentation file: $docFile" -ForegroundColor Red
            $allSectionsPresent = $false
        }
    }
    
    return $allSectionsPresent
}

# Function to validate GitHub Actions workflow
function Test-GitHubActions {
    Write-Host "Validating GitHub Actions workflow..." -ForegroundColor Cyan
    
    $workflowFile = ".github/workflows/mev-mitigation.yml"
    if (Test-Path $workflowFile) {
        $content = Get-Content $workflowFile -Raw
        $requiredElements = @(
            "name:.*MEV Mitigation Tests",
            "on:",
            "push:",
            "pull_request:",
            "jobs:",
            "mev-tests:",
            "runs-on:",
            "steps:",
            "foundry-toolchain",
            "run.*forge test.*MEVTests",
            "run.*forge test.*MEVAndFairnessTest"
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
    
    # Check for common vulnerabilities in CommitReveal contract
    $contractFile = "contracts/src/core/CommitReveal.sol"
    if (Test-Path $contractFile) {
        $content = Get-Content $contractFile -Raw
        
        # Check for reentrancy guard
        if ($content -match "nonReentrant") {
            Write-Host "  [OK] Reentrancy guard present" -ForegroundColor Green
        } else {
            Write-Host "  [WARNING] No explicit reentrancy guard (may be handled elsewhere)" -ForegroundColor Yellow
        }
        
        # Check for access control
        if ($content -match "onlyOwner") {
            Write-Host "  [OK] Access control modifiers present" -ForegroundColor Green
        } else {
            Write-Host "  [WARNING] No access control modifiers found" -ForegroundColor Yellow
        }
        
        # Check for proper state management
        if ($content -match "hasSufficientStake|notInAntiSandwichWindow|batchIntervalRespected") {
            Write-Host "  [OK] State management modifiers present" -ForegroundColor Green
        } else {
            Write-Host "  [WARNING] No state management modifiers found" -ForegroundColor Yellow
        }
        
        return $true
    } else {
        Write-Host "  [ERROR] Contract file not found: $contractFile" -ForegroundColor Red
        return $false
    }
}

# Function to validate test coverage
function Test-TestCoverage {
    Write-Host "Validating test coverage..." -ForegroundColor Cyan
    
    # Check that test files exist and have content
    $testFiles = @(
        "contracts/test/core/MEVTests.sol",
        "contracts/test/core/MEVAndFairnessTest.sol"
    )
    
    $coverageValid = $true
    foreach ($testFile in $testFiles) {
        if (Test-Path $testFile) {
            $content = Get-Content $testFile -Raw
            if ($content.Length -gt 1000) {  # Arbitrary threshold for meaningful content
                Write-Host "  [OK] Test file has sufficient content: $testFile" -ForegroundColor Green
            } else {
                Write-Host "  [WARNING] Test file may be too short: $testFile" -ForegroundColor Yellow
                $coverageValid = $false
            }
        } else {
            Write-Host "  [MISSING] Test file: $testFile" -ForegroundColor Red
            $coverageValid = $false
        }
    }
    
    return $coverageValid
}

# Main validation function
function Invoke-MEVValidation {
    Write-Host "=== MEV & Fairness Security Layer Validation ===" -ForegroundColor Green
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
    if (-not (Test-MEVTestSuite)) {
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
    
    # Validate test coverage
    if (-not (Test-TestCoverage)) {
        $validationPassed = $false
    }
    Write-Host ""
    
    # Final result
    if ($validationPassed) {
        Write-Host "=== MEV & FAIRNESS VALIDATION PASSED ===" -ForegroundColor Green
        Write-Host "All components validated successfully!" -ForegroundColor Green
        return 0
    } else {
        Write-Host "=== MEV & FAIRNESS VALIDATION FAILED ===" -ForegroundColor Red
        Write-Host "One or more validation steps failed. Please check the output above." -ForegroundColor Red
        return 1
    }
}

# Main execution
$env:FOUNDRY_PROFILE = $FoundryProfile
exit (Invoke-MEVValidation)