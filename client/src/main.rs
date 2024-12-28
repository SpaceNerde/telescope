use std::io::prelude::*;
use std::net::TcpStream;

// Client

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:5001")?;
    
    stream.write(&[1])?;
    Ok(())
}
