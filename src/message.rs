use bytes::{BytesMut, BufMut};

use crate::action::Action;

#[derive(Debug)]
pub struct Message {
    action: Action,
    topic: String,
    message: String
}

impl Message {

    pub fn new(action: Action, topic: &str, message: &str) -> Self {
        Message { action, topic: topic.to_string(), message: message.to_string() }
    }

    pub fn to_buffer(&self) -> BytesMut {
        
        let mut buffer = BytesMut::new();
        buffer.put_u8(match self.action {
            Action::Publish => 0,
            Action::Consume => 1,
        });

        buffer.put(self.topic.as_bytes());
        buffer.put(self.message.as_bytes());

        buffer
    }

}