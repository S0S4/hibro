use std::net::{TcpListener, TcpStream};


fn handle_client(stream: TcpStream){

    println!("{:?}", stream) ;

}

pub fn tcp_socket() -> std::io::Result<()> {

    let mut localhost = String::from("127.0.0.1:");
    let port = "8080";
    localhost.push_str(&port);
    let listener = TcpListener::bind(localhost)?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

