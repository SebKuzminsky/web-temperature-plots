use std::time::Duration;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::Callback;
use yew::{html, Component, Context, Html};

use yew::prelude::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use gloo_console::log;
use futures::StreamExt;


mod stats;
mod counter;


async fn make_up_stats(stats_cb: Callback<stats::Stats>) {
    loop {
        match get_stats_inner(&stats_cb).await {
            Ok(_) => (),
            Err(e) => {
                log!(format!("error getting stats: {:#?}", e));
            },
        };
        sleep(Duration::from_secs(1)).await;
    }
}

async fn get_stats_inner(stats_cb: &Callback<stats::Stats>) -> Result<(), anyhow::Error> {
    let mut ws = gloo_net::websocket::futures::WebSocket::open("ws://127.0.0.1:7655/")?;
    println!("connected");
    loop {
        while let Some(Ok(gloo_net::websocket::Message::Text(msg))) = ws.next().await {
            let stats: stats::Stats = serde_json::from_str(&msg)?;
            stats_cb.emit(stats);
        }
        log!("disconnected");
    }
}


#[derive(Clone, PartialEq)]
pub struct App {
    stats: Option<stats::Stats>,
    canvas: NodeRef,
}


#[derive(Debug)]
pub enum Msg {
    Stats(stats::Stats),
    Redraw,
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
            canvas: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!(format!("App::update(msg={:?})", msg));
        match msg {
            Msg::Stats(s) => {
                // log!(format!("got stats: {:#?}", s));
                self.stats = Some(s);
                ctx.link().send_message(Msg::Redraw);
            },

            Msg::Redraw => {
                let element: HtmlCanvasElement = self.canvas.cast().unwrap();
                let rect = element.get_bounding_client_rect();
                log!(format!("bounding rectangle width={} height={}", rect.width(), rect.height()));
                element.set_height(rect.height() as u32);
                element.set_width(rect.width() as u32);
                let backend = CanvasBackend::with_canvas_object(element).unwrap();
                let drawing_area = backend.into_drawing_area();
                drawing_area.fill(&RGBColor(200,200,200)).unwrap();
                let mut chart = ChartBuilder::on(&drawing_area)
                    .caption("y=x^2", ("sans-serif", 14).into_font())
                    .margin(5)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(-1_f32..1_f32, -0.1_f32..1_f32).unwrap();
                chart.configure_mesh().draw().unwrap();
                chart
                    .draw_series(LineSeries::new(
                        (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x*x)),
                        &RED,
                    )).unwrap()
                    .label("y = x^2")
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
                return false;
            },

            Msg::Ping => log!("ping"),
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        log!("App::view()");

        // insert an svg into the html:
        // let svg_string = draw_things_with_plotters();
        // let js_svg = js! {
        //     var div = document.createElement("div");
        //     div.innerHTML = @{svg_string};
        //     return div;
        // };
        // let node = Node::try_from(js_svg).expect("convert js_svg");
        // let vnode = VNode::VRef(node);
        // vnode.into()

        html! {
            <>
                <counter::Counter/>
                <hr/>
                if let Some(stats) = self.stats {
                    <p>{format!("{:#?}", stats)}</p>
                } else {
                    <p>{"Waiting for stats..."}</p>
                }
                <hr/>
                <canvas ref={self.canvas.clone()}/>
            </>
        }
    }
}


fn main() {
    log!("started!");
    yew::Renderer::<App>::new().render();
}
