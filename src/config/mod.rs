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

pub fn blacklist(refresh: bool) -> Mutex<Vec<String>> {

    if refresh {
        let mut lines: Vec<String> = read_lines(path::blacklist_file());
        let mut lock = BLACKLIST.lock().unwrap();
        lock.clear();
        lock.append(&mut lines);
    }

    return BLACKLIST.lock().unwrap();

}

pub fn whitelist(refresh: bool) -> Arc<Mutex<Vec<String>>> {

    if refresh {
        let lines: Vec<String> = read_lines(path::blacklist_file());
        // check how to change vector capacity
        WHITELIST.clone().lock().unwrap().clear();
        for line in lines {
            WHITELIST.clone().lock().unwrap().push(line);
        }
    }

    return WHITELIST.clone();

}

pub fn sync_plugins() {

    let repos: Vec<String> = read_lines(path::sync_file());

    thread::spawn(|| {
        let _ = sync::sync(repos, path::sync_dir());
    });

}
