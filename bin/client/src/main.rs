use futures::prelude::*;

#[tokio::main]
async fn main() {
    loop {
        let stream = tokio::net::TcpStream::connect("localhost:7654").await.unwrap();
        println!("connected");

        let length_delimited = tokio_util::codec::FramedRead::new(stream, tokio_util::codec::LengthDelimitedCodec::new());

        let mut deserialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            tokio_serde::formats::SymmetricalJson::<yew_hello_world::Stats>::default()
        );

        loop {
            match deserialized.next().await {
                None => println!("why am i awake??"),
                Some(Ok(s)) => println!("got s: {:?}", s),
                Some(Err(e)) => {
                    println!("eror while reading socket: {:?}", e);
                    return;
                },
            }
        }
    }
}
