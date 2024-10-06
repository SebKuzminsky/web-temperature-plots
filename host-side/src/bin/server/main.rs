mod websockets;
mod tcp;
mod error;


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


#[tokio::main]
async fn main() {
    let stats = std::sync::Arc::new(tokio::sync::Mutex::new(yew_hello_world::Stats::new()));

    let (_, _, _) = tokio::join!(
        tokio::spawn(poll_stats(std::sync::Arc::clone(&stats))),
        tokio::spawn(tcp::tcp_server(std::sync::Arc::clone(&stats))),
        tokio::spawn(websockets::ws_server(std::sync::Arc::clone(&stats))),
    );
}
