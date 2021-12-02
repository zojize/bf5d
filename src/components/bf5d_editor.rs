use web_sys::HtmlParagraphElement;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct BF5DEditorProps {
    pub oninput: Callback<InputEvent>,
    pub instruction_pointers: Vec<usize>,
    pub value: String,
}

#[function_component(BF5DEditor)]
pub fn view(props: &BF5DEditorProps) -> Html {
    let oninput = props.oninput.clone();

    let editor_ref = use_node_ref();

    {
        let editor_ref = editor_ref.clone();
        use_effect_with_deps(
            move |value| {
                if let Some(editor) = editor_ref.cast::<HtmlParagraphElement>() {
                    // editor.set_inner_html(value.replace("\n", "<br />").as_str());
                };
                || ()
            },
            props.value.clone(),
        );
    }

    html! {
        <p
            ref={editor_ref}
            contenteditable="true"
            // disable system spell check
            spellcheck="false"
            // disable grammarly
            data-gramm="false"
            oninput={oninput}
            class="
                w-full h-50 min-h-16 max-h-100 p-2 pt-0
                resize-y overflow-auto
                border-2 border-black border-dashed
                leading-10
                font-cascadia tracking-widest break-all
                outline-none
            "
        />
    }
}
