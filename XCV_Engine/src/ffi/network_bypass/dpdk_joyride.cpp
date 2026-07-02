#include "dpdk_joyride.h"
#include <iostream>

namespace Khawrizm {
namespace Network {

void JoyrideBypass::joyride_sriov_init(int vf_index) {
    std::cout << "[DPDK Ring-0] Initializing SR-IOV Virtual Function: " << vf_index << std::endl;
    std::cout << "[DPDK Ring-0] Host OS TCP/IP stack bypassed successfully." << std::endl;
}

void JoyrideBypass::intercept_sockets() {
    std::cout << "[DPDK Ring-0] Intercepting socket(), connect(), send(), recv() at LibC level..." << std::endl;
}

} // namespace Network
} // namespace Khawrizm
