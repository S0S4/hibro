use std::net::UdpSocket;
use clap::Error;


pub fn open () -> Result<(), Error>{


    let mut localhost = String::from("127.0.0.1:");
    let port = "8081";
    localhost.push_str(&port);

    let socket = UdpSocket::bind(localhost)?;

    let mut buf = [0; 64];
    let (amt, src) = socket.recv_from(&mut buf)?;

    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, &src)?;
    println!("{:?}", socket);
    Ok(())
}
