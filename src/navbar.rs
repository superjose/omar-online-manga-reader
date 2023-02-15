use super::components::atoms::buttons::button::Button;
use super::components::molecules::dropdowns::manga_dropdown::MangaDropdown;
use yew::prelude::*;

use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};
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
    let win = window().expect_throw("window is undefined");

    use_effect(move || {
        let callback = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let key = e.key();
            let key_str = key.as_str();
            match key_str {
                "ArrowLeft" => {
                    state.dispatch(MangaAction::Prev);
                }
                "ArrowRight" => {
                    state.dispatch(MangaAction::Next);
                }
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);

        win.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())
            .unwrap();

        move || {
            win.remove_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())
                .unwrap();
        }
    });

    html! {
        <section class="mt-6 mb-6 flex justify-around">
            <Button text={"<"} on_click={go_prev} />
            <MangaDropdown />
            <Button text={">"} on_click={go_next} />
        </section>
    }
}
