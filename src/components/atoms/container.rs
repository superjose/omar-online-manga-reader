use dioxus::prelude::*;
#[derive(Props)]
pub struct ContainerProps<'a> {
    children: Element<'a>,
}

pub fn Container<'a>(cx: Scope<'a, ContainerProps<'a>>) -> Element<'a> {
    cx.render(rsx! (
        div {
            class: "my-auto mx-auto md:w-3/5 w-full",
            &cx.props.children
        }
    ))
}
