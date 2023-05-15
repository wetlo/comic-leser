UPDATE chapterordering
SET rank = rank - 1
WHERE comic_id = (?1) and rank > (?2)