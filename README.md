# Rusty TCP Handler

A simple TCP server written in Rust that listens on port 8000, logs incoming requests to a file, and responds with a predefined message. 

## Features

- Listens for TCP connections on port 8000.
- Logs incoming requests to a file named `tcp_handler.log` in the `received_requests` directory.
- Responds to each request with a simple HTTP response containing a custom header.

## Requirements

- Rust (latest stable version)
- Cargo (latest stable version)

## Installation

1. **Clone the repository:**

    ```sh
    git clone <repository_url>
    cd rusty_tcp_handler
    ```

2. **Build the project:**

    ```sh
    cargo build
    ```

## Usage

1. **Run the server:**

    ```sh
    cargo run
    ```

2. **Send requests to the server:**

    You can use tools like `curl`, `Postman`, or any other HTTP client to send requests to the server.

    Example using `curl`:

    ```sh
    curl -v http://localhost:8000
    ```

3. **Check the logs:**

    The requests will be logged in `received_requests/tcp_handler.log`.

## Code Overview

- **Main Function:**
    - Initializes a TCP listener on port 8000.
    - Accepts incoming connections and handles them serially.

- **handle_client Function:**
    - Reads the incoming request.
    - Logs the request to a file.
    - Sends a simple HTTP response back to the client.

- **save_request_to_file Function:**
    - Ensures the `received_requests` directory exists.
    - Appends the request to `tcp_handler.log`.

## Example Response

The server responds to each request with:

```http
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 16
Tcp-Handler: novakj

request received
