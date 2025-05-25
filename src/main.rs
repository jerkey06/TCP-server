use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0 as u8; 512];
    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client Disconnected.");
                return Ok(());
            },
            Ok(n) => n,
            Err(e) => {
                eprintln!("Connection Error: {}", e);
                return Err(e);
            }
        };
        if bytes_read == 0 {
            break;
        }
        println!("<<<<REQUEST>>>>\r\n{}", String::from_utf8_lossy(&buffer[..bytes_read]));
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let addr = "0.0.0.0:8888";
    let listener = TcpListener::bind(addr)?;

    for stream in listener.incoming(){
        match stream {
            Ok(strm) => {
                println!("Got a connection!");
                handle_connection(strm)?;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
    Ok(())
}
