use std::collections::HashMap;

use bytes::BytesMut;
use tokio::{net::TcpStream, io::AsyncWriteExt};

use crate::{topic::{Topic, self}, traits::topic::BrokerTopic};

#[allow(dead_code)]
struct Consumer {
    socket: TcpStream,
    subscribed_topics: HashMap<String, Topic>
}

#[allow(dead_code)]
impl Consumer {

    pub async fn connect() -> Self {
        let socket = TcpStream::connect("127.0.0.1:3000")
            .await
            .expect("Failed to connect to consumer"); 
        Consumer { socket, subscribed_topics: HashMap::new() }
    }

}

impl BrokerTopic for Consumer {
    
    /* 
        action topic_name message
        publish hero ilia 
    */

    fn publish(&mut self, topic_name: &str, message: &str) {
        todo!()
    }

    fn consume(&mut self, topic_name: &str) -> Option<String> {
        todo!()
    }

}