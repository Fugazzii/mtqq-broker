use std::collections::HashMap;
use async_trait::async_trait;
use tokio::net::TcpStream;
use crate::{topic::Topic, traits::topic::ConsumeTopic};

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
    async fn consume(&mut self, _topic_name: &str) -> Option<String> {
        todo!()
    }
}