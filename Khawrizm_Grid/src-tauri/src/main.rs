// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;

#[command]
fn evaluate_xcv(formula: String) -> String {
    // Advanced math and programming formula parsing bridge
    if formula.starts_with("=KHAWRIZM.STAT") {
        return "Advanced Statistical Analysis Executed".to_string();
    }
    format!("Evaluated: {}", formula)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![evaluate_xcv])
        .run(tauri::generate_context!())
        .expect("error while running Khawrizm Enterprise application");
}
