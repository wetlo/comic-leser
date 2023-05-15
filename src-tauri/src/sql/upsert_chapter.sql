INSERT INTO chapter (file_path, chapter_number, read, pages, comic_id, name)
VALUES (?1, ?2, ?3, ?4, ?5, ?6)
ON CONFLICT(file_path) DO UPDATE SET chapter_number=excluded.chapter_number;