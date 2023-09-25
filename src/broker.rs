use std::collections::HashMap;

use async_trait::async_trait;
use tokio::net::{TcpListener, TcpStream};

use crate::{topic::Topic, traits::topic::{PublishTopic, ConsumeTopic}};


/**
 * Represents a message broker.
 */
pub struct Broker {
    listener: TcpListener,
    topics: HashMap<String, Topic>,
}

impl Broker {
    /**
     * Create a TCP socket for listening to incoming streams.
     
     # Arguments

     * `host` - The hostname or IP address to bind to.
     * `port` - The port to bind to.

     # Returns
     * A new `Broker` instance that is ready to accept incoming connections.
    */
    pub async fn listen((host, port): (&str, u32)) -> Self {
        let addr = format!("{}:{}", host, port);

        let listener = TcpListener::bind(addr)
            .await
            .expect("Failed to listen");

        Broker {
            listener,
            topics: HashMap::new(),
        }
    }

    /**
     * Accept an incoming stream and return a `TcpStream` representing the connection.
     # Returns
     A `TcpStream` representing the accepted connection.
     */
    pub async fn accept(&self) -> TcpStream {
        let (stream, _) = self.listener
            .accept()
            .await
            .expect("Error in socket");

        stream
    }

    /**
     * Add a new topic with the specified name.
     # Arguments
     * `topic_name` - The name of the topic to add.
     */
    pub fn add_topic(&mut self, topic_name: &str) {
        let name = topic_name.to_string();
        let new_topic = Topic::new(topic_name);
        self.topics.insert(name, new_topic);
    }

    /**
     * Delete a topic with the specified name.
     # Arguments
     * `topic_name` - The name of the topic to delete.
     */
    pub fn delete_topic(&mut self, topic_name: &str) {
        if let Some(entry) = self.topics.remove_entry(topic_name) {
            println!("Deleted topic: {}", entry.0);
        } else {
            eprintln!("Could not find topic: {}", topic_name);
        }
    }

}

#[async_trait]
impl PublishTopic for Broker {
    /**
     * Publish a message to a specified topic.
     # Arguments
     * `topic_name` - The name of the topic to publish to.
     * `message` - The message to publish.
     */
    async fn publish(&mut self, topic_name: &str, message: &str) {
        if !self.topics.contains_key(topic_name) {
            println!("Could not find topic {}\n Creating one...", topic_name);
            self.add_topic(topic_name);
        }
    
        if let Some(topic) = self.topics.get_mut(topic_name) {
            topic.publish(topic_name, message).await;
        }
    }
    
}

#[async_trait]
impl ConsumeTopic for Broker {
    /**
     * Consume a message from a specified topic.
     # Arguments
     * `topic_name` - The name of the topic to consume from.
     # Returns
     * An `Option` containing the consumed message, or `None` if the topic is not found or empty.
     */
    async fn consume(&mut self, topic_name: &str) -> Option<String> {

        let topic_name = topic_name.replace('\u{1}', "");
        let topic_name = topic_name.as_str();

        let topic = self.topics.get_mut(topic_name);

        if let Some(topic) = topic {
            let message = topic.consume(topic_name);

            message.await
        } else {
            eprintln!("Could not find topic: {:?}", topic_name);
            None
        }
    }
}
