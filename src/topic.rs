use std::collections::VecDeque;
use crate::traits::topic::BrokerTopic;

#[derive(Debug)]
pub struct Topic {
    queue: VecDeque<String>
}

impl Topic {
    pub fn new() -> Self {
        Topic { queue: VecDeque::new() }
    }
}

impl BrokerTopic for Topic {

    fn publish(&mut self, message: &str) {
        self.queue.push_back(message.into());
    }

    fn consume(&mut self) -> Option<String> {
        if let Some(message) = self.queue.pop_back() {
            Some(message)
        } else {
            None
        }
    }

}