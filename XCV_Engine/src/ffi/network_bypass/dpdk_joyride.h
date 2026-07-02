#pragma once
namespace Khawrizm {
namespace Network {
class JoyrideBypass {
public:
    static void joyride_sriov_init(int vf_index);
    static void intercept_sockets();
};
}
}
