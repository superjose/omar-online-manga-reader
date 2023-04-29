use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlImageElement;
use yew::prelude::*;

use crate::state::MangaAction;

use super::state::use_manga_context;

// #[derive(PartialEq, Props)]
// pub struct MangaProps<'a> {
//     page_state: &'a UseState<i32>,
// }

#[function_component(Manga)]
pub fn manga() -> Html {
    let state = use_manga_context().unwrap();
    let img_ref = use_node_ref();
    // let page = cx.props.page_state;
    let chapter = state.chapter.to_owned();
    let page = state.page.to_owned();

    let prepend = if &page < &10 { "0" } else { "" };

    // let url = format!("/assets/manga/one_piece/1042/{}{}.jpg", prepend, &page);
    let url = format!(
        "/assets/manga/one_piece/{}/{}{}.jpg",
        chapter, prepend, page
    );

    {
        let img = img_ref.clone();
        let state = state.clone();
        use_effect(move || {
            let callback = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                state.dispatch(MangaAction::Next);
            }) as Box<dyn FnMut(_)>);

            let img_elem = img
                .cast::<HtmlImageElement>()
                .expect("image element not set");

            let listener = callback.as_ref().unchecked_ref();
            img_elem
                .add_event_listener_with_callback("click", listener)
                .unwrap();

            move || {
                let listener = callback.as_ref().unchecked_ref();
                img_elem
                    .remove_event_listener_with_callback("click", listener)
                    .unwrap();
            }
        });
    }

    html! {
        <div class="p-4 md:p-0">
            <img class="my-0 mx-auto" src={url} alt="manga" ref={img_ref} />
            <p class="text-center">{"Press Arrow Keys to move backwards and forward."}</p>
            <p class="text-center">{"Press the image to move forward."}</p>
        </div>
    }
}
