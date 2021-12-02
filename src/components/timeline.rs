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

    html! {
        <ul class="flex gap-1 w-full overflow-x-scroll scrollbar-hidden">
            {
                for timeline.data_backwards
                    .iter()
                    .rev()
                    .zip((1..).map(isize::wrapping_neg))
                    .map(|(data, index)| {
                        html!{
                            <TimelineData { index } data={ data.clone() } />
                        }
                    })
            }
            {
                for timeline.data
                    .iter()
                    .enumerate()
                    .map(|(index, data)| {
                        html!{
                            <TimelineData index={ index as isize } data={ data.clone() } />
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
}

#[function_component(TimelineData)]
pub fn view_data(props: &TimelineDataProps) -> Html {
    html! {
        <li class="flex flex-col gap-1 items-center font-cascadia">
            <span class="
                relative
                flex items-center justify-center
                min-h-14 min-w-14
                border-2 border-dashed
                text-xl
            "
            >
            { props.data }
            </span>
            <span> { props.index } </span>
        </li>
    }
}
