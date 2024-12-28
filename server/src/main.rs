use std::io::prelude::*;
use std::net::TcpListener;

// Server

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5001")?;

    for stream in listener.incoming() {
        let message = stream?.read(&mut [0; 128])?;
        println!("{}", message);
    }
    Ok(())
}
