use yew::prelude::*;

use crate::states::state::use_manga_context;

struct BookPages {
    left_page: Option<String>,
    right_page: String,
}

#[function_component(Book)]
pub fn book() -> Html {
    let state = use_manga_context().unwrap();
    let total_pages = state.total_pages;

    let mut pages: Vec<BookPages> = Vec::new();

    for i in (1..=total_pages).step_by(2) {
        let left_page = if i == total_pages && total_pages % 2 != 0 {
            None
        } else {
            Some(state.manga.to_url(&state.chapter, &(i + 1)))
        };

        pages.push(BookPages {
            left_page: left_page,
            right_page: state.manga.to_url(&state.chapter, &i),
        });
    }

    html! {
        <div>
            {pages.iter().map(|page| {
                html! {
                    <div class="grid grid-cols-2">
                        {if let Some(left_page) = &page.left_page {
                            html! {
                                <img src={left_page.clone()} />
                            }
                        } else {
                            html! {}
                        }}

                        <img src={page.right_page.clone()} />
                    </div>
                }
            }).collect::<Html>()}
        </div>
    }
}
