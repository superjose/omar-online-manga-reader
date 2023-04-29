use crate::manga::single::SingleManga;
use yew::prelude::*;

#[function_component(Manga)]
pub fn manga() -> Html {
    html! {
        <SingleManga />
    }
}
