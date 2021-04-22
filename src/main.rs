use crate::server::run_server;

mod http;
mod routes;
mod server;

fn main() {
    run_server("0.0.0.0:8080");
}
