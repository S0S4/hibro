use std::io::Write;
use std::path::PathBuf;
use std::fs;
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
pub fn exec(path: String, ip: String, data: String, fingerprint: String) -> std::io::Result<()> {

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
