// Uncomment this block to pass the first stage
use core::result::Result::Ok;
use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use anyhow::Error;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let mut reader = std::io::BufReader::new(&stream);

    let mut buffer = [0; 1024];
    let _ = reader.read(&mut buffer).expect("Failed to read");

    let req_str = String::from_utf8_lossy(&buffer[..]).to_string();

    let request_array = req_str.split("\r\n").collect::<Vec<&str>>();
    let request_array: Vec<String> = request_array.iter().map(|s| s.replace("\0", "")).collect();

    println!("Req_str \n{:?}", request_array);

    // Parse the request line to extract the path
    let request_parts: Vec<&str> = request_array[0].trim().split(' ').collect();
    println!("Request parts: {:?}", request_parts);
    if request_parts.len() >= 2 {
        let method = request_parts[0];
        let path = request_parts[1];

        if method == "POST" {
            println!("Method: {}", method);
            let args: Vec<String> = env::args().collect();
            let directory = env::current_dir()?.join(&args[2]);

            let file_name = &path[7..];
            let file_loc = directory.join(file_name);

            let mut file_content = String::new();

            println!("Hello ssss");

            file_content = request_array[request_array.len() - 1].clone();

            println!("File content: {}", file_content);

            std::fs::write(file_loc, file_content).expect("Unable to write");

            let response = "HTTP/1.1 201 OK\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();

            return Ok(());
        }

        println!("Path : {}", path);

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
                // println!("Hello");ddf
                // let mut recursive_s = String::new();
                // while recursive_s.starts_with("User") == false {
                //     recursive_s.clear();
                //     reader.read_line(&mut recursive_s).expect("Read next line");
                //     // println!("Recursive : {}", recursive_s);
                // }
                let user_agent_string = request_array
                    .iter()
                    .find(|s| s.starts_with("User-Agent"))
                    .unwrap();
                let user_agent_val = &user_agent_string[12..];
                // println!("User agent val: {}", user_agent_val);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    user_agent_val.len(),
                    user_agent_val
                );
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            p if p.starts_with("/files") => {
                let args: Vec<String> = env::args().collect();
                let directory = env::current_dir()?.join(&args[2]);

                let file_name = &path[7..];
                println!("Filename: {}", file_name);

                let mut content = String::new();

                let entries = std::fs::read_dir(directory)?;

                for entry in entries {
                    let entry = entry?;

                    let path = entry.path();
                    // hheehfhdfhdgdgdgdfsdfsf
                    if path.is_file() {
                        println!("Entry : {}", path.display().to_string());
                        if path.display().to_string().split("/").last() == Some(file_name) {
                            content = std::fs::read_to_string(path).expect("Unable to read");
                        }
                    }
                }

                if content.len() == 0 {
                    let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                } else {
                    let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                            content.len(),
                            content
                        );
                    stream.write_all(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            }
            _ => {
                let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
    }

    Ok(())

    // Respond to the client (you would handle the request here)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
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
