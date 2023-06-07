mod websocket;
mod sync;

use std::thread;
use core::time::Duration;
use std::sync::{Arc, Mutex};
use std::io::{stdin, stdout, Write};
use minify_js::{Session, TopLevelMode, minify};
use base64::{Engine as _, engine::general_purpose};

fn main() {
    test_websocket();
    // test_sync();
}

fn test_path() {
    println!("{}", websocket::path::config_file());
    println!("{}", websocket::path::sync());
}

fn test_sync() {
    thread::spawn(|| {
        sync::sync(websocket::path::config_file(), websocket::path::sync())
    });
    thread::sleep(Duration::from_millis(5000))
}

fn test_websocket() {
    // open a web socket
    let connections: Arc<Mutex<Vec<websocket::connection::Connection>>> = Arc::new(Mutex::new(Vec::new()));
    let connections_clone = connections.clone();
    thread::spawn(move || {
        websocket::open_ws("0.0.0.0", "4444", connections_clone);
    });

    loop {
        print!("command: ");
        stdout().flush().expect("Error flushing stdout");
        let mut response = String::new();
        stdin().read_line(&mut response).expect("Error reading line from stdin");
        let response = response.trim();

        if response == "connections" {

            for connection in connections.clone().lock().unwrap().iter() {
                println!("{} - {}", connection.sender.connection_id(), connection.ip);
            }

        } else if response.contains("send")  {

            let splitted_command = response.split(" ");
            let connection_id = splitted_command.last().unwrap();

            let session = Session::new();
            let mut out = Vec::new();
            for patata in connections.clone().lock().unwrap().iter() {
                if patata.clone().sender.connection_id() == connection_id.parse::<u32>().unwrap() {
                    minify(&session, TopLevelMode::Global, b"const main = () => { let my_first_variable = 1; };", &mut out).unwrap();
                    let _ = patata.clone().sender.send(general_purpose::STANDARD.encode(out.as_slice()));
                }
            }

        }

    }
}
