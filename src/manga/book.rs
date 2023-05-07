use yew::prelude::*;

use crate::states::state::use_manga_context;

#[function_component(Book)]
pub fn book() -> Html {
    let state = use_manga_context().unwrap();
    let class = match state.left_page == state.page {
        true => "grid-cols-1",
        false => "grid-cols-2",
    };
    html! {
        <div>

            <div class={format!("grid {}", class)}>
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
