use futures::prelude::*;

#[tokio::main]
async fn main() {
    let s = yew_hello_world::Stats {
        temperatures: [ 0.1, 1.0, 2.0, 3.0, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9, 10.10 ],
    };
    println!("s = {:?}", s);

    let js = serde_json::to_string(&s).unwrap();
    println!("js = {}", js);

    let new_s = serde_json::from_str::<yew_hello_world::Stats>(&js).unwrap();
    println!("back to s: {:?}", new_s);

    let listener = tokio::net::TcpListener::bind("localhost:7654").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("accepted");

        let length_delimited = tokio_util::codec::FramedRead::new(socket, tokio_util::codec::LengthDelimitedCodec::new());

        let mut deserialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            tokio_serde::formats::SymmetricalJson::<yew_hello_world::Stats>::default()
        );

        tokio::spawn(async move {
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
        });
    }
}
