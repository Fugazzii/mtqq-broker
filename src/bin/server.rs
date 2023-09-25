use bytes::BytesMut;
use tokio::net::{TcpListener, TcpStream};

struct Server {
    listener: TcpListener
}

impl Server {
    
    pub async fn listen((host, port): (&str, u32)) -> Self {
        let addr = format!("{}:{}", host, port);

        let listener = TcpListener::bind(addr)
            .await
            .expect("Failed to listen");

        Server { listener }
    }

    pub async fn accept(&self) -> TcpStream {
        let (stream, _) = self.listener
            .accept()
            .await
            .expect("Error in socket");

        stream
    }
}




#[tokio::main]
pub async fn main() {
    let server = Server::listen(("127.0.0.1", 3000)).await;

    loop {
        let stream = server
            .accept()
            .await;

        let mut buffer = BytesMut::with_capacity(1024);

        let _ = stream.try_read_buf(&mut buffer);

        println!("{:?}", buffer);
    }
    
}