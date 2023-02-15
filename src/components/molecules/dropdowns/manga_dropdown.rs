use super::dropdown::*;
use crate::state::{use_manga_context, MangaAction};

use yew::prelude::*;

#[function_component(MangaDropdown)]
pub fn manga_dropdown() -> Html {
    let state = use_manga_context().unwrap();

    let mut options = Vec::<SelectOption>::new();
    let chapter = state.chapter.to_owned();

    // When you include the "=" after the 2 dots,
    // it means that you're including the last value
    for i in 1..=chapter {
        let value = SelectOptionValue::Int(i);
        let label = i.to_string();
        options.push(SelectOption { label, value });
    }

    options.reverse();
    html! {
        <Dropdown
            options={options}
            selected={chapter}
        />
    }
}
