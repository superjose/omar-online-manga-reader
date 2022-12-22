use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct MangaProps<'a> {
    page_state: &'a UseState<i32>,
}

pub fn Manga<'a>(cx: Scope<'a, MangaProps<'a>>) -> Element<'a> {
    let page = cx.props.page_state;

    let prepend = if (page.get() < &10) { "0" } else { "" };

    let url = format!("/assets/manga/one_piece/1042/{}{}.jpg", prepend, &page);

    cx.render(rsx! {
        img {
            src: "{url}"
        }
    })
}
