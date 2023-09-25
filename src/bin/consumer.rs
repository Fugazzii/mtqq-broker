use mqtt_broker::{consumer::Consumer, utils::handle_args};

#[tokio::main]
pub async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let _parsed_args = handle_args(&args);

    let mut _socket = Consumer::connect("127.0.0.1:3000").await;

    // socket.consume(parsed_args[0]).await;

    println!("Sending...");
}
