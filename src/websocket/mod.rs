pub mod connection;
pub mod path;
pub mod data;

use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Server {
    connections: Arc<Mutex<Vec<connection::Connection>>>,
    server_sender: Sender,
}

impl Handler for Server {

    fn on_open(&mut self, handshake: Handshake) -> Result<()> {

        println!("New connection - IP: {}", Some(handshake.remote_addr()?).unwrap().unwrap());

        self.connections.lock().unwrap().push(connection::Connection{
            ip: Some(handshake.remote_addr()?).unwrap().unwrap(),
            sender: self.server_sender.clone()
        });

        Ok(())

    }

    /// Handle messages that comes from the websocket connection
    fn on_message(&mut self, message: Message) -> Result<()> {

        let connections = self.connections.lock().unwrap();

        // find the current IP
        for index in (0..connections.iter().len()).rev() {
            if connections[index].sender.connection_id() == self.server_sender.connection_id() {
                let connection_ip_clone = connections[index].ip.clone();
                // save message on file
                thread::spawn(move || {
                    data::save(path::connections(), connection_ip_clone, message.clone().to_string())
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
            server_sender: sender,
        }
    }).unwrap();

}
