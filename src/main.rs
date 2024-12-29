#![allow(unused_imports)]
use std::{io::Write, net::TcpListener};

fn main() -> std::io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut header = [0;8];
                header[7] = 7;
                stream.write(&header)?;
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
