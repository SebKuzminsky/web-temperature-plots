use yew::prelude::*;
use gloo_console::log;

mod stats;

#[function_component]
fn App() -> Html {
    let temp_state = use_state(|| 1.234);
    let counter = use_state(|| 0);

    let temp_state_clone = temp_state.clone();
    let counter_clone = counter.clone();
    let plusonclick = move |_| {
        log!("clicked +1");
        temp_state_clone.set(*temp_state_clone + 1.0);
        counter_clone.set(*counter_clone + 1);
    };

    let temp_state_clone = temp_state.clone();
    let counter_clone = counter.clone();
    let minusonclick = move |_| {
        log!("clicked -1");
        temp_state_clone.set(*temp_state_clone - 0.1);
        counter_clone.set(*counter_clone - 1);
    };

    log!("App");

    html! {
        <div>
            <button onclick={plusonclick}>{ "+1 " }</button>
            <button onclick={minusonclick}>{ "-1 " }</button>
            <p>{ *counter }</p>
            <hr />
            <stats::Stats temperature={*temp_state}/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
