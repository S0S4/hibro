pub mod sync;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::path;
use once_cell::sync::Lazy;

static BLACKLIST: Lazy<Mutex<Vec<String>>> = Lazy::new(|| {
    let bl = Mutex::new(read_lines(path::blacklist_file()));
    bl
});


static WHITELIST: Lazy<Arc<Mutex<Vec<String>>>> = Lazy::new(|| {
    let wl = Arc::new(Mutex::new(read_lines(path::whitelist_file())));
    wl
});

fn read_lines(file_path: String) -> Vec<String> {

    let file = File::open(file_path).unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().filter_map(|l| l.ok()).collect();

    return lines;

}

pub fn blacklist(refresh: bool) -> Vec<String> {

    if refresh {
        let mut lines: Vec<String> = read_lines(path::blacklist_file());
        let mut lock = BLACKLIST.lock().unwrap();
        lock.clear();
        lock.append(&mut lines);
    }

    return BLACKLIST.lock().unwrap().to_vec();

}

pub fn whitelist(refresh: bool) -> Vec<String> {

    if refresh {
        let mut lines: Vec<String> = read_lines(path::blacklist_file());
        let mut lock = WHITELIST.lock().unwrap();
        lock.clear();
        lock.append(&mut lines);
    }

    return WHITELIST.lock().unwrap().to_vec();

}

pub fn sync_plugins() {

    let repos: Vec<String> = read_lines(path::sync_file());

    thread::spawn(|| {
        let _ = sync::sync(repos, path::sync_dir());
    });

}
