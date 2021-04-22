use std::net::{ToSocketAddrs, TcpListener, TcpStream, SocketAddr, Shutdown};
use std::thread;
use std::io::{Write, Read, Error};
use crate::http::{HttpResponse, HttpMethod, HttpParseError, HttpRequestLine};

/// Runs a simple http server.
pub fn run_server(address: impl ToSocketAddrs) {
    // setup listening socket
    let server_socket = TcpListener::bind(address).unwrap();
    println!("Listening for HTTP connections on {}", server_socket.local_addr().unwrap());

    // accept incoming connections in loop
    loop {
        let accept_result = server_socket.accept();

        match accept_result {
            Ok(peer_socket) => {
                // if ok, spawn a thread to handle the request
                thread::spawn(|| {
                    process_connection(peer_socket.0, peer_socket.1)
                });
            }
            Err(e) => {
                println!("Can not establish a connection: {}", e);
            }
        }
    }
}


/// Processes the incoming connection
fn process_connection(mut peer_socket: TcpStream, peer_addr: SocketAddr) {
    let mut buffer = [0u8; 8192];

    match peer_socket.read(&mut buffer) {
        Ok(0) => {
            println!("Connection closed by peer.");
        }
        Ok(n) => {
            // convert request line into UTF8 (request line must be in ASCII encoding which is subset of UTF8)
            let request_text = String::from_utf8_lossy(&buffer[..n]);

            // parse request line
            let request_line = HttpRequestLine::parse(&request_text);

            match request_line {
                Ok(Some(request_line)) => {
                    // request line is complete

                    // create response
                    let response = handle_request(&request_line);

                    println!("{} {} : {}", request_line.method(), request_line.uri(), response.status_code());

                    // serialize response into bytes
                    let response_bytes = response.to_bytes();

                    if let Err(e) = peer_socket.write(&response_bytes) {
                        println!("Error while writing to socket: {}", e);
                    }
                }
                Ok(None) => {
                    // request line is incomplete / too long
                    println!("Request line is too long.");
                }
                Err(e) => {
                    // malformed request line
                    todo!("generate error response")
                }
            }
        }
        Err(e) => {
            println!("Error while reading from socket: {}", e)
        }
    }

    // close socket
    peer_socket.shutdown(Shutdown::Both).unwrap();
}

/// Processes the request and return corresponding response.
fn handle_request(request: &HttpRequestLine) -> HttpResponse {
    HttpResponse::new(
        request.version().clone(),
        200,
        Some(Vec::from("<html><body>Hello world!</body></html>"))
    )
}


