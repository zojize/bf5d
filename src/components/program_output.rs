use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct ProgramOutputProps {
    pub children: Children,
}

#[function_component(ProgramOutput)]
pub fn view(props: &ProgramOutputProps) -> Html {
    html! {
        <p
            // disable system spell check
            spellcheck="false"
            // disable grammarly
            data-gramm="false"
            class="
                w-full h-full p-2
                overflow-hidden
                border-2 border-black border-dashed
                font-cascadia tracking-widest break-normal
                outline-none
            "
        >
            { props.children.clone() }
        </p>
    }
}
