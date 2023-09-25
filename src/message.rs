use bytes::{BytesMut, BufMut};

use crate::action::Action;

#[derive(Debug)]
pub struct Message {
    action: Action,
    topic: String,
    message: Option<String>
}

impl Message {

    pub fn new(action: Action, topic: &str, message: Option<&str>) -> Self {

        let message = {
            if let Some(msg) = message {
                Some(msg.to_string())
            } else {
                None
            }
        };

        
        Message { 
            action,
            topic: topic.to_string(),
            message
        }
    }

    pub fn to_buffer(&self) -> BytesMut {
        
        let mut buffer = BytesMut::new();
        buffer.put_u8(match self.action {
            Action::Publish => 0,
            Action::Consume => 1,
        });

        buffer.put(self.topic.as_bytes());

        if let Some(msg) = &self.message {
            buffer.put(msg.as_bytes());
        }

        buffer
    }

}