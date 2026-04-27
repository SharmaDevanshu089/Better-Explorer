mod browse;
mod initial_setup;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            browse::browse_direcotries,
            browse::update_current_directory,
            initial_setup::get_titlebar_color
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
