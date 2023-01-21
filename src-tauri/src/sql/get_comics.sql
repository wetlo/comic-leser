SELECT c.id, dir_path, name, cover_path, is_manga, COUNT(*) as chapter_count,
-- gets the first one
(
    -- get all chapter number which are not read (ordered)
    SELECT chapter_number
    FROM (SELECT * FROM chapter ORDER BY chapter_number)
    WHERE c.id = comic_id
    AND read < pages
) -1 as read_chapters
FROM comic c
JOIN chapter ON c.id = comic_id
GROUP BY c.id
ORDER BY name;