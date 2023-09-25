use std::collections::VecDeque;
use async_trait::async_trait;

use crate::traits::topic::{PublishTopic, ConsumeTopic};

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

#[async_trait]
impl PublishTopic for Topic {
    async fn publish(&mut self, name: &str, message: &str) {

        if name.to_string() != self.name { panic!("Invalid name") }

        self.queue.push_back(message.into());
    }
}

#[async_trait]
impl ConsumeTopic for Topic {
    async fn consume(&mut self, name: &str) -> Option<String> {

        if name.to_string() != self.name { panic!("Invalid name") }

        if let Some(message) = self.queue.pop_back() {
            Some(message)
        } else {
            None
        }
    }
}