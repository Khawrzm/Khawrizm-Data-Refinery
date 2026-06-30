#pragma once
#include <memory>
#include <string>
#include "rust/cxx.h"

struct WriterDocument {
    void load_document(rust::Str path) const;
    void save_document(rust::Str path) const;
    rust::String extract_text() const;
};

std::unique_ptr<WriterDocument> new_writer_document();
