pub mod grid;
use grid::{XcvSheet, CellValue};

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("XCV_Engine/src/xcv_wrapper.h");
        type FormulaEngine;
        fn new_engine() -> UniquePtr<FormulaEngine>;
        fn evaluate_formula(self: &FormulaEngine, formula: &str) -> f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_initialization() {
        println!("\n[XCV Grid] 🚀 Creating new Sovereign Worksheet...");
        let mut sheet = XcvSheet::new("Financial_Report_Q1");
        
        sheet.set_cell("A1", CellValue::Number(50000.0));
        sheet.set_cell("B2", CellValue::Text("KSA_Sovereign_Fund".to_string()));
        sheet.set_cell("C3", CellValue::Formula("=SUM(A1:A10)".to_string()));

        assert_eq!(sheet.get_cell("A1"), CellValue::Number(50000.0));
        println!("[XCV Grid] ✅ Worksheet Memory allocated successfully.");
    }
}
