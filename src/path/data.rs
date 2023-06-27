use super::home;
use std::env;
use std::path::PathBuf;

/// Return path where data will be stored
pub fn path() -> String {

    let mut config_path = PathBuf::new();
    config_path.push(home::path().to_owned());

    if cfg!(target_os = "windows") {
        if let Ok(value) = env::var("LOCALAPPDATA") {
            config_path = PathBuf::new();
            config_path.push(value);
        } else {
            config_path.push("AppData");
            config_path.push("Local");
            config_path.push("hibro");
        }
    } else {
        if let Ok(value) = env::var("XDG_DATA_HOME") {
            config_path = PathBuf::new();
            config_path.push(value);
        } else {
            config_path.push(".local");
            config_path.push("share");
            config_path.push("hibro");
        }
    }

    return config_path.to_string_lossy().into_owned();

}
