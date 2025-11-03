# DECENTRALIZED-APP Hardening Blueprint

This playbook unifies on-chain, off-chain, and infrastructure defenses for `DECENTRALIZED-APP`. Use it as the authoritative baseline before any deployment, audit, or compliance handoff.

## 1. Scope & Objectives
- **Surface**: smart contracts under `contracts/`, Rust crates in `crates/` and `services/`, infra manifests in `infra/`, UI in `ui/`, and automation in `scripts/` and `tests/`.
- **Goal**: ship only builds that pass reproducible security checks, run with least privilege, and expose continuous signals (Prometheus, Grafana, logs) mapped to `dapp_slos.yaml`.
- **Process tenets**: key custody before code, immutable artifacts signed on release, and automated evidence for every control.

## 2. Governance, Identity & Secrets
1. **Key management**
   - Enforce multisig/MPC for on-chain owners; map signers per module (`contracts/governance`, `contracts/amm`).
   - Git commits already require GPG (see `CREDENTIALS-AND-CONFIGURATION.md`); add automated verification in CI (`scripts/verify-signed-commits.*`).
2. **Access control**
   - Introduce `.github/CODEOWNERS` aligned to crates/services to gate PR reviews.
   - Mirror RBAC in runtime via OPA/Cedar policies stored under `infra/policies/`; enable admission controls before applying `infra/k8s` manifests.
   - Keep `docs/security/vendor-metrics.json` updated (see `docs/security/vendor-automation.md`) so the vendor enforcement workflows have a single, auditable source of truth.
3. **Secrets**
   - Never commit `.env`; source runtime secrets from Vault/SOPS, referencing the variables listed in `CREDENTIALS-AND-CONFIGURATION.md`.
   - Rotate Postgres/Redis/IPFS tokens quarterly; enforce short-lived JSON Web Tokens for the `services/api-rs` gateway.

## 3. Smart Contract Hardening (`contracts/`)
1. **Baseline patterns**
   - Apply CEI, `ReentrancyGuard`, pausability, emergency withdraw, and governance timelocks as laid out in `PAUSE-README.md` and `UPGRADEABILITY-README.md`.
   - Maintain invariant documentation per module (`MATH-SAFETY-README.md`, `LOGIC-PATTERNS-README.md`).
2. **Testing pipeline**
   - Mandatory commands: `forge fmt`, `forge test -vv --ffi`, `forge coverage`, `forge snapshot`, `slither .`, `pnpm hardhat storage-layout` (for proxy safety).
   - For cross-chain adapters run `foundryup` (in `foundry/`) and `scripts/shadow-fork-simulations/*` before deployment.
3. **Automated analysis**
   - Schedule nightly fuzz (`forge fuzz`) and invariant jobs targeting `contracts/test/invariants/`.
   - Integrate Slither + Echidna + Halmos/Narnia into CI; export SARIF for GitHub Security tab.
4. **Deployment gating**
   - No `forge script deploy` until: audits resolved, `shadow-fork` tests green, gas deltas reviewed, and storage layouts hashed.
   - Tag immutable release artifacts (bytecode, ABI, config) and sign with `cosign`.

## 4. Rust Workspace & Services (`crates/`, `services/`)
1. **Compilation profile**
   - Enforce `cargo fmt --all`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace`, `cargo nextest run`, `cargo audit`, and `cargo deny check` in CI.
   - Turn on `panic = "abort"` for release bins that handle secrets (e.g., `services/keepers-rs`).
2. **Secure coding**
   - Wrap external RPC interactions (ethers, solana, cosmwasm) with typed clients and rate limiters; log raw payloads to tamper-evident sinks.
   - Use `secrecy::Secret` or `ring` for key material; avoid storing secrets in logs.
3. **Microservice boundaries**
   - `services/api-rs`: require mTLS between API ↔ indexer, enforce GraphQL depth limits, and apply WAF rules for REST endpoints.
   - `services/indexer-rs`: validate chain heights with quorum RPCs and detect reorgs by comparing `indexer_lag_blocks` from `dapp_slos.yaml`.
   - `services/mev-monitor`: forward detections to PagerDuty/OnCall and persist incidents in append-only storage.
4. **Binary integrity**
   - Produce SBOMs via `cargo auditable` and sign binaries using `cosign sign-blob`. Store in `release.json` manifest.

## 5. UI & Desktop Clients (`ui/`)
- Harden CSP/COOP/COEP headers using Tauri/Yew configs; disallow eval, inline JS.
- Implement wallet provider allowlist (EIP-6963) and deny auto-switch without consent.
- Run `npm audit`, `pnpm audit`, or `yarn audit` depending on the package manager and block high severity.
- Automate Lighthouse/axe accessibility/security regression in CI for `ui/pwa` and `ui/tauri`.

## 6. Container & Host Hardening
1. **Docker images**
   - Convert each `services/*/Dockerfile` to multi-stage builds that finish on `gcr.io/distroless/cc-debian12:nonroot`, copy binaries under `/app/bin`, and run as the built-in `nonroot` user with a read-only root filesystem.
   - Pin digests for postgres/redis/ipfs/prometheus/grafana in `docker-compose.yml`; scan via `trivy` pre-merge.
2. **Kernel/host**
   - Apply the sysctl set described in `crates/security_layers/tests/network_infra_security_validation.rs` (ASLR, rp_filter, modules_disabled) via `scripts/hardening/apply-sysctl-hardening.sh` (writes `/etc/sysctl.d/dex-os-hardening.conf`).
   - Enforce SELinux/AppArmor profiles and log via `crates/core/src/bin/container_hardening_simulation.rs` outputs.
3. **Kubernetes**
   - For `infra/k8s`, require PodSecurity `restricted`, enable NetworkPolicies, inject Istio/Linkerd mTLS, and use Kyverno/OPA for policy guardrails.

## 7. Data & Messaging Layers
- **Postgres / ClickHouse**: enable TLS, SCRAM auth, logical replication slots for audit, and nightly `pg_dump`/`CLICKHOUSE_BACKUP` with immutable retention.
- **Redis**: disable `FLUSH*`, require ACLs, and run in "protected mode" off localhost in prod.
- **NATS**: enforce nkey creds and JetStream limits.
- **IPFS**: pin critical assets via `services/ipfs-rs`, run dedicated gateways, and monitor pin coverage (`ipfs_pin_coverage_percent`).

## 8. Network, API, and Edge
- Terminate TLS 1.3 with mutual auth at ingress; prefer Cloudflare Spectrum or Envoy filters with WASM-based threat intel.
- Introduce rate classes defined in `infra/policies/rate-classes.yaml` and enforce through API gateway + chain RPC proxies.
- Maintain multiple RPC providers; automatic failover with health probes ensures order integrity and prevents stuck transactions.
- For cross-chain bridges, run watcher/challenger agents from separate trust domains and record proofs in append-only logs.

## 9. Observability & Detection
- Adopt OpenTelemetry for all Rust services; export traces to Tempo/Jaeger, metrics to Prometheus, logs to Loki.
- Align dashboards with `dapp_slos.yaml` (latency, error rate, indexer lag, IPFS coverage, MEV incidents).
- Add anomaly detection for key metrics (swap volume cliffs, price deviation) inside `services/security-monitor`.
- Mirror validator/keeper logs into tamper-evident buckets (write-once storage) for forensics.

## 10. Testing & Simulation Strategy (`tests/`, `foundry/`, `GROUP-A/B`)
- **Unit/Integration**: `cargo test --workspace`, `tests/security/*`, `tests/fuzz/*`.
- **Property/Fuzz**: leverage `proptest`, `quickcheck`, and Foundry fuzzers; ensure reproducibility using seeds captured in CI logs.
- **Chaos**: run scenarios in `tests/chaos/` plus `group_b_infra_security_tests.exe` for Windows hardening.
- **Shadow forks & load**: execute `contracts/shadow-fork-simulations` and `foundry` scenarios against forked mainnet/testnet snapshots.
- **UI/E2E**: maintain Cypress/Playwright tests under `tests/e2e/` with wallet automation.

## 11. CI/CD & Release Pipeline
1. **Pipelines**
   - Stage 1: lint/format (`make check`), unit tests, `forge test`.
   - Stage 2: security scans (`cargo audit`, `cargo deny`, `aquasecurity/trivy`, `slither`, `codeql` per `codeql-config.yml`, custom queries in `codeql-custom-queries-rust/`).
   - Stage 3: artifact build, SBOM + attestations, `cosign` signing, provenance attestation (`in-toto` or `slsa-verifier`).
2. **Policies**
   - Require branch protection, mandatory reviewers, passing status checks, and drift detection vs. `release.json`.
   - Store CI secrets in OIDC-backed vault (no long-lived tokens); use workload identity to fetch RPC keys.

## 12. Deployment & Operations Checklist
- Run `docker-compose config` or `kustomize build infra/k8s` with policy validation before apply.
- Confirm config drift via `infra/offline` scripts for air-gapped signing nodes.
- Document change windows in `logs/` and keep rollback plans for each component (contracts via upgrade proxy, services via blue/green, infra via Argo Rollouts).
- Ensure incident runbooks (`GUIDELINE-ROLE-DAPP.MD`, `ERROR_HANDLING_SUMMARY.md`, `DATABASE_DEBUGGING_TESTING_ENHANCEMENTS_SUMMARY.md`) are updated with pager rotations and contact trees.

## 13. Continuous Improvement
- Quarterly threat modeling workshop per subsystem (contracts, services, infra) capturing findings in `GROUP-A-TESTING-DOCS`.
- Bug bounty scope document referencing hardened artifacts and monitoring hooks.
- Annual chaos game-days validating pause/timelock flows, RPC isolation, and data restoration drills.

Adopting the above makes every delivery auditable, reproducible, and defensible. Treat this document as a living baseline—update it whenever new modules land or when external reviews (audits, pen tests) uncover gaps.
