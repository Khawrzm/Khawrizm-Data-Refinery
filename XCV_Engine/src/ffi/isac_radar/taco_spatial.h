#pragma once
#include <stdint.h>
#include <iostream>

namespace Khawrizm {
namespace Radar {
class SpatialISAC {
public:
    static void joyride_csi_extract(const uint8_t *frame);
};
}
}
