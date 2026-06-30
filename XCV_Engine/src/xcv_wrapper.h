#pragma once
#include "rust/cxx.h"
#include <memory>
#include <string>

// هذه الكلاس ستكون الواجهة التي تخاطب محرك LibreOffice لاحقاً
struct FormulaEngine {
    double evaluate_formula(rust::Str formula) const;
};

std::unique_ptr<FormulaEngine> new_engine();
