use std::fs;
use std::path::PathBuf;
use chrono;

/// save data creating the needed directory structure in the desired path
pub async fn save(path: String, ip: String, data: String) {

    let mut final_path = PathBuf::new();
    final_path.push(path.to_owned());
    final_path.push(ip.to_owned());

    // create path if it does not exist
    if let Ok(metadata) = fs::metadata(&path) {
        if !metadata.is_dir() {
            let _ = fs::create_dir_all(&path);
        }
    }

    // create final_path if it does not exist
    if let Ok(metadata) = fs::metadata(&final_path) {
        if !metadata.is_dir() {
            let _ = fs::create_dir_all(&final_path);
        }
    }

    if data.contains("fingerprint") {

        // create file path, and using it to create the fingerprint directory
        let mut file_path = PathBuf::new();
        file_path.push(final_path.to_owned());
        file_path.push("fingerprint");

        // create fingerprint dir if it does not exist
        if let Ok(metadata) = fs::metadata(&file_path) {
            if !metadata.is_dir() {
                let _ = fs::create_dir_all(&file_path);
            }
        }

        file_path.push(chrono::offset::Local::now().to_owned().to_rfc3339());
        let _ = fs::File::create(file_path);

    } else {

        let mut file_path = PathBuf::new();
        file_path.push(final_path.to_owned());
        file_path.push(chrono::offset::Local::now().to_owned().to_rfc3339());
        let _ = fs::File::create(file_path);

    }

}
