
use std::time::Duration;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::Callback;
use yew::{html, Component, Context, Html};

use yew::prelude::*;
use gloo_console::log;
use futures::FutureExt;
use futures::{SinkExt, StreamExt};

//use testbench_util::orion::orion::Stats;



mod stats;
mod counter;



async fn make_up_stats(stats_cb: Callback<stats::Stats>) {
    log!("connecting to stats server");

    let mut stats = stats::Stats::default();

    // loop {
    //     stats.temperature += 0.9;
    //     stats_cb.emit(stats.clone());
    //     sleep(Duration::from_secs(1)).await;
    // }

    loop {
        let mut ws = match gloo_net::websocket::futures::WebSocket::open("ws://127.0.0.1:7655/") {
            Ok(ws) => ws,
            Err(e) => {
                log!(format!("error connecting: {:#?}", e));
                sleep(Duration::from_secs(1)).await;
                continue;
            },
        };
        println!("connected");

        loop {
            while let Some(Ok(gloo_net::websocket::Message::Text(msg))) = ws.next().await {
                log!(format!("read {:#?}", msg));
                stats = serde_json::from_str(&msg).unwrap();
                log!(format!("parsed {:#?}", stats));
                stats_cb.emit(stats.clone());
            }
            log!("disconnected");
        }


        // let length_delimited = tokio_util::codec::FramedRead::new(stream, tokio_util::codec::LengthDelimitedCodec::new());

        // let mut deserialized = tokio_serde::SymmetricallyFramed::new(
        //     length_delimited,
        //     tokio_serde::formats::SymmetricalJson::<yew_hello_world::Stats>::default()
        // );

        // loop {
        //     match deserialized.next().await {
        //         None => {
        //             println!("disconnected");
        //             break;
        //         },
        //         Some(Ok(s)) => println!("{:?}", s),
        //         Some(Err(e)) => {
        //             println!("eror while reading socket: {:?}", e);
        //             break;
        //         },
        //     }
        // }
    }
}


#[derive(Clone, PartialEq)]
pub struct App {
    stats: Option<stats::Stats>,
}


#[derive(Debug)]
pub enum Msg {
    Stats(stats::Stats),
    Ping,
}


async fn stats_soon() {
    log!("stats_soon");
    yew::platform::time::sleep(std::time::Duration::from_secs(2)).await;
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        log!("App::create()");

        // let stats_soon_future = stats_soon();
        // ctx.link().send_future(
        //     stats_soon_future.map(|_| {
        //         Msg::Stats(stats::Stats {
        //             temperature: 567.89,
        //         })
        //     })
        // );

        let stats_cb = ctx.link().callback(Msg::Stats);
        spawn_local(make_up_stats(stats_cb));

        log!("App::create() is done");
        Self {
            stats: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!(format!("App::update(msg={:#?})", msg));
        match msg {
            Msg::Stats(s) => {
                log!(format!("got stats: {:#?}", s));
                self.stats = Some(s);
            },
            Msg::Ping => log!("ping"),
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        log!("App::view()");
        html! {
            <>
                <counter::Counter/>
                <hr/>
                if let Some(stats) = self.stats {
                    <p>{format!("{:#?}", stats)}</p>
                } else {
                    <p>{"Waiting for stats..."}</p>
                }
            </>
        }
    }
}


fn main() {
    log!("started!");
    yew::Renderer::<App>::new().render();
}
