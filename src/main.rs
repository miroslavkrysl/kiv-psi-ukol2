use crate::server::run_server;

mod server;
mod request;
mod http;

fn main() {
    run_server("0.0.0.0:8080");
}