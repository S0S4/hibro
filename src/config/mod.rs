pub mod sync;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use crate::path;

fn read_lines(file_path: String) -> Vec<std::string::String> {

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            lines.push(line)
        }
    }

    return lines;

}

pub fn blacklist() -> Vec<String> {

    let ips: Vec<String> = read_lines(path::blacklist_file());
    return ips;

}

pub fn whitelist() -> Vec<String> {

    let ips: Vec<String> = read_lines(path::whitelist_file());
    return ips;

}

pub fn sync_plugins() {

    let repos: Vec<String> = read_lines(path::sync_file());

    thread::spawn(|| {
        let _ = sync::sync(repos, path::sync_dir());
    });

}
