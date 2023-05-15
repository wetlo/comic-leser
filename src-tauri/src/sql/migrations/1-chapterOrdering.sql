CREATE TABLE chapterordering (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    regex TEXT NOT NULL,
    rank INTEGER NOT NULL,

    comic_id INTEGER,

    FOREIGN KEY (comic_id)
    REFERENCES comic (id)
	ON DELETE CASCADE
);