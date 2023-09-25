use mqtt_broker::broker::Broker;
use bytes::BytesMut;

/** Entry function for broker */
#[tokio::main]
pub async fn main() {
    let broker = Broker::listen(("127.0.0.1", 3000)).await;

    loop {

        /* Stop the thread until stream comes */
        let stream = broker
            .accept()
            .await;

        /* Create empty space for saving buffer */
        let mut buffer = BytesMut::with_capacity(1024);

        /* Write incoming stream into empty buffer space */
        let _ = stream.try_read_buf(&mut buffer);

        /* Convert buffer to string */
        let message = String::from_utf8_lossy(&buffer);

        println!("{:?}", message);
    }
    
}