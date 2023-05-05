use yew::prelude::*;

use crate::states::{
    options::{use_manga_options_context, MangaOptionsAction, ReadingMode},
    state::{use_manga_context, MangaAction},
};

#[function_component(GridPreview)]
pub fn grid_preview() -> Html {
    let state = use_manga_context().unwrap();
    let options = use_manga_options_context().unwrap();
    let current_page = state.page;
    let mut pages = state.get_url_list_for_current_chapter();

    html! {
        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 text-center
            
        ">{
            for pages.iter().enumerate().map(|(index, page)| {

                let page_selected = if (index as i8) == &current_page - 1 {
                    "bg-cyan-400 text-black"
                } else {
                    ""
                };
                let state = state.clone();
                let options = options.clone();
                html! {
                    <div class={format!("{} hover:bg-cyan-700 hover:text-white hover:dark:text-white hover:cursor-pointer p-4",page_selected)} key={page.clone()}
                        onclick={move |e: MouseEvent| {
                            e.prevent_default();
                            state.dispatch(MangaAction::ChangePage(index as i16 + 1));
                            options.dispatch(MangaOptionsAction::SetReadingMode(ReadingMode::Page));
                        }}
                    >
                        <p>{format!("Page {}", index +1)}</p>
                        <img class="my-0 mx-auto" src={page.clone()} width="250px" />
                    </div>
                }
            })
        }</div>
    }
}
