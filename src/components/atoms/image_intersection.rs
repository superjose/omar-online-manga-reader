use gloo::utils::window;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use yew::prelude::*;

use web_sys::{
    HtmlImageElement, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
};

use crate::states::state::{use_manga_context, ChangedBy, MangaAction};

#[derive(Debug, PartialEq, Properties)]
pub struct IntersectionImageProps {
    pub src: String,
    pub page_number: usize,
}

#[function_component(IntersectionImage)]
pub fn intersection_image(props: &IntersectionImageProps) -> Html {
    let img_ref = use_node_ref();
    let state = use_manga_context().unwrap();

    {
        let state = state.clone();
        let img = img_ref.clone();
        let page_number = props.page_number.clone();
        let changed_by = state.changed_by.clone();
        use_effect(move || {
            let mut options = IntersectionObserverInit::new();
            options.threshold(&JsValue::from(0.5));

            let img = img.cast::<HtmlImageElement>().expect("image not set");
            let callback = Closure::wrap(Box::new(
                move |entries: Vec<JsValue>, _observer: IntersectionObserver| {
                    for entry in entries {
                        let entry = IntersectionObserverEntry::from(entry);
                        let is_intersecting = entry.is_intersecting();
                        if is_intersecting && changed_by == ChangedBy::Manually {
                            state.dispatch(MangaAction::ChangePageManually(
                                page_number.try_into().unwrap(),
                            ));
                        }
                    }
                },
            )
                as Box<dyn FnMut(Vec<JsValue>, IntersectionObserver)>);

            let observer =
                IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
                    .unwrap();

            observer.observe(&img);

            move || {
                callback.forget();
                observer.unobserve(&img);
            }
        });
    }
    // As soon as the user starts to scroll in ScrollView, we reset the
    // ChangedBy to Manually, which is set to Navigation whenever the user
    // interacts with the buttons, keyboard or dropdowns.
    {
        use_effect(|| {
            let callback = Closure::wrap(Box::new(move |_: web_sys::Event| {
                state.dispatch(MangaAction::SetChangedBy(ChangedBy::Manually));
            }) as Box<dyn FnMut(_)>);

            let listener = callback.as_ref().unchecked_ref();
            window()
                .add_event_listener_with_callback("scroll", listener)
                .unwrap();
            move || {
                let listener = callback.as_ref().unchecked_ref();
                window()
                    .remove_event_listener_with_callback("scroll", listener)
                    .unwrap();
            }
        });
    }

    html! {
        <img src={props.src.clone()}
            ref={img_ref.clone()}
             alt={format!("Manga Page {}", props.page_number.clone())}
        />
    }
}
