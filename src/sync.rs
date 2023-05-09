use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio::task;
use git2::Repository;

/// Clone a repository into the given path
async fn clone_repo(url: &str, dir_path: &str) {

    let repo = match Repository::clone(url, dir_path) {
        Ok(repo) => repo,
        Err(e) => return println!("{}", e.to_string()),
    };

    println!("Repository cloned to {:?}", repo.path());

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
async fn clone_repos(urls: Arc<Mutex<Vec<String>>>, dir_path: &str) {
    println!("clone repos...");
    let mut handles = Vec::new();
    for url in urls.lock().unwrap().iter() {
        let dir_path = dir_path.to_owned();
        let url = url.clone();
        println!("trying to clone... {}", url);
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
pub async fn sync(config_file_path: &str) {
    let sync_lines: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(read_sync_lines("/home/iruzo/dev/hibro/testingsync")));
    for line in sync_lines.lock().unwrap().iter() {
        println!("{}", line.clone());
    }
    tokio::task::spawn(clone_repos(sync_lines, "/home/iruzo/dev/hibro/testingboys")).await;
}
