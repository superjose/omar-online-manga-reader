use crate::components::atoms::{
    buttons::button::{Button, ButtonVariant},
    icon::{Icon, IconElem},
};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct IconButtonProps {
    #[prop_or(Callback::noop())]
    pub on_click: Callback<MouseEvent>,
    pub icon: Icon,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub variant: ButtonVariant,
}

#[function_component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    html! {
        <Button
            class={props.class.clone()}
            on_click={&props.on_click}
            disabled={&props.disabled}
            variant={props.variant.clone()}
            >
            <>
            <IconElem icon={props.icon.clone()}/>
                {for props.children.iter()}
            </>
        </Button>
    }
}
