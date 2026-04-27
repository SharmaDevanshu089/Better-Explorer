mod browse;
mod initial_setup;
use tauri::Manager;
use window_vibrancy::apply_mica;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "windows")]
            {
                apply_mica(&window, None).expect("Failed to apply acrylic effect");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            browse::browse_direcotries,
            browse::update_current_directory,
            initial_setup::get_titlebar_color
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
