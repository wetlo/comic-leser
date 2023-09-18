//#![feature(is_some_and)]
#![feature(try_blocks)]
#![feature(lazy_cell)]
#![feature(iterator_try_collect)]
#![feature(async_closure)]
#![feature(let_chains)]
#![feature(result_flattening)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{error::Error, fs::File, io::BufReader, io::Read, path::Path};

use api::{LibState, SettingsState};
use settings::Settings;
use tauri::{
    http::{self, ResponseBuilder},
    AppHandle, Manager, Runtime,
};
use tokio::runtime::Handle;
use url::Url;

mod api;
mod db;
mod entities;
mod library;
mod settings;
mod util;

fn get_comic_page<R: Runtime>(
    app: &AppHandle<R>,
    req: &http::Request,
) -> Result<http::Response, Box<dyn Error>> {
    let state = app.state::<SettingsState>();
    // custom schemes can't be async yet, so this workaround needs to be used
    let path = {
        let handle = Handle::current();
        let _ = handle.enter();
        futures::executor::block_on(state.access())?
            .library()
            .ok_or("no library selected")?
            .path
            .clone()
    };

    let uri = Url::parse(req.uri())?;

    let chapter_path = &percent_encoding::percent_decode_str(uri.path()).decode_utf8()?[1..];
    let path = path.join(chapter_path);

    let page = uri
        .query_pairs()
        .find(|(k, _)| k == "page")
        .map(|(_, v)| v)
        .ok_or("Missing query param: page")?
        .parse::<usize>()?;

    let file = File::open(path)?;
    let r = BufReader::new(file);
    let mut zip = zip::ZipArchive::new(r)?;

    let mut page = zip.by_index(page - 1)?;
    let mut content = Vec::new();
    page.read_to_end(&mut content)?;

    let path: &Path = page.name().as_ref();
    let ext = path
        .extension()
        .ok_or("no extension in cbz file")?
        .to_string_lossy();

    ResponseBuilder::new()
        .status(200)
        .mimetype(&format!("image/{}", &ext))
        .body(content)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::load_from_config()?;
    // TODO: handle load when no library is configured
    let library = library::Library::new(&settings.library().expect("no library").path).await?;

    tauri::Builder::default()
        .manage(LibState::from_lib(library))
        .manage(SettingsState::from_settings(settings))
        .register_uri_scheme_protocol("comic", get_comic_page)
        .invoke_handler(api::get_invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
