use yew::prelude::*;

use crate::states::state::use_manga_context;

#[function_component(Scroller)]
pub fn scroller() -> Html {
    let state = use_manga_context().unwrap();

    let mut manga_pages: Vec<String> = Vec::new();
    let chapter = state.chapter;
    let total_pages = state.total_pages.to_owned();
    for i in 1..=total_pages {
        let page = state.manga.to_url(&chapter, &i);
        manga_pages.push(page);
    }

    html! {
        <div class="p-4 md:p-0 grid grid-cols-1 place-content-center gap-4 justify-items-center">
            {manga_pages.iter().enumerate().map(|(index, page)| {
                html! {
                    <>
                        <p>{format!("Page #{}", index + 1)}</p>
                        <img key={page.clone()} src={page.clone()} />
                    </>
                }
                }).collect::<Html>()
            }
        </div>
    }
}
