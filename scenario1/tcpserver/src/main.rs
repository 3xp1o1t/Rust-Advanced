use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    println!("Connection established");

    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    stream.write_all(&buffer)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    println!("Running on port 3000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(err) = handle_client(stream) {
                    eprint!("Error handling client: {}", err);
                }
            }
            Err(err) => {
                eprint!("Failed to establish connection: {}", err);
            }
        }
    }

    Ok(())
}
