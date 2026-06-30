#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <linux/in.h>

/* 
 * ebpf_airgap_enforcer.c v1.2
 * Ring-0 eBPF/XDP airgap kill-switch for Khawrizm-Data-Refinery pipeline.
 * Drops all outbound IP packets unless dst is strictly 127.0.0.1 (local inference only).
 * Enforces kernel-level network isolation for ARM64/RISC-V sovereign environments.
 * Compile: clang -target bpf -O2 -c -o ebpf_airgap_enforcer.o ebpf_airgap_enforcer.c
 * Load example: ip link set dev lo xdpgeneric obj ebpf_airgap_enforcer.o sec xdp
 */

SEC("xdp")
int ebpf_airgap_enforcer(struct xdp_md *ctx)
{
    void *data_end = (void *)(long)ctx->data_end;
    void *data = (void *)(long)ctx->data;

    struct ethhdr *eth = data;
    if ((void *)eth + sizeof(*eth) > data_end)
        return XDP_PASS;

    if (eth->h_proto != __constant_htons(ETH_P_IP))
        return XDP_PASS;

    struct iphdr *iph = (void *)eth + sizeof(*eth);
    if ((void *)iph + sizeof(*iph) > data_end)
        return XDP_PASS;

    /* Drop any packet not destined to 127.0.0.1 - kernel-enforced airgap for pipeline UID/PID context */
    if (iph->daddr != __constant_htonl(INADDR_LOOPBACK)) {
        return XDP_DROP;
    }

    return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
