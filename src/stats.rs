use yew::prelude::*;
use gloo_console::log;

//use testbench_util::orion::orion::Stats;


#[derive(serde::Deserialize, Clone, Copy, Debug, Default, PartialEq)]
pub struct Stats {
    pub temperatures: [f32; 11],
}


#[derive(Properties, Clone, Copy, Debug, PartialEq)]
pub struct Props {
    pub stats: Stats,
}


#[function_component(StatsDisplay)]
pub fn stats_display(Props { stats }: &Props) -> Html {
    log!("StatsDisplay");
    html! {
        <p>{format!("{:#?}", *stats)}</p>
    }
}
