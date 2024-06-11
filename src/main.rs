use std::net::TcpListener;

use zero_to_prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // propagate io:Error if address bind fails, otherwise await server
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    run(listener)?.await
}
