# Run safety tests script

Write-Host "Running safety tests..." -ForegroundColor Yellow

# Run all tests
Write-Host "Running all tests..." -ForegroundColor Green
forge test -vvv

# Run specific safety tests with gas reports
Write-Host "Running safety tests with gas reports..." -ForegroundColor Green
forge test --match-contract SafetyTests --gas-report

# Run invariant tests if any
Write-Host "Running invariant tests..." -ForegroundColor Green
forge test --match-test invariant

Write-Host "Safety tests completed!" -ForegroundColor Green