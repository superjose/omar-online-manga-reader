use std::{cmp::max, rc::Rc};

use yew::prelude::*;

pub enum MangaAction {
    Prev,
    Next,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MangaState {
    pub page: i32,
    pub chapter: i32,
}

pub type MangaContext = UseReducerHandle<MangaState>;

impl Default for MangaState {
    fn default() -> Self {
        Self {
            page: 1,
            chapter: 1043,
        }
    }
}

impl Reducible for MangaState {
    // Reducer Action Type
    type Action = MangaAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            MangaAction::Prev => Self {
                page: max(self.page - 1, 1),
                chapter: self.chapter,
            }
            .into(),
            MangaAction::Next => Self {
                page: self.page + 1,
                chapter: self.chapter,
            }
            .into(),
        }
    }
}

#[derive(PartialEq, Debug, Properties)]
pub struct MangaContextProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(MangaContextProvider)]
pub fn manga_context_provider(props: &MangaContextProps) -> Html {
    let state = use_reducer(MangaState::default);
    html! {
        <ContextProvider<MangaContext> context={state}>
            {props.children.clone()}
        </ContextProvider<MangaContext>>
    }
}

pub fn use_manga_context() -> impl Hook<Output = Option<UseReducerHandle<MangaState>>> {
    use_context::<MangaContext>()
}
