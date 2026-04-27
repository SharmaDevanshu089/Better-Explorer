use windows::UI::ViewManagement::UIColorType;
use windows::UI::ViewManagement::UISettings;

#[tauri::command]
pub fn get_titlebar_color() {
    let settings = UISettings::new().unwrap();
    let color = settings.GetColorValue(UIColorType::Background).unwrap();
    println!("Titlebar color: {:?}", color);
}
