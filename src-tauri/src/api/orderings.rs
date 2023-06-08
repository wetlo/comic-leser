use tauri::State;

use crate::entities::ChapterOrdering;

use super::LibState;

#[tauri::command]
pub async fn chapter_orderings(
    comic_id: u32,
    library: State<'_, LibState>,
) -> Result<Vec<ChapterOrdering>, String> {
    library
        .access()
        .await?
        .database
        .chapter_orderings(comic_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn insert_ordering(
    ordering: ChapterOrdering,
    library: State<'_, LibState>,
) -> Result<(), String> {
    library
        .access()
        .await?
        .database
        .insert_chapter_ordering(ordering)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_ordering(id: u32, library: State<'_, LibState>) -> Result<(), String> {
    library
        .access()
        .await?
        .database
        .delete_chapter_ordering(id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_ordering(
    ordering: ChapterOrdering,
    library: State<'_, LibState>,
) -> Result<(), String> {
    library
        .access()
        .await?
        .database
        .update_chapter_ordering(ordering)
        .await
        .map_err(|e| e.to_string())
}
