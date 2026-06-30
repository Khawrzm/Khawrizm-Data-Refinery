#pragma once
#include <memory>
#include <string>
#include "rust/cxx.h"

struct XcvSheet;

struct FormulaEngine {
    double evaluate_formula(rust::Str formula, const XcvSheet& sheet) const;
};

std::unique_ptr<FormulaEngine> new_engine();
