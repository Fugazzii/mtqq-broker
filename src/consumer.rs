use std::collections::HashMap;
use async_trait::async_trait;
use tokio::{net::TcpStream, io::AsyncWriteExt};
use crate::{topic::Topic, traits::topic::ConsumeTopic, message::Message, action::Action};

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
}

#[async_trait]
impl ConsumeTopic for Consumer {
    async fn consume(&mut self, topic_name: &str) -> Option<String> {
        let message = Message::new(Action::Consume, topic_name, None);
    
        let buffer = message.to_buffer();

        let _ = self.socket
            .write_all(&buffer)
            .await
            .expect("Failed to consume");

        println!("Consumed");

        Some("".to_string())
    }
}