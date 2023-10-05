// Uncomment this block to pass the first stage
use std::{
    io::{BufRead, Write},
    net::{TcpListener, TcpStream},
};

use anyhow::Error;

fn handle_client(mut stream: TcpStream) {
    let mut reader = std::io::BufReader::new(&stream);
    let mut request_line = String::new();

    // Read the first line of the HTTP request (request line)
    if reader.read_line(&mut request_line).is_ok() {
        // Parse the request line to extract the path
        let request_parts: Vec<&str> = request_line.trim().split(' ').collect();
        if request_parts.len() >= 2 {
            // let method = request_parts[0];
            let path = request_parts[1];

            match path {
                "/" => {
                    let response = "HTTP/1.1 200 OK\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                p if p.starts_with("/echo") => {
                    let echo_str = &path[6..];
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-length: {}\r\n\r\n{}",
                        echo_str.len(),
                        echo_str
                    );
                    stream.write_all(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                p if p.starts_with("/user-agent") => {
                    // println!("Hello");
                    let mut recursive_s = String::new();
                    while recursive_s.starts_with("User") == false {
                        recursive_s.clear();
                        reader.read_line(&mut recursive_s).expect("Read next line");
                        // println!("Recursive : {}", recursive_s);
                    }
                    let user_agent_parts: Vec<&str> = recursive_s.trim().split(":").collect();
                    let user_agent_val = user_agent_parts[1].trim();
                    println!("User agent val: {}", user_agent_val);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-length: {}\r\n\r\n{}",
                        user_agent_val.len(),
                        user_agent_val
                    );
                    stream.write_all(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                _ => {
                    let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            }

            // println!("Method: {}", method);
            // println!("Path: {}", echo_str);

            // if path == "/" {
            //     let response = "HTTP/1.1 200 OK\r\n\r\n";
            //     stream.write_all(response.as_bytes()).unwrap();
            //     stream.flush().unwrap();
            // } else {
            //     let response = "HTTP/1.1 404 Not Found\r\n\r\n";
            //     stream.write_all(response.as_bytes()).unwrap();
            //     stream.flush().unwrap();
            // }

            // You can now use the 'path' variable in your application.
        }
    }

    // Respond to the client (you would handle the request here)
}

fn main() -> Result<(), Error> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_client(stream);

                // stream
                //     .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                //     .unwrap()
            }
            Err(e) => {
                println!("error: {}", e);
            }
        };
    }
    Ok(())
}
