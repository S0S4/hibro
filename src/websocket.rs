// use ws::{listen, connect, CloseCode};
use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};
use std::sync::{Arc, Mutex};

pub struct Server {
    connections: Arc<Mutex<Vec<Sender>>>,
    server_sender: Sender,
}

impl Handler for Server {
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        // let connection = handshake;
        // println!("New WebSocket connection from {}", connection.request.client_addr().ok().unwrap().unwrap());

        // Save the new connection to the list of connections
        self.connections.lock().unwrap().push(self.server_sender.clone());


        Ok(())
    }

    fn on_message(&mut self, message: Message) -> Result<()> {
        let connections = self.connections.lock().unwrap();

        // Send the message to each connected client
        for connection in connections.iter() {
            connection.send(message.clone())?;
        }

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket connection closed with code {:?} and reason '{}'", code, reason);
    }
}

pub async fn open_ws(url: &str, port: &str, connections: Arc<Mutex<Vec<Sender>>>) {

    // let connections: Arc<Mutex<Vec<Sender>>> = Arc::new(Mutex::new(Vec::new()));

    listen(format!("{url}:{port}"), |sender| {
        Server {
            connections: connections.clone(),
            server_sender: sender,
        }
    }).unwrap();

}

// pub fn send_message(url: &str, port: &str, message: &str) {
//
//     if let Err(error) = connect(format!("ws://{url}:{port}"), move |out| {
//
//         if out.send(message).is_err() {
//             println!("Websocket couldn't queue an initial message.")
//         } else {
//             println!("Client sent message 'Hello WebSocket'. ")
//         }
//
//         move |msg| {
//             // Handle messages received on this connection
//             println!("Client got message '{}'. ", msg);
//
//             // Close the connection
//             out.close(CloseCode::Normal)
//         }
//     }) {
//         // Inform the user of failure
//         println!("Failed to create WebSocket due to: {:?}", error);
//     }
// }
