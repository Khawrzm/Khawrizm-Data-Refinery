pub mod grid;
pub mod lexer;
pub mod taco;

pub use grid::XcvSheet;

#[cxx::bridge]
pub mod ffi {
    extern "Rust" {
        type XcvSheet;
        fn get_cell_value(self: &XcvSheet, ref_id: &str) -> f64;
        fn get_cell_formula(self: &XcvSheet, ref_id: &str) -> String;
    }

    unsafe extern "C++" {
        include!("xcv_wrapper.h");
        type FormulaEngine;
        fn new_engine() -> UniquePtr<FormulaEngine>;
        fn evaluate_formula(self: &FormulaEngine, formula: &str, sheet: &XcvSheet) -> f64;

        include!("writer_wrapper.h");
        type WriterDocument;
        fn new_writer_document() -> UniquePtr<WriterDocument>;
        fn load_document(self: &WriterDocument, path: &str);
        fn save_document(self: &WriterDocument, path: &str);
        fn extract_text(self: &WriterDocument) -> String;

        include!("impress_wrapper.h");
        type ImpressPresentation;
        fn new_impress_presentation() -> UniquePtr<ImpressPresentation>;
        fn load_presentation(self: &ImpressPresentation, path: &str);
        fn save_presentation(self: &ImpressPresentation, path: &str);
        fn extract_slides_text(self: &ImpressPresentation) -> String;
    }
}
