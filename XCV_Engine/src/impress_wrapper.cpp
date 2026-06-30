#include "impress_wrapper.h"
#include <iostream>

void ImpressPresentation::load_presentation(rust::Str path) const {
    std::string p(path.data(), path.size());
    std::cout << "[Impress Core] Loading: " << p << std::endl;
}

void ImpressPresentation::save_presentation(rust::Str path) const {
    std::string p(path.data(), path.size());
    std::cout << "[Impress Core] Saving: " << p << std::endl;
}

rust::String ImpressPresentation::extract_slides_text() const {
    return rust::String("Sovereign Presentation Text Extract");
}

std::unique_ptr<ImpressPresentation> new_impress_presentation() {
    return std::make_unique<ImpressPresentation>();
}
