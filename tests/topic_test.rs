#[cfg(test)]
mod tests {
    use mqtt_broker::{topic::Topic, traits::topic::BrokerTopic};

    #[test]
    fn test_publish_and_consume() {
        let mut topic = Topic::new("test_topic");

        // Test publishing and consuming a message
        topic.publish("test_topic", "Hello, World!");
        let consumed_message = topic.consume("test_topic").unwrap();

        assert_eq!(consumed_message, "Hello, World!");
    }

    #[test]
    #[should_panic]
    fn test_invalid_publish() {
        let mut topic = Topic::new("test_topic");

        // Attempt to publish to an invalid topic
        topic.publish("invalid_topic", "This should panic");
    }

    #[test]
    #[should_panic]
    fn test_invalid_consume() {
        let mut topic = Topic::new("test_topic");

        // Attempt to consume from an invalid topic
        topic.consume("invalid_topic");
    }
}