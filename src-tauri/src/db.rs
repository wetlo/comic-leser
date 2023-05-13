use std::{path::Path, sync::LazyLock};

use rusqlite::{params, Connection, Result};
use rusqlite_migration::{Migrations, M};

use crate::entities::{Chapter, Comic};

static MIGRATIONS: LazyLock<Migrations<'static>> = LazyLock::new(|| {
    Migrations::new(vec![M::up(include_str!(
        "sql/migrations/initial-migration.sql"
    ))])
});

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

const COMIC_QUERY: &str = include_str!("sql/get_comics.sql");
const COMIC_QUERY_ID: &str = include_str!("sql/get_comic.sql");
const COMIC_INSERT: &str =
    "INSERT INTO comic (dir_path, name, cover_path, is_manga) VALUES (?1, ?2, ?3, ?4)";

const CHAPTER_QUERY: &str =
    "SELECT id, file_path, chapter_number, read, pages, comic_id, name FROM chapter WHERE comic_id = (?1) ORDER BY chapter_number";
const CHAPTER_ORDER_QUERY: &str =
    "SELECT id, file_path, chapter_number, read, pages, comic_id, name FROM chapter WHERE comic_id = (?1) AND chapter_number = (?2)";
const CHAPTER_INSERT: &str =
    "INSERT INTO chapter (file_path, chapter_number, read, pages, comic_id, name) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
const CHAPTER_UPSERT: &str = include_str!("sql/upsert_chapter.sql");
const CHAPTER_PAGE_UPDATE: &str = "UPDATE chapter SET read = (?2) WHERE id = (?1)";

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let mut conn = Connection::open(path)?;
        MIGRATIONS.to_latest(&mut conn)?;

        Ok(Self { conn })
    }

    fn _from_conn(mut conn: Connection) -> anyhow::Result<Self> {
        MIGRATIONS.to_latest(&mut conn)?;

        Ok(Self { conn })
    }

    pub fn comics(&self) -> Result<Vec<Comic>> {
        let mut query = self.conn.prepare(COMIC_QUERY)?;

        let mut comics = query.query_map([], comic_from_row)?;

        comics.try_collect::<Vec<Comic>>()
    }

    pub fn comic_with_chapters(&self, comic_id: u32) -> Result<Comic> {
        let mut comic = self
            .conn
            .query_row(COMIC_QUERY_ID, [comic_id], comic_from_row)?;

        let chaps = self
            .conn
            .prepare(CHAPTER_QUERY)?
            .query_map([comic_id], chapter_from_row)?
            .try_collect::<Vec<_>>()?;

        comic.chapters = chaps;

        Ok(comic)
    }

    pub fn chapter_by_number(&self, comic_id: u32, chapter_number: u32) -> Result<Chapter> {
        self.conn.query_row(
            CHAPTER_ORDER_QUERY,
            [comic_id, chapter_number],
            chapter_from_row,
        )
    }

    pub fn update_chapter_page(&mut self, chapter_id: u32, page: u32) -> Result<()> {
        self.conn
            .execute(CHAPTER_PAGE_UPDATE, [chapter_id, page])
            .map(|_| ())
    }

    pub fn insert_comics(&mut self, comics: &[Comic]) -> Result<()> {
        let tx = self.conn.transaction()?;
        let mut insert = tx.prepare(COMIC_INSERT)?;

        for c in comics {
            let id = insert.insert(params![
                c.dir_path.to_string_lossy(),
                c.name,
                c.cover_path.as_ref().map(|p| p.to_string_lossy()),
                c.is_manga,
            ])?;

            Self::insert_chapters_transaction(&tx, &c.chapters, Some(id as u32))?;
        }

        drop(insert);
        tx.commit()?;

        Ok(())
    }

    pub fn insert_chapters(&mut self, chapters: &[Chapter], comic_id: Option<u32>) -> Result<()> {
        let tx = self.conn.transaction()?;
        Self::insert_chapters_transaction(&tx, chapters, comic_id)?;
        tx.commit()?;
        Ok(())
    }

    fn insert_chapters_transaction(
        tx: &rusqlite::Transaction,
        chapters: &[Chapter],
        comic_id: Option<u32>,
    ) -> Result<()> {
        let mut insert = tx.prepare(CHAPTER_UPSERT)?;

        for c in chapters {
            let comic = comic_id.unwrap_or(c.comic_id);
            insert.insert(params![
                c.path.to_string_lossy(),
                c.chapter_number,
                c.read,
                c.pages,
                comic,
                c.name,
            ])?;
        }

        Ok(())
    }
}
fn comic_from_row(r: &rusqlite::Row) -> Result<Comic> {
    Ok(Comic {
        id: r.get(0)?,
        dir_path: r.get::<_, String>(1)?.into(),
        name: r.get(2)?,
        cover_path: r.get::<_, Option<String>>(3)?.map(|s| s.into()),
        is_manga: r.get(4)?,
        chapter_count: r.get(5).ok(),
        chapter_read: r.get(6).ok(),
        chapters: vec![],
    })
}

fn chapter_from_row(r: &rusqlite::Row) -> Result<Chapter> {
    Ok(Chapter {
        id: r.get(0)?,
        path: r.get::<_, String>(1)?.into(),
        chapter_number: r.get(2)?,
        read: r.get(3)?,
        pages: r.get(4)?,
        comic_id: r.get(5)?,
        name: r.get(6)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn migrations_test() {
        MIGRATIONS.validate().unwrap();
    }

    #[test]
    fn add_comics() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        let mut db = Database::_from_conn(conn)?;

        let comics = [
            Comic {
                id: 0,
                dir_path: "/one_piece".into(),
                name: "One Piece".to_string(),
                cover_path: None,
                is_manga: true,
                chapter_count: None,
                chapter_read: None,
                chapters: vec![
                    Chapter {
                        id: 0,
                        comic_id: 0,
                        path: "/one_piece/1.cbz".into(),
                        name: "1".to_string(),
                        read: 30,
                        pages: 30,
                        chapter_number: 1,
                    },
                    Chapter {
                        id: 0,
                        comic_id: 0,
                        path: "/one_piece/2.cbz".into(),
                        name: "2".to_string(),
                        read: 12,
                        pages: 23,
                        chapter_number: 2,
                    },
                ],
            },
            Comic {
                id: 0,
                dir_path: "/berserk".into(),
                name: "Berserk".to_string(),
                cover_path: Some("/berserk/cover.jpg".into()),
                is_manga: true,
                chapter_count: None,
                chapter_read: None,
                chapters: vec![
                    Chapter {
                        id: 0,
                        comic_id: 0,
                        path: "/berserk/1.cbz".into(),
                        name: "1".to_string(),
                        read: 67,
                        pages: 97,
                        chapter_number: 1,
                    },
                    Chapter {
                        id: 0,
                        comic_id: 0,
                        path: "/berserk/3.cbz".into(),
                        name: "3".to_string(),
                        read: 0,
                        pages: 54,
                        chapter_number: 2,
                    },
                ],
            },
        ];

        db.insert_comics(&comics)?;

        assert_eq!(db.comics()?.len(), 2);
        assert_eq!(db.comic_with_chapters(2)?.chapters.len(), 2);

        Ok(())
    }
}
