use std::fs;
#[tauri::command]
pub async fn browse_direcotries() -> Result<(), String> {
    let directories_iterator = fs::read_dir(".").expect("Unable to read current directory");
    let mut directories = Vec::new();
    let mut files = Vec::new();
    for dirtectory in directories_iterator {
        let dir = dirtectory.unwrap();
        if dir.path().is_dir() {
            directories.push(dir.file_name());
        } else {
            files.push(dir.file_name());
        }
    }
    Ok(())
}
