use crate::generated::chapter_map::get_chapters;

use std::{
    cmp::{max, min},
    collections::HashMap,
    rc::Rc,
};
use yew::prelude::*;

extern crate web_sys;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MangaBook {
    OnePiece,
}

impl MangaBook {
    pub fn to_url(&self, chapter: &i16, page: &i8) -> String {
        let manga_base = match self {
            MangaBook::OnePiece => "one_piece".to_owned(),
        };
        let prepend = if page < &10 { "0" } else { "" };
        format!(
            "/assets/manga/{}/{}/{}{}.jpg",
            manga_base, chapter, prepend, page
        )
    }
}

pub enum MangaAction {
    Prev,
    Next,
    PrevChapter,
    NextChapter,
    ChangeChapter(i16),
    ChangePage(i16),
    ChangePageManually(i16),
    SetChangedBy(ChangedBy),
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ChangedBy {
    Navigation,
    Manually,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MangaState {
    chapter_state: HashMap<i16, i8>,
    pub page: i8,
    pub total_pages: i8,
    pub total_chapters: i16,
    pub chapter: i16,
    pub manga: MangaBook,
    pub changed_by: ChangedBy,
}

pub type MangaContext = UseReducerHandle<MangaState>;

impl Default for MangaState {
    fn default() -> Self {
        let chapter_state = get_chapters();

        let chapter = chapter_state.keys().max().unwrap().to_owned();
        let total_chapters = chapter.clone();
        let total_pages = chapter_state.get(&chapter).unwrap().to_owned();

        Self {
            chapter_state,
            page: 1,
            chapter,
            total_pages,
            total_chapters,
            manga: MangaBook::OnePiece,
            changed_by: ChangedBy::Navigation,
        }
    }
}

impl Reducible for MangaState {
    // Reducer Action Type
    type Action = MangaAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            MangaAction::Prev => {
                Self {
                    page: max(self.page - 1, 1),
                    changed_by: ChangedBy::Navigation,
                    ..(*self).clone()
                }
            }
            .into(),
            MangaAction::Next => {
                let mut new_page = self.page + 1;
                let mut chapter = self.chapter;
                let mut total_pages = self.total_pages;

                if new_page > self.total_pages && chapter < self.total_chapters {
                    chapter = self.chapter + 1;
                    new_page = 1;
                    total_pages = *self.chapter_state.get(&chapter).unwrap_or(&1);
                }

                Self {
                    page: min(new_page, self.total_pages),
                    chapter,
                    total_pages,
                    changed_by: ChangedBy::Navigation,
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
                    changed_by: ChangedBy::Navigation,
                    ..(*self).clone()
                }
                .into()
            }
            MangaAction::NextChapter => {
                let next_chapter = max(self.chapter + 1, self.total_chapters);
                let total_pages = self.chapter_state.get(&next_chapter).unwrap_or(&1);
                Self {
                    page: 1,
                    chapter: next_chapter,
                    total_pages: *total_pages,
                    changed_by: ChangedBy::Navigation,
                    ..(*self).clone()
                }
                .into()
            }
            MangaAction::PrevChapter => {
                let prev_chapter = max(self.chapter - 1, 1);
                let total_pages = self.chapter_state.get(&prev_chapter).unwrap_or(&1);

                Self {
                    page: 1,
                    chapter: prev_chapter,
                    total_pages: *total_pages,
                    changed_by: ChangedBy::Navigation,
                    ..(*self).clone()
                }
                .into()
            }
            MangaAction::ChangePage(page) => {
                Self {
                    page: i8::try_from(page).unwrap(),
                    changed_by: ChangedBy::Navigation,
                    ..(*self).clone()
                }
            }
            .into(),
            MangaAction::SetChangedBy(changed_by) => Self {
                changed_by,
                ..(*self).clone()
            }
            .into(),
            MangaAction::ChangePageManually(page) => {
                let new_page = i8::try_from(page).unwrap();

                Self {
                    page: new_page,
                    changed_by: ChangedBy::Manually,
                    ..(*self).clone()
                }
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
