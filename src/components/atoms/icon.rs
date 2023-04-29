// This will be improved in future versions, when we decompose the svg
// into its own component which we can pass the classes to.

use crate::utils::utils::Color;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;
use yew::prelude::*;

const BASE_ASSET_URL: &str = "/assets/icons";

fn get_icon_color(is_dark: &bool, is_disabled: &bool) -> Color {
    match is_disabled {
        true => match is_dark {
            true => return Color::Black,
            false => return Color::White,
        },

        false => match is_dark {
            true => return Color::White,
            false => return Color::Black,
        },
    }
}

fn get_color(icon: &str, is_dark: &bool, is_disabled: &bool) -> String {
    get_color_with_ext(icon, is_dark, is_disabled, None)
}
fn get_color_with_ext(icon: &str, is_dark: &bool, is_disabled: &bool, ext: Option<&str>) -> String {
    let icon_color = get_icon_color(is_dark, is_disabled);
    let suffix = match icon_color {
        Color::Black => "-black",
        Color::White => "",
    };

    let extension = ext.unwrap_or("svg");
    format!("{}/{}{}.{}", BASE_ASSET_URL, icon, suffix, extension)
}

#[derive(PartialEq, Clone)]
pub enum Icon {
    LeftArrow,
    RightArrow,
    DoubleLeftArrow,
    DoubleRightArrow,
    Settings,
}

impl Icon {
    fn to_icon_path(&self, is_dark: &bool, is_disabled: &bool) -> String {
        let map_color = |icon: &str| get_color(icon, is_dark, is_disabled);
        match self {
            Icon::LeftArrow => map_color("chevron-left"),
            Icon::RightArrow => map_color("chevron-right"),
            Icon::DoubleLeftArrow => map_color("chevron-double-left"),
            Icon::DoubleRightArrow => map_color("chevron-double-right"),
            Icon::Settings => map_color("settings"),
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct IconProps {
    pub icon: Icon,
    #[prop_or_default]
    pub class: String,
}

#[function_component(IconElem)]
pub fn icon(props: &IconProps) -> Html {
    let icon_win = window().expect_throw("window is undefined");
    let is_dark = icon_win
        .match_media("(prefers-color-scheme: dark)")
        .unwrap_throw()
        .unwrap()
        .matches();
    html! {
        <img src={props.icon.to_icon_path(&is_dark, &false)} class={props.class.clone()} alt={"icon"} />
    }
}
