use minify_js::{Session, TopLevelMode, minify};
use base64::{Engine as _, engine::general_purpose};
use crate::data::connection::Connection;

pub fn send(connections: &Vec<Connection>, message: String) {

    let session = Session::new();
    let mut out = Vec::new();
    for connection in connections {
        minify(&session, TopLevelMode::Global, message.as_bytes(), &mut out).unwrap();
        let _ = connection.sender.unwrap().send(general_purpose::STANDARD.encode(out.as_slice()));
    }

}
