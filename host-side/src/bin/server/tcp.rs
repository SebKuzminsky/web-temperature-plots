use futures::prelude::*;

use crate::error::Error;


pub async fn tcp_server(
    stats: std::sync::Arc<tokio::sync::Mutex<yew_hello_world::Stats>>,
) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7654").await.unwrap();
    println!("listening for TCP connections {:#?}", listener);
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("accepted tcp: {:#?}", socket);
        tokio::spawn(handle_tcp_client(socket, std::sync::Arc::clone(&stats)));
    }
}


async fn handle_tcp_client(
    socket: tokio::net::TcpStream,
    stats: std::sync::Arc<tokio::sync::Mutex<yew_hello_world::Stats>>
) {
    let client_id = format!("{:?}", &socket);

    let length_delimited = tokio_util::codec::FramedWrite::new(
        socket,
        tokio_util::codec::LengthDelimitedCodec::new()
    );

    let serialized = tokio_serde::SymmetricallyFramed::new(
        length_delimited,
        tokio_serde::formats::SymmetricalJson::<yew_hello_world::Stats>::default()
    );

    println!("sending to tcp client {client_id}");
    match handle_tcp_client_inner(serialized, stats).await {
        Ok(()) => println!("done with client {:?}: Ok!", client_id),
        Err(e) => println!("error with client {:?}: {:#?}", client_id, e),
    }
}


async fn handle_tcp_client_inner(
    mut serialized: tokio_serde::SymmetricallyFramed<
        tokio_util::codec::FramedWrite<tokio::net::TcpStream, tokio_util::codec::LengthDelimitedCodec>,
        yew_hello_world::Stats,
        tokio_serde::formats::Json<yew_hello_world::Stats, yew_hello_world::Stats>>,
    stats: std::sync::Arc<tokio::sync::Mutex<yew_hello_world::Stats>>,
) -> Result<(), Error> {
    loop {
        let locked_stats = stats.lock().await;
        println!("{:#?}", *locked_stats);
        serialized.send(*locked_stats).await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}
