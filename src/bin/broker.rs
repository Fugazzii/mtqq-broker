use mqtt_broker::{broker::Broker, utils::buffer_to_array};
use bytes::BytesMut;
use tokio::io::AsyncReadExt;

/** Entry function for broker */
#[tokio::main]
pub async fn main() {
    let mut broker = Broker::listen(("127.0.0.1", 3000)).await;

    broker.add_topic("tanks");

    loop {

        /* Stop the thread until stream comes */
        let mut stream = broker
            .accept()
            .await;

        /* Create empty space for saving buffer */
        let mut buffer = BytesMut::with_capacity(1024);

        /* Write incoming stream into empty buffer space */
        let _ = stream.read_buf(&mut buffer).await.expect("Failed");

        let a = buffer_to_array(&mut buffer);

        println!("Received: {:?}", a);  
    }
    
}