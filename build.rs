use std::{collections::HashMap, fs};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let out_dir = "./src/";

    let mut chapter_state: HashMap<i16, i8> = HashMap::new();

    let dir_path = "./src/assets/manga/one_piece"; // replace with your directory path

    let manga_folders = fs::read_dir(dir_path).expect("Failed to read directory");

    for read in manga_folders {
        let entry = match read {
            Err(_) => continue,
            Ok(e) => e,
        };

        if !entry.path().is_dir() {
            continue;
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

    let mut sorted_manga_folders: Vec<(i16, i8)> = chapter_state.into_iter().collect();
    sorted_manga_folders.sort_by_key(|&(chapter, _)| -chapter);

    let mut chapter_concat: String = "[".to_owned();
    for (index, (chapter, page)) in sorted_manga_folders.iter().enumerate() {
        let comma_suffix = if index == sorted_manga_folders.len() - 1 {
            ""
        } else {
            ","
        };
        chapter_concat.push_str(&format!("({}, {}){}", chapter, page, comma_suffix));
    }

    chapter_concat.push_str("]");

    println!("cargo:warning={:?}", &chapter_concat);

    let info_rs = format!(
        "
        use std::collections::HashMap;
        pub fn get_chapters() -> HashMap<i16, i8> {{
        let chapter_state: HashMap<i16, i8> = HashMap::from({});
            chapter_state
        }}
    
    ",
        chapter_concat
    );

    let dest_path = format!("{}/info.rs", out_dir);
    fs::write(&dest_path, info_rs).unwrap();
}
