use std::io::{Read, Write};
use std::net::TcpListener;

fn parse_host_header(request: &str) -> Option<&str> {
    for line in request.lines() {
        if let Some(value) = line.strip_prefix("Host:") {
            return Some(value.trim());
        }
    }
    None
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").expect("Failed to bind to port 80");
    println!("Listening on port 80...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                let n = stream.read(&mut buffer).unwrap_or(0);

                let request = String::from_utf8_lossy(&buffer[..n]);
                let host = parse_host_header(&request).unwrap_or("unknown").to_string();

                let body = format!("Hello, World! You reached me via: {}\n", host);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).unwrap_or(());
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }
}
