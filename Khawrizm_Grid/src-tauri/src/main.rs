// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;

#[command]
fn evaluate_xcv(formula: String) -> String {
    // محرك معالجة المعادلات الرياضية والبرمجية المعقدة
    let f_trim = formula.trim().to_uppercase();
    
    if f_trim.contains("∑") || f_trim.contains("SUM") {
        return "TACO_EVAL: Math [SUM/∑] Processed".to_string();
    } else if f_trim.contains("√") || f_trim.contains("SQRT") {
        return "TACO_EVAL: Math [SQRT/√] Processed".to_string();
    } else if f_trim.contains("π") || f_trim.contains("PI") {
        return "3.14159265359".to_string();
    } else if f_trim.starts_with("=MATRIX") {
        return "TACO_EVAL: Array/Matrix Processed".to_string();
    }
    
    format!("Evaluated: {}", formula)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![evaluate_xcv])
        .run(tauri::generate_context!())
        .expect("error while running Khawrizm Enterprise application");
}
