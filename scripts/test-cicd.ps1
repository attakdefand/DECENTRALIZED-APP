# Test script to verify CI/CD components

Write-Host "Testing CI/CD components..." -ForegroundColor Green

# Test that all workflow files exist
$workflowFiles = @(
    ".github/workflows/ci.yml",
    ".github/workflows/release.yml",
    ".github/workflows/deploy.yml"
)

Write-Host "Checking workflow files..." -ForegroundColor Yellow
foreach ($file in $workflowFiles) {
    if (Test-Path $file) {
        Write-Host "  ✓ $file" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $file" -ForegroundColor Red
        exit 1
    }
}

# Test that documentation exists
$docs = @(
    "docs/cicd_controls_implementation.md"
)

Write-Host "Checking documentation..." -ForegroundColor Yellow
foreach ($doc in $docs) {
    if (Test-Path $doc) {
        Write-Host "  ✓ $doc" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $doc" -ForegroundColor Red
        exit 1
    }
}

# Test that the CICD controls document is updated
$cicdControls = "dapp_cicd_controls.md"
if (Test-Path $cicdControls) {
    $content = Get-Content $cicdControls -Raw
    if ($content -match "\[x\] Build matrix") {
        Write-Host "  ✓ $cicdControls updated" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $cicdControls not updated" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "  ✗ $cicdControls not found" -ForegroundColor Red
    exit 1
}

Write-Host "All CI/CD components verified successfully!" -ForegroundColor Green