use tauri::State;

use crate::entities::Comic;

use super::LibState;

#[tauri::command]
pub async fn all_comics(library: State<'_, LibState>) -> Result<Vec<Comic>, String> {
    library
        .access()
        .await?
        .database
        .comics()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn comic_with_chapters(id: u32, library: State<'_, LibState>) -> Result<Comic, String> {
    library
        .access()
        .await?
        .database
        .comic_with_chapters(id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn comic(id: u32, library: State<'_, LibState>) -> Result<Comic, String> {
    library
        .access()
        .await?
        .database
        .comic(id)
        .await
        .map_err(|e| e.to_string())
}
