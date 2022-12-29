#![feature(is_some_with)]
#![feature(try_blocks)]
#![feature(once_cell)]
#![feature(iterator_try_collect)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{error::Error, fs::File, io::BufReader, io::Read, path::Path};

use tauri::{
    http::{self, ResponseBuilder},
    AppHandle, Runtime,
};
use url::Url;

mod db;
mod entities;
mod library;

fn get_comic_page<R: Runtime>(
    _app: &AppHandle<R>,
    req: &http::Request,
) -> Result<http::Response, Box<dyn Error>> {
    let uri = Url::parse(dbg!(req.uri()))?;

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

fn main() -> anyhow::Result<()> {
    let _library = library::Library::new("/media/manga/")?;

    tauri::Builder::default()
        .register_uri_scheme_protocol("comic", get_comic_page)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
