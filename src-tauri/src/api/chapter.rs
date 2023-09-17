use tauri::State;

use crate::{entities::Chapter, util::str_error::StringResult};

use super::LibState;

#[tauri::command]
pub async fn chapter(
    comic_id: u32,
    chapter_number: u32,
    library: State<'_, LibState>,
) -> Result<Chapter, String> {
    library
        .access()
        .await?
        .database
        .chapter_by_number(comic_id, chapter_number)
        .await
        .str_err()
}

#[tauri::command]
pub async fn chapter_page_update(
    id: u32,
    page: u32,
    library: State<'_, LibState>,
) -> Result<(), String> {
    library
        .access()
        .await?
        .database
        .update_chapter_page(id, page)
        .await
        .str_err()
}
