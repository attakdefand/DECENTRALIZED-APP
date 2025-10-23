# Verification script for the repository scaffold implementation

Write-Host "Verifying repository scaffold implementation..." -ForegroundColor Green

# Define the required paths
$requiredPaths = @(
    "contracts/src",
    "contracts/script",
    "contracts/test",
    "services/aa-bundler",
    "infra/compose",
    "infra/k8s",
    "infra/policies",
    "docs/api",
    "docs/api/sdk/ts",
    "docs/api/sdk/rust",
    "tests/e2e",
    "tests/perf",
    "tests/chaos"
)

# Define the required files
$requiredFiles = @(
    "contracts/src/README.md",
    "contracts/script/README.md",
    "contracts/test/README.md",
    "services/aa-bundler/Cargo.toml",
    "services/aa-bundler/src/main.rs",
    "services/aa-bundler/Dockerfile",
    "infra/compose/docker-compose.yml",
    "infra/k8s/README.md",
    "infra/k8s/helm/Chart.yaml",
    "infra/k8s/helm/values.yaml",
    "infra/policies/README.md",
    "infra/policies/opa/policy.rego",
    "infra/policies/cedar/policy.cedar",
    "docs/api/openapi.yaml",
    "docs/api/sdk/ts/README.md",
    "docs/api/sdk/rust/README.md",
    "tests/e2e/README.md",
    "tests/perf/README.md",
    "tests/chaos/README.md"
)

# Check required paths
Write-Host "Checking required directories..." -ForegroundColor Yellow
$allPathsExist = $true
foreach ($path in $requiredPaths) {
    if (Test-Path $path) {
        Write-Host "  ✓ $path" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $path" -ForegroundColor Red
        $allPathsExist = $false
    }
}

# Check required files
Write-Host "Checking required files..." -ForegroundColor Yellow
$allFilesExist = $true
foreach ($file in $requiredFiles) {
    if (Test-Path $file) {
        Write-Host "  ✓ $file" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $file" -ForegroundColor Red
        $allFilesExist = $false
    }
}

# Final verification
if ($allPathsExist -and $allFilesExist) {
    Write-Host "✅ All required paths and files exist!" -ForegroundColor Green
    Write-Host "Repository scaffold implementation is complete." -ForegroundColor Green
} else {
    Write-Host "❌ Some required paths or files are missing!" -ForegroundColor Red
    exit 1
}