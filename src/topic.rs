use std::collections::VecDeque;
use crate::traits::topic::BrokerTopic;

#[derive(Debug)]
pub struct Topic {
    name: String,
    queue: VecDeque<String>
}

impl Topic {
    pub fn new(name: &str) -> Self {
        Topic { name: name.into(), queue: VecDeque::new() }
    }
}

impl BrokerTopic for Topic {

    fn publish(&mut self, name: &str, message: &str) {

        if name.to_string() != self.name { panic!("Invalid name") }

        self.queue.push_back(message.into());
    }

    fn consume(&mut self, name: &str) -> Option<String> {

        if name.to_string() != self.name { panic!("Invalid name") }

        if let Some(message) = self.queue.pop_back() {
            Some(message)
        } else {
            None
        }
    }

}