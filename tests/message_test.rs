#[cfg(test)]
mod tests {
    use bytes::{BytesMut, BufMut};
    use mqtt_broker::{message::Message, action::Action};

    #[test]
    fn test_to_buffer_publish() {
        let message = Message::new(Action::Publish, "test_topic", "test_message");
        let buffer = message.to_buffer();
        let mut bytes = BytesMut::new();
        bytes.put_u8(0); // Expected binary representation for Action::Publish
        bytes.put("test_topic".as_bytes());
        bytes.put("test_message".as_bytes());
        assert_eq!(buffer, bytes);
    }

    #[test]
    fn test_to_buffer_consume() {
        let message = Message::new(Action::Consume, "test_topic", "test_message");
        let buffer = message.to_buffer();
        let mut bytes = BytesMut::new();
        bytes.put_u8(1); // Expected binary representation for Action::Consume
        bytes.put("test_topic".as_bytes());
        bytes.put("test_message".as_bytes());
        assert_eq!(buffer, bytes);
    }
}
