use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, HtmlImageElement};
use yew::prelude::*;

use crate::states::state::{use_manga_context, MangaAction};

#[function_component(SingleManga)]
pub fn single_manga() -> Html {
    let state = use_manga_context().unwrap();
    let img_ref = use_node_ref();
    // let page = cx.props.page_state;
    let chapter = state.chapter;
    let page = state.page;

    let url = state.manga.to_url(&chapter, &page);

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
    {
        use_effect_with_deps(
            move |_| {
                window().unwrap().scroll_to_with_x_and_y(0.0, 0.0);
            },
            page,
        );
    }
    html! {
        <div class="">
            <a href={format!("#{}",&page)} />
            <img class="my-0 mx-auto" src={url} alt="manga" ref={img_ref} />
            <p class="text-center">{"Press Arrow Keys to move backwards and forward."}</p>
            <p class="text-center">{"Press the image to move forward."}</p>
        </div>
    }
}
