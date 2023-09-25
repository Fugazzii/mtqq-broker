use std::collections::HashMap;
use tokio::{net::TcpStream, io::AsyncWriteExt};
use crate::{topic::Topic, traits::topic::BrokerTopic, message::Message, action::Action};

#[allow(dead_code)]
pub struct Consumer {
    socket: TcpStream,
    subscribed_topics: HashMap<String, Topic>
}

#[allow(dead_code)]
impl Consumer {

    pub async fn connect(host: &str) -> Self {
        let socket = TcpStream::connect(host)
            .await
            .expect("Failed to connect to consumer"); 
        Consumer { socket, subscribed_topics: HashMap::new() }
    }

    pub async fn send(&mut self, topic_name: &str, msg: &str) {
        let message = Message::new(Action::Publish, topic_name, msg);
    
        let buffer = message.to_buffer();

        let _ = self.socket.write_all(&buffer).await.expect("Failed to sent message");

        println!("Sent");
    }

}