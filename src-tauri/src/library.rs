use std::{
    fs::{DirEntry, FileType},
    path::{Path, PathBuf},
    sync::LazyLock,
};

use anyhow::Result;
use itertools::Itertools;

use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};

#[derive(Debug)]
pub struct Library {
    connection: Connection,
    pub path: PathBuf,
    pub is_manga_db: bool,
}

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
    pub chapter_number: Option<u32>,

    pub read: u32,
    pub pages: u32,

    pub comic_id: u32,
}

static MIGRATIONS: LazyLock<Migrations<'static>> =
    LazyLock::new(|| Migrations::new(vec![M::up(include_str!("sql/initial-migration.sql"))]));

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
        let connection = Connection::open(db_path)?;

        let mut library = Self {
            connection,
            is_manga_db: true,
            path: path.as_ref().into(),
        };

        library.create_db()?;
        let comics = library.scan_library()?;
        dbg!(comics);

        Ok(library)
    }

    fn create_db(&mut self) -> Result<()> {
        MIGRATIONS
            .to_latest(&mut self.connection)
            .map_err(|e| e.into())
    }

    fn scan_library(&mut self) -> Result<Vec<Comic>> {
        let dir_path = self
            .connection
            .path()
            .and_then(|p| p.parent())
            .ok_or(NoFilePathToSqlite)?;
        dbg!(&dir_path);

        let comics: Vec<_> = read_entries_with_file_type(dir_path, |_| true, |t| t.is_dir())?
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
                chapter_number: Some(chap_num),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_test() {
        MIGRATIONS.validate().unwrap();
    }
}
