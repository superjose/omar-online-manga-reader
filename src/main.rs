use yew::prelude::*;
// mod navbar;
mod components;
mod manga;
mod navbar;
mod state;

use components::atoms::container::Container;
use manga::Manga;
use navbar::Navbar;
use state::MangaContextProvider;

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

#[function_component(App)]
fn app() -> Html {
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
