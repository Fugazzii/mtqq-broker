use mqtt_broker::consumer::Consumer;

#[tokio::main]
pub async fn main() {
    let mut socket = Consumer::connect("127.0.0.1:3000").await;

    socket.send("tanks", "Panzerkampfwagen").await;

    println!("Sending...");
}