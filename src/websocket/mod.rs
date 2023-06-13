pub mod connection;
pub mod path;
pub mod data;

use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::path::PathBuf;
use substring::Substring;
use rand::Rng;

pub struct Server {
    connections: Arc<Mutex<Vec<connection::Connection>>>,
    server_sender: Sender,
}

impl Handler for Server {

    fn on_open(&mut self, handshake: Handshake) -> Result<()> {

        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(100000000..=999999999);

        self.connections.lock().unwrap().push(connection::Connection{
            ip: Some(handshake.remote_addr()?).unwrap().unwrap(),
            sender: self.server_sender.clone(),
            fingerprint: random_number.to_string()
        });

        Ok(())

    }

    /// Handle messages that comes from the websocket connection
    fn on_message(&mut self, message: Message) -> Result<()> {

        let mut connections = self.connections.lock().unwrap();

        // find the current IP
        for index in (0..connections.iter().len()).rev() {
            if connections[index].sender.connection_id() == self.server_sender.connection_id() {

                let connection_ip_clone = connections[index].ip.clone();
                let message_clone = message.clone().to_string();

                if message_clone.contains("fingerprint") {
                    let start = message_clone.find("fingerprint\": \"").unwrap_or(0);
                    let end = message_clone.find("\",").unwrap_or(message_clone.len());
                    let new_fingerprint = message_clone.substring(start, end).split(": \"").last().unwrap().split("\"").take(1).last().unwrap().to_string();
                    if new_fingerprint != connections[index].fingerprint {

                        let fingerprint_clone = connections[index].fingerprint.clone();
                        let new_fingerprint_clone = new_fingerprint.clone();

                        let mut source_folder = PathBuf::new();
                        source_folder.push(path::connections());
                        source_folder.push(fingerprint_clone);

                        let mut destination_folder = PathBuf::new();
                        destination_folder.push(path::connections());
                        destination_folder.push(new_fingerprint_clone);

                        println!("{}", source_folder.to_str().unwrap());

                        thread::spawn(move || {
                            // let _ = data::move_files(source_folder.to_str().unwrap(), destination_folder.to_str().unwrap());
                            // println!("{}", Result::Expects(result));
                            if let Err(_err) = data::move_files(source_folder.to_str().unwrap(), destination_folder.to_str().unwrap()) {
                                println!("Failed to perform necessary steps");
                                println!("{}", _err.to_string());
                            }
                        });

                        connections[index].fingerprint = new_fingerprint;

                    }
                }

                let fingerprint_clone = connections[index].fingerprint.clone();

                // save message on file
                thread::spawn(move || {
                    data::save(path::connections(), connection_ip_clone, message_clone, fingerprint_clone)
                });
                break;
            }
        }

        Ok(())

    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {

        let mut connections = self.connections.lock().unwrap();

        for index in (0..connections.iter().len()).rev() {
            if connections[index].sender.connection_id() == self.server_sender.connection_id() {
                connections.remove(index);
                println!("WebSocket connection closed with code {:?} reason '{}', id {}", code, reason, index);
            }
        }

    }
}

/// Open a websocket and manage every connection on the given list
pub fn open_ws(url: &str, port: &str, connections: Arc<Mutex<Vec<connection::Connection>>>) {

    listen(format!("{url}:{port}"), |sender| {
        Server {
            connections: connections.clone(),
            server_sender: sender
        }
    }).unwrap();

}
