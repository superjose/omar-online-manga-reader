use yew::prelude::*;

use super::state::use_manga_context;

// #[derive(PartialEq, Props)]
// pub struct MangaProps<'a> {
//     page_state: &'a UseState<i32>,
// }

#[function_component(Manga)]
pub fn manga() -> Html {
    let state = use_manga_context().unwrap();

    // let page = cx.props.page_state;
    let chapter = state.chapter.to_owned();
    let page = state.page.to_owned();

    let prepend = if &page < &10 { "0" } else { "" };

    // let url = format!("/assets/manga/one_piece/1042/{}{}.jpg", prepend, &page);
    let url = format!(
        "/assets/manga/one_piece/{}/{}{}.jpg",
        chapter, prepend, page
    );
    // let url = "/assets/manga/one_piece/1042/01.jpg";
    html! {
        <img src={url} alt="manga" />
    }
    // cx.render(rsx! {
    //     img {
    //         src: "{url}"
    //     }
    // })
}
