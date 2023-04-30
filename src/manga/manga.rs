use crate::{
    manga::{book::Book, grid_preview::GridPreview, scroller::Scroller, single::SingleManga},
    states::options::{use_manga_options_context, ReadingMode},
};
use yew::prelude::*;

#[function_component(Manga)]
pub fn manga() -> Html {
    let options_state = use_manga_options_context().unwrap();

    html! {
        if options_state.reading_mode == ReadingMode::Page {
            <SingleManga />
        } else if options_state.reading_mode == ReadingMode::Scroller {
            <Scroller />
        }
        else if options_state.reading_mode == ReadingMode::GridPreview {
            <GridPreview />
        }
        else if options_state.reading_mode == ReadingMode::Book {
            <Book />
        }
        else {
            <div>{"Hola 😊"}</div>
        }
    }
}
