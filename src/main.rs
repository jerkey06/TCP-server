use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};

struct Command<'a> {
    kind: String,
    args: &'a str
}

fn parse_command(input: &str) -> Command {
    let mut parts = input.trim().splitn(2, char::is_whitespace);
    let kind = parts.next().unwrap_or("").to_uppercase();
    let args = parts.next().unwrap_or("");
    Command {
        kind,
        args,
    }
}

fn handle_protocol(cmd: Command) -> Result<String, &'static str> {
    match cmd.kind.as_str() { 
        "000" => Ok("Welcome user".into()),
        "001" => Ok("Operation: connected!".into()),
        _ => Err("Operation: Unknown".into())
    }
}

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
        let command = parse_command(&request);
        println!("Command: {} | Args: {}", command.kind, command.args);
        match handle_protocol(command) { 
            Ok(response) => {
                stream.write(response.as_bytes())?;
            },
            Err(err_response) => {
                stream.write(err_response.as_bytes())?;
            }
        }
        
    }
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
