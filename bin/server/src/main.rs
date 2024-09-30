use futures::prelude::*;

#[tokio::main]
async fn main() {
    let stats = yew_hello_world::Stats {
        temperatures: [ 0.1, 1.0, 2.0, 3.0, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9, 10.10 ],
    };
    println!("stats = {:?}", stats);

    let js = serde_json::to_string(&stats).unwrap();
    println!("js = {}", js);

    let new_stats = serde_json::from_str::<yew_hello_world::Stats>(&js).unwrap();
    println!("back to s: {:?}", new_stats);

    let listener = tokio::net::TcpListener::bind("localhost:7654").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("accepted");

        tokio::spawn(async move {
            let length_delimited = tokio_util::codec::FramedWrite::new(socket, tokio_util::codec::LengthDelimitedCodec::new());

            let mut serialized = tokio_serde::SymmetricallyFramed::new(
                length_delimited,
                tokio_serde::formats::SymmetricalJson::<yew_hello_world::Stats>::default()
            );

            while let Ok(_) = serialized.send(stats.clone()).await {
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            }
            println!("something went wrong with the socket, bye");
        });
    }
}
