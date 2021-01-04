use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // Listen to incoming TCP/HTTP requests on localhost:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Interate incoming streams and process one by one sequentically;
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Create a 1024 byte buffer to store incoming byte-stream, convert to
    // string and print it;

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
