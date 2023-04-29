use crate::components::atoms::buttons::button::{Button, ButtonVariant};
use yew::prelude::*;

const BASE_ASSET_URL: &str = "/assets/icons";

#[derive(PartialEq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn to_color(&self, icon: &str) -> String {
        self.to_color_with_ext(icon, None)
    }
    pub fn to_color_with_ext(&self, icon: &str, ext: Option<&str>) -> String {
        let suffix = match self {
            Color::Black => "-black",
            Color::White => "",
        };

        let extension = ext.unwrap_or("svg");
        format!("{}/{}{}.{}", BASE_ASSET_URL, icon, suffix, extension)
    }
}

#[derive(PartialEq)]
pub enum Icon {
    LeftArrow(Color),
    RightArrow(Color),
    DoubleLeftArrow(Color),
    DoubleRightArrow(Color),
    Settings(Color),
}

impl Icon {
    fn to_icon_path(&self) -> String {
        match self {
            Icon::LeftArrow(color) => color.to_color("chevron-left"),
            Icon::RightArrow(color) => color.to_color("chevron-right"),
            Icon::DoubleLeftArrow(color) => color.to_color("chevron-double-left"),
            Icon::DoubleRightArrow(color) => color.to_color("chevron-double-right"),
            Icon::Settings(color) => color.to_color("settings"),
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
            <img src={props.icon.to_icon_path()} alt={"icon"} />
                {for props.children.iter()}
            </>
        </Button>
    }
}
