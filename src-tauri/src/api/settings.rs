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
pub async fn add_library<R: tauri::Runtime>(
    mut lib: LibraryConfig,
    settings: tauri::State<'_, SettingsState>,
    library: tauri::State<'_, super::LibState>,
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    let mut sett = settings.access().await?;
    let id = sett.next_library_id;
    lib.id = id;
    sett.next_library_id += 1;
    sett.libraries.push(lib);

    if sett.libraries.len() == 1 {
        drop(sett); // drop reference
        select_library(id, settings, library, app).await?;
    }
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
    if !settings.selected_library.is_some_and(|s| s == id) {
        settings.selected_library = Some(id);

        let lib = settings.library().ok_or("Invalid Library id")?.clone();
        drop(settings);
        load_library(&lib, library, app).await?;
    }
    Ok(())
}

async fn load_library<R: tauri::Runtime>(
    lib: &'_ LibraryConfig,
    library: tauri::State<'_, super::LibState>,
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    let path = lib.path.clone();

    // only allow absolute paths here
    if path.is_relative() {
        return Ok(());
    }

    let mut lib = library.access_option().await;
    let tmp = std::thread::spawn(move || create_new_library(path))
        .join()
        .expect("thread panicked again :(")?;
    *lib = Some(tmp);

    // TODO: maybe give the comics with the event for less communication errors
    app.emit_all("comics_reloaded", ()).str_err()?;

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
            sett.selected_library = None;
            app.emit_all("comics_reloaded", ()).str_err()?;
        }
        _ => (),
    }

    Ok(())
}

#[tauri::command]
pub async fn update_library<R: tauri::Runtime>(
    lib: LibraryConfig,
    settings: tauri::State<'_, SettingsState>,
    library: tauri::State<'_, super::LibState>,
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    let mut sett = settings.access().await?;

    let idx = get_idx(&sett.libraries, lib.id)?;
    let id = lib.id;
    let old_path = sett.libraries[idx].path.clone();
    sett.libraries[idx] = lib;

    if let Some(lib) = sett.library().cloned()
     && lib.id == id
     && lib.path != old_path {
        drop(sett);
        load_library(&lib, library, app).await?;
    }
    //TODO: save settings to disk
    Ok(())
}

fn get_idx(libraries: &[LibraryConfig], id: usize) -> Result<usize, String> {
    libraries
        .iter()
        .position(|l| l.id == id)
        .ok_or_else(|| "library doesn't exist".to_string())
}
