use tauri::State;

use crate::entities::ChapterOrdering;

use super::LibState;

#[tauri::command]
pub fn chapter_orderings(
    comic_id: u32,
    library: State<'_, LibState>,
) -> Result<Vec<ChapterOrdering>, String> {
    library
        .access()?
        .database
        .chapter_orderings(comic_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn insert_ordering(
    ordering: ChapterOrdering,
    library: State<'_, LibState>,
) -> Result<(), String> {
    library
        .access()?
        .database
        .insert_chapter_ordering(&ordering)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_ordering(id: u32, library: State<'_, LibState>) -> Result<(), String> {
    library
        .access()?
        .database
        .delete_chapter_ordering(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_ordering(
    ordering: ChapterOrdering,
    library: State<'_, LibState>,
) -> Result<(), String> {
    library
        .access()?
        .database
        .update_chapter_ordering(&ordering)
        .map_err(|e| e.to_string())
}
