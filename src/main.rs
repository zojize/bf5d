use gloo::storage::{LocalStorage, Storage};
use gloo::timers::callback::{Interval, Timeout};
use web_sys::{HtmlDivElement, HtmlInputElement};
extern crate console_error_panic_hook;

use yew::prelude::*;

mod components;
mod interpreter;
mod model;
mod parser;

use components::{
    bf5d_editor::BF5DEditor, program_input_editor::ProgramInputEditor,
    program_output::ProgramOutput, timeline::Timeline as TimelineComp,
};
use model::*;

#[function_component(App)]
fn view() -> Html {
    use Msg::*;
    // Model
    let model = use_reducer(Model::default);

    // Effects
    {
        use_effect_with_deps(
            move |raw_program| {
                LocalStorage::set(RAW_PROGRAM_KEY, raw_program)
                    .expect("failed setting LocalStorage");
                || ()
            },
            model.context.borrow().raw_program.clone(),
        );
    }

    // Callbacks
    let update_raw_program = {
        let dispatcher = model.clone();
        Callback::from(move |e: InputEvent| {
            dispatcher.dispatch(RawProgram(
                e.target_unchecked_into::<HtmlDivElement>().inner_text(),
            ))
        })
    };

    let update_program_input = {
        let dispatcher = model.clone();
        Callback::from(move |e: InputEvent| {
            dispatcher.dispatch(ProgramInput(
                e.target_unchecked_into::<HtmlInputElement>().value(),
            ))
        })
    };

    let on_parse = {
        let dispatcher = model.clone();
        Callback::from(move |_| dispatcher.dispatch(ParseUserInput))
    };

    let on_step = {
        let dispatcher = model.clone();
        Callback::from(move |_| dispatcher.dispatch(StepProgram))
    };

    let on_reset = {
        let dispatcher = model.clone();
        Callback::from(move |_| dispatcher.dispatch(ResetProgram))
    };

    let instruction_pointers: Vec<_> = model
        .timelines
        .clone()
        .borrow()
        .iter()
        .map(|t| (*t).instruction_pointer)
        .collect();

    html! {
        <main class="flex relative w-screen h-screen">
            <article class="relative flex flex-col flex-1 max-w-1/2 p-2 gap-2">
                <section class="flex flex-col">
                    <div class="flex items-center">
                        <h2> { "BF5D Program" } </h2>
                        <div class="flex-1" />
                        <button
                            class="w-6 h-6 cursor-pointer i-mdi-refresh"
                            title="refresh"
                            onclick={ on_reset }
                        />
                        <button
                            class="w-6 h-6 cursor-pointer i-mdi-play-outline"
                            title="parse"
                            onclick={ on_parse }
                        />
                        <button
                            class="w-6 h-6 cursor-pointer i-mdi-play"
                            title="run"
                            // onclick={ todo!() }
                        />
                        <button
                            class="w-6 h-6 cursor-pointer i-mdi-step-forward"
                            title="step"
                            onclick={ on_step }
                        />
                    </div>
                    <BF5DEditor
                        { instruction_pointers }
                        oninput={ update_raw_program }
                        value={ model.context.borrow().raw_program.clone() }
                    />
                </section>

                <section class="flex flex-col">
                    <h2> { "Program Input" } </h2>
                    <ProgramInputEditor
                        oninput={ update_program_input }
                        value={ model.context.borrow().program_input.clone() }
                    />
                </section>
                <section class="flex flex-col flex-1">
                    <h2> { "Program Input" } </h2>
                    <ProgramOutput>
                        { model.context.borrow().program_output.clone() }
                    </ProgramOutput>
                </section>
            </article>

            <article class="flex flex-col flex-1 max-w-1/2 p-2 overflow-scroll">
                <section class="flex flex-col">
                    <h2> { "Timelines" } </h2>
                    <section class="flex flex-col gap-2">
                        {
                            for model.timelines.clone().borrow().iter().map(|timeline| {
                                html! { <TimelineComp timeline={ timeline.clone() } /> }
                            })
                        }
                    </section>
                </section>

                <h2 class="mt-8"> { "Debug" } </h2>
                <pre class="max-h-100 overflow-scroll border-2 border-dashed p-2">
                    { format!("{:#?}", *model) }
                </pre>
            </article>
        </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    yew::start_app::<App>();
}
