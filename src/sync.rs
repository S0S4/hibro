use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio::task;
use git2::Repository;

/// Clone a repository into the given path
async fn clone_repo(url: &str, dir_path: &str) {

    let repo = match Repository::clone(url, dir_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    // let output = std::process::Command::new("git")
    //     .arg("clone")
    //     .arg(url)
    //     .arg(dir_path)
    //     .output()
    //     .expect("failed to execute process");

    // if !output.status.success() {
    //     panic!("failed to clone repository {}", url);
    // }
}

/// Example:
///
/// ```
/// let urls = vec![
///     String::from("https://github.com/rust-lang/rust.git"),
///     String::from("https://github.com/tensorflow/tensorflow.git"),
/// ];
/// let target_dir = "/home/a/";
///
/// tokio::runtime::Runtime::new().unwrap().block_on(clone_repos(&urls, &target_dir));
/// ```
async fn clone_repos(urls: &[String], dir_path: &str) {
    let mut handles = Vec::new();
    for url in urls {
        let dir_path = dir_path.to_owned();
        let url = url.clone();
        handles.push(task::spawn(async move {
            clone_repo(&url, &dir_path).await;
        }));
    }
    for handle in handles {
        handle.await.expect("failed to clone repository");
    }
}

/// Read lines from the given file and return the ones that starts with `sync=` without that part
fn read_sync_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut sync_lines = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("sync=") {
                sync_lines.push(line["sync=".len()..].to_string());
            }
        }
    }

    return sync_lines
}

/// Sync repositories from the config file
pub fn sync(config_file_path: &str) {
    let sync_lines: Vec<String> = read_sync_lines(config_file_path);
    tokio::runtime::Runtime::new().unwrap().block_on(clone_repos(&sync_lines[..], "sync_path"));
}
