use std::fs;
use std::path::PathBuf;
use std::thread;
use git2::Repository;

/// Clone a repository into the given path
fn clone_repo(url: String, dir_path: String) {

    // split the url by "/" and getting the user name
    let parts: Vec<&str> = url.split("/").collect();
    let mut usr_repo = parts.get(parts.len() - 2).unwrap().to_string();
    usr_repo.push_str(".");
    usr_repo.push_str(parts.get(parts.len() - 1).unwrap());
    let user_repo_name = usr_repo.clone();

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
///   thread::spawn(|| {clone_repos(&urls, &target_dir)});
///   ```
fn clone_repos(urls: Vec<String>, dir_path: String) {
    println!("cloning repos...");
    let mut handles = Vec::new();
    for url in urls.iter() {
        println!("trying to clone... {}", &url);
        let url_clone = url.clone();
        let dir_path_clone = dir_path.to_owned().clone();
        let handle = thread::spawn(|| {
            clone_repo(url_clone, dir_path_clone);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("Failed to join git clone threads!");
    }
}

/// Sync repositories from the config file to the desired directory
pub fn sync(repos: Vec<String>, sync_dir_path: String) -> std::io::Result<()> {
    for line in repos.iter() {
        println!("{}", line.clone());
    }
    let sync_lines_clone = repos.clone();

    // Iterate over the files in the source directory
    for entry in fs::read_dir(sync_dir_path.clone())? {
        let entry = entry?;
        let path = entry.path();
        fs::remove_dir_all(&path)?;
    }

    thread::spawn(move ||{
        clone_repos(sync_lines_clone, sync_dir_path.to_owned());
    });

    Ok(())
}
