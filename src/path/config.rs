use super::home;
use std::env;
use std::path::PathBuf;

/// Return path where configuration is stored
pub fn path() -> String {

    let mut config_path = PathBuf::new();
    config_path.push(home::path().to_owned());

    if cfg!(target_os = "windows") {
        if let Ok(value) = env::var("APPDATA") {
            config_path = PathBuf::new();
            config_path.push(value);
        } else {
            config_path.push("AppData");
            config_path.push("Roaming");
            config_path.push("hibro");
        }
    } else {
        if let Ok(value) = env::var("XDG_CONFIG_HOME") {
            config_path = PathBuf::new();
            config_path.push(value);
        } else {
            config_path.push(".config");
            config_path.push("hibro");
        }
    }

    return config_path.to_string_lossy().into_owned();

}
