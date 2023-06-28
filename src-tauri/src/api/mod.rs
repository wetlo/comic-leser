use std::sync::{Arc, Mutex as StdMutex, MutexGuard as StdMutexGuard};

use tokio::sync::{Mutex, MutexGuard};

use crate::{library::Library, settings::Settings};

mod chapter;
mod comics;
mod orderings;

pub struct SettingsState(Arc<StdMutex<Settings>>);

impl SettingsState {
    pub fn from_settings(settings: Settings) -> Self {
        SettingsState(Arc::new(StdMutex::new(settings)))
    }

    pub fn access(&'_ self) -> Result<StdMutexGuard<'_, Settings>, String> {
        self.0.lock().map_err(|e| e.to_string())
    }
}

pub struct LibState(Arc<Mutex<Library>>);

impl LibState {
    pub fn from_lib(lib: Library) -> Self {
        LibState(Arc::new(Mutex::new(lib)))
    }

    pub async fn access(&'_ self) -> Result<MutexGuard<'_, Library>, String> {
        Ok(self.0.lock().await)
    }
}

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
