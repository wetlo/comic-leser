use std::{
    fs::File,
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
    pub comics: Vec<Comic>,
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
            comics: database.comics()?,
            is_manga_db: true,
            path: path.as_ref().into(),
            database,
        };

        dbg!(&library.comics);

        library.update()?;

        Ok(library)
    }

    pub fn comic_with_chapters(&self, comic_id: u32) -> Option<Comic> {
        self.database.comic_with_chapters(comic_id).ok()
    }

    /// scan the library directory for new comics/chapters and update the database
    fn update(&mut self) -> Result<()> {
        // TODO: handle deletion
        let scanned_comics = self.scan()?;

        let mut new_comics = vec![];
        let mut new_chapters = vec![];

        for c in scanned_comics {
            let lib_comic = self.comics.iter().find(|l| l.dir_path == c.dir_path);

            if lib_comic.is_some() {
                new_chapters.extend(self.get_new_chaps_for(lib_comic.unwrap().id, c)?)
            } else {
                new_comics.push(c)
            }
        }

        // only get the page count when the chapter is new
        for c in new_chapters.iter_mut() {
            c.pages = zip::ZipArchive::new(File::open(&c.path).unwrap())
                .unwrap()
                .len() as u32;
        }

        self.database.insert_comics(&new_comics)?; // add the new comic
        self.database.insert_chapters(&new_chapters, None)?;

        Ok(())
    }

    /// compare the db chapters of a comic and the scanned chapter to get the new chapters
    fn get_new_chaps_for(&self, comic_id: u32, scanned: Comic) -> Result<Vec<Chapter>> {
        let db_chaps = self.database.comic_with_chapters(comic_id)?.chapters;

        let new = scanned
            .chapters
            .into_iter()
            .filter(|c| !db_chaps.iter().any(|d| d.path == c.path))
            .collect_vec();

        Ok(new)
    }

    /// scan the comic directory, to get every comic + chapter inside
    fn scan(&mut self) -> Result<Vec<Comic>> {
        let comics = read_entries_with_file_type(&self.path, is_not_hidden, |t| t.is_dir())?
            // only use entries with valid paths
            .filter_map(|d| Some(d.ok()?.path()))
            // create comic from entry
            .map(|d| Comic {
                id: 0,
                name: d
                    .file_name()
                    .map_or(String::from(""), |n| n.to_string_lossy().to_string()),
                chapters: self
                    .scan_chapters(&d, 0, 1)
                    .expect("couldn't read comic chapters"),
                is_manga: self.is_manga_db,
                cover_path: None,
                dir_path: d,
            })
            .collect_vec();

        Ok(comics)
    }

    /// get the chapters inside a comic directory
    fn scan_chapters<P: AsRef<Path>>(
        &self,
        path: P,
        comic_id: u32,
        chapter_number: u32,
    ) -> Result<Vec<Chapter>> {
        let mut chap_num = chapter_number;
        let chaps = read_entries_with_file_type(
            path,
            |f| f.extension().is_some_and(|e| e.to_string_lossy() == "cbz"),
            |t| t.is_file(),
        )?
        .map(|r| r.unwrap().path())
        // sort them by their chapter number for numbering them
        .sorted_by_key(|p| self.chapter_number_from_path(p))
        .map(|p| {
            let c = Chapter {
                id: 0,
                pages: 0,
                path: p,
                chapter_number: chap_num,
                read: 0,
                comic_id,
            };
            chap_num += 1;
            c
        })
        .collect_vec();

        Ok(chaps)
    }

    /// parse the chapter number from the chapter location
    /// this also handles in between chapters like 10.5
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
    FE: Fn(&Path) -> bool + Copy,
{
    let entries = std::fs::read_dir(path)?;

    let result = entries.filter(move |f| {
        f.as_ref()
            .ok()
            .and_then(|f| Some((f.path(), f.file_type().ok()?)))
            .is_some_and(|(f, t)| pred_type(t) && pred_entry(f))
    });

    Ok(result)
}

fn is_not_hidden(entry: &Path) -> bool {
    entry
        .file_name()
        .and_then(|n| n.to_string_lossy().chars().next())
        .is_some_and(|c| *c != '.')
}
