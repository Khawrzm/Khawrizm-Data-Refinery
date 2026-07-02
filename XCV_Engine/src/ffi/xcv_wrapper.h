#pragma once
#include "rust/cxx.h"
#include <memory>
#include <string>

struct TacoEngine {
    rust::String evaluate_cell(rust::Str formula) const;
};

std::unique_ptr<TacoEngine> create_taco_engine();
