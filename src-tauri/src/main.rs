//#![feature(is_some_and)]
#![feature(try_blocks)]
#![feature(lazy_cell)]
#![feature(iterator_try_collect)]
#![feature(async_closure)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{error::Error, fs::File, io::BufReader, io::Read, path::Path};

use api::LibState;
use settings::Settings;
use tauri::{
    http::{self, ResponseBuilder},
    AppHandle, Runtime,
};
use url::Url;

mod api;
mod db;
mod entities;
mod library;
mod settings;

fn get_comic_page<R: Runtime>(
    _app: &AppHandle<R>,
    req: &http::Request,
) -> Result<http::Response, Box<dyn Error>> {
    let uri = Url::parse(req.uri())?;

    let path = percent_encoding::percent_decode_str(uri.path()).decode_utf8()?;

    let page = uri
        .query_pairs()
        .find(|(k, _)| k == "page")
        .map(|(_, v)| v)
        .ok_or("Missing query param: page")?
        .parse::<usize>()?;

    let file = File::open(&*path)?;
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
    let library = library::Library::new(settings.comic_path).await?;

    tauri::Builder::default()
        .manage(LibState::from_lib(library))
        .register_uri_scheme_protocol("comic", get_comic_page)
        .invoke_handler(api::get_invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
