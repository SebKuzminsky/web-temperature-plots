pub async fn poll_stats(
    stats: std::sync::Arc<tokio::sync::Mutex<web_temperature_plots::Stats>>
) {
    println!("polling for stats");

    loop {
        let mut locked_stats = stats.lock().await;

        for i in 0..=10 {
            let filename = format!("/sys/devices/virtual/thermal/thermal_zone{i}/temp");
            let contents = std::fs::read_to_string(&filename)
                .unwrap_or_else(|e|{todo!("failed to read file '{filename}': {e:?}")});
            let contents = contents.trim();
            let t = contents.parse::<f32>().unwrap_or_else(|e| { todo!("failed to parse f32 from '{contents}': {e:?}") }) / 1_000.0;
            locked_stats.temperatures[i] = t;
        }

        drop(locked_stats);

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}
