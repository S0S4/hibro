use fs;

/// save data creating the needed directory structure in the desired path
pub async fn save(path: String, ip: String, data: String) {

    // create dir_path if it does not exist
    if let Ok(metadata) = fs::metadata(&path) {
        if !metadata.is_dir() {
            let _ = fs::create_dir_all(&path);
        }
    }
    if let Ok(metadata) = fs::metadata(&path) {
        if !metadata.is_dir() {
            let _ = fs::create_dir_all(&path);
        }
    }

}
