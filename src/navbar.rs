use crate::components::molecules::icon_button::icon_button::{Color, Icon, IconButton};

use super::components::molecules::dropdowns::manga_dropdown::MangaDropdown;
use yew::prelude::*;

use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};
use web_sys::window;

use super::state::{use_manga_context, MangaAction};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let state = use_manga_context().unwrap();

    let go_next_page_disabled = &state.page == &state.total_pages;
    let go_next_chapter_disabled = &state.chapter == &state.total_chapters;
    let prev_page_disabled = &state.page == &1;
    let prev_chapter_disabled = &state.chapter == &1;

    let go_prev_chapter = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(MangaAction::PrevChapter);
        })
    };

    let go_next_chapter = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(MangaAction::NextChapter);
        })
    };

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

    // This will be improved in future versions, when we decompose the svg
    // into its own component which we can pass the classes to.
    let icon_win = window().expect_throw("window is undefined");
    let is_dark = icon_win
        .match_media("(prefers-color-scheme: dark)")
        .unwrap_throw()
        .unwrap()
        .matches();

    let go_prev_chapter_icon =
        Icon::DoubleLeftArrow(get_icon_color(&prev_chapter_disabled, &is_dark));

    let go_prev_page_icon = Icon::LeftArrow(get_icon_color(&prev_chapter_disabled, &is_dark));

    let go_next_page_icon = Icon::RightArrow(get_icon_color(&prev_chapter_disabled, &is_dark));

    let go_next_chapter_icon =
        Icon::DoubleRightArrow(get_icon_color(&prev_chapter_disabled, &is_dark));

    html! {
        <section class="mt-6 mb-6 flex justify-around">
            <IconButton
                class="hidden sm:block"
                on_click={go_prev_chapter}
                icon={go_prev_chapter_icon}
                disabled={prev_chapter_disabled}
            />
            <IconButton
                on_click={go_prev}
                icon={go_prev_page_icon}
                disabled={prev_page_disabled}
            />
            <MangaDropdown />
            <IconButton
                on_click={go_next}
                icon={go_next_page_icon}
                disabled={go_next_page_disabled}
                />
            <IconButton
                class="hidden sm:block"
                on_click={go_next_chapter}
                icon={go_next_chapter_icon}
                disabled={go_next_chapter_disabled}
                />

        </section>
    }
}

fn get_icon_color(is_disabled: &bool, is_dark: &bool) -> Color {
    match is_disabled {
        true => match is_dark {
            true => return Color::Black,
            false => return Color::White,
        },

        false => match is_dark {
            true => return Color::White,
            false => return Color::Black,
        },
    }
}
