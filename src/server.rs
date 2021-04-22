use crate::http::{HttpMethod, HttpRequestLine, HttpResponse, HttpVersion};
use crate::routes::{route_hello, route_joke, route_lorem, route_root};
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::thread;

/// Runs a simple http server.
pub fn run_server(address: impl ToSocketAddrs) {
    // setup listening socket
    let server_socket =
        TcpListener::bind(address).expect("Error while binding the socket to the address.");

    println!(
        "Listening for HTTP connections on {}",
        server_socket.local_addr().unwrap()
    );

    // accept incoming connections in loop
    loop {
        let accept_result = server_socket.accept();

        match accept_result {
            Ok(peer_socket) => {
                // if ok, spawn a thread to handle the request
                thread::spawn(|| process_connection(peer_socket.0, peer_socket.1));
            }
            Err(e) => {
                println!("Can not establish a connection: {}", e);
            }
        }
    }
}

/// Processes the incoming connection
fn process_connection(mut peer_socket: TcpStream, _peer_addr: SocketAddr) {
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

            let response = match request_line {
                Ok(Some(request_line)) => {
                    // request line is complete

                    // create response
                    let response = handle_request(&request_line);
                    println!(
                        "{} {} : {}",
                        request_line.method(),
                        request_line.uri(),
                        response.status_code()
                    );

                    response
                }
                Ok(None) => {
                    // request line is incomplete / too long
                    println!("Request line is too long.");
                    HttpResponse::new(HttpVersion::Http1_1, 400, None)
                }
                Err(_) => {
                    // malformed request line
                    println!("Request is malformed.");
                    HttpResponse::new(HttpVersion::Http1_1, 400, None)
                }
            };

            // serialize response into bytes
            let response_bytes = response.to_bytes();

            // write response
            if let Err(e) = peer_socket.write(&response_bytes) {
                println!("Error while writing to socket: {}", e);
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
    if let HttpVersion::Other(_) = request.version() {
        // unsupported http version
        return HttpResponse::new(request.version().clone(), 505, None);
    }

    if let HttpMethod::Other(_) = request.method() {
        // unsupported http method
        return HttpResponse::new(request.version().clone(), 501, None);
    }

    let response_content = match request.uri() {
        "/" => route_root(),
        "/hello" => route_hello(),
        "/lorem" => route_lorem(),
        "/joke" => route_joke(),
        _ => {
            return HttpResponse::new(
                request.version().clone(),
                404,
                Some("<html><body>404 NOT FOUND</body></html>".into()),
            )
        }
    };

    HttpResponse::new(request.version().clone(), 200, Some(response_content))
}
