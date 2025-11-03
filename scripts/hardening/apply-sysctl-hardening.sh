#!/usr/bin/env bash
set -euo pipefail

TARGET_FILE=${1:-/etc/sysctl.d/dex-os-hardening.conf}

cat <<'CONF' | sudo tee "$TARGET_FILE" > /dev/null
kernel.kptr_restrict = 2
kernel.dmesg_restrict = 1
kernel.yama.ptrace_scope = 2
kernel.kexec_load_disabled = 1
kernel.randomize_va_space = 2
kernel.modules_disabled = 1
kernel.perf_event_paranoid = 3
kernel.unprivileged_bpf_disabled = 1
net.ipv4.conf.all.rp_filter = 1
net.ipv4.conf.default.rp_filter = 1
net.ipv4.conf.all.accept_redirects = 0
net.ipv4.conf.default.accept_redirects = 0
net.ipv4.conf.all.send_redirects = 0
net.ipv4.conf.default.send_redirects = 0
net.ipv4.conf.all.accept_source_route = 0
net.ipv4.conf.default.accept_source_route = 0
net.ipv6.conf.all.accept_redirects = 0
net.ipv6.conf.all.accept_source_route = 0
fs.protected_hardlinks = 1
fs.protected_symlinks = 1
CONF

sudo sysctl --system > /dev/null

echo "Applied hardened sysctl profile via $TARGET_FILE"
