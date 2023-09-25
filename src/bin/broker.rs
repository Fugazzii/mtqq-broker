use mqtt_broker::{broker::Broker, utils::buffer_to_array, action::Action, traits::topic::{PublishTopic, ConsumeTopic}};
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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

        let arguments_vec = buffer_to_array(&mut buffer);

        let action = match arguments_vec.len() {
            1 => Action::Consume,
            2 => Action::Publish,
            _ => {
                /* If received invalid arguments, just ignore this connection and continue loop */
                eprintln!("Server received invalid arguments");
                continue;
            }
        };

        match action {
            Action::Consume => {
                let _ = stream.write_all(b"c ");
                let topic_name = &arguments_vec[0];
                let _ = broker
                    .consume(topic_name.as_str())
                    .await;
            },
            Action::Publish => {
                let _ = stream.write_all(b"p ");
                let topic_name = &arguments_vec[0];
                let message = &arguments_vec[0];
                let _ = broker
                    .publish(
                        topic_name.as_str(),
                        message.as_str()
                    )
                    .await;
            }
        }

        println!("{:?}", arguments_vec);

        // match std::str::from_utf8(&mut buffer) {
        //     Ok(resp) => println!("{}", resp),
        //     Err(err) => eprintln!("{:?}", err)
        // }
        
    }
    
}