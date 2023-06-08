use std::{path::Path, sync::LazyLock};

use rusqlite::params;
use rusqlite_migration::{Migrations, M};
use tokio_rusqlite::{Connection, Result};

use crate::entities::{Chapter, ChapterOrdering, Comic};

static MIGRATIONS: LazyLock<Migrations<'static>> = LazyLock::new(|| {
    Migrations::new(vec![
        M::up(include_str!("sql/migrations/initial-migration.sql")),
        M::up(include_str!("sql/migrations/1-chapterOrdering.sql")),
    ])
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
// const CHAPTER_INSERT: &str =
//     "INSERT INTO chapter (file_path, chapter_number, read, pages, comic_id, name) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
const CHAPTER_UPSERT: &str = include_str!("sql/upsert_chapter.sql");
const CHAPTER_PAGE_UPDATE: &str = "UPDATE chapter SET read = (?2) WHERE id = (?1)";

const CHAPTER_ORDERING_QUERY: &str =
    "SELECT id, comic_id, rank, regex FROM chapterordering WHERE comic_id = (?1) ORDER BY rank";
const CHAPTER_ORDERING_BY_ID: &str =
    "SELECT id, comic_id, rank, regex FROM chapterordering WHERE id = (?1)";
const CHAPTER_ORDER_INSERT: &str =
    "INSERT INTO chapterordering (comic_id, rank, regex) VALUES (?1, ?2, ?3)";
const CHAPTER_ORDER_DELETE: &str = "DELETE FROM chapterordering WHERE id = (?1)";
const CHAPTER_ORDER_DECREMENT: &str = include_str!("sql/decrement_rank_ordering.sql");
const CHAPTER_ORDER_UPDATE: &str =
    "UPDATE chapterordering SET regex = (?2), rank = (?3) WHERE id = (?1)";

impl Database {
    pub async fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let conn = Connection::open(path).await?;

        conn.call(|c| {
            MIGRATIONS
                .to_latest(c)
                .map_err(|_| rusqlite::Error::InvalidQuery) // wait for github.com/programatik29/tokio-rusqlite/issues/18
        })
        .await?;

        Ok(Self { conn })
    }

    async fn _from_conn(conn: Connection) -> anyhow::Result<Self> {
        conn.call(|c| {
            MIGRATIONS
                .to_latest(c)
                .map_err(|_| rusqlite::Error::InvalidQuery) // wait for github.com/programatik29/tokio-rusqlite/issues/18
        })
        .await?;

        Ok(Self { conn })
    }

    pub async fn comics(&self) -> Result<Vec<Comic>> {
        self.conn
            .call(|c| {
                let mut query = c.prepare(COMIC_QUERY)?;
                let mut comics = query.query_map([], comic_from_row)?;
                comics.try_collect::<Vec<Comic>>()
            })
            .await
    }

    pub async fn comic(&self, comic_id: u32) -> Result<Comic> {
        self.conn
            .call(move |c| c.query_row(COMIC_QUERY_ID, [comic_id], comic_from_row))
            .await
    }

    pub async fn comic_with_chapters(&self, comic_id: u32) -> Result<Comic> {
        let mut comic = self.comic(comic_id).await?;
        self.conn
            .call(move |c| {
                let chaps = c
                    .prepare(CHAPTER_QUERY)?
                    .query_map([comic_id], chapter_from_row)?
                    .try_collect::<Vec<_>>()?;

                comic.chapters = chaps;

                Ok(comic)
            })
            .await
    }

    pub async fn chapter_by_number(&self, comic_id: u32, chapter_number: u32) -> Result<Chapter> {
        self.conn
            .call(move |c| {
                c.query_row(
                    CHAPTER_ORDER_QUERY,
                    [comic_id, chapter_number],
                    chapter_from_row,
                )
            })
            .await
    }

    pub async fn chapter_orderings(&self, comic_id: u32) -> Result<Vec<ChapterOrdering>> {
        self.conn
            .call(move |c| {
                let mut query = c.prepare(CHAPTER_ORDERING_QUERY)?;

                let mut orderings = query.query_map([comic_id], chapter_order_from_row)?;

                orderings.try_collect()
            })
            .await
    }

    pub async fn chapter_ordering(&self, id: u32) -> Result<ChapterOrdering> {
        self.conn
            .call(move |c| c.query_row(CHAPTER_ORDERING_BY_ID, [id], chapter_order_from_row))
            .await
    }

    pub async fn update_chapter_page(&mut self, chapter_id: u32, page: u32) -> Result<()> {
        self.conn
            .call(move |c| {
                c.execute(CHAPTER_PAGE_UPDATE, [chapter_id, page])
                    .map(|_| ())
            })
            .await
    }

    pub async fn update_chapter_ordering(&mut self, o: ChapterOrdering) -> Result<()> {
        self.conn
            .call(move |c| {
                c.execute(CHAPTER_ORDER_UPDATE, params![o.id, o.regex, o.rank])
                    .map(|_| ())
            })
            .await
    }

    pub async fn insert_comics(&mut self, comics: Vec<Comic>) -> Result<()> {
        self.conn
            .call(|c| {
                let tx = c.transaction()?;
                let mut insert = tx.prepare(COMIC_INSERT)?;

                for c in comics {
                    let id = insert.insert(params![
                        c.dir_path.to_string_lossy(),
                        c.name,
                        c.cover_path.as_ref().map(|p| p.to_string_lossy()),
                        c.is_manga,
                    ])?;

                    //Self::insert_chapters_transaction(&tx, &c.chapters, Some(id as u32))?;

                    let mut insert = tx.prepare(CHAPTER_UPSERT)?;

                    for c in c.chapters {
                        let comic = id;
                        insert.insert(params![
                            c.path.to_string_lossy(),
                            c.chapter_number,
                            c.read,
                            c.pages,
                            comic,
                            c.name,
                        ])?;
                    }
                }

                drop(insert);
                tx.commit()?;

                Ok(())
            })
            .await
    }

    pub async fn insert_chapters(&mut self, chapters: Vec<Chapter>) -> Result<()> {
        self.conn
            .call(move |c| {
                let tx = c.transaction()?;
                let mut insert = tx.prepare(CHAPTER_UPSERT)?;

                for c in chapters {
                    let comic = c.comic_id;
                    insert.insert(params![
                        c.path.to_string_lossy(),
                        c.chapter_number,
                        c.read,
                        c.pages,
                        comic,
                        c.name,
                    ])?;
                }
                drop(insert);
                tx.commit()?;
                Ok(())
            })
            .await
    }

    // fn insert_chapters_transaction(
    //     tx: &rusqlite::Transaction,
    //     chapters: &[Chapter],
    //     comic_id: Option<u32>,
    // ) -> Result<()> {
    //     let mut insert = tx.prepare(CHAPTER_UPSERT)?;

    //     for c in chapters {
    //         let comic = comic_id.unwrap_or(c.comic_id);
    //         insert.insert(params![
    //             c.path.to_string_lossy(),
    //             c.chapter_number,
    //             c.read,
    //             c.pages,
    //             comic,
    //             c.name,
    //         ])?;
    //     }

    //     Ok(())
    // }

    pub async fn insert_chapter_ordering(&mut self, ordering: ChapterOrdering) -> Result<()> {
        self.conn
            .call(move |c| {
                c.execute(
                    CHAPTER_ORDER_INSERT,
                    params![ordering.comic_id, ordering.rank, ordering.regex],
                )
                .map(|_| ())
            })
            .await
    }

    pub async fn delete_chapter_ordering(&mut self, id: u32) -> Result<()> {
        let order = self.chapter_ordering(id).await?;

        self.conn
            .call(move |c| {
                c.execute(CHAPTER_ORDER_DELETE, params![id])?;

                c.execute(CHAPTER_ORDER_DECREMENT, params![order.comic_id, order.rank])
                    .map(|_| ())
            })
            .await
    }
}
fn comic_from_row(r: &rusqlite::Row) -> rusqlite::Result<Comic> {
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

fn chapter_from_row(r: &rusqlite::Row) -> rusqlite::Result<Chapter> {
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

fn chapter_order_from_row(r: &rusqlite::Row) -> rusqlite::Result<ChapterOrdering> {
    Ok(ChapterOrdering {
        id: r.get(0)?,
        comic_id: r.get(1)?,
        rank: r.get(2)?,
        regex: r.get(3)?,
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

    #[tokio::test]
    async fn add_comics() -> Result<()> {
        let conn = Connection::open_in_memory().await?;
        let mut db = Database::_from_conn(conn).await?;

        let comics = vec![
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

        db.insert_comics(comics).await?;

        assert_eq!(db.comics().await?.len(), 2);
        assert_eq!(db.comic_with_chapters(2).await?.chapters.len(), 2);

        Ok(())
    }
}
