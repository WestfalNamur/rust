use std::fs;
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
    // Read bytes into buffer;
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Indication of GET HTTP 1.1 in HTTP as variable get;
    let get = b"GET / HTTP/1.1\r\n";

    // If incoming byte stream, stored in buffer and read as chars[], starts
    // with "GET HTTP 1.1 in HTTP" set status_line, filename to success etc.
    // else to 404 etc.;
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // Write concent of file to string;
    let contents = fs::read_to_string(filename).unwrap();

    // Create response;
    let response = format!("{}{}", status_line, contents);

    // Return;
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
