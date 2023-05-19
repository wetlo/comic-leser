use std::sync::{Arc, Mutex};

use crate::library::Library;

mod chapter;
mod comics;
mod orderings;
pub struct LibState(pub Arc<Mutex<Library>>);

pub fn get_invoke_handler() -> impl Fn(tauri::Invoke<tauri::Wry>) + Send + Sync {
    tauri::generate_handler![
        comics::all_comics,
        comics::comic_with_chapters,
        comics::comic,
        chapter::chapter,
        chapter::chapter_page_update,
        orderings::chapter_orderings,
        orderings::insert_ordering,
        orderings::delete_ordering,
        orderings::update_ordering,
    ]
}
