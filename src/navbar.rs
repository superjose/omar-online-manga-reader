use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use std::cmp::{max, min};

#[derive(PartialEq, Props)]
pub struct NavbarProps<'a> {
    page_state: &'a UseState<i32>,
}

pub fn Navbar<'a>(cx: Scope<'a, NavbarProps<'a>>) -> Element<'a> {
    let go_next = move |_: UiEvent<MouseData>| cx.props.page_state.modify(|val| min(val + 1, 17));
    let go_prev = move |_: UiEvent<MouseData>| cx.props.page_state.modify(|val| max(val - 1, 1));

    cx.render(rsx! (
        div {
            button {
                onclick: go_prev,
                "<",
            }
            button {
                onclick: go_next,
                ">"
            }
        }
    ))
}
