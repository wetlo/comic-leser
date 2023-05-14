import { invoke } from "@tauri-apps/api";
import type { Comic } from "../entities/Comic";
import type { Chapter } from "../entities/Chapter";

export function getAllComics(): Promise<Comic[]> {
    return invoke("all_comics");
}

export function getComicWithChapters(comic_id: number): Promise<Comic | null> {
    return invoke("comic_with_chapters", { id: comic_id })
}

export function getChapterByNumber(comic_id: number, chapter_number: number): Promise<Chapter> {
    return invoke("chapter", { comicId: comic_id, chapter_number: 1 });
}

export function updateChapterReadStatus(chapter_id: number, page: number): Promise<void> {
    return invoke("chapter_page_update", { id: chapter_id, page });
}