use yew::prelude::*;

#[derive(PartialEq, Debug, Properties)]
pub struct ButtonProps {
    #[prop_or(Callback::noop())]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let on_click = props.on_click.clone();

    html! {
        <button
            class="p-2 bg-cyan-500  text-white py-3 px-5 rounded-lg"
            onclick={move |e: MouseEvent| on_click.emit(e)}
            disabled={props.disabled}
            >
          {for props.children.iter()}
        </button>
    }
}
