use std::env;
use std::env::consts::OS;

/// Return $HOME or $USERPROFILE value
pub fn path() -> String {

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
