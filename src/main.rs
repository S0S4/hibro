mod websocket;
mod sync;
use std::sync::{Arc, Mutex};
use std::io::{stdin, stdout, Write};
use ws::Sender;

fn main() {

    // open a web socket
    let connections: Arc<Mutex<Vec<Sender>>> = Arc::new(Mutex::new(Vec::new()));
    tokio::runtime::Runtime::new().unwrap().block_on(websocket::open_ws("0.0.0.0", "4444", &connections));

    loop {
        print!("command: ");
        stdout().flush().expect("Error flushing stdout");
        let mut response = String::new();
        stdin().read_line(&mut response).expect("Error reading line from stdin");
        let response = response.trim();

        if response == "connections" {

            for connection in connections.clone().lock().unwrap().iter() {
                println!("{}", connection.connection_id());
            }

        } else if response.contains("send")  {

            let splitted_command = response.split(" ");
            let connection_id = splitted_command.last().unwrap();

            for patata in connections.clone().lock().unwrap().iter() {
                if patata.clone().connection_id() == connection_id.parse::<u32>().unwrap() {
                   patata.clone().send("hello there !");
                }
            }

        }

    }

    // for connection in connections.clone().lock().unwrap().iter() {
    //     connection.send("");
    // }

    // sync::sync("config_file_path")

}
