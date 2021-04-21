use std::net::{ToSocketAddrs, TcpListener, TcpStream, SocketAddr, Shutdown};
use std::thread;
use std::io::Write;

/// Runs a simple http server.
pub fn run_server(address: impl ToSocketAddrs) {
    // setup listening socket
    let server_socket = TcpListener::bind(address).unwrap();
    println!("Listening for http connections on port {}", server_socket.local_addr().unwrap().port());

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
    println!("Handling connection");

    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    peer_socket.write(response).expect("error while writing to socket");

    peer_socket.shutdown(Shutdown::Both).unwrap();
}

