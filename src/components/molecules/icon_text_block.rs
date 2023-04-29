use yew::prelude::*;

use crate::components::atoms::icon::{Icon, IconElem};

#[derive(PartialEq, Properties)]
pub struct IconTextBlockProps {
    pub icon: Icon,
    pub text: String,
    #[prop_or(Callback::noop())]
    pub on_click: Callback<MouseEvent>,
}

#[function_component(IconTextBlock)]
pub fn icon_text_block(props: &IconTextBlockProps) -> Html {
    html! {
     <div class="cursor-pointer p-4 flex flex-row gap-4" onclick={props.on_click.clone()}>
        <IconElem icon={props.icon.clone()} />
            {props.text.clone()}
      </div>
    }
}
