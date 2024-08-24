use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 80);
    let listener = TcpListener::bind(addr)?;
    println!("Listening port 80");
    loop {
        let (mut sock, _) = listener.accept()?;
        let response = String::from("HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=UTF-8\r\nContent-Length: 24\r\nServer: Nexel (Unix)\r\n\r\nHello! Welcome to Nexel.");
        sock.write_all(response.as_bytes())?;
    }
}
