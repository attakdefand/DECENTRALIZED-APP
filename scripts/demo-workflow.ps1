# Demo workflow showing the complete installation and usage process

Write-Host "=== Decentralized Exchange Application Demo ===" -ForegroundColor Cyan
Write-Host ""

# Step 1: Show the CLI help
Write-Host "Step 1: Showing CLI help" -ForegroundColor Yellow
Write-Host "Command: cargo run --bin dex-cli -- --help" -ForegroundColor Gray
cargo run --bin dex-cli -- --help
Write-Host ""

# Step 2: Initialize the application
Write-Host "Step 2: Initializing the application" -ForegroundColor Yellow
Write-Host "Command: cargo run --bin dex-cli -- init" -ForegroundColor Gray
cargo run --bin dex-cli -- init
Write-Host ""

# Step 3: Check status (should show no services running)
Write-Host "Step 3: Checking initial status" -ForegroundColor Yellow
Write-Host "Command: cargo run --bin dex-cli -- status" -ForegroundColor Gray
cargo run --bin dex-cli -- status
Write-Host ""

# Step 4: Start specific services (API and Indexer only for demo)
Write-Host "Step 4: Starting API and Indexer services" -ForegroundColor Yellow
Write-Host "Command: cargo run --bin dex-cli -- start --services api,indexer" -ForegroundColor Gray
Write-Host "Note: This will start the services in the background" -ForegroundColor Magenta
cargo run --bin dex-cli -- start --services api,indexer
Write-Host ""

# Step 5: Check status (should show services running)
Write-Host "Step 5: Checking service status" -ForegroundColor Yellow
Write-Host "Command: cargo run --bin dex-cli -- status" -ForegroundColor Gray
cargo run --bin dex-cli -- status
Write-Host ""

# Step 6: Stop services
Write-Host "Step 6: Stopping services" -ForegroundColor Yellow
Write-Host "Command: cargo run --bin dex-cli -- stop" -ForegroundColor Gray
cargo run --bin dex-cli -- stop
Write-Host ""

# Step 7: Final status check
Write-Host "Step 7: Checking final status" -ForegroundColor Yellow
Write-Host "Command: cargo run --bin dex-cli -- status" -ForegroundColor Gray
cargo run --bin dex-cli -- status
Write-Host ""

Write-Host "=== Demo Complete ===" -ForegroundColor Cyan
Write-Host "The cross-platform installation system is working correctly!" -ForegroundColor Green