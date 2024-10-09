pub async fn poll_stats(stats: std::sync::Arc<tokio::sync::Mutex<web_temperature_plots::Stats>>) {
    println!("polling for stats");

    loop {
        let mut temps: Vec<f32> = vec![];

        let paths = match glob::glob("/sys/devices/virtual/thermal/thermal_zone*/temp") {
            Ok(paths) => paths,
            Err(e) => {
                println!("error globbing: {e:?}");
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                continue;
            }
        };

        for try_filename in paths {
            let filename = match try_filename {
                Ok(filename) => filename,
                Err(e) => {
                    println!("failed to get filename: {e:?}");
                    continue;
                }
            };
            let contents = std::fs::read_to_string(&filename)
                .unwrap_or_else(|e| todo!("failed to read file '{filename:?}': {e:?}"));
            let contents = contents.trim();
            let t = contents
                .parse::<f32>()
                .unwrap_or_else(|e| todo!("failed to parse f32 from '{contents}': {e:?}"))
                / 1_000.0;
            temps.push(t);
        }

        let mut locked_stats = stats.lock().await;
        locked_stats.temperatures = temps;
        drop(locked_stats);

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}
