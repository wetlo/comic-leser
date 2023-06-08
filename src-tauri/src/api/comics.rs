use tauri::State;

use crate::entities::Comic;

use super::LibState;

#[tauri::command]
pub fn all_comics(library: State<'_, LibState>) -> Result<Vec<Comic>, String> {
    library
        .access()?
        .database
        .comics()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn comic_with_chapters(id: u32, library: State<'_, LibState>) -> Result<Comic, String> {
    library
        .access()?
        .database
        .comic_with_chapters(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn comic(id: u32, library: State<'_, LibState>) -> Result<Comic, String> {
    library
        .access()?
        .database
        .comic(id)
        .map_err(|e| e.to_string())
}
