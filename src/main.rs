#![allow(dead_code)]

use std::{
    io::{self, Read, Write},
    net::TcpListener,
};

use server::Server;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    loop {
        let (mut socket, _) = listener.accept().unwrap();

        let mut buf = [0; 1024];

        match socket.read(&mut buf) {
            Ok(_) => {
                let request = String::from_utf8_lossy(&buf);

                match extract_path(&request) {
                    Some(path) => {
                        let children: Vec<&str> = path.split('/').collect();

                        let res = if path == "/" {
                            "HTTP/1.1 200 OK\r\n\r\n".to_string()
                        } else {
                            if children[1] == "echo" {
                                format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n{}\r\n\r\n", children[2].len(), children[2])
                            } else {
                                "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
                            }
                        };

                        socket.write(res.as_bytes())?
                    }
                    None => socket.write(b"HTTP/1.1 404 Not Found\r\n\r\n")?,
                }
            }
            Err(_e) => socket.write(b"HTTP/1.1 404 Not Found\r\n\r\n")?,
        };
    }
}

fn extract_path(req: &str) -> Option<&str> {
    for (idx, line) in req.lines().enumerate() {
        if idx == 0 {
            let parts: Vec<&str> = line.split_whitespace().collect();

            return parts.get(1).copied();
        }
    }

    None
}
