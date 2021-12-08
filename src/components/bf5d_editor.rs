use gloo::utils::document;
use web_sys::{
    Document, HtmlDivElement, HtmlInputElement, HtmlParagraphElement, HtmlTextAreaElement, Node,
    Selection,
};
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
        let value = props.value.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(editor) = editor_ref.cast::<HtmlParagraphElement>() {
                    editor.set_inner_text(value.as_str());
                }
                || ()
            },
            (),
        );
    }

    // {
    //     let editor_ref = editor_ref.clone();
    //     use_effect_with_deps(
    //         move |value| {
    //             if let Some(editor) = editor_ref.cast::<HtmlParagraphElement>() {
    //                 // log::info!(
    //                 //     "{:?}",
    //                 //     value
    //                 //         .split("\n")
    //                 //         .map(|s| format!("<div>{}</div>\n", s))
    //                 //         .collect::<Vec<_>>()
    //                 // );
    //                 // let selection = Document::get_selection(&document());
    //                 // if let Ok(Some(selection)) = selection {
    //                 //     log::info!("{:?}", Selection::anchor_offset(&selection));
    //                 //     // Selection::anchor_offset(&selection);
    //                 // }
    //                 // editor.set_inner_html(
    //                 //     value
    //                 //         .split("\n")
    //                 //         .map(|s| format!("<div>{}</div>\n", s))
    //                 //         .collect::<String>()
    //                 //         .as_str(),
    //                 // );
    //             };
    //             || ()
    //         },
    //         props.value.clone(),
    //     );
    // }

    // let onkeydown = {
    //     Callback::from(move |e: KeyboardEvent| {
    //         let selection = Document::get_selection(&document());
    //         if let Ok(Some(selection)) = selection {
    //             log::info!(
    //                 "{:?}, {:?}",
    //                 Selection::anchor_offset(&selection),
    //                 Selection::anchor_node(&selection).map(Into::into).map(HtmlDivElement::from)
    //             );
    //             // Selection::anchor_offset(&selection);
    //         }
    //     })
    // };

    html! {
        <p
            ref={editor_ref}
            { oninput }
            // { onkeydown }
            contenteditable="true"
            // disable system spell check
            spellcheck="false"
            // disable grammarly
            data-gramm="false"
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
