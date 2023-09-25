use async_trait::async_trait;

#[async_trait]
pub trait PublishTopic {
    async fn publish(&mut self, topic_name: &str, message: &str);
}

#[async_trait]
pub trait ConsumeTopic {
    async fn consume(&mut self, topic_name: &str) -> Option<String>;
}