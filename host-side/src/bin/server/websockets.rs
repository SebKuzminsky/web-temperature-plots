use futures::prelude::*;

use crate::error::Error;

pub async fn ws_server(stats: std::sync::Arc<tokio::sync::Mutex<web_temperature_plots::Stats>>) {
    loop {
        match ws_server_inner(std::sync::Arc::clone(&stats)).await {
            Ok(()) => println!("ws_server returned ok??"),
            Err(e) => println!("ws_server returned error: {:#?}", e),
        };
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn ws_server_inner(
    stats: std::sync::Arc<tokio::sync::Mutex<web_temperature_plots::Stats>>,
) -> Result<(), Error> {
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:7655").await?;
    println!("listening for WS connections {:#?}", tcp_listener);
    loop {
        let (tcp_stream, _) = tcp_listener.accept().await?;
        println!("ws_server accepted TCP connection");
        let ws_stream = tokio_websockets::ServerBuilder::new()
            .accept(tcp_stream)
            .await?;
        println!("accepted ws: {:#?}", ws_stream);
        tokio::spawn(handle_ws_client(ws_stream, std::sync::Arc::clone(&stats)));
    }
}

async fn handle_ws_client(
    stream: tokio_websockets::WebSocketStream<tokio::net::TcpStream>,
    stats: std::sync::Arc<tokio::sync::Mutex<web_temperature_plots::Stats>>,
) {
    let client_id = format!("{:?}", &stream);
    match handle_ws_client_inner(stream, stats).await {
        Ok(()) => println!("done with ws client {:?}: Ok!", client_id),
        Err(e) => println!("error with ws client {:?}: {:#?}", client_id, e),
    }
}

async fn handle_ws_client_inner(
    mut stream: tokio_websockets::WebSocketStream<tokio::net::TcpStream>,
    stats: std::sync::Arc<tokio::sync::Mutex<web_temperature_plots::Stats>>,
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
