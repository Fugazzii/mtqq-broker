use bytes::BytesMut;
use tokio::net::{TcpListener, TcpStream};

struct Broker {
    listener: TcpListener
}

impl Broker {
    
    /** Create TCP socket for listening incoming streams */
    pub async fn listen((host, port): (&str, u32)) -> Self {
        let addr = format!("{}:{}", host, port);

        let listener = TcpListener::bind(addr)
            .await
            .expect("Failed to listen");

        Broker { listener }
    }

    /** Accept incoming stream */
    pub async fn accept(&self) -> TcpStream {
        let (stream, _) = self.listener
            .accept()
            .await
            .expect("Error in socket");

        stream
    }

}

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