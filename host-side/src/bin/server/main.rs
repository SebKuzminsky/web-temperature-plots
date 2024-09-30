use futures::prelude::*;


async fn handle_client(
    socket: tokio::net::TcpStream,
    stats: std::sync::Arc<tokio::sync::Mutex<yew_hello_world::Stats>>
) -> Result<(), std::io::Error> {
    let client_id = format!("{:?}", &socket);

    let length_delimited = tokio_util::codec::FramedWrite::new(
        socket,
        tokio_util::codec::LengthDelimitedCodec::new()
    );

    let mut serialized = tokio_serde::SymmetricallyFramed::new(
        length_delimited,
        tokio_serde::formats::SymmetricalJson::<yew_hello_world::Stats>::default()
    );

    loop {
        println!("sending to {client_id}");
        {
            let locked_stats = stats.lock().await;
            serialized.send(*locked_stats).await?;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}


async fn poll_stats(
    stats: std::sync::Arc<tokio::sync::Mutex<yew_hello_world::Stats>>
) -> Result<(), std::io::Error> {
    loop {
        {
            let mut locked_stats = stats.lock().await;
            locked_stats.temperatures[0] += 0.0;
            locked_stats.temperatures[1] += 0.1;
            locked_stats.temperatures[2] += 0.2;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}


#[tokio::main]
async fn main() {
    let stats = std::sync::Arc::new(tokio::sync::Mutex::new(yew_hello_world::Stats::new()));

    {
        let s = stats.lock().await;
        println!("stats = {:?}", *s);

        let js = serde_json::to_string(&*s).unwrap();
        println!("js = {}", js);

        let new_stats = serde_json::from_str::<yew_hello_world::Stats>(&js).unwrap();
        println!("back to s: {:?}", new_stats);
    }

    tokio::spawn(poll_stats(std::sync::Arc::clone(&stats)));

    let listener = tokio::net::TcpListener::bind("localhost:7654").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("accepted");
        tokio::spawn(handle_client(socket, std::sync::Arc::clone(&stats)));
    }
}
