# Host & Kernel Hardening

Run `scripts/hardening/apply-sysctl-hardening.sh` on every validator, signer, and build host to enforce the kernel parameters referenced in `crates/security_layers/tests/network_infra_security_validation.rs`. The script writes `/etc/sysctl.d/dex-os-hardening.conf`, reloads the values via `sysctl --system`, and locks down networking features such as rp_filter, redirect handling, and module loading.

For container hosts, combine these sysctl settings with the distroless images in `services/*/Dockerfile` (non-root user, read-only root filesystem) and keep audit evidence in `logs/host-hardening/`.
