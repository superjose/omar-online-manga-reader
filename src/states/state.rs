use crate::generated::chapter_map::{get_chapters, ChapterState};
use gloo::console::log;

use std::{
    cmp::{max, min},
    collections::HashMap,
    rc::Rc,
};
use yew::prelude::*;

extern crate web_sys;

const BASE_PATH: &str = "/assets/manga";

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MangaBook {
    OnePiece,
}

impl MangaBook {
    pub fn to_state_key(&self) -> String {
        match self {
            MangaBook::OnePiece => "one_piece".to_owned(),
        }
    }
}

#[derive(PartialEq)]
enum Direction {
    Prev,
    Next,
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
    ToggleDualPage,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ChangedBy {
    Navigation,
    Manually,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MangaState {
    chapter_state: HashMap<String, HashMap<i16, Vec<ChapterState>>>,
    pub page: i8,
    pub left_page: i8,
    pub total_pages: i8,
    pub total_chapters: i16,
    pub chapter: i16,
    pub manga: MangaBook,
    pub changed_by: ChangedBy,
    pub dual_page_enabled: bool,
}

impl MangaState {
    fn get_chapter_total_pages(&self, chapter: &i16) -> i8 {
        *(&self
            .chapter_state
            .get(&self.manga.to_state_key())
            .unwrap()
            .get(chapter)
            .unwrap_or(&Vec::<ChapterState>::new())
            .len()) as i8
    }

    fn get_page(&self, page: &i8) -> &ChapterState {
        self.chapter_state
            .get(&self.manga.to_state_key())
            .unwrap()
            .get(&self.chapter)
            .unwrap()
            .get(usize::try_from(page - 1).unwrap())
            .unwrap()
    }

    fn has_reached_last_chapter(&self) -> bool {
        self.chapter == self.total_chapters
    }

    pub fn get_url(&self, page: &i8) -> String {
        let page = self.get_page(page);
        format!(
            "{}/{}/{}/{}",
            BASE_PATH,
            self.manga.to_state_key(),
            self.chapter,
            page.name
        )
        .into()
    }

    pub fn get_url_list_for_current_chapter(&self) -> Vec<String> {
        let mut pages = Vec::<String>::new();
        for i in 1..=self.total_pages {
            let url = self.get_url(&i);
            pages.push(url)
        }
        pages
    }

    pub fn get_current_url(&self) -> String {
        self.get_url(&self.page)
    }

    fn is_page_dual_page(&self, page: i8) -> bool {
        let curr_page = self.get_page(&page);
        curr_page.is_dual
    }

    fn advance_dual_page(&self, dir: Direction) -> (i8, i8) {
        let incr = if self.dual_page_enabled { 2 } else { 1 };
        let page_base = if self.dual_page_enabled {
            self.total_pages - self.total_pages % 2
        } else {
            self.total_pages
        };

        let page = match dir {
            Direction::Prev => max(self.page - incr, 1),
            Direction::Next => min(self.page + incr, page_base),
        };
        let left_page = match dir {
            Direction::Prev => max(self.left_page - incr, 2),
            Direction::Next => min(self.page + incr, self.total_pages),
        };

        return self.process_dual_page(page, left_page);
    }

    fn process_dual_page(&self, page: i8, left_page: i8) -> (i8, i8) {
        let is_page_dual_page = self.is_page_dual_page(page);
        let is_left_page_dual_page = self.is_page_dual_page(left_page);

        let (new_page, new_left_page) = if !self.dual_page_enabled {
            (page, page)
        } else if is_page_dual_page {
            (page, page)
        } else if !is_page_dual_page && is_left_page_dual_page {
            (page, page)
        } else if !is_page_dual_page && is_left_page_dual_page {
            (left_page, left_page)
        } else {
            (page, left_page)
        };

        (new_page, new_left_page)
    }
}

pub type MangaContext = UseReducerHandle<MangaState>;

impl Default for MangaState {
    fn default() -> Self {
        let chapter_state = get_chapters();
        let one_piece_state = chapter_state.get("one_piece").unwrap();
        let chapter = one_piece_state.keys().max().unwrap().to_owned();
        let total_chapters = chapter.clone();
        let total_pages = one_piece_state.get(&chapter).unwrap().len() as i8;

        Self {
            chapter_state,
            page: 1,
            left_page: 2,
            chapter,
            total_pages,
            total_chapters,
            manga: MangaBook::OnePiece,
            changed_by: ChangedBy::Navigation,
            dual_page_enabled: false,
        }
    }
}

impl Reducible for MangaState {
    // Reducer Action Type
    type Action = MangaAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            MangaAction::Prev => {
                let (page, left_page) = self.advance_dual_page(Direction::Prev);
                Self {
                    page,
                    left_page,
                    changed_by: ChangedBy::Navigation,
                    ..(*self).clone()
                }
            }
            .into(),
            MangaAction::Next => {
                let old_page = self.page;
                let (mut page, mut left_page) = self.advance_dual_page(Direction::Next);
                log!(
                    "old_page: {}, page: {}, left_page: {}",
                    old_page,
                    page,
                    left_page
                );
                let mut chapter = self.chapter;
                let mut total_pages = self.total_pages;

                if old_page == page && !self.has_reached_last_chapter() {
                    chapter = self.chapter + 1;
                    let (new_page, new_left_page) = self.process_dual_page(1, 2);
                    page = new_page;
                    left_page = new_left_page;
                    total_pages = self.get_chapter_total_pages(&chapter);
                }

                Self {
                    page,
                    left_page,
                    chapter,
                    total_pages,
                    changed_by: ChangedBy::Navigation,
                    ..(*self).clone()
                }
            }
            .into(),
            MangaAction::ChangeChapter(chapter) => {
                let total_pages = self.get_chapter_total_pages(&chapter);

                Self {
                    page: 1,
                    chapter,
                    total_pages,
                    changed_by: ChangedBy::Navigation,
                    ..(*self).clone()
                }
                .into()
            }
            MangaAction::NextChapter => {
                let next_chapter = min(self.chapter + 1, self.total_chapters);
                let total_pages = self.get_chapter_total_pages(&next_chapter);
                Self {
                    page: 1,
                    chapter: next_chapter,
                    total_pages,
                    changed_by: ChangedBy::Navigation,
                    ..(*self).clone()
                }
                .into()
            }
            MangaAction::PrevChapter => {
                let prev_chapter = max(self.chapter - 1, 1);
                let total_pages = self.get_chapter_total_pages(&prev_chapter);

                Self {
                    page: 1,
                    chapter: prev_chapter,
                    total_pages,
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
            MangaAction::ToggleDualPage => Self {
                dual_page_enabled: !self.dual_page_enabled,
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
