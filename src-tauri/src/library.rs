use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use anyhow::Result;
use itertools::Itertools;
use tokio::fs::DirEntry;
use tokio_stream::{Stream, StreamExt};

use crate::db::Database;
use crate::entities::{Chapter, Comic};

#[derive(Debug)]
pub struct Library {
    pub database: Database,
    pub path: PathBuf,
    pub is_manga_db: bool,
}

static CHAPTER_NUMBER_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(\d+)\.?|\.?(\d+)").expect("invalid chapter number regex")
});

impl Library {
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db_path = path.as_ref().join(".comicdb");
        let database = Database::new(db_path).await?;

        let mut library = Self {
            is_manga_db: true,
            path: path.as_ref().into(),
            database,
        };

        library.update().await?;

        Ok(library)
    }

    /// scan the library directory for new comics/chapters and update the database
    async fn update(&mut self) -> Result<()> {
        let lib_comics = self.database.comics().await?;
        // TODO: handle deletion
        let scanned_comics = self.scan().await?;

        let mut new_comics = vec![];
        let mut new_chapters = vec![];

        for c in scanned_comics {
            let lib_comic = lib_comics.iter().find(|l| l.dir_path == c.dir_path);

            if let Some(lib_comic) = lib_comic {
                new_chapters.extend(self.get_new_chaps_for(lib_comic, c).await?)
            } else {
                new_comics.push(c)
            }
        }

        // only get the page count when the chapter is new
        let chaps_from_new_comics = new_comics.iter_mut().flat_map(|c| c.chapters.iter_mut());
        for c in new_chapters.iter_mut().chain(chaps_from_new_comics) {
            c.pages = zip::ZipArchive::new(File::open(self.abs_path(&c.path)).unwrap())
                .unwrap()
                .len() as u32;
        }

        dbg!(&new_chapters);

        self.database.insert_comics(new_comics).await?; // add the new comics
        self.database.insert_chapters(new_chapters).await?;

        Ok(())
    }

    /// compare the db chapters of a comic and the scanned chapter to get the new chapters
    async fn get_new_chaps_for(&self, lib_comic: &Comic, scanned: Comic) -> Result<Vec<Chapter>> {
        let db_chaps = self
            .database
            .comic_with_chapters(lib_comic.id)
            .await?
            .chapters;

        let new = scanned
            .chapters
            .into_iter()
            .filter(|c| {
                !db_chaps
                    .iter()
                    .any(|d| d.path == c.path && c.chapter_number == d.chapter_number)
            })
            // add the id of the comic
            .map(|mut c| {
                c.comic_id = lib_comic.id;
                c
            })
            .collect_vec();

        Ok(new)
    }

    /// scan the comic directory, to get every comic + chapter inside
    async fn scan(&mut self) -> Result<Vec<Comic>> {
        let db_comics = self.comic_path_hashmap().await?;

        let comics =
            read_entries_with_file_type(&self.path, |f: &Path| is_not_hidden(f) && f.is_dir())
                .await?
                // only use entries with valid paths
                .filter_map(|d| Some(d.ok()?.path()))
                // create comic from entry
                .then(|d| self.create_scanned_comic(d, &db_comics))
                .collect::<Result<Vec<_>>>()
                .await;
        comics

        //Ok(comics)
    }

    async fn create_scanned_comic(
        &self,
        d: PathBuf,
        db_comics: &HashMap<PathBuf, Comic>,
    ) -> Result<Comic> {
        let id = db_comics.get(&d).map(|c| c.id).unwrap_or_default();
        Ok(Comic {
            id,
            name: d
                .file_name()
                .expect("comic path returns in ..")
                .to_string_lossy()
                .to_string(),
            chapters: self.scan_chapters(&d, id).await?,
            chapter_count: None,
            chapter_read: None,
            is_manga: self.is_manga_db,
            cover_path: None,
            dir_path: self.relative_path(d),
        })
    }

    /// get the chapters inside a comic directory
    async fn scan_chapters<P: AsRef<Path>>(&self, path: P, comic_id: u32) -> Result<Vec<Chapter>> {
        let mut chap_num = 1;
        let chapter_orderings = self.get_chapter_orderings(comic_id).await?;

        let mut chap_paths = read_entries_with_file_type(path, |f| {
            f.extension().is_some_and(|e| e == "cbz") && f.is_file()
        })
        .await?
        .map(|r| r.unwrap().path())
        // sort them by their chapter number for numbering them
        .collect::<Vec<_>>()
        .await;

        chap_paths.sort_by_key(|p| self.chapter_number_from_path(p, &chapter_orderings));
        let chaps = chap_paths
            .into_iter()
            .map(|p| {
                let c = Chapter {
                    id: 0,
                    pages: 0,
                    name: p.file_stem().unwrap().to_string_lossy().into_owned(),
                    path: self.relative_path(p),
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
    fn chapter_number_from_path(
        &self,
        path: &Path,
        chapter_orderings: &[regex::Regex],
    ) -> Vec<u32> {
        let name = path.file_stem().unwrap_or_default().to_string_lossy();

        let ordering = chapter_orderings
            .iter()
            .enumerate()
            .filter(|(_, r)| r.is_match(&name))
            .map(|(i, _)| i as u32)
            .next();

        let mut num_matches = CHAPTER_NUMBER_REGEX
            .captures_iter(&name)
            // get the number from either the first or second capture
            .map(|c| c.get(1).or_else(|| c.get(2)))
            .map(|o| o.unwrap().as_str())
            .map(|s| s.parse::<u32>().unwrap())
            .collect_vec();

        // begin with the ordering and if no right ordering has been found just use the biggest possible
        num_matches.insert(0, ordering.unwrap_or(u32::MAX));
        num_matches
    }

    async fn comic_path_hashmap(&self) -> Result<HashMap<PathBuf, Comic>> {
        Ok(self
            .database
            .comics()
            .await?
            .into_iter()
            .map(|c| (c.dir_path.clone(), c))
            .collect())
    }

    async fn get_chapter_orderings(&self, comic_id: u32) -> Result<Vec<regex::Regex>> {
        Ok(self
            .database
            .chapter_orderings(comic_id)
            .await?
            .into_iter()
            .map(|o| regex::Regex::new(&o.regex))
            .collect::<Result<_, _>>()?)
    }

    fn relative_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        path.as_ref()
            .strip_prefix(&self.path)
            .expect("should be prefix")
            .to_owned()
    }

    fn abs_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.path.join(path)
    }
}

async fn read_entries_with_file_type<P, FE>(
    path: P,
    pred_entry: FE,
) -> Result<impl Stream<Item = std::io::Result<DirEntry>>>
where
    P: AsRef<Path>,
    //FT: Fn(&FileType) -> bool,
    FE: Fn(&Path) -> bool,
{
    let entries = tokio::fs::read_dir(path).await?;
    let entries = tokio_stream::wrappers::ReadDirStream::new(entries);
    //let entries = std::fs::read_dir(path)?;

    let result = entries.filter(move |f| {
        f.as_ref()
            .ok()
            //.and_then(|f| Some((f.path(), f.file_type().ok()?))) // handle errors before preds
            .is_some_and(|f| pred_entry(&f.path()))
    });

    Ok(result)
}

fn is_not_hidden(entry: &Path) -> bool {
    entry
        .file_name()
        .and_then(|n| n.to_string_lossy().chars().next())
        .is_some_and(|c| c != '.')
}
