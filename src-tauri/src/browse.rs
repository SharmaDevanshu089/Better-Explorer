use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct ReturningData {
    directories: Vec<PathBuf>,
    files: Vec<PathBuf>,
}

#[tauri::command]
pub async fn browse_direcotries() -> Result<ReturningData, String> {
    let directories_iterator = fs::read_dir(".").expect("Unable to read current directory");
    let mut directories = Vec::new();
    let mut files = Vec::new();
    for dirtectory in directories_iterator {
        let dir = dirtectory.unwrap();
        if dir.path().is_dir() {
            directories.push(PathBuf::from(dir.file_name()));
        } else {
            files.push(PathBuf::from(dir.file_name()));
        }
    }
    println!("Directories: {:?}", directories);
    println!("Files: {:?}", files);
    let returning_instance = ReturningData {
        directories: directories,
        files: files,
    };

    Ok(returning_instance)
}
