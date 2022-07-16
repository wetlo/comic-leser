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

fn image_format<P: AsRef<Path>>(path: P) -> Option<&'static str> {
    let accepted_formats = ["png", "jpg", "jpeg"];

    path.as_ref()
        .extension()
        .and_then(|ext| {
            accepted_formats
                .iter()
                .find(|i| ext.to_string_lossy() == **i)
        })
        .copied()
}

fn get_image<R: Runtime>(
    _app: &AppHandle<R>,
    req: &http::Request,
) -> Result<http::Response, Box<dyn Error>> {
    let uri = Url::parse(dbg!(req.uri()))?;

    let path = uri.path();

    let res = if let Some(i) = image_format(path) {
        let f = File::open(path)?;
        let mut r = BufReader::new(f);
        let mut content = Vec::new();

        r.read_to_end(&mut content)?;

        ResponseBuilder::new()
            .status(200)
            .mimetype(&format!("image/{}", i))
            .body(content)
    } else {
        ResponseBuilder::new()
            .status(400)
            .body("invalid file type".into())
    }?;

    Ok(res)
}

fn main() {
    dbg!("Hello world");
    tauri::Builder::default()
        .register_uri_scheme_protocol("image", get_image)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
