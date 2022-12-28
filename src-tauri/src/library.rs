use std::{
    fs::{DirEntry, FileType},
    path::{Path, PathBuf},
    sync::LazyLock,
};

use anyhow::Result;
use itertools::Itertools;

use crate::db::Database;
use crate::entities::{Chapter, Comic};

#[derive(Debug)]
pub struct Library {
    database: Database,
    pub path: PathBuf,
    pub is_manga_db: bool,
}

static CHAPTER_NUMBER_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(\d+)\.?|\.?(\d+)").expect("invalid chapter number regex")
});

#[derive(Debug)]
struct NoFilePathToSqlite;
impl std::error::Error for NoFilePathToSqlite {}
impl std::fmt::Display for NoFilePathToSqlite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Library {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db_path = path.as_ref().join(".comicdb");
        let database = Database::new(db_path)?;

        let mut library = Self {
            database,
            is_manga_db: true,
            path: path.as_ref().into(),
        };

        let comics = library.scan_library()?;
        dbg!(comics);

        Ok(library)
    }

    fn scan_library(&mut self) -> Result<Vec<Comic>> {
        let comics: Vec<_> = read_entries_with_file_type(&self.path, |_| true, |t| t.is_dir())?
            .filter_map(|d| Some(d.ok()?.path()))
            .map(|d| Comic {
                id: 0,
                name: d
                    .file_name()
                    .map_or(String::from(""), |n| n.to_string_lossy().to_string()),
                chapters: self
                    .scan_comic_directory(&d, 0, 1)
                    .expect("couldn't read comic chapters"),
                is_manga: self.is_manga_db,
                cover_path: None,
                dir_path: d,
            })
            .collect();

        Ok(comics)
    }

    fn scan_comic_directory<P: AsRef<Path>>(
        &self,
        path: P,
        comic_id: u32,
        chapter_number: u32,
    ) -> Result<Vec<Chapter>> {
        let mut chap_num = chapter_number;
        let chaps = read_entries_with_file_type(
            path,
            |f| {
                f.path()
                    .extension()
                    .is_some_and(|e| e.to_string_lossy() == "cbz")
            },
            |t| t.is_file(),
        )?
        .map(|r| r.unwrap().path())
        // sort them by their chapter number for numbering them
        .sorted_by_key(|p| self.chapter_number_from_path(p))
        .map(|p| {
            let c = Chapter {
                id: 0,
                path: p,
                chapter_number: chap_num,
                read: 0,
                pages: 0, // TODO: read chapters from cbz
                comic_id,
            };
            chap_num += 1;
            c
        })
        .collect_vec();

        Ok(chaps)
    }

    fn chapter_number_from_path(&self, path: &Path) -> Vec<u32> {
        let name = path.file_stem().unwrap_or_default().to_string_lossy();
        let num_matches = CHAPTER_NUMBER_REGEX
            .captures_iter(&name)
            // get the number from either the first or second capture
            .map(|c| c.get(1).or_else(|| c.get(2)))
            .map(|o| o.unwrap().as_str())
            .map(|s| s.parse::<u32>().unwrap())
            .collect_vec();
        num_matches
    }
}

fn read_entries_with_file_type<P, FT, FE>(
    path: P,
    pred_entry: FE,
    pred_type: FT,
) -> Result<impl Iterator<Item = std::io::Result<DirEntry>>>
where
    P: AsRef<Path>,
    FT: Fn(&FileType) -> bool + Copy,
    FE: Fn(&DirEntry) -> bool + Copy,
{
    let entries = std::fs::read_dir(path)?;

    let result = entries.filter(move |f| {
        f.as_ref()
            .ok()
            .and_then(|f| Some((f, f.file_type().ok()?)))
            .is_some_and(|(f, t)| pred_type(t) && pred_entry(f))
    });

    Ok(result)
}
