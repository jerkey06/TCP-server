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
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let response = "200 OK\r\n\r\n";
        println!("<<<<REQUEST>>>>\r\n{}", String::from_utf8_lossy(&buffer[..bytes_read]));
        
        stream.write(response.as_bytes())?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let addr = "0.0.0.0:8888";
    let listener = TcpListener::bind(addr)?;
    println!("Listening on {}", addr);

    for stream in listener.incoming(){
        match stream {
            Ok(strm) => {
                println!("Got a connection!");
                std::thread::spawn(|| handle_connection(strm));
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
    Ok(())
}
