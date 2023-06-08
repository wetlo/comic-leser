use tauri::State;

use crate::entities::Chapter;

use super::LibState;

#[tauri::command]
pub fn chapter(
    comic_id: u32,
    chapter_number: u32,
    library: State<'_, LibState>,
) -> Result<Chapter, String> {
    library
        .access()?
        .database
        .chapter_by_number(comic_id, chapter_number)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn chapter_page_update(id: u32, page: u32, library: State<'_, LibState>) -> Result<(), String> {
    library
        .access()?
        .database
        .update_chapter_page(id, page)
        .map_err(|e| e.to_string())
}
