use dirs::home_dir;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct ReturningData {
    current_directory: PathBuf,
    directories: Vec<PathBuf>,
    files: Vec<PathBuf>,
}

#[tauri::command]
pub async fn browse_direcotries() -> Result<ReturningData, String> {
    let directories_iterator =
        fs::read_dir(home_dir().unwrap()).expect("Unable to read current directory");
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
    let returning_instance = ReturningData {
        directories: directories,
        files: files,
        current_directory: home_dir().unwrap(),
    };

    Ok(returning_instance)
}

#[tauri::command]
pub async fn update_current_directory(new_directory: PathBuf) -> Result<ReturningData, String> {
    println!("New directory: {:?}", new_directory);
    let directories_iterator =
        fs::read_dir(&new_directory).expect("Unable to read current directory");
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
    let returning_instance = ReturningData {
        directories: directories,
        files: files,
        current_directory: PathBuf::from(new_directory),
    };

    Ok(returning_instance)
}
