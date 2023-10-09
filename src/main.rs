// Uncomment this block to pass the first stage
use core::result::Result::Ok;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use anyhow::Error;
use request::IncomingRequest;
mod request;

fn handle_client(stream: TcpStream) -> Result<(), Error> {
    let mut reader = std::io::BufReader::new(&stream);

    let mut buffer = [0; 1024];
    let _ = reader.read(&mut buffer).expect("Failed to read");

    let req_str = String::from_utf8_lossy(&buffer[..]).to_string();

    let request = IncomingRequest::new(&req_str);

    println!("Request: {:?}", request);

    request.execute_path(stream)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");

                tokio::spawn(async move {
                    println!("Accepted new connection");
                    let _ = handle_client(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        };
    }
    Ok(())
}
