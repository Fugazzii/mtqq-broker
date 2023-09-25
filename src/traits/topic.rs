pub trait BrokerTopic {
    fn publish(&mut self, topic_name: &str, message: &str);
    fn consume(&mut self, topic_name: &str) -> Option<String>;
}