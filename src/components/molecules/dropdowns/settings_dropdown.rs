use yew::prelude::*;

use crate::{
    components::{
        atoms::buttons::button::ButtonVariant,
        atoms::icon::Icon,
        molecules::{icon_button::icon_button::IconButton, icon_text_block::IconTextBlock},
    },
    states::{
        options::{use_manga_options_context, MangaOptionsAction, ReadingMode},
        state::{use_manga_context, MangaAction},
    },
    utils::utils::Align,
};

#[function_component(SettingsDropdown)]
pub fn settings_dropdown() -> Html {
    let options_state = use_manga_options_context().unwrap();
    let state = use_manga_context().unwrap();

    let set_page_view = {
        let options_state = options_state.clone();
        let state = state.clone();
        Callback::from(move |_| {
            options_state.dispatch(MangaOptionsAction::SetReadingMode(ReadingMode::Page));
            state.dispatch(MangaAction::SetDualPage(false));
        })
    };
    let set_scroller_view = {
        let options_state = options_state.clone();
        let state = state.clone();
        Callback::from(move |_| {
            options_state.dispatch(MangaOptionsAction::SetReadingMode(ReadingMode::Scroller));
            state.dispatch(MangaAction::SetDualPage(false));
        })
    };

    let set_grid_preview = {
        let options_state = options_state.clone();
        let state = state.clone();
        Callback::from(move |_| {
            options_state.dispatch(MangaOptionsAction::SetReadingMode(ReadingMode::GridPreview));
            state.dispatch(MangaAction::SetDualPage(false));
        })
    };
    let set_book_mode = {
        let options_state = options_state.clone();
        let state = state.clone();
        Callback::from(move |_| {
            options_state.dispatch(MangaOptionsAction::SetReadingMode(ReadingMode::Book));
            state.dispatch(MangaAction::SetDualPage(true));
        })
    };

    let icon_text_block =
        |reading_mode: ReadingMode, callback: Callback<MouseEvent>, icon: Icon, text: String| {
            if reading_mode == options_state.reading_mode {
                return html! {<></>};
            }
            html! {
                <IconTextBlock
                    on_click={callback}
                    icon={icon}
                    text={text} />
            }
        };

    html! {
        <>

        <div class="fixed top-2 right-2 [&>.dropdown-content]:hover:block
        [&>button]:hover:rounded-bl-none
        [&>button]:hover:rounded-br-none
        focus-within[&>.dropdown-content]:block
        [&>.dropdown-content]:focus-within:block
        
         ">
            <IconButton
                class="w-20"
                    icon={Icon::Settings}
                    icon_align={Align::Center}
                    variant={ButtonVariant::Secondary} />
            <div class="hidden
                        absolute z-10 dropdown-content bg-white dark:bg-darkness
                        hover:[&>div]:bg-slate-100 hover:[&>div]:dark:bg-darkness-primary
                        w-40 right-0
                        [&>div]:first-of-type:rounded-tl-lg
                        [&>div]:first-of-type:rounded-tr-lg
                        [&>div]:last-of-type:rounded-bl-lg
                        [&>div]:last-of-type:rounded-br-lg
                        
                        
            ">

          {icon_text_block(ReadingMode::Scroller, set_scroller_view.clone(), Icon::Scroll, "Scroll View".to_string())}
          {icon_text_block(ReadingMode::Page, set_page_view.clone(), Icon::Page, "Page View".to_string())}
          {icon_text_block(ReadingMode::GridPreview, set_grid_preview.clone(), Icon::Grid, "Grid Preview".to_string())}
          {icon_text_block(ReadingMode::Book, set_book_mode.clone(), Icon::Book, "Book Mode".to_string())}

            </div>
          </div>
          </>
    }
}
