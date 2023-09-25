use async_trait::async_trait;
use tokio::{net::TcpStream, io::AsyncWriteExt};

use crate::{message::Message, action::Action, traits::topic::PublishTopic};

pub struct Producer {
    socket: TcpStream   
}

impl Producer {   
    pub async fn connect(host: &str) -> Self {
        let socket = TcpStream::connect(host)
            .await
            .expect("Failed to connect to consumer");

        Producer { socket }
    }
}

#[async_trait]
impl PublishTopic for Producer {
    async fn publish(&mut self, topic_name: &str, msg: &str) {
        let message: Message = Message::new(Action::Publish, topic_name, Some(msg));
    
        let buffer = message.to_buffer();

        let _ = self.socket
            .write_all(&buffer)
            .await
            .expect("Failed to publish");

        println!("Published");
    }
}