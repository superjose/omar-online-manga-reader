use crate::components::atoms::buttons::button::Button;
use yew::prelude::*;

const BASE_ASSET_URL: &str = "/assets/icons";

#[derive(PartialEq)]
pub enum Icon {
    LeftArrow,
    RightArrow,
    DoubleLeftArrow,
    DoubleRightArrow,
}

impl Icon {
    fn to_icon_path(&self) -> String {
        match self {
            Icon::LeftArrow => format!("{}/{}", BASE_ASSET_URL, "chevron-left.svg"),
            Icon::RightArrow => format!("{}/{}", BASE_ASSET_URL, "chevron-right.svg"),
            Icon::DoubleLeftArrow => format!("{}/{}", BASE_ASSET_URL, "chevron-double-left.svg"),
            Icon::DoubleRightArrow => format!("{}/{}", BASE_ASSET_URL, "chevron-double-right.svg"),
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct IconButtonProps {
    #[prop_or(Callback::noop())]
    pub on_click: Callback<MouseEvent>,
    pub icon: Icon,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    html! {
        <Button on_click={&props.on_click}>
            <>
            <img src={props.icon.to_icon_path()} alt={"icon"} />
                {for props.children.iter()}
            </>
        </Button>
    }
}
