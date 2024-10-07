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


async fn get_stats(stats_cb: Callback<stats::Stats>) {
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


#[derive(Clone, Debug, PartialEq)]
struct Plot {
    canvas: NodeRef,
    data: Vec<(f32, f32)>,
}

impl Plot {
    pub fn default() -> Self {
        Self {
            canvas: NodeRef::default(),
            data: vec![],
        }
    }
}


#[derive(Clone, PartialEq)]
pub struct App {
    stats: Vec<stats::Stats>,
    plots: [Plot; 11],
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
        spawn_local(get_stats(stats_cb));

        log!("App::create() is done");
        Self {
            stats: vec![],
            plots: [
                Plot::default(),
                Plot::default(),
                Plot::default(),
                Plot::default(),
                Plot::default(),
                Plot::default(),
                Plot::default(),
                Plot::default(),
                Plot::default(),
                Plot::default(),
                Plot::default(),
            ],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!(format!("App::update(msg={:?})", msg));
        match msg {
            Msg::Stats(s) => {
                // log!(format!("got stats: {:#?}", s));
                self.stats.push(s);
                for i in 0..=10 {
                    self.plots[i].data.push((self.plots[i].data.len() as f32, s.temperatures[i]));
                }
                ctx.link().send_message(Msg::Redraw);
            },

            Msg::Redraw => {
                for (index, plot) in self.plots.iter().enumerate() {
                    // massage data into the format plotters wants
                    let x_max = plot.data.len() - 1;
                    let y_max = plot.data
                        .iter()
                        .map(|item| item.1)
                        .reduce(f32::max)
                        .unwrap() + 1.0;
                    let y_min = plot.data
                        .iter()
                        .map(|item| item.1)
                        .reduce(f32::min)
                        .unwrap() - 1.0;
                    let line_series = LineSeries::new(plot.data.clone(), &RED);

                    let element: HtmlCanvasElement = plot.canvas.cast().unwrap();
                    let _rect = element.get_bounding_client_rect();
                    element.set_width(600);
                    element.set_height(400);

                    let backend = CanvasBackend::with_canvas_object(element).unwrap();
                    let drawing_area = backend.into_drawing_area();
                    drawing_area.fill(&RGBColor(240,240,240)).unwrap();

                    let mut chart = ChartBuilder::on(&drawing_area)
                        .caption(format!("Temperature {index}"), ("sans-serif", 14).into_font())
                        .margin(5)
                        .x_label_area_size(25)
                        .y_label_area_size(50)
                        .build_cartesian_2d(0_f32..(x_max as f32), y_min..y_max).unwrap();

                    chart.configure_mesh()
                        .x_label_formatter(&|x| format!("{}", x.round()))
                        .y_label_formatter(&|y| format!("{y:.1}"))
                        .draw()
                        .unwrap();

                    chart
                        .draw_series(line_series).unwrap()
                        .label("y = x^2")
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
                }

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

        let stats_html = match self.stats.last() {
            None => html!{<p>{"Waiting for stats..."}</p>},
            Some(s) => html!{<p>{format!("{:#?}", s)}</p>},
        };

        html! {
            <>
                <counter::Counter/>
                <hr/>
                {stats_html}
                <hr/>
                <canvas ref={self.plots[0].canvas.clone()}/>
                <canvas ref={self.plots[1].canvas.clone()}/>
                <canvas ref={self.plots[2].canvas.clone()}/>
                <canvas ref={self.plots[3].canvas.clone()}/>
                <canvas ref={self.plots[4].canvas.clone()}/>
                <canvas ref={self.plots[5].canvas.clone()}/>
                <canvas ref={self.plots[6].canvas.clone()}/>
                <canvas ref={self.plots[7].canvas.clone()}/>
                <canvas ref={self.plots[8].canvas.clone()}/>
                <canvas ref={self.plots[9].canvas.clone()}/>
                <canvas ref={self.plots[10].canvas.clone()}/>
            </>
        }
    }
}


fn main() {
    log!("started!");
    yew::Renderer::<App>::new().render();
}
