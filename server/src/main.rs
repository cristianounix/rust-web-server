
use std::fs;
use std::io::Read;
// use std::{net::{TcpListener, TcpStream}, fmt::Error}
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("Hello, world!");
        handle_connection(stream)
    }
}

/**
 * HTTP-VErsion Status-Code Reason-Phrase CRLF
 * headers CRLF
 * message-body
 * 
 * ec: HTTP/1.1 200 OK\r\n\r\n
 */

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];

    stream.read(&mut buffer).unwrap();

    println!(
        "Req: {}",
        String::from_utf8_lossy(&buffer[..])
    );

    let contents = fs::read_to_string("index.html").unwrap();

    // let response = "HTTP/1.2 200 OK\r\n\r\n";

    let response = format!(
        "HTTP/1.2 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}