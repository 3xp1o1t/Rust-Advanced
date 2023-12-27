use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("localhost:3000")?;

    stream.write("Hola".as_bytes())?;

    let mut buffer = [0; 5];

    stream.read(&mut buffer)?;

    let response_str = str::from_utf8(&buffer)?;

    println!("Got response from server:{:?}", response_str);

    Ok(())
}
