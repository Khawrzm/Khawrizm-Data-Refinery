#[tauri::command]
fn evaluate_xcv(formula: String) -> String {
    // استدعاء محرك TACO المكتوب بـ C++ لمعالجة الخلية بتعقيد O(1)
    let engine = xcv_engine::ffi::create_taco_engine();
    engine.evaluate_cell(&formula)
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
