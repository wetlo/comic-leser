CREATE TABLE library (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    dir_path TEXT NOT NULL UNIQUE,
    is_manga_db BOOLEAN DEFAULT TRUE
);

CREATE TABLE comic (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    dir_path TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,

    library_id INTEGER NOT NULL,

    cover_path TEXT,

    is_manga BOOLEAN,

    FOREIGN KEY (library_id)
    REFERENCES library (id)
	ON DELETE CASCADE
);

CREATE TABLE chapter (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_path TEXT NOT NULL UNIQUE,
    chapter_number INTEGER,

    read INTEGER DEFAULT 0 NOT NULL,
    pages INTEGER NOT NULL,

    comic_id INTEGER,

    FOREIGN KEY (comic_id)
    REFERENCES comic (id)
	ON DELETE CASCADE
);
