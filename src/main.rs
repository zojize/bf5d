use std::rc::Rc;

use yew::prelude::*;

type Model = i32;

enum Msg {
    Increment,
    Decrement,
}

fn update(x: Rc<Model>, msg: Msg) -> Model {
    match msg {
        Msg::Increment => *x + 1,
        Msg::Decrement => *x - 1,
    }
}

#[function_component(App)]
fn view() -> Html {
    let counter = use_reducer(update, 0);

    let inc = {
        let counter = counter.clone();
        Callback::from(move |_| counter.dispatch(Msg::Increment))
    };
    let dec = {
        let counter = counter.clone();
        Callback::from(move |_| counter.dispatch(Msg::Decrement))
    };

    html! {
        <>
            <h1> { *counter } </h1>
            <button onclick={inc}> {"+"} </button>
            <button onclick={dec}> {"-"} </button>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
