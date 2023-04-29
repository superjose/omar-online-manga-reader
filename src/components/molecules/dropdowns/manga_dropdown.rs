use super::dropdown::*;
use crate::states::state::{use_manga_context, MangaAction};

use yew::prelude::*;

#[function_component(MangaDropdown)]
pub fn manga_dropdown() -> Html {
    let state = use_manga_context().unwrap();

    let mut options = Vec::<SelectOption>::new();
    let total_chapters = state.total_chapters.to_owned();
    let chapter = state.chapter.to_owned();

    let total_pages: i8 = state.total_pages.to_owned();
    let mut pages_options = Vec::<SelectOption>::new();

    // When you include the "=" after the 2 dots,
    // it means that you're including the last value
    for i in 1..=total_chapters {
        let value = SelectOptionValue::Int(i);
        let label = i.to_string();
        options.push(SelectOption { label, value });
    }

    for i in 1..=total_pages {
        let value = SelectOptionValue::Int(i as i16);
        let label = i.to_string();
        pages_options.push(SelectOption { label, value });
    }

    let handle_chapter_change = {
        let state = state.clone();
        Callback::from(move |value: i16| {
            state.dispatch(MangaAction::ChangeChapter(value));
        })
    };

    let handle_page_change = {
        let state = state.clone();
        Callback::from(move |value: i16| {
            state.dispatch(MangaAction::ChangePage(value));
        })
    };

    options.reverse();

    html! {
       <div class="flex gap-4">
            <Dropdown
                options={options}
                selected={chapter}
                onchange={handle_chapter_change}
                options_reversed={true}
            />
            <Dropdown
                options={pages_options}
                selected={state.page as i16}
                onchange={handle_page_change}
            />
       </div>
    }
}
