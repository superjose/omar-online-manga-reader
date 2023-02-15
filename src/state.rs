use std::{cmp::max, collections::HashMap, fs, rc::Rc};
use yew::prelude::*;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub enum MangaAction {
    Prev,
    Next,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MangaState {
    chapter_state: HashMap<i32, i32>,
    pub page: i32,
    pub chapter: i32,
}

pub type MangaContext = UseReducerHandle<MangaState>;

impl Default for MangaState {
    fn default() -> Self {
        // let dir = "./assets/manga/one_piece";
        // let total_chapters = fs::read_dir(dir)
        //     .unwrap()
        //     .filter(|entry| entry.as_ref().unwrap().metadata().unwrap().is_file())
        //     .count();
        // log!("TOTAL CHAPTERS {}", total_chapters);
        Self {
            chapter_state: HashMap::new(),
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
                chapter_state: self.chapter_state.to_owned(),
            }
            .into(),
            MangaAction::Next => {
                log!("self.page {}", self.page);
                Self {
                    page: self.page + 1,
                    chapter: self.chapter,
                    chapter_state: self.chapter_state.to_owned(),
                }
                .into()
            }
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
