use std::collections::HashMap;

use tokio::net::{TcpListener, TcpStream};

use crate::{topic::Topic, traits::topic::BrokerTopic};


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
        let name = topic_name.into();
        let new_topic = Topic::new();
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

    /**
     * Publish a message to a specified topic.
     # Arguments
     * `topic_name` - The name of the topic to publish to.
     * `message` - The message to publish.
     */
    pub fn publish(&mut self, topic_name: &str, message: &str) {
        let topic = self.topics.get_mut(topic_name);

        if let Some(topic) = topic {
            topic.publish(message);
        } else {
            eprintln!("Could not find topic: {}", topic_name);
        }
    }

    /**
     * Consume a message from a specified topic.
     # Arguments
     * `topic_name` - The name of the topic to consume from.
     # Returns
     * An `Option` containing the consumed message, or `None` if the topic is not found or empty.
     */
    pub fn consume(&mut self, topic_name: &str) -> Option<String> {
        let topic = self.topics.get_mut(topic_name);

        if let Some(topic) = topic {
            topic.consume()
        } else {
            eprintln!("Could not find topic: {}", topic_name);
            None
        }
    }

}
