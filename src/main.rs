use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;
use std::fs::{OpenOptions};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            // Convert buffer to string and print request details
            let request = str::from_utf8(&buffer[..bytes_read]).unwrap();
            println!("Received request: {}", request);

            // Save request to a file
            save_request_to_file(request);

            // Create a simple response message
            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 16\r\nTcp-Handler: novakj\r\n\r\nrequest received";
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => println!("Failed to read from connection: {}", e),
    }
}

fn save_request_to_file(request: &str) {
    // Define the log file path
    let file_path = "received_requests/tcp_handler.log";

    // Open the file in append mode, create if it doesn't exist
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .unwrap();

    // Write the request to the file with a separator
    file.write_all(request.as_bytes()).unwrap();
    file.write_all(b"\n|---END_OF_LOG---|\n").unwrap();
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
