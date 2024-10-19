use yew::{html, Component, Context, Html};

use futures::StreamExt;
use gloo_console::log;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use yew::prelude::*;

mod counter;
mod stats;

async fn get_stats(stats_cb: yew::Callback<stats::Stats>) {
    loop {
        match get_stats_inner(&stats_cb).await {
            Ok(_) => (),
            Err(e) => {
                log!(format!("error getting stats: {:#?}", e));
            }
        };
        yew::platform::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

async fn get_stats_inner(stats_cb: &yew::Callback<stats::Stats>) -> Result<(), anyhow::Error> {
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
    // Optional index of right-hand side of the plot, follow new data
    // if None.
    x_max: Option<usize>,
}

impl Plot {
    pub fn default() -> Self {
        Self {
            canvas: NodeRef::default(),
            data: vec![],
            x_max: None,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct App {
    stats: Option<stats::Stats>,
    plots: Vec<Plot>,
    mousedown_cb: yew::Callback<web_sys::MouseEvent>,
    mouseup_cb: yew::Callback<web_sys::MouseEvent>,
}

#[derive(Debug)]
pub enum Msg {
    Stats(stats::Stats),
    Redraw,
    // plot index, MouseEvent, FIXME: maybe make this a struct?
    MouseEvent(usize, yew::events::MouseEvent),
    Ping,
}

fn log_mouse_event(e: &web_sys::MouseEvent) {
    // log!(format!("    (x, y): ({}, {})", e.x(), e.y()));
    // log!(format!(
    //     "    (screen_x, screen_y): ({}, {})",
    //     e.screen_x(),
    //     e.screen_y()
    // ));
    // log!(format!(
    //     "    (client_x, client_y): ({}, {})",
    //     e.client_x(),
    //     e.client_y()
    // ));
    // log!(format!(
    //     "    (offset_x, offset_y): ({}, {})",
    //     e.offset_x(),
    //     e.offset_y()
    // ));
    log!(format!(
        "    (movement_x, movement_y): ({}, {})",
        e.movement_x(),
        e.movement_y()
    ));
    // log!(format!("    related_target: {:?}", e.related_target()));
    // log!(format!("    region: {:?}", e.region()));
    // log!(format!("    view: {:?}", e.view()));
    // log!(format!("    detail: {:?}", e.detail()));
    // log!(format!("    which: {:?}", e.which()));
    // log!(format!("    target: {:?}", e.target()));
    // log!(format!("    current_target: {:?}", e.current_target()));
    log!(format!("    ctrl_key: {}", e.ctrl_key()));
    log!(format!("    alt_key: {}", e.alt_key()));
    log!(format!("    shift_key: {}", e.shift_key()));
    // log!(format!("    button: {}", e.button()));
    log!(format!("    buttons: {}", e.buttons()));

    // let unwrapped_target = e.target().unwrap().dyn_ref::<web_sys::HtmlCanvasElement>().unwrap().clone();
    // log!(format!("    unwrapped target: {:?}", unwrapped_target));
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        log!("App::create()");

        let stats_cb = ctx.link().callback(Msg::Stats);
        yew::platform::spawn_local(get_stats(stats_cb));

        let mousedown_cb = ctx.link().callback(|e: MouseEvent| {
            log!(format!("mouse down"));
            log_mouse_event(&e);
            // e.stop_propagation();
            Msg::Ping
        });

        let mouseup_cb = ctx.link().callback(|e: MouseEvent| {
            log!(format!("mouse up"));
            log_mouse_event(&e);
            // e.stop_propagation();
            Msg::Ping
        });

        log!("App::create() is done");
        Self {
            stats: None,
            plots: vec![],
            mousedown_cb,
            mouseup_cb,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // log!(format!("App::update(msg={:?})", msg));
        match msg {
            Msg::Stats(s) => {
                // log!(format!("got stats: {:#?}", s));
                self.stats = Some(s.clone());
                for (i, t) in s.temperatures.iter().enumerate() {
                    if i >= self.plots.len() {
                        self.plots.push(Plot::default());
                    }
                    let data_len = self.plots[i].data.len();
                    self.plots[i].data.push((data_len as f32, *t));
                }
                ctx.link().send_message(Msg::Redraw);
            }

            Msg::Redraw => {
                for (index, plot) in self.plots.iter().enumerate() {
                    // massage data into the format plotters wants
                    // let x_max = usize::max(plot.data.len() - 1, 0);
                    let x_max = match plot.x_max {
                        None => plot.data.len() - 1,
                        Some(x_max) => x_max,
                    };
                    let x_min = if x_max <= 10 { 0 } else { x_max - 10 };
                    let y_max = plot
                        .data
                        .iter()
                        .map(|item| item.1)
                        .reduce(f32::max)
                        .unwrap()
                        + 1.0;
                    let y_min = plot
                        .data
                        .iter()
                        .map(|item| item.1)
                        .reduce(f32::min)
                        .unwrap()
                        - 1.0;
                    let line_series =
                        LineSeries::new(plot.data[x_min..=x_max].iter().cloned(), &RED);

                    let element: web_sys::HtmlCanvasElement = match plot.canvas.cast() {
                        Some(element) => element,
                        None => continue,
                    };
                    let _rect = element.get_bounding_client_rect();
                    let window_width = web_sys::window()
                        .expect("There should be a window")
                        .inner_width()
                        .expect("the windows should have Some width")
                        .as_f64()
                        .expect("the width should be a numer")
                        as u32;
                    let window_height = web_sys::window()
                        .expect("There should be a window")
                        .inner_height()
                        .expect("the windows should have Some height")
                        .as_f64()
                        .expect("the height should be a number")
                        as u32;
                    let width = window_width - 30;
                    let height = std::cmp::min(window_height - 30, window_width * 3 / 4);
                    element.set_width(width);
                    element.set_height(height);

                    let backend = CanvasBackend::with_canvas_object(element).unwrap();
                    let drawing_area = backend.into_drawing_area();
                    drawing_area.fill(&RGBColor(240, 240, 240)).unwrap();

                    let mut chart = ChartBuilder::on(&drawing_area)
                        .caption(
                            format!("Temperature {index}"),
                            ("sans-serif", 14).into_font(),
                        )
                        .margin(5)
                        .x_label_area_size(25)
                        .y_label_area_size(50)
                        .build_cartesian_2d((x_min as f32)..(x_max as f32), y_min..y_max)
                        .unwrap();

                    chart
                        .configure_mesh()
                        .x_label_formatter(&|x| format!("{}", x.round()))
                        .y_label_formatter(&|y| format!("{y:.1}"))
                        .draw()
                        .unwrap();

                    chart
                        .draw_series(line_series)
                        .unwrap()
                        .label("y = x^2")
                        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
                }

                return false;
            }

            Msg::MouseEvent(plot_index, e) => {
                log!("handling MouseEvent message");
                log!(format!("    plot_index is {plot_index}"));
                log_mouse_event(&e);
                if (e.buttons() & 0x1_u16) != 0 {
                    log!("dragging with button 1");
                    self.plots[plot_index].x_max = match self.plots[plot_index].x_max {
                        None => {
                            if e.movement_x() > 0 {
                                let x_max: i32 =
                                    self.plots[plot_index].data.len() as i32 - e.movement_x();
                                let x_max = i32::max(0, x_max) as usize;
                                Some(x_max)
                            } else {
                                None
                            }
                        }
                        Some(old_x_max) => {
                            let x_max: i32 = old_x_max as i32 - e.movement_x();
                            if x_max >= self.plots[plot_index].data.len() as i32 {
                                None
                            } else if x_max < 0 {
                                Some(1)
                            } else {
                                Some(x_max as usize)
                            }
                        }
                    };
                    ctx.link().send_message(Msg::Redraw);
                }
            }

            Msg::Ping => log!("ping"),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // log!("App::view()");

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

        let stats_html = match &self.stats {
            None => html! {<p>{"Waiting for stats..."}</p>},
            Some(s) => html! {<p>{format!("{:#?}", s)}</p>},
        };

        html! {
            <>
                <counter::Counter/>
                <hr/>
                {stats_html}
                <hr/>
                <center>
                    {
                        self.plots.clone().into_iter().enumerate().map(|(i, p)| {
                            html! {
                                <canvas
                                    ref={p.canvas}
                                    onmousedown={&self.mousedown_cb}
                                    onmouseup={&self.mouseup_cb}
                                    onmousemove={ctx.link().callback(move |e| Msg::MouseEvent(i, e))}
                                />
                            }
                        }).collect::<Html>()
                    }
                </center>
            </>
        }
    }
}

fn main() {
    // log!("started!");
    yew::Renderer::<App>::new().render();
}
