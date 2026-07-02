#[tauri::command]
fn evaluate_xcv(formula: String) -> String {
    // This is the IPC entry point. Later, it will route to XCV_Engine C++ bindings.
    format!("XCV_RESULT: Processed '{}' via Ring-0 Engine", formula)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![evaluate_xcv])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
