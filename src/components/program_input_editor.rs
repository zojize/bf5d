use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct ProgramInputEditorProps {
    pub oninput: Callback<InputEvent>,
    pub value: String,
}

#[function_component(ProgramInputEditor)]
pub fn view(props: &ProgramInputEditorProps) -> Html {
    let oninput = &props.oninput;
    let value = props.value.clone();

    html! {
        <textarea
            // disable system spell check
            spellcheck="false"
            // disable grammarly
            data-gramm="false"
            oninput={oninput}
            value={value}
            class="
                w-full max-w-full h-14 px-2
                leading-14
                overflow-hidden
                border-2 border-black border-dashed
                resize-none
                font-cascadia tracking-widest break-normal
                outline-none
            "
        />
    }
}
