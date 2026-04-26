use std::fs;
#[tauri::command]
pub async fn browse_direcotries() -> Result<(), String> {
    let directories = fs::read_dir(".").expect("Unable to read current directory");
    // println!("Directories in current directory: {:?}", directories);
    for q in directories {
        let dir = q.unwrap();
        println!("Directory: {:?}", dir.path());
    }
    Ok(())
}
