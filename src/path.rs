use std::fs;
use std::env;
use std::env::consts::OS;
use std::path::PathBuf;

fn home() -> String {

    if "linux" == OS
        || "macos" == OS
            || "android" == OS
            || "ios" == OS
            {
                return env::var_os("HOME").unwrap().to_str().unwrap().to_string()
            }

    if "windows" == OS {
        return env::var_os("USERPROFILE").unwrap().to_str().unwrap().to_string()
    }

    return env::current_dir().unwrap().to_str().unwrap().to_string();

}

/// Return path where configuration is stored
/// * **return**: String
fn config() -> String {

    let mut config_path = PathBuf::new();
    config_path.push(home().to_owned());

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

/// Return path where data will be stored
/// * **return**: String
fn data() -> String {

    let mut config_path = PathBuf::new();
    config_path.push(home().to_owned());

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

/// Return config file path where users configuration is stored
pub fn config_file() -> String {
    let mut config_path = PathBuf::new();

    config_path.push(config());
    config_path.push("config");

    return config_path.to_string_lossy().into_owned();

}

/// Return sync path where synced repos will be stored
pub fn sync() -> String {

    let mut sync_path = PathBuf::new();

    sync_path.push(data());
    sync_path.push("sync");

    return sync_path.to_string_lossy().into_owned();

}

/// Return path to store connections data
pub fn connections() -> String {

    let mut sync_path = PathBuf::new();

    sync_path.push(data());
    sync_path.push("connections");

    return sync_path.to_string_lossy().into_owned();

}

/// Return path where data from the current session will be stored
/// * **return**: String
pub fn runtime() -> String {

    let mut config_path = PathBuf::new();

    if cfg!(target_os = "windows") {
        if let Ok(value) = env::var("TEMP") {
            config_path.push(value);
        } else {
            config_path.push(home().to_owned());
            config_path.push("AppData");
            config_path.push("Local");
            config_path.push("Temp");
            config_path.push("hibro");
        }
    } else {
        if let Ok(value) = env::var("XDG_RUNTIME_DIR") {
            config_path.push(value);
            config_path.push("hibro");
        } else {
            if let Ok(value) = env::var("ROOTDIR") {
                config_path.push(value);
            } else {
                config_path.push("/");
            }
            config_path.push("run");
            config_path.push("usr");
            if let Ok(uid) = env::var("UID") {
                config_path.push(uid)
            } else {
                config_path.push("1000")
            }
            config_path.push("hibro");
        }
    }

    return config_path.to_string_lossy().into_owned();

}

/// Create all data paths if they do not exist'
/// * **usage**: `tokio::task::spawn(create());`
pub async fn create() {
    let _ = fs::create_dir_all(sync());
}
