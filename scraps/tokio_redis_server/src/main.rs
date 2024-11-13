use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    println!("Hello, Redis Server!");

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, client) = listener.accept().await.unwrap();
        println!("Got client {:?}", client);

        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Got: {:?}", frame);

        // respond with error
        let response = Frame::Error("unimplemented :(".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
