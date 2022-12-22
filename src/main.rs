use dioxus::{core::UiEvent, events::*, prelude::*};
mod navbar;
use log::LevelFilter;
use std::cmp::{max, min};
mod manga;

// impl PartialOrd for UseState<i32> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for UseState<i32> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.x.cmp(&other.x)
//     }
// }

/**
 * Specify <link data-trunk rel="copy-dir" href="src/assets" />
 * in the index.html to copy the files!!
 *
 * You'll see them in the dist directory!
 */

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let mut index = use_state(&cx, || 1);
    let val = index.get() as &i32;

    let change_evt = move |evt: KeyboardEvent| match evt.key.as_str() {
        "ArrowRight" => index += 1,
        "ArrowLeft" => index.modify(|val| max(1, *val - 1)), //index = max(&num.try_into(), &val.try_into()),
        _ => {}
    };

    cx.render(rsx!(
        div {
            class: "display",
            onkeydown: change_evt,
            navbar::Navbar {
                page_state: &index,
            }
            manga::Manga {
                page_state: &index,
            }

        },
    ))
}
