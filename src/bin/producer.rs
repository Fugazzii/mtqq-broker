use mqtt_broker::{producer::Producer, utils::handle_args, traits::topic::PublishTopic};

#[tokio::main]
pub async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let parsed_args = handle_args(&args);

    let mut socket = Producer::connect("127.0.0.1:3000").await;

    socket.publish(parsed_args[0].as_str(), parsed_args[1].as_str()).await;

    println!("Sending...");
}
