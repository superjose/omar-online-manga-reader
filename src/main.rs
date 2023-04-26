use yew::prelude::*;
mod chapter_map;
mod components;
mod manga;
mod navbar;
mod state;

use components::atoms::container::Container;
use manga::Manga;
use navbar::Navbar;
use state::MangaContextProvider;

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
        <MangaContextProvider>
           <Container>
                <Navbar />
                <Manga />
           </Container>
        </MangaContextProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
