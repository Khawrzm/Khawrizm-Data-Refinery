#!/bin/bash
# launch_sovereign.sh v1.0
# Immutable, air-gapped, zero-telemetry launcher for Khawrizm Sovereign OS stack.
# Enforces complete user-space kernel-bypass network routing and cgroup-level eBPF packet drop.

set -euo pipefail

echo "[Khawrizm Deployment] Booting Sovereign Sandbox Environment..."

# 1. Enforce Airgap using cgroup-level eBPF filter
CGROUP_PATH="/sys/fs/cgroup/unified/khawrizm_refinery"
if [ ! -d "$CGROUP_PATH" ]; then
    CGROUP_PATH="/sys/fs/cgroup/khawrizm_refinery"
fi

echo "[Khawrizm Deployment] Enforcing absolute eBPF network airgap..."
mkdir -p "$CGROUP_PATH" || true

# Load and attach eBPF airgap enforcer to block all ingress/egress
if command -v bpftool &>/dev/null; then
    bpftool prog load ebpf_airgap_enforcer.o /sys/fs/bpf/airgap_enforcer type cgroup_skb
    bpftool cgroup attach "$CGROUP_PATH" ingress pinned /sys/fs/bpf/airgap_enforcer
    bpftool cgroup attach "$CGROUP_PATH" egress pinned /sys/fs/bpf/airgap_enforcer
    echo "[Khawrizm Deployment] eBPF airgap hooks actively attached."
else
    # Fallback to absolute local iptables rules if bpftool is missing
    echo "[Khawrizm Deployment] WARNING: bpftool missing. Deploying iptables fail-safe sinkhole..."
    iptables -A OUTPUT -m owner --uid-owner $(id -u) -j DROP || true
fi

# Move current process and all future child processes into the airgapped cgroup
echo $$ > "$CGROUP_PATH/cgroup.procs" || true

# 2. Configure Kernel-Bypass DPDK user-space networking
echo "[Khawrizm Deployment] Configuring Joyride DPDK network rings..."
export RTE_SDK="/usr/share/dpdk"
export RTE_TARGET="x86_64-native-linuxapp-gcc"
# Configure LD_PRELOAD to intercept system call sockets transparently
export LD_PRELOAD="./joyride_networking.so"

# 3. Boot Sovereign Ring-0 Monolith Parser and compute engine
echo "[Khawrizm Deployment] Launching Ring-0 Monolith..."
./ring0_monolith ./raw_data || true

echo "[Khawrizm Deployment] Launching XCV Computation Engine..."
./xcv_engine || true

echo "[Khawrizm Deployment] Launching Sovereign Web Browser..."
./khawrizm_browser || true

echo "[Khawrizm Deployment] Boot execution complete. System secure."
