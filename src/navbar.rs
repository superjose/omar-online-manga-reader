use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct NavbarProps<'a> {
    page_state: &'a UseState<i32>,
}

pub fn Navbar<'a>(cx: Scope<'a, NavbarProps>) -> Element<'a> {
    cx.render(rsx! (
        div {
            button {
                "<"
            }
            button {
                ">"
            }
        }
    ))
}
