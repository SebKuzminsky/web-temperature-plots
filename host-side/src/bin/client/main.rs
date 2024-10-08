use futures::prelude::*;

#[tokio::main]
async fn main() {
    loop {
        let stream = match tokio::net::TcpStream::connect("localhost:7654").await {
            Ok(stream) => stream,
            Err(_) => {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                continue;
            },
        };
        println!("connected");

        let length_delimited = tokio_util::codec::FramedRead::new(stream, tokio_util::codec::LengthDelimitedCodec::new());

        let mut deserialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            tokio_serde::formats::SymmetricalJson::<web_temperature_plots::Stats>::default()
        );

        loop {
            match deserialized.next().await {
                None => {
                    println!("disconnected");
                    break;
                },
                Some(Ok(s)) => println!("{:?}", s),
                Some(Err(e)) => {
                    println!("eror while reading socket: {:?}", e);
                    break;
                },
            }
        }
    }
}
