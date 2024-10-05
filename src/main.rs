use std::time::Duration;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::Callback;
use yew::{html, Component, Context, Html};

use yew::prelude::*;
use gloo_console::log;
use futures::StreamExt;


mod stats;
mod counter;


async fn make_up_stats(stats_cb: Callback<stats::Stats>) {
    log!("connecting to stats server");

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
                let stats: stats::Stats = serde_json::from_str(&msg).unwrap();
                stats_cb.emit(stats);
            }
            log!("disconnected");
        }
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


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        log!("App::create()");

        let stats_cb = ctx.link().callback(Msg::Stats);
        spawn_local(make_up_stats(stats_cb));

        log!("App::create() is done");
        Self {
            stats: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!(format!("App::update(msg={:?})", msg));
        match msg {
            Msg::Stats(s) => {
                // log!(format!("got stats: {:#?}", s));
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
