use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    println!("port: {}", &port);
    run(listener)?.await
}
