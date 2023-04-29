use yew::prelude::*;

#[derive(PartialEq, Debug, Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Primary
    }
}

impl ButtonVariant {
    fn to_class(&self) -> String {
        match self {
            ButtonVariant::Primary => "bg-cyan-500",
            ButtonVariant::Secondary => "bg-slate-200 dark:bg-darkness-secondary",
        }
        .to_string()
    }
}

#[derive(PartialEq, Debug, Properties)]
pub struct ButtonProps {
    #[prop_or(Callback::noop())]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub variant: ButtonVariant,
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
        "p-2 py-3 q px-5 rounded-lg {}  {} {}",
        props.variant.to_class(),
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
