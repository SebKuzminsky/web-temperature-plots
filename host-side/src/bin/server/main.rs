mod error;
mod stats;
mod tcp;
mod websockets;

#[tokio::main]
async fn main() {
    let stats = std::sync::Arc::new(tokio::sync::Mutex::new(web_temperature_plots::Stats::new()));

    let (_, _, _) = tokio::join!(
        tokio::spawn(stats::poll_stats(std::sync::Arc::clone(&stats))),
        tokio::spawn(tcp::tcp_server(std::sync::Arc::clone(&stats))),
        tokio::spawn(websockets::ws_server(std::sync::Arc::clone(&stats))),
    );
}
