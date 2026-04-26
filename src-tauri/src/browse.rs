use std::fs;
#[tauri::command]
pub async fn browse_direcotries() -> Result<(), String> {
    let directories_iterator = fs::read_dir(".").expect("Unable to read current directory");
    let directories = Vec::new();
    let files = Vec::new();
    for dirtectory in directories_iterator {
        let dir = dirtectory.unwrap();
        if dir.path().is_dir() {
            directories.push(dir.file_name().into_string().unwrap());
        } else {
            files.push(dir.file_name().into_string().unwrap());
        }
    }
    let directories = Vec::new();
    Ok(())
}
