---
name: Release Request
about: Request a new release of the project
title: 'Release: v0.0.0'
labels: 'release'
assignees: ''

---

## Release Request

### Version
<!-- Specify the version number for the release (e.g., v1.2.3) -->
v0.0.0

### Release Type
<!-- Select the type of release -->
- [ ] Major
- [ ] Minor
- [ ] Patch
- [ ] Pre-release

### Description
<!-- Describe what this release will include -->

### Changes Included
<!-- List the major changes included in this release -->
- [ ] Feature 1
- [ ] Bug fix 1
- [ ] Security update

### Checklist
<!-- Check all that apply -->
- [ ] Version numbers updated in all Cargo.toml files
- [ ] CHANGELOG.md updated
- [ ] Documentation updated
- [ ] All tests passing
- [ ] Code review completed

### Security & Infra Review
- Security owner: `@AttakDefand`
- Infra owner: `@AttakDefand` (acting infra lead until successor assigned)
- Review date: __ / __ / ____
- Link to meeting notes / decision log: ________
- Reference process: see `docs/HARDENING-REVIEW.md` and `docs/RELEASE-COMMS.md`

### Hardening Blueprint Checklist (see HARDENING-GUIDE.md)
- [ ] Governance & key custody verified (multisig/MPC, CODEOWNERS enforced)
- [ ] Secrets & configuration validated against `CREDENTIALS-AND-CONFIGURATION.md`
- [ ] Smart contracts: `forge test`, `forge coverage`, `slither`, `shadow-fork` suites passed
- [ ] Rust/services: `cargo fmt`, `cargo clippy`, `cargo test`, `cargo audit`, `cargo deny` clean
- [ ] Container & host baselines applied (distroless images, sysctl/AppArmor, `container_hardening` sims)
- [ ] Data/messaging layers hardened (Postgres/ClickHouse TLS, Redis ACL, IPFS pin coverage)
- [ ] Network/API controls confirmed (rate classes, mTLS, RPC failover)
- [ ] Observability dashboards updated & SLOs (`dapp_slos.yaml`) met
- [ ] Testing & simulations executed (`tests/security`, chaos, fuzz, E2E)
- [ ] Supply-chain artifacts published (SBOM + cosign signatures from CI run)
- [ ] Deployment runbooks/rollback plans reviewed (`GUIDELINE-ROLE-DAPP.MD`, `ERROR_HANDLING_SUMMARY.md`)
- [ ] Security owner approval recorded
- [ ] Infra owner approval recorded

### Additional Context
<!-- Add any other context about the release request here -->
