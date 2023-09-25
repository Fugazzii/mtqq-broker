pub trait BrokerTopic {
    fn publish(&mut self, message: &str);
    fn consume(&mut self) -> Option<String>;
}