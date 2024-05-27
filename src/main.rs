use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;
use std::fs::{self, File};
use std::io::prelude::*;
use std::time::SystemTime;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Convert buffer to string and print request details
            let request = str::from_utf8(&buffer).unwrap();
            println!("Received request: {}", request);

            // Save request to a file
            save_request_to_file(request);

            // Create a simple response message
            let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => println!("Failed to read from connection: {}", e),
    }
}

fn save_request_to_file(request: &str) {
    // Ensure the directory exists
    let dir_path = "/received_requests";
    fs::create_dir_all(dir_path).unwrap();

    // Create a unique filename based on the current timestamp
    let file_name = format!("{}/request_{}.txt", dir_path, SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());
    let mut file = File::create(file_name).unwrap();

    // Write the request to the file
    file.write_all(request.as_bytes()).unwrap();
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
