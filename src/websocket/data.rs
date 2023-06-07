use std::fs;
use std::io::Write;
use std::path::PathBuf;
use chrono;
use substring::Substring;

/// Save data creating the needed directory structure in the desired path.
/// If the data is in JSON format and contains a variable called "fingerprint", that value will be
/// used to create a directory to save the data
/// * Structure:
///
///   - data/
///     - timestamp (file if fingerprint is not present in data)
///     - fingerprint/
///       - timestamp (file)
pub fn save(path: String, ip: String, data: String) -> std::io::Result<()> {

    let mut final_path = PathBuf::new();
    final_path.push(path.to_owned());
    final_path.push(ip.to_owned());

    if data.contains("fingerprint") {

        // create file path, and using it to create the fingerprint directory
        let mut file_path = PathBuf::new();
        file_path.push(final_path.to_owned());

        // get fingerprint in data
        let start = data.find("fingerprint\": \"").unwrap_or(0);
        let end = data.find("\",").unwrap_or(data.len());
        let fingerprint = data.substring(start, end).split(": \"").last().unwrap().split("\"").take(1).last().unwrap();
        // let fingerprint = data.substring(start, end).split(": \"").last().unwrap().split("\"").first().unwrap();

        file_path.push(fingerprint);
        file_path.push(chrono::offset::Local::now().to_owned().format("%Y%m%d%H%M%S-%f").to_string());

        // create parent directories
        fs::create_dir_all(file_path.parent().unwrap()).unwrap();

        // create file with data
        let mut file = fs::File::create(file_path)?;
        file.write_all(data.as_bytes())?;

        Ok(())

    } else {

        let mut file_path = PathBuf::new();
        file_path.push(final_path.to_owned());
        file_path.push(chrono::offset::Local::now().to_owned().format("%Y%m%d%H%M%S-%f").to_string());

        // create parent directories
        fs::create_dir_all(file_path.parent().unwrap()).unwrap();

        // create file with data
        let mut file = fs::File::create(file_path)?;
        file.write_all(data.as_bytes())?;

        Ok(())

    }

}
