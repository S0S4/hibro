use std::fs;
use std::io::Write;
use std::path::PathBuf;
use chrono;

/// Save data creating the needed directory structure in the desired path.
/// A fingerprint is always expected, if there is not fingerprint,
/// data will be stored in root of IP direcotry
/// * Structure:
///
///   - data/
///     - timestamp (file if fingerprint is not present in data)
///     - fingerprint/
///       - timestamp (file)
pub fn save(path: String, ip: String, data: String, fingerprint: String) -> std::io::Result<()> {

    let mut final_path = PathBuf::new();
    final_path.push(path.to_owned());
    final_path.push(ip.to_owned());


    // create file path, and using it to create the fingerprint directory
    let mut file_path = PathBuf::new();
    file_path.push(final_path.to_owned());

    if fingerprint != "" {
        file_path.push(fingerprint);
    }
    file_path.push(chrono::offset::Local::now().to_owned().format("%Y%m%d%H%M%S-%f").to_string());

    // create parent directories
    fs::create_dir_all(file_path.parent().unwrap()).unwrap();

    // create file with data
    let mut file = fs::File::create(file_path)?;
    file.write_all(data.as_bytes())?;

    Ok(())

}

/// Move all files from one folder to another and remove the source directory
pub fn move_files(source_dir: &str, dest_dir: &str) -> std::io::Result<()> {

    println!("yes");

    // Create the destination directory if it doesn't exist
    fs::create_dir_all(dest_dir)?;

    println!("yes");

    let source_path = PathBuf::from(source_dir);

    // let mut source_path = PathBuf::new();
    // source_path.push(source_dir);

    // Iterate over the files in the source directory
    for entry in fs::read_dir(source_path)? {
        println!("yes");
        let entry = entry?;
        let path = entry.path();

        // If the entry is a file, copy it to the destination directory
        if path.is_file() {
            let dest_path = format!("{}/{}", dest_dir, path.file_name().unwrap().to_str().unwrap());
            fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())

}
