use futures::prelude::*;


#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("websockets error")]
    TWSError(#[from] tokio_websockets::Error),
    #[error("std::io error")]
    StdIoError(#[from] std::io::Error),
    #[error("serde_json error")]
    SerdeJsonError(#[from] serde_json::Error),
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


async fn handle_ws_client(
    stream: tokio_websockets::WebSocketStream<tokio::net::TcpStream>,
    stats: std::sync::Arc<tokio::sync::Mutex<yew_hello_world::Stats>>
) {
    let client_id = format!("{:?}", &stream);
    match handle_ws_client_inner(stream, stats).await {
        Ok(()) => println!("done with ws client {:?}: Ok!", client_id),
        Err(e) => println!("error with ws client {:?}: {:#?}", client_id, e),
    }
}


async fn handle_ws_client_inner(
    mut stream: tokio_websockets::WebSocketStream<tokio::net::TcpStream>,
    stats: std::sync::Arc<tokio::sync::Mutex<yew_hello_world::Stats>>
) -> Result<(), Error> {
    let client_id = format!("{:?}", &stream);
    loop {
        println!("sending to ws client {client_id}");
        let locked_stats = stats.lock().await;
        let json = serde_json::to_string(&*locked_stats)?;
        drop(locked_stats);
        println!("{}", json);
        stream.send(tokio_websockets::Message::text(json)).await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}


async fn poll_stats(
    stats: std::sync::Arc<tokio::sync::Mutex<yew_hello_world::Stats>>
) {
    println!("polling for stats");
    loop {
        let mut locked_stats = stats.lock().await;
        locked_stats.temperatures[0] += 0.0;
        locked_stats.temperatures[1] += 0.1;
        locked_stats.temperatures[2] += 0.2;
        drop(locked_stats);

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}


async fn tcp_server(
    stats: std::sync::Arc<tokio::sync::Mutex<yew_hello_world::Stats>>,
) {
    let listener = tokio::net::TcpListener::bind("localhost:7654").await.unwrap();
    println!("listening for TCP connections {:#?}", listener);
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("accepted tcp: {:#?}", socket);
        tokio::spawn(handle_tcp_client(socket, std::sync::Arc::clone(&stats)));
    }
}


async fn ws_server(
    stats: std::sync::Arc<tokio::sync::Mutex<yew_hello_world::Stats>>,
) {
    loop {
        match ws_server_inner(std::sync::Arc::clone(&stats)).await {
            Ok(()) => println!("ws_server returned ok??"),
            Err(e) =>  println!("ws_server returned error: {:#?}", e),
        };
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}


async fn ws_server_inner(
    stats: std::sync::Arc<tokio::sync::Mutex<yew_hello_world::Stats>>,
) -> Result<(), Error> {
    let tcp_listener = tokio::net::TcpListener::bind("localhost:7655").await?;
    println!("listening for WS connections {:#?}", tcp_listener);
    loop {
        let (tcp_stream, _) = tcp_listener.accept().await?;
        println!("ws_server accepted TCP connection");
        let ws_stream = tokio_websockets::ServerBuilder::new().accept(tcp_stream).await?;
        println!("accepted ws: {:#?}", ws_stream);
        tokio::spawn(handle_ws_client(ws_stream, std::sync::Arc::clone(&stats)));
    }
}


#[tokio::main]
async fn main() {
    let stats = std::sync::Arc::new(tokio::sync::Mutex::new(yew_hello_world::Stats::new()));

    // {
    //     let s = stats.lock().await;
    //     println!("stats = {:?}", *s);

    //     let js = serde_json::to_string(&*s).unwrap();
    //     println!("js = {}", js);

    //     let new_stats = serde_json::from_str::<yew_hello_world::Stats>(&js).unwrap();
    //     println!("back to s: {:?}", new_stats);
    // }

    let (_, _, _) = tokio::join!(
        tokio::spawn(poll_stats(std::sync::Arc::clone(&stats))),
        tokio::spawn(tcp_server(std::sync::Arc::clone(&stats))),
        tokio::spawn(ws_server(std::sync::Arc::clone(&stats))),
    );
}
