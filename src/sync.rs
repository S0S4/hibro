use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio::task;
use futures::future::join_all;
use git2::Repository;

/// Clone a repository into the given path
async fn clone_repo(url: String, dir_path: String) {

    // split the url by "/" and getting the user name
    let parts: Vec<&str> = url.split("/").collect();
    let user_repo_name = parts.get(parts.len() - 2).unwrap();

    // create dir_path if it does not exist
    if let Ok(metadata) = fs::metadata(&dir_path) {
        if !metadata.is_dir() {
            let _ = fs::create_dir_all(&dir_path);
        }
    }

    // creating the full path where the repository will be cloned
    let mut path = PathBuf::new();
    path.push(dir_path);
    path.push(user_repo_name);
    let full_path = path.to_str().unwrap();

    if let Ok(metadata) = fs::metadata(&full_path) {
        if metadata.is_dir() {
            println!("Repository already in path! {}", full_path);
            return;
        }
    }

    // cloning the repo
    let repo = match Repository::clone(&url, &full_path) {
        Ok(repo) => repo,
        Err(e) => return println!("{}", e.to_string()),
    };

    println!("Repository cloned to {:?}", repo.path());

}

/// * **example**:
///   ```
///   let urls = vec![
///       String::from("https://github.com/rust-lang/rust.git"),
///       String::from("https://github.com/tensorflow/tensorflow.git"),
///   ];
///   let target_dir = "/home/a/";
///
///   tokio::runtime::Runtime::new().unwrap().block_on(clone_repos(&urls, &target_dir));
///   ```
async fn clone_repos(urls: Arc<Mutex<Vec<String>>>, dir_path: String) {
    println!("clone repos...");
    let mut tasks: Vec<tokio::task::JoinHandle<()>> = vec![];
    for url in urls.lock().unwrap().iter() {
        println!("trying to clone... {}", &url);
        tasks.push(task::spawn(clone_repo(url.clone(), dir_path.to_owned())));
    }
    join_all(tasks).await;
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

/// Sync repositories from the config file to the desired directory
pub async fn sync(config_file_path: &str, sync_directory: &str) {
    let sync_lines: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(read_sync_lines(config_file_path)));
    for line in sync_lines.lock().unwrap().iter() {
        println!("{}", line.clone());
    }
    tokio::task::spawn(clone_repos(sync_lines, sync_directory.to_owned()));
}
