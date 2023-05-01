// use ws::{listen, connect, CloseCode};
use ws::listen;

pub fn open_ws(url: &str, port: &str) {
    // Listen on an address and call the closure for each connection
    if let Err(error) = listen(format!("{url}:{port}"), |out| {
        // The handler needs to take ownership of out, so we use move
        move |msg| {
            println!("Server got message '{}'. ", msg);
            out.send(msg)
        }
    }) {
        // Inform the user of failure
        println!("Failed to create WebSocket due to {:?}", error);
    }
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
