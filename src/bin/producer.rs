use mqtt_broker::{producer::Producer, utils::handle_args, traits::topic::PublishTopic};

const PRODUCER_ARGS_LEN: u8 = 3; 

#[tokio::main]
pub async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let parsed_args = handle_args(&args, PRODUCER_ARGS_LEN);

    let mut socket = Producer::connect("127.0.0.1:3000").await;

    let topic = parsed_args[0].as_str();
    let message = parsed_args[1].as_str();

    socket.publish(topic, message).await;

    println!("Publishing...");
}
