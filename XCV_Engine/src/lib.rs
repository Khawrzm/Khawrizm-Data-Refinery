pub mod grid;
pub mod lexer;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("xcv_wrapper.h");
        
        type TacoEngine;
        fn create_taco_engine() -> UniquePtr<TacoEngine>;
        fn evaluate_cell(self: &TacoEngine, formula: &str) -> String;
    }
}
