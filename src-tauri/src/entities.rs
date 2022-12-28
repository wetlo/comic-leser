use std::path::PathBuf;

#[derive(Debug)]
pub struct Comic {
    pub id: u32,
    pub dir_path: PathBuf,
    pub name: String,
    pub cover_path: Option<PathBuf>,
    pub is_manga: bool,

    pub chapters: Vec<Chapter>,
}

#[derive(Debug)]
pub struct Chapter {
    pub id: u32,
    pub path: PathBuf,
    pub chapter_number: u32,

    pub read: u32,
    pub pages: u32,

    pub comic_id: u32,
}
