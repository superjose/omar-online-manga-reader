#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChapterState {
    pub chapter: i16,
    pub page: i8,
    pub name: String,
    pub ext: String,
    pub is_dual: bool,
}

// Automatically generated - see build.rs
// Do not modify manually!

use std::collections::HashMap;
pub fn get_chapters_v2() -> HashMap<i16, Vec<ChapterState>> {
    let chapter_state: HashMap<i16, Vec<ChapterState>> = HashMap::from([(
        1082,
        vec![
            ChapterState {
                chapter: 1082,
                ext: ".png".into(),
                name: "random-page-name".into(),
                is_dual: true,
                page: 1,
            },
            ChapterState {
                chapter: 1082,
                ext: ".png".into(),
                name: "random-page-name".into(),
                is_dual: true,
                page: 2,
            },
        ],
    )]);
    return chapter_state;
}
