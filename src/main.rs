use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Convert buffer to string and print request details
            let request = str::from_utf8(&buffer).unwrap();
            println!("Received request: {}", request);

            // Create a simple response message
            let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => println!("Failed to read from connection: {}", e),
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    println!("Server listening on port 8000");

    // Accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}
