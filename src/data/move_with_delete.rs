use std::fs;

/// Move all files from one folder to another and remove the source directory
pub fn exec(source_dir: &str, dest_dir: &str) -> std::io::Result<()> {

    // Create the destination directory if it doesn't exist
    fs::create_dir_all(dest_dir)?;

    // Iterate over the files in the source directory
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();

        // If the entry is a file, copy it to the destination directory
        if path.is_file() {
            let dest_path = format!("{}/{}", dest_dir, path.file_name().unwrap().to_str().unwrap());
            fs::copy(&path, &dest_path)?;
        }
    }

    // remove source directory
    fs::remove_dir_all(source_dir)?;

    Ok(())

}
