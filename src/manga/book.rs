use yew::prelude::*;

use crate::states::state::use_manga_context;

struct BookPages {
    left_page: Option<String>,
    right_page: String,
}

#[function_component(Book)]
pub fn book() -> Html {
    let state = use_manga_context().unwrap();

    html! {
        <div>

            <div class="grid grid-cols-2">
                {if state.left_page == state.page {
                    html! {
                        <img src={state.get_current_url()} />
                    }
                } else {
                    html! {
                        <>
                            <img src={state.get_url(&state.left_page)} />
                            <img src={state.get_url(&state.page)} />
                        </>
                    }
                }}

            </div>
        </div>
    }
}
