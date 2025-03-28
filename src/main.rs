use std::net::TcpListener;
use zero2prod_rust::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run(TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to 8000"))?.await
}
