use std::collections::VecDeque;

pub struct Topic {
    name: String,
    queue: VecDeque<String>
}

impl Topic {

    pub fn new(name: &str) -> Self {
        Topic { name: name.into(), queue: VecDeque::new() }
    }

    pub fn publish(&mut self, message: &str) {
        self.queue.push_back(message.into());
    }

    pub fn consume(&mut self) -> Option<String> {
        if let Some(message) = self.queue.pop_back() {
            Some(message)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}