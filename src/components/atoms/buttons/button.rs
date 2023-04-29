use yew::prelude::*;

#[derive(PartialEq, Debug, Properties)]
pub struct ButtonProps {
    #[prop_or(Callback::noop())]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: String,
}

impl ButtonProps {
    fn to_disable(&self) -> String {
        if self.disabled {
            // I'll add these to tailwind safelist.
            "bg-slate-200 dark:bg-darkness-disabled".to_string()
        } else {
            "bg-cyan-500".to_string()
        }
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let on_click = props.on_click.clone();
    let class = format!(
        "p-2 py-3 q px-5 rounded-lg  {} {}",
        props.class,
        props.to_disable()
    );
    html! {
        <button
            class={class}
            onclick={move |e: MouseEvent| on_click.emit(e)}
            disabled={props.disabled}
            >
          {for props.children.iter()}
        </button>
    }
}
