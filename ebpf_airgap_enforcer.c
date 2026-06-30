/* 
 * ebpf_airgap_enforcer.c v2.0
 * Pure-deterministic cgroup-level network airgap enforcer.
 * Hooks to cgroup/skb ingress and egress paths.
 * Unconditionally drops all packets for processes within the refinery's cgroup.
 * Zero external dependencies.
 */

#include <linux/bpf.h>

#ifndef SEC
#define SEC(NAME) __attribute__((section(NAME), used))
#endif

SEC("cgroup_skb/ingress")
int airgap_ingress(struct __sk_buff *skb)
{
    (void)skb;
    // Unconditional drop for ingress packets in the target cgroup
    return 0; // 0 in cgroup_skb means drop/reject the packet
}

SEC("cgroup_skb/egress")
int airgap_egress(struct __sk_buff *skb)
{
    (void)skb;
    // Unconditional drop for egress packets in the target cgroup
    return 0; // 0 in cgroup_skb means drop/reject the packet
}

char _license[] SEC("license") = "GPL";
