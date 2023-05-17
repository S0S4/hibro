mod websocket;
mod sync;
mod path;

use std::thread;
use core::time::Duration;
use std::sync::{Arc, Mutex};
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() {

}

fn test_path() {
    println!("{}", path::config());
    println!("{}", path::data());
    println!("{}", path::sync());
}

fn test_sync() {
    tokio::task::spawn(sync::sync("/home/iruzo/dev/hibro/testingsync"));
    thread::sleep(Duration::from_millis(5000))
}

fn test_websocket() {
    // open a web socket
    let connections: Arc<Mutex<Vec<websocket::connection::Connection>>> = Arc::new(Mutex::new(Vec::new()));
    tokio::task::spawn(websocket::open_ws("0.0.0.0", "4444", connections.clone()));

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

            for patata in connections.clone().lock().unwrap().iter() {
                if patata.clone().sender.connection_id() == connection_id.parse::<u32>().unwrap() {
                    let _ = patata.clone().sender.send("hello there !");
                }
            }

        }

    }
}
