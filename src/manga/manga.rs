use crate::{
    manga::{book::Book, grid_preview::GridPreview, scroller::Scroller, single::SingleManga},
    states::options::{use_manga_options_context, ReadingMode},
};
use yew::prelude::*;

#[function_component(Manga)]
pub fn manga() -> Html {
    let options_state = use_manga_options_context().unwrap();

    html! {
        match options_state.reading_mode {
            ReadingMode::Page => {
               html!{ <SingleManga />}
            }
            ReadingMode::Scroller => {
                html! {<Scroller />}
            }
            ReadingMode::GridPreview => {
                html! {<GridPreview />}
            }
            ReadingMode::Book => {
                html! {<Book /> }
            }
        }
    }
}
