use crate::server::run_server;

mod server;
mod http;
mod routes;

fn main() {
    run_server("0.0.0.0:8080");
}