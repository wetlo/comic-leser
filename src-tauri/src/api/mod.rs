use std::sync::Arc;

use tokio::sync::{MappedMutexGuard, Mutex, MutexGuard};

use crate::{library::Library, settings::Settings, util::str_error::StringResult};

mod chapter;
mod comics;
mod orderings;
mod settings;

#[derive(Clone)]
pub struct SettingsState(Arc<Mutex<Settings>>);

impl SettingsState {
    pub fn from_settings(settings: Settings) -> Self {
        SettingsState(Arc::new(Mutex::new(settings)))
    }

    pub async fn access(&'_ self) -> Result<MutexGuard<'_, Settings>, String> {
        Ok(self.0.lock().await)
    }
}

pub struct LibState(Arc<Mutex<Option<Library>>>);

impl LibState {
    pub fn from_lib(lib: Library) -> Self {
        LibState(Arc::new(Mutex::new(Some(lib))))
    }

    pub async fn access(&'_ self) -> Result<MappedMutexGuard<'_, Library>, String> {
        let guard = self.0.lock().await;

        //guard.map(|l| MutexGuard::map(guard, |_| &mut l));
        //MutexGuard::map(guard, |)

        MutexGuard::try_map(guard, |l| l.as_mut())
            .map_err(|_| "no library loaded")
            .str_err()
    }

    pub async fn access_option(&'_ self) -> MutexGuard<'_, Option<Library>> {
        self.0.lock().await
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
        settings::get_settings,
        settings::add_library,
        settings::select_library,
        settings::delete_library,
        settings::update_library,
    ]
}
