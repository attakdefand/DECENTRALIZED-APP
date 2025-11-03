# Repository Guidelines

## Project Structure & Module Organization
The workspace root (`Cargo.toml`) aggregates modular crates under `crates/` (core protocol, keeper agents, governance, security_layers) and domain-specific smart contracts in `contracts/` (Foundry layout with `src/`, `script/`, `test/`). Off-chain Rust services live in `services/`, while infrastructure manifests are under `infra/` and reusable automation sits in `scripts/`. Scenario and regression suites reside in `tests/`, grouped by domain (`security/`, `chaos/`, `fuzz/`, `e2e/`). Use these directories when adding new artifacts so ownership stays clear.

## Build, Test, and Development Commands
- `make build` / `cargo build --workspace`: compile every crate and shared binary.
- `make test` / `cargo test --workspace`: run the core Rust suite, including simulations in `tests/`.
- `make clippy` and `make fmt`: enforce linting and formatting before review.
- `forge test -vv` (run from `contracts/`): execute Solidity/CosmWasm simulations defined by Foundry.
- `scripts/run-automation-defense-tests.sh` and similar runners: execute domain-specific bundles; inspect each script for required env vars.
- `docker-compose up` (root): bring up local chain services defined in `docker-compose.yml` for integration trials.

## Coding Style & Naming Conventions
Rust code follows edition 2024 defaults: four-space indentation, snake_case modules, CamelCase types, and doc comments for public APIs. Run `cargo fmt --all` after edits and treat `cargo clippy --workspace -- -D warnings` as non-negotiable. Solidity and foundry scripts inherit the `contracts/remappings.txt` aliases; name scripts with the scenario suffix (`Upgradeability`, `ProofOfReserves`) to match existing patterns. Bash/PowerShell helpers in `scripts/` prefer kebab-case filenames and uppercase env vars.

## Testing Guidelines
Scope new tests alongside the feature: crate-level unit tests next to modules, cross-cutting simulations under the relevant `tests/*` domain, and Foundry specs in `contracts/test/`. Benchmark or stress cases belong in `tests/perf/`. Keep Rust test names descriptive (`*_simulation.rs`, `*_tests.rs`) to integrate with existing runners. Before opening a PR, run `cargo test --workspace`, the relevant `scripts/run-*.sh` bundle, and `forge test` if contracts are touched. Capture unusual coverage requirements or skipped cases in the PR notes.

## Commit & Pull Request Guidelines
Recent history favors concise, lower-case subjects (e.g., `governance_features`) that describe the change; stick to the imperative mood and limit to ~72 characters. Sign commits with your GPG key—`scripts/verify-signed-commits.ps1` guards the pipeline. For pull requests, link GitHub issues, describe affected subsystems, list validation commands, and attach artifacts (logs, screenshots) for UI or service changes. Small, focused PRs reviewing a single concern ship faster.

## Security & Configuration Tips
Secrets and API credentials are documented in `CREDENTIALS-AND-CONFIGURATION.md`; never check them into the repo. Validate security posture with the PowerShell/ shell scripts under `scripts/validate-*` before integration handoff. When composing new services, ensure they respect the hardening baselines captured in `infra/` and update `docker-compose.yml` if additional ports are exposed.
