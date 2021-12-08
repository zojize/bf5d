use std::num::Wrapping;

use crate::interpreter::types;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct TimelineProps {
    pub timeline: types::Timeline,
}

#[function_component(Timeline)]
pub fn view(props: &TimelineProps) -> Html {
    let timeline = &props.timeline;
    let mut data = timeline
        .data_backwards
        .iter()
        .enumerate()
        .map(|(i, x)| (isize::wrapping_neg(i as isize + 1), x, 0))
        .rev()
        .chain(
            timeline
                .data
                .iter()
                .enumerate()
                .map(|(i, x)| (i as isize, x, 0)),
        )
        .collect::<Vec<_>>();

    let backwards_len = timeline.data_backwards.len() as isize;
    for ptr in timeline.pointers.iter() {
        let data = data.get_mut((ptr + backwards_len) as usize);
        if let Some(data) = data {
            data.2 += 1;
        }
    }

    html! {
        <ul class="flex gap-1 w-full overflow-x-scroll scrollbar-hidden">
            {
                for data
                    .iter()
                    .map(|(index, data, pointer_count)| {
                        html!{
                            <TimelineData
                                index={ index.clone() }
                                data={ (*data).clone() }
                                pointer_count={ pointer_count.clone() }
                            />
                        }
                    })
            }
        </ul>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct TimelineDataProps {
    pub data: Wrapping<u8>,
    pub index: isize,
    pub pointer_count: usize,
}

#[function_component(TimelineData)]
pub fn view_data(props: &TimelineDataProps) -> Html {
    html! {
        <li class="
            flex flex-col gap-1 items-center
            font-cascadia w-16 max-w-16
        ">
            <div
                class={
                    format!("
                        relative
                        flex items-center justify-center
                        min-h-14 min-w-14
                        border-2 border-dashed
                        text-xl
                        {}
                    ", if props.pointer_count > 0 { "pointed" } else { "" })
                }
                style={
                    format!(
                        "--pointers-count: \"{:?}\";",
                        props.pointer_count
                    )
                }
            >
                { props.data }
            </div>
            <span> { props.index } </span>
        </li>
    }
}
