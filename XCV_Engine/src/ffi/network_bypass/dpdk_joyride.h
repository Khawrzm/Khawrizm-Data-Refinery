#pragma once
#include <stdint.h>
#include <cstddef>

// Pillar 03: DPDK Kernel-Bypass & SR-IOV Isolation
// Severs the Linux TCP/IP stack, bridging directly to raw virtual rings.
namespace Khawrizm {
namespace Network {

class JoyrideBypass {
public:
    // Initialize Single Root I/O Virtualization (SR-IOV)
    static void joyride_sriov_init(int vf_index);
    
    // Intercept standard POSIX sockets to prevent OS telemetry leaks
    static void intercept_sockets();
};

} // namespace Network
} // namespace Khawrizm
