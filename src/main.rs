#![allow(unused_imports)]
use std::{io::{Cursor, Read, Write}, net::TcpListener};
use bytes::BufMut;
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
                let mut request = Cursor::new(buffer);
                let request_api_key = request.read_u16().await?;
                let request_api_version = request.read_u16().await?;
                let correlation_id = request.read_u32().await?;
                let mut response = Vec::new();
                let error_core = match request_api_version <= 4 {
                    true => 0,
                    false => 35
                };
                if request_api_key == 18 {
                    response.put(&correlation_id.to_be_bytes()[..]);
                    response.put(&[0, error_core][..]);
                    response.put_i8(2); // num api key records + 1
                    response.put_i16(18); // api key
                    response.put_i16(0); // min version
                    response.put_i16(4); // max version
                    response.put_i8(0); // TAG_BUFFER length
                    response.put_i32(420); // throttle time ms
                    response.put_i8(0); // TAG_BUFFER length
                }                
                let len = response.len() as i32;
                stream.write_all(&len.to_be_bytes())?;
                stream.write_all(&response)?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
