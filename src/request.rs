use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::net::TcpStream;

use anyhow::Error;

#[derive(Debug, Clone)]
pub struct IncomingRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl IncomingRequest {
    pub fn new(request_str: &str) -> Self {
        let request_array = request_str.split("\r\n").collect::<Vec<&str>>();
        let request_array = request_array
            .iter()
            .map(|s| s.replace("\0", ""))
            .collect::<Vec<String>>();

        println!("Request array: {:?}", request_array);
        Self {
            method: request_array[0].trim().split(' ').collect::<Vec<&str>>()[0].to_string(),
            path: request_array[0].trim().split(' ').collect::<Vec<&str>>()[1].to_string(),
            version: request_array[0].trim().split(' ').collect::<Vec<&str>>()[2].to_string(),
            headers: request_array[1..request_array.len() - 1]
                .iter()
                .filter_map(|s| {
                    if s == "" {
                        None
                    } else {
                        let header_parts = s.split(':').collect::<Vec<&str>>();
                        println!("Header parts: {:?}", header_parts);

                        Some((
                            header_parts[0].to_string(),
                            header_parts[1].trim().to_string(),
                        ))
                    }
                })
                .collect::<HashMap<String, String>>(),
            body: Some(request_array[request_array.len() - 1].clone()),
        }
    }

    pub fn execute_path(&self, mut stream: TcpStream) -> Result<(), Error> {
        match &self.path[..] {
            "/" => {
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            p if p.starts_with("/echo") => {
                let echo_str = &self.path[6..];
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-length: {}\r\n\r\n{}",
                    echo_str.len(),
                    echo_str
                );
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            p if p.starts_with("/user-agent") => {
                let user_agent_val = &self.headers["User-Agent"];
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

                let file_name = &self.path[7..];
                let file_location = directory.join(file_name);
                println!("Filename: {}", file_name);

                if self.method == "POST" {
                    let content = self.body.clone().unwrap();

                    std::fs::write(file_location, content).expect("Unable to write");

                    let response = "HTTP/1.1 201 OK\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                } else {
                    let mut content = String::new();

                    let entries = std::fs::read_dir(directory)?;

                    for entry in entries {
                        let path = entry?.path();

                        if path.is_file() {
                            println!("Entry : {}", path.display().to_string());
                            if path == file_location {
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
            }
            _ => {
                let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
        Ok(())
    }
}
