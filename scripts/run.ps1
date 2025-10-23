# Run script for the decentralized application CLI

Write-Host "Starting decentralized application CLI..." -ForegroundColor Green

# Run the CLI with help to show available commands
cargo run --bin dex -- --help

if ($LASTEXITCODE -eq 0) {
    Write-Host "CLI started successfully!" -ForegroundColor Green
} else {
    Write-Host "Failed to start CLI!" -ForegroundColor Red
    exit $LASTEXITCODE
}