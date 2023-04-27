use super::chapter_map::get_chapters;
use gloo::utils::window;
use std::{
    cmp::{max, min},
    collections::HashMap,
    rc::Rc,
};
use yew::prelude::*;

extern crate web_sys;

pub enum MangaAction {
    Prev,
    Next,
    ChangeChapter(i16),
    ChangePage(i16),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MangaState {
    chapter_state: HashMap<i16, i8>,
    pub page: i8,
    pub total_pages: i8,
    pub total_chapters: i16,
    pub chapter: i16,
}

pub type MangaContext = UseReducerHandle<MangaState>;

impl Default for MangaState {
    fn default() -> Self {
        let chapter_state = get_chapters();

        let chapter = chapter_state.keys().max().unwrap().to_owned();
        let total_chapters = chapter.clone();
        let total_pages = chapter_state.get(&chapter).unwrap().to_owned();
        // let dir = "./assets/manga/one_piece";
        // let total_chapters = fs::read_dir(dir)
        //     .unwrap()
        //     .filter(|entry| entry.as_ref().unwrap().metadata().unwrap().is_file())
        //     .count();
        // log!("TOTAL CHAPTERS {}", total_chapters);
        Self {
            chapter_state,
            page: 1,
            chapter,
            total_pages,
            total_chapters,
        }
    }
}

impl Reducible for MangaState {
    // Reducer Action Type
    type Action = MangaAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let win = window();
        win.scroll_to_with_x_and_y(0.0, 0.0);
        match action {
            MangaAction::Prev => Self {
                page: max(self.page - 1, 1),
                ..(*self).clone()
            }
            .into(),
            MangaAction::Next => {
                let mut new_page = self.page + 1;
                let mut chapter = self.chapter;

                if new_page > self.total_pages && chapter < self.total_chapters {
                    chapter = self.chapter + 1;
                    new_page = 1;
                }

                Self {
                    page: min(new_page, self.total_pages),
                    chapter,
                    ..(*self).clone()
                }
            }
            .into(),
            MangaAction::ChangeChapter(chapter) => {
                let total_pages = self.chapter_state.get(&chapter).unwrap_or(&1);

                Self {
                    page: 1,
                    chapter,
                    total_pages: *total_pages,
                    ..(*self).clone()
                }
                .into()
            }
            MangaAction::ChangePage(page) => Self {
                page: i8::try_from(page).unwrap(),
                ..(*self).clone()
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
