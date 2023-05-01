mod websocket;
mod sync;

fn main() {
    // open a web socket
    websocket::open_ws("0.0.0.0", "4444");
    // sync::sync("config_file_path")

}
