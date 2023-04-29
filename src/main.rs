use yew::prelude::*;
mod components;
mod generated;
mod manga;
mod navbar;
mod states;
mod utils;

use components::atoms::container::Container;
use manga::manga::Manga;
use navbar::Navbar;
use states::{options::MangaOptionsContextProvider, state::MangaContextProvider};

/**
 * Specify <link data-trunk rel="copy-dir" href="src/assets" />
 * in the index.html to copy the files!!
 *
 * You'll see them in the dist directory!
 */

#[function_component(App)]
fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());
    html! {
        <MangaOptionsContextProvider>
            <MangaContextProvider>
                <Container>
                        <Navbar />
                        <Manga />
                </Container>
            </MangaContextProvider>
        </MangaOptionsContextProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
