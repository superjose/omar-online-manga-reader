use super::components::atoms::buttons::button::Button;
use yew::prelude::*;

use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;

use super::state::{use_manga_context, MangaAction};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let state = use_manga_context().unwrap();

    let go_prev = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(MangaAction::Prev);
        })
    };

    let go_next = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(MangaAction::Next);
        })
    };

    html! {
        <section class="mt-6 mb-6 flex justify-around">
            <Button text={"<"} on_click={go_prev} />
            <Button text={">"} on_click={go_next} />
        </section>
    }
}
