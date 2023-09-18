use std::path::Path;

use tauri::Manager;

use crate::{
    library::Library,
    settings::{LibraryConfig, Settings},
    util::str_error::StringResult,
};

use super::SettingsState;

#[tauri::command]
pub async fn get_settings(settings: tauri::State<'_, SettingsState>) -> Result<Settings, String> {
    settings.access().await.map(|s| s.clone())
}

#[tauri::command]
pub async fn add_library(
    mut lib: LibraryConfig,
    settings: tauri::State<'_, SettingsState>,
) -> Result<(), String> {
    let mut settings = settings.access().await?;
    lib.id = settings.next_library_id;
    settings.next_library_id += 1;
    settings.libraries.push(lib);
    Ok(())
}

#[tauri::command]
pub async fn select_library<'a, R: tauri::Runtime>(
    id: usize,
    settings: tauri::State<'a, SettingsState>,
    library: tauri::State<'a, super::LibState>,
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    let mut settings = settings.access().await?;
    if settings.selected_library.is_some_and(|s| s != id) {
        settings.selected_library = Some(id);

        let path = settings.library().ok_or("Invalid Library id")?.path.clone();

        let mut lib = library.access().await?;
        *lib = std::thread::spawn(move || create_new_library(path))
            .join()
            .expect("thread panicked again :(")?;

        // TODO: maybe give the comics with the event for less communication errors
        app.emit_all("comics_reloaded", ()).str_err()?;
    }
    Ok(())
}

/// needs to be run on another thread because
/// some kind of higher order lifetime error
#[tokio::main]
async fn create_new_library<P: AsRef<Path>>(p: P) -> Result<Library, String> {
    Library::new(p).await.str_err()
}

#[tauri::command]
pub async fn delete_library<R: tauri::Runtime>(
    id: usize,
    settings: tauri::State<'_, SettingsState>,
    library: tauri::State<'_, super::LibState>,
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    let mut sett = settings.access().await?;
    let idx = get_idx(&sett.libraries, id)?;
    sett.libraries.remove(idx);

    match (
        sett.selected_library,
        sett.libraries.iter().find(|l| l.id != id),
    ) {
        (Some(sel), Some(l)) if sel == id => {
            let id = l.id;
            drop(sett); // drop the reference to avoid deadlocks
            select_library(id, settings, library, app).await?
        }
        (Some(_), None) => {
            let mut lib = library.access_option().await;
            *lib = None;
            app.emit_all("comics_reloaded", ()).str_err()?;
        }
        _ => (),
    }
    // TODO: handle when the selected one is deleted
    // TODO: handle when there are no more libraries left

    Ok(())
}

#[tauri::command]
pub async fn update_library(
    lib: LibraryConfig,
    settings: tauri::State<'_, SettingsState>,
) -> Result<(), String> {
    let mut settings = settings.access().await?;

    let idx = get_idx(&settings.libraries, lib.id)?;
    settings.libraries[idx] = lib;

    //TODO: save settings to disk
    Ok(())
}

fn get_idx(libraries: &[LibraryConfig], id: usize) -> Result<usize, String> {
    libraries
        .iter()
        .position(|l| l.id == id)
        .ok_or_else(|| "library doesn't exist".to_string())
}
