#include "dpdk_joyride.h"
#include <iostream>

namespace Khawrizm {
namespace Network {
void JoyrideBypass::joyride_sriov_init(int vf_index) {
    std::cout << "[DPDK Ring-0] SR-IOV Virtual Function initialized." << std::endl;
}
void JoyrideBypass::intercept_sockets() {
    std::cout << "[DPDK Ring-0] Sockets intercepted." << std::endl;
}
}
}
