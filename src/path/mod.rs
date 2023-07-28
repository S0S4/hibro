mod home;
mod config;
mod data;

use std::{fs,env};
use std::path::PathBuf;

/// Return blacklist file path
pub fn blacklist_file() -> String {

    let mut config_path = PathBuf::new();

    config_path.push(config::path());
    config_path.push("blacklist");

    return config_path.to_string_lossy().into_owned();

}

/// Return whitelist file path
pub fn whitelist_file() -> String {

    let mut config_path = PathBuf::new();

    config_path.push(config::path());
    config_path.push("whitelist");

    return config_path.to_string_lossy().into_owned();

}

/// Return sync file path
pub fn sync_file() -> String {

    let mut config_path = PathBuf::new();

    config_path.push(config::path());
    config_path.push("sync");

    return config_path.to_string_lossy().into_owned();

}

/// Return sync path where synced repos will be stored
pub fn sync_dir() -> String {

    let mut sync_path = PathBuf::new();

    sync_path.push(data::path());
    sync_path.push("sync");

    return sync_path.to_string_lossy().into_owned();

}

/// Return path to store connections data
pub fn connections_dir() -> String {

    let mut sync_path = PathBuf::new();

    sync_path.push(data::path());
    sync_path.push("connections");

    return sync_path.to_string_lossy().into_owned();

}

/// Return path where data from the current session will be stored
/// * **return**: String
pub fn runtime_path() -> String {

    let mut config_path = PathBuf::new();

    if cfg!(target_os = "windows") {
        if let Ok(value) = env::var("TEMP") {
            config_path.push(value);
        } else {
            config_path.push(home::path().to_owned());
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
/// * **usage**: `thread::spawn(create());`
pub fn create() {
    let _ = fs::create_dir_all(config::path());
    let _ = fs::create_dir_all(config::path());
    let _ = fs::create_dir_all(sync_dir());
    let _ = fs::create_dir_all(runtime_path());
    let _ = fs::create_dir_all(connections_dir());
    let _ = fs::OpenOptions::new().create_new(true).write(true).open(whitelist_file());
    let _ = fs::OpenOptions::new().create_new(true).write(true).open(blacklist_file());
    let _ = fs::OpenOptions::new().create_new(true).write(true).open(sync_file());
}

