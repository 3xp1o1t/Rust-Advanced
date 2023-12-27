use std::io::{Read, Write};
use std::net::TcpListener;
fn main() {
    let connection_listener = TcpListener::bind("127.0.0.1:3000");
    println!("Running on port 3000");
    for stream in connection_listener.unwrap().incoming() {
        let mut stream = stream.expect("Failed to establish connection");
        println!("Connection established!");
        let mut buffer = [0; 1024];
        stream
            .read(&mut buffer)
            .expect("Failed to read data from stream");
        stream
            .write(&buffer)
            .expect("Failed to write data to stream");
    }
}
