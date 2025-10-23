# Test script to verify all services build correctly

Write-Host "Testing build of all services..." -ForegroundColor Green

# Test build of AA Bundler service
Write-Host "Building AA Bundler service..." -ForegroundColor Yellow
cargo build -p aa-bundler
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to build AA Bundler service" -ForegroundColor Red
    exit 1
}

# Test build of MEV Monitor service
Write-Host "Building MEV Monitor service..." -ForegroundColor Yellow
cargo build -p mev-monitor
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to build MEV Monitor service" -ForegroundColor Red
    exit 1
}

# Test build of IPFS Monitor service
Write-Host "Building IPFS Monitor service..." -ForegroundColor Yellow
cargo build -p ipfs-monitor
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to build IPFS Monitor service" -ForegroundColor Red
    exit 1
}

# Test build of Keeper service
Write-Host "Building Keeper service..." -ForegroundColor Yellow
cargo build -p keepers-service
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to build Keeper service" -ForegroundColor Red
    exit 1
}

# Test build of Indexer service
Write-Host "Building Indexer service..." -ForegroundColor Yellow
cargo build -p indexer-service
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to build Indexer service" -ForegroundColor Red
    exit 1
}

# Test build of API service
Write-Host "Building API service..." -ForegroundColor Yellow
cargo build -p api-service
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to build API service" -ForegroundColor Red
    exit 1
}

Write-Host "All services built successfully!" -ForegroundColor Green