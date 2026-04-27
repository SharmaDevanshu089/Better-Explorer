use windows::UI::ViewManagement::UISettings;

#[tauri::command]
pub fn get_titlebar_color() {
    let settings = UISettings::new().unwrap();
    let color = settings.GetColorValue(UIColorType::Accent).unwrap();
}
