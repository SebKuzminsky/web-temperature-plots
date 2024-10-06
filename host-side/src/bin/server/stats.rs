pub async fn poll_stats(
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
