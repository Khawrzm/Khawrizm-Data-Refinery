#pragma once
#include <memory>
#include <string>
#include "rust/cxx.h"

struct ImpressPresentation {
    void load_presentation(rust::Str path) const;
    void save_presentation(rust::Str path) const;
    rust::String extract_slides_text() const;
};

std::unique_ptr<ImpressPresentation> new_impress_presentation();
