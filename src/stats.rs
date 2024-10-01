use yew::prelude::*;
use gloo_console::log;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub temperature: f32,
}

#[function_component(Stats)]
pub fn stats(props: &Props) -> Html {
    log!("stats");
    html! {
        <p>{"stats: "}{props.temperature}</p>
    }
}
