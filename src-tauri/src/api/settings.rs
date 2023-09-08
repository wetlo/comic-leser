use std::path::Path;

use crate::settings::{LibraryConfig, Settings};

use super::SettingsState;

#[tauri::command]
pub fn get_settings(settings: tauri::State<'_, SettingsState>) -> Result<Settings, String> {
    settings.access().map(|s| s.clone())
}

#[tauri::command]
pub fn add_library(
    lib: LibraryConfig,
    settings: tauri::State<'_, SettingsState>,
) -> Result<(), String> {
    let mut settings = settings.access()?;
    settings.libraries.push(lib);
    Ok(())
}

#[tauri::command]
pub fn select_library(
    path: String,
    settings: tauri::State<'_, SettingsState>,
) -> Result<(), String> {
    let mut settings = settings.access()?;
    settings.selected_library = get_idx(&settings.libraries, &path)?;

    Ok(())
}

#[tauri::command]
pub fn delete_library(
    path: String,
    settings: tauri::State<'_, SettingsState>,
) -> Result<(), String> {
    let mut settings = settings.access()?;
    let idx = get_idx(&settings.libraries, &path)?;
    settings.libraries.swap_remove(idx);
    Ok(())
}

fn get_idx(libraries: &[LibraryConfig], path: &str) -> Result<usize, String> {
    let path: &Path = path.as_ref();
    libraries
        .iter()
        .position(|p| p.path == path)
        .ok_or_else(|| "library doesn't exist".to_string())
}
