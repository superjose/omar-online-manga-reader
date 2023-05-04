use std::{collections::HashMap, fs};

pub struct ChapterState {
    pub chapter: i16,
    pub page: i8,
    pub name: String,
    pub ext: String,
    pub is_dual: bool,
}

type MangaName = String;

/**
 * Recreates a HashMap<i16, i8> that from the directory structure of the manga
 * that is consumed by the state.rs
 */
fn main() {
    println!("cargo:rerun-if-changed=src/assets/manga");
    let out_dir = "./src/";

    let mut chapter_state: HashMap<MangaName, HashMap<i16, Vec<ChapterState>>> = HashMap::new();

    let dir_path = "./src/assets/manga/"; // replace with your directory path

    let manga_folders = fs::read_dir(dir_path).expect("Failed to read directory");

    for read in manga_folders {
        let entry = match read {
            Err(_) => continue,
            Ok(e) => e,
        };

        if !entry.path().is_dir() {
            continue;
        }

        let manga_name_option = entry.path().file_name().and_then(|n| n.to_str());
        // This isn't possible https://stackoverflow.com/questions/49784874/what-is-the-rust-way-of-using-continue-from-inside-a-closure
        // .map_or_else(|| continue, |name| name);

        let manga_name = match manga_name_option {
            None => continue,
            Some(name) => name,
        };

        let manga_chapters_dir = dir_path.to_owned() + manga_name;
        let err_msg = format!("Failed to read {}", manga_chapters_dir);
        let chapters_folders = fs::read_dir(manga_chapters_dir).expect(&err_msg);

        for chapter_folder in chapters_folders {
            if let Some(folder_entry) = chapter_folder
             && let Some(folder_name) = folder_entry
                .path()
                .file_name()
                .and_then(|name| (name.to_str()))
            && let Ok(chapter_number) = folder_name.parse::<i16>()
                
            {
                
            }
        }

        if let Some(folder_name) = entry.path().file_name().and_then(|n| n.to_str()) {
            if let Ok(folder_num) = folder_name.parse::<i16>() {
                let count = fs::read_dir(entry.path())
                    .expect("Failed to read directory")
                    .count() as i8;

                chapter_state.insert(folder_num, count);
            }
        }
    }

    // let mut sorted_manga_folders: Vec<(i16, i8)> = chapter_state.into_iter().collect();
    // sorted_manga_folders.sort_by_key(|&(chapter, _)| -chapter);

    // let mut chapter_concat: String = "[".to_owned();
    // for (index, (chapter, page)) in sorted_manga_folders.iter().enumerate() {
    //     let comma_suffix = if index == sorted_manga_folders.len() - 1 {
    //         ""
    //     } else {
    //         ","
    //     };
    //     chapter_concat.push_str(&format!("({}, {}){}", chapter, page, comma_suffix));
    // }

    // chapter_concat.push_str("]");

    println!("cargo:warning={:?}", &chapter_concat);

    let chapter_map_rs = format!(
        "
        // Automatically generated - see build.rs
        // Do not modify manually!

        use std::collections::HashMap;
        pub fn get_chapters() -> HashMap<i16, i8> {{
        let chapter_state: HashMap<i16, i8> = HashMap::from({});
            chapter_state
        }}
    
    ",
        chapter_concat
    );

    let dest_path = format!("{}/generated/chapter_map.rs", out_dir);
    fs::write(&dest_path, chapter_map_rs).unwrap();
}
