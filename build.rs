use image;
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug)]
pub struct ChapterState {
    pub chapter: i16,
    pub page: i8,
    pub name: String,
    pub ext: String,
    pub is_dual: bool,
}

type MangaName = String;

/**
 * This will generate the entire manga reading structure so the state.rs file can consume it
 */
fn main() -> Result<(), std::io::Error> {
    println!("cargo:rerun-if-changed=src/assets/manga");
    let out_dir = "./src/";

    let mut chapter_state: HashMap<MangaName, HashMap<i16, Vec<ChapterState>>> = HashMap::new();

    let dir_path = "./src/assets/manga/"; // replace with your directory path

    let manga_folders = fs::read_dir(dir_path)?;

    for read in manga_folders {
        let entry = match read {
            Err(_) => continue,
            Ok(e) => e,
        };

        let entry_path = entry.path();

        if !entry_path.is_dir() {
            continue;
        }

        let manga_name_option = entry_path.file_name().and_then(|n| n.to_str());
        // This isn't possible https://stackoverflow.com/questions/49784874/what-is-the-rust-way-of-using-continue-from-inside-a-closure
        // .map_or_else(|| continue, |name| name);

        let manga_name = match manga_name_option {
            None => continue,
            Some(name) => name,
        };

        let manga_chapters_dir = dir_path.to_owned() + manga_name;
        let err_msg = format!("Failed to read {}", manga_chapters_dir);
        let chapters_folders = fs::read_dir(manga_chapters_dir).expect(&err_msg);
        let mut chapter_folder_state: HashMap<i16, Vec<ChapterState>> = HashMap::new();
        for chapter_folder in chapters_folders {
            // https://stackoverflow.com/questions/74966090/while-let-chain-causing-rust-analyzer-to-complain-about-the-feature-being-unstab
            if let Ok(folder_entry) = chapter_folder {
                if let Some(folder_name) = folder_entry
                    .path()
                    .file_name()
                    .and_then(|name| (name.to_str()))
                {
                    if let Ok(chapter_number) = folder_name.parse::<i16>() {
                        let chapter_path = folder_entry.path();
                        let err_msg = format!("Failed to read {:?}", chapter_path);
                        let pages = fs::read_dir(chapter_path).expect(&err_msg);

                        let mut chapter_pages: Vec<ChapterState> = Vec::new();

                        for (index, page) in pages.enumerate() {
                            if let Ok(page_entry) = page {
                                if let Some(page_name) = page_entry
                                    .path()
                                    .file_name()
                                    .and_then(|name| (name.to_str()))
                                {
                                    let page_path = page_entry.path();
                                    let err_msg = format!("Failed to read {:?}", page_path);
                                    let page_extension = Path::new(page_name)
                                        .extension()
                                        .and_then(|ext| ext.to_str())
                                        .unwrap_or("");
                                    let image = image::open(page_path).expect(&err_msg);

                                    let page_is_dual = image.width() >= image.height();

                                    let chapter_state = ChapterState {
                                        chapter: chapter_number,
                                        page: i8::try_from(index + 1).unwrap(),
                                        name: page_name.to_owned(),
                                        ext: page_extension.to_owned(),
                                        is_dual: page_is_dual,
                                    };

                                    chapter_pages.push(chapter_state);
                                }
                            }
                        }
                        chapter_folder_state.insert(chapter_number, chapter_pages);
                    }
                }
            }
        }
        chapter_state.insert(manga_name.to_owned(), chapter_folder_state);
    }

    // Generated using ChatGPT
    // This will create the chapter_map.rs file
    let mut code_template = format!("use std::collections::HashMap;\n");
    code_template.push_str("use serde::{Deserialize, Serialize};\n\n");

    code_template.push_str("#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]\n");
    code_template.push_str("pub struct ChapterState {\n");
    code_template.push_str("    pub chapter: i16,\n");
    code_template.push_str("    pub page: i8,\n");
    code_template.push_str("    pub name: String,\n");
    code_template.push_str("    pub ext: String,\n");
    code_template.push_str("    pub is_dual: bool,\n");
    code_template.push_str("}\n\n");

    code_template.push_str("type MangaName = String;\n");

    code_template.push_str(&format!(
        "pub fn get_chapters() -> HashMap<MangaName, HashMap<i16, Vec<ChapterState>>> {{\n"
    ));
    code_template.push_str(&format!("    let mut chapter_state: HashMap<MangaName, HashMap<i16, Vec<ChapterState>>> = HashMap::new();\n"));

    for (manga_name, chapters) in chapter_state {
        code_template.push_str(&format!(
            "    let mut {}_chapters: HashMap<i16, Vec<ChapterState>> = HashMap::new();\n",
            manga_name
        ));
        for (chapter, chapter_states) in chapters {
            code_template.push_str(&format!(
                "    let mut chapter_{}: Vec<ChapterState> = Vec::new();\n",
                chapter
            ));
            for chapter_state in chapter_states {
                code_template.push_str(&format!("    chapter_{}.push(ChapterState {{ chapter: {}, page: {}, name: \"{}\".to_string(), ext: \"{}\".to_string(), is_dual: {} }});\n", chapter, chapter_state.chapter, chapter_state.page, chapter_state.name, chapter_state.ext, chapter_state.is_dual));
            }
            code_template.push_str(&format!(
                "    {}_chapters.insert({}, chapter_{});\n",
                manga_name, chapter, chapter
            ));
        }
        code_template.push_str(&format!(
            "    chapter_state.insert(\"{}\".to_string(), {}_chapters);\n",
            manga_name, manga_name
        ));
    }

    code_template.push_str("    chapter_state\n");
    code_template.push_str("}\n");

    let dest_path = format!("{}/generated/chapter_map.rs", out_dir);
    fs::write(&dest_path, code_template).unwrap();
    Ok(())
}
