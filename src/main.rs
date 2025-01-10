#![allow(unused_imports)]
use std::{io::{Cursor, Read, Write}, net::TcpListener};
use tokio::io::AsyncReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    //
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0;4];
                stream.read_exact(&mut buffer)?;
                let mut message_length_cursor = Cursor::new(buffer);
                let message_length = message_length_cursor.read_u32().await?;
                let mut buffer = vec![0; message_length as usize];
                stream.read_exact(&mut buffer)?;
                let mut message = Cursor::new(buffer);
                let _request_api_key = message.read_u16().await?;
                let _request_api_version = message.read_u16().await?;
                let correlation_id = message.read_u32().await?;

                stream.write_all(&[0, 0, 0, 4])?;
                stream.write_all(&correlation_id.to_be_bytes())?;

            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
