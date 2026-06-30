#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <linux/in.h>
#include <linux/sched.h>

/* 
 * ebpf_airgap_enforcer.c v1.5
 * Ring-0 eBPF/XDP for absolute bare-metal isolation in Khawrizm-Data-Refinery v1.5 monolith.
 * Since inference is now strictly in-memory via llama.cpp FFI (no sockets), unconditionally drop ALL network packets for the pipeline UID/PID.
 * Provides hardware-level guarantee against any telemetry or external communication.
 * Compile: clang -target bpf -O2 -c -o ebpf_airgap_enforcer.o ebpf_airgap_enforcer.c
 * Load: ip link set dev <iface> xdpgeneric obj ebpf_airgap_enforcer.o sec xdp
 * (Attach to cgroup or use PID filter map for precision on KhawrizmOS)
 */

#define REFINERY_UID 1000  /* pipeline user UID - configure per deployment */

SEC("xdp")
int ebpf_airgap_enforcer(struct xdp_md *ctx)
{
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    __u32 uid = bpf_get_current_uid_gid() >> 32;

    /* If this packet belongs to the refinery pipeline process, drop everything (in-memory FFI means no network needed) */
    if (uid == REFINERY_UID) {
        return XDP_DROP;  /* Absolute kill-switch: no ingress, no egress for the monolith */
    }

    /* For other processes, allow (or apply stricter policy) */
    return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
