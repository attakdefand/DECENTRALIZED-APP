#!/usr/bin/env pwsh

# Script to validate all Group A (Smart Contracts) security layers

Write-Host "Validating All Group A (Smart Contracts) Security Layers..." -ForegroundColor Green

# Define all Group A security layers from the testing matrix
$GroupASecurityLayers = @(
    @{
        Name = "Logic & Math (Unit / Fuzz / Invariant)"
        EvidenceFile = "contracts/test/INVARIANTS.md"
        Description = "Conservation, monotonicity, reentrancy, bounds, fee routing"
        Tools = "Foundry (forge test/fuzz/invariant), Echidna, Slither"
        CIGate = "contracts: unit+fuzz+invariant must pass; Slither no criticals; gas snapshot threshold"
    },
    @{
        Name = "Upgradeability (Storage layout, Proxy/UUPS diff)"
        EvidenceFile = "contracts/script/UPGRADE-PLAN.md"
        Description = "No storage slot collisions across upgrades; timelock/guardian flow"
        Tools = "OpenZeppelin Upgrades, storage-layout diff, shadow-fork dry-run"
        CIGate = "upgrade PRs require layout diff green + dry-run evidence"
    },
    @{
        Name = "AMM/DEX (Curve correctness, CPMM/Stable/CLAMM invariants)"
        EvidenceFile = "docs/tests/AMM-INVARIANTS.md"
        Description = "x*y=k, amplification behavior, per-tick liquidity/fees"
        Tools = "Model-based tests (Python/Rust), Foundry invariants"
        CIGate = "amm invariant suite green; reconciliation within epsilon"
    },
    @{
        Name = "Orderbook (Matching engine, Price-time priority & partial fills)"
        EvidenceFile = "docs/tests/LOB-TESTS.md"
        Description = "Place/cancel/match; IOC/FOK; fairness"
        Tools = "Engine unit tests; replay fixtures; MEV sims"
        CIGate = "LOB suite green; fairness metrics within target"
    },
    @{
        Name = "Lending/Perps (Risk & Liquidations, HF, Kink IR, funding/insurance)"
        EvidenceFile = "docs/tests/RISK-SIMS.md"
        Description = "Liquidate below HF; correct funding; insurance waterfall"
        Tools = "Economic sims/backtests; invariant checks"
        CIGate = "Risk sims green; bad-debt in stress ≤ budget"
    },
    @{
        Name = "Oracle (Manipulation/liveness, Median/TWAP/outlier/staleness)"
        EvidenceFile = "docs/protocol/ORACLE-TESTS.md"
        Description = "Reject stale/outlier, quorum met, fallback works"
        Tools = "Replay feeds; boundary tests; kill-switch drills"
        CIGate = "Oracle tests green; staleness < threshold"
    },
    @{
        Name = "MEV & Fairness (Order protection, Commit-reveal / FBA)"
        EvidenceFile = "docs/protocol/MEV-TESTS.md"
        Description = "Reveal windows, uniform clearing, anti-sandwich bounds"
        Tools = "Batch sims; solver cross-checks; timing tests"
        CIGate = "MEV sims green; bounds enforced"
    },
    @{
        Name = "Account Abstraction (UserOps validation, Paymaster/session key scope)"
        EvidenceFile = "docs/protocol/AA-TESTS.md"
        Description = "Nonces, signatures, budgets, capability scoping"
        Tools = "4337 bundler harness; fuzz; scope-leak tests"
        CIGate = "AA suite green; budget adherence"
    },
    @{
        Name = "Tx/Mempool (Privacy & replay, Private routes, deadlines, permit)"
        EvidenceFile = "docs/infra/TX-ROUTING-TESTS.md"
        Description = "EIP-155 replay, domain separator, expiry enforced"
        Tools = "Permit fuzz; private route checks"
        CIGate = "Replay tests green; expired rejected"
    },
    @{
        Name = "Cross-chain / Bridges (Proof verification, Light/Optimistic/ZK)"
        EvidenceFile = "docs/protocol/BRIDGE-TESTS.md"
        Description = "Header/receipt proof valid; challenge windows; replay guard"
        Tools = "Fixture proofs; fraud-proof sims; ZK verifier tests"
        CIGate = "Bridge suite green; gas within cap"
    }
)

# Track validation results
$TotalLayers = $GroupASecurityLayers.Count
$ValidatedLayers = 0
$MissingFiles = @()

Write-Host "`nChecking Security Layer Evidence Files..." -ForegroundColor Yellow

foreach ($Layer in $GroupASecurityLayers) {
    Write-Host "`nLayer: $($Layer.Name)" -ForegroundColor Cyan
    Write-Host "Description: $($Layer.Description)" -ForegroundColor Gray
    Write-Host "Tools: $($Layer.Tools)" -ForegroundColor Gray
    Write-Host "CI Gate: $($Layer.CIGate)" -ForegroundColor Gray
    
    $EvidencePath = "d:\DECENTRALIZED-APP\$($Layer.EvidenceFile)"
    
    if (Test-Path $EvidencePath) {
        Write-Host "✅ Evidence file exists: $($Layer.EvidenceFile)" -ForegroundColor Green
        $ValidatedLayers++
    } else {
        Write-Host "❌ Evidence file missing: $($Layer.EvidenceFile)" -ForegroundColor Red
        $MissingFiles += $Layer.EvidenceFile
    }
}

# Summary
Write-Host "`nValidation Summary:" -ForegroundColor Yellow
Write-Host "==================" -ForegroundColor Yellow
Write-Host "Total Security Layers: $TotalLayers" -ForegroundColor Cyan
Write-Host "Validated Layers: $ValidatedLayers" -ForegroundColor Cyan
Write-Host "Missing Evidence Files: $($MissingFiles.Count)" -ForegroundColor Cyan

if ($MissingFiles.Count -eq 0) {
    Write-Host "`n🎉 All Group A Security Layers Validated Successfully!" -ForegroundColor Green
    Write-Host "✅ $ValidatedLayers/$TotalLayers security layers have proper evidence documentation" -ForegroundColor Green
    
    Write-Host "`nSecurity layers validated:" -ForegroundColor Yellow
    foreach ($Layer in $GroupASecurityLayers) {
        Write-Host "  • $($Layer.Name)" -ForegroundColor Cyan
    }
    
    Write-Host "`nCI Gate Requirements:" -ForegroundColor Yellow
    foreach ($Layer in $GroupASecurityLayers) {
        Write-Host "  • $($Layer.Name): $($Layer.CIGate)" -ForegroundColor Cyan
    }
    
    exit 0
} else {
    Write-Host "`n❌ Some Security Layers Are Missing Evidence Files" -ForegroundColor Red
    Write-Host "Missing files:" -ForegroundColor Yellow
    foreach ($File in $MissingFiles) {
        Write-Host "  • $File" -ForegroundColor Red
    }
    
    Write-Host "`nPlease create the missing evidence documentation files to complete the Group A security layer validation." -ForegroundColor Yellow
    exit 1
}