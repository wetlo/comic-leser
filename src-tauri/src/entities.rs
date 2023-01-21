use std::path::PathBuf;

use serde::Serialize;
use ts_rs::TS;

//TODO: maybe add insert and get dtos?
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "../src/entities/")]
pub struct Comic {
    pub id: u32,
    pub dir_path: PathBuf,
    pub name: String,
    pub cover_path: Option<PathBuf>,
    pub is_manga: bool,

    pub chapters: Vec<Chapter>,
    pub chapter_count: Option<usize>,
    pub chapter_read: Option<usize>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "../src/entities/")]
pub struct Chapter {
    pub id: u32,
    pub path: PathBuf,
    pub name: String,
    pub chapter_number: u32,

    pub read: u32,
    pub pages: u32,

    pub comic_id: u32,
}
