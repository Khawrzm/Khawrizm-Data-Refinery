#include "writer_wrapper.h"
#include <iostream>

void WriterDocument::load_document(rust::Str path) const {
    std::string p(path.data(), path.size());
    std::cout << "[Writer Core] Loading: " << p << std::endl;
}

void WriterDocument::save_document(rust::Str path) const {
    std::string p(path.data(), path.size());
    std::cout << "[Writer Core] Saving: " << p << std::endl;
}

rust::String WriterDocument::extract_text() const {
    return rust::String("Sovereign Document Text Extract");
}

std::unique_ptr<WriterDocument> new_writer_document() {
    return std::make_unique<WriterDocument>();
}
