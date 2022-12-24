use yew::prelude::*;

#[derive(PartialEq, Debug, Properties)]
pub struct ButtonProps {
    #[prop_or(Callback::noop())]
    pub on_click: Callback<MouseEvent>,
    pub text: &'static str,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let on_click = props.on_click.clone();
    html! {
        <button
            class="p-2 bg-cyan-700  text-white w-32"
            onclick={move |e: MouseEvent| on_click.emit(e)}>
          {props.text}
        </button>
    }
}
