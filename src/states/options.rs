use std::rc::Rc;

use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ReadingMode {
    Page,
    Scroller,
}

pub enum MangaOptionsAction {
    SetReadingMode(ReadingMode),
    ToggleScrollView,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MangaOptionsState {
    pub reading_mode: ReadingMode,
}

pub type MangaOptionsContext = UseReducerHandle<MangaOptionsState>;

impl Default for MangaOptionsState {
    fn default() -> Self {
        Self {
            reading_mode: ReadingMode::Page,
        }
    }
}

impl Reducible for MangaOptionsState {
    type Action = MangaOptionsAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            MangaOptionsAction::SetReadingMode(reading_mode) => Self {
                reading_mode,
                ..(*self).clone()
            }
            .into(),
            MangaOptionsAction::ToggleScrollView => Self {
                reading_mode: match self.reading_mode {
                    ReadingMode::Page => ReadingMode::Scroller,
                    ReadingMode::Scroller => ReadingMode::Page,
                },
                ..(*self).clone()
            }
            .into(),
        }
    }
}

#[derive(PartialEq, Debug, Properties)]
pub struct MangaOptionsContextProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(MangaOptionsContextProvider)]
pub fn manga_options_context_provider(props: &MangaOptionsContextProps) -> Html {
    let state = use_reducer(MangaOptionsState::default);
    html! {
        <ContextProvider<MangaOptionsContext> context={state}>
            {props.children.clone()}
        </ContextProvider<MangaOptionsContext>>
    }
}

pub fn use_manga_options_context() -> impl Hook<Output = Option<UseReducerHandle<MangaOptionsState>>>
{
    use_context::<MangaOptionsContext>()
}
