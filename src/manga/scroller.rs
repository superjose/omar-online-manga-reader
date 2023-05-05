use yew::prelude::*;

use crate::{
    components::atoms::image_intersection::IntersectionImage,
    states::state::{use_manga_context, ChangedBy},
    utils::utils::scroll_into_view,
};

#[function_component(Scroller)]
pub fn scroller() -> Html {
    let state = use_manga_context().unwrap();

    let mut manga_pages = state.get_url_list_for_current_chapter();
    {
        let changed_by = state.changed_by.clone();
        let page = state.page.clone();
        use_effect_with_deps(
            move |_| {
                if changed_by == ChangedBy::Navigation {
                    scroll_into_view(&page)
                }
            },
            page,
        )
    }

    html! {
        <div class="p-4 md:p-0 grid grid-cols-1 place-content-center gap-4 justify-items-center">
            {manga_pages.iter().enumerate().map(|(index, page)| {
                html! {
                    <>
                        <a href={format!("#{}",&index + 1)} />
                        <p>{format!("Page #{}", &index + 1)}</p>
                        <IntersectionImage key={page.clone()} src={page.clone()}
                            page_number={index + 1}
                        />
                    </>
                }
                }).collect::<Html>()
            }
        </div>
    }
}
