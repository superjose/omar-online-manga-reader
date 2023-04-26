use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    html! {
        <div class="my-auto mx-auto md:w-4/5 w-full">
            {props.children.clone()}
        </div>
    }
}
