use gloo_console::log;
use yew::prelude::*;

#[function_component]
pub fn Counter() -> Html {
    let counter_state = use_state(|| 0);

    let counter_state_clone = counter_state.clone();
    let plusonclick = move |_| {
        log!("+1");
        counter_state_clone.set(*counter_state_clone + 1);
    };

    let counter_state_clone = counter_state.clone();
    let minusonclick = move |_| {
        log!("-1");
        counter_state_clone.set(*counter_state_clone - 1);
    };

    log!("Counter");
    html! {
        <div>
            <button onclick={plusonclick}>{ "+1 " }</button>
            <button onclick={minusonclick}>{ "-1 " }</button>
            <p>{ *counter_state }</p>
        </div>
    }
}
