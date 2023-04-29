use yew::prelude::*;

use crate::{
    components::{
        atoms::buttons::button::ButtonVariant,
        atoms::icon::Icon,
        molecules::{icon_button::icon_button::IconButton, icon_text_block::IconTextBlock},
    },
    states::options::{use_manga_options_context, MangaOptionsAction, ReadingMode},
    utils::utils::Align,
};

#[function_component(SettingsDropdown)]
pub fn settings_dropdown() -> Html {
    let options_state = use_manga_options_context().unwrap();

    let toggle_scroll_view = {
        let options_state = options_state.clone();
        Callback::from(move |_| {
            options_state.dispatch(MangaOptionsAction::ToggleScrollView);
        })
    };

    html! {
        <div class="fixed top-2 right-2 absolute [&>.dropdown-content]:hover:block
        [&>button]:hover:rounded-bl-none
        [&>button]:hover:rounded-br-none
            ">
            <IconButton
                class="w-20"
                    icon={Icon::Settings}
                    icon_align={Align::Center}
                    variant={ButtonVariant::Secondary} />
            <div class="hidden
                        absolute z-10 dropdown-content bg-white dark:bg-darkness
                        w-40 right-0
                        [&>div]:first-of-type:rounded-tl-lg
                        [&>div]:first-of-type:rounded-tr-lg
                        [&>div]:last-of-type:rounded-bl-lg
                        [&>div]:last-of-type:rounded-br-lg
                        
                        
            ">
            {
                if options_state.reading_mode == ReadingMode::Page {
                    html! {
                        <IconTextBlock
                            on_click={toggle_scroll_view}
                            icon={Icon::Scroll}
                            text="Scroll View" />
                    }
                } else {
                    html! {
                        <IconTextBlock
                            on_click={toggle_scroll_view}
                            icon={Icon::Page}
                            text="Page View" />
                    }
                }
            }

            </div>
          </div>
    }
}
