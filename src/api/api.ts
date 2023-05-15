import { invoke } from "@tauri-apps/api";
import type { Comic } from "../entities/Comic";
import type { Chapter } from "../entities/Chapter";
import type { ChapterOrdering } from "../entities/ChapterOrdering";

export function getAllComics(): Promise<Comic[]> {
    return invoke("all_comics");
}

export function getComic(comicId: number): Promise<Comic> {
    return invoke("comic", { id: comicId })
}

export function getComicWithChapters(comicId: number): Promise<Comic> {
    return invoke("comic_with_chapters", { id: comicId })
}

export function getChapterByNumber(comicId: number, chapterNumber: number): Promise<Chapter> {
    return invoke("chapter", { comicId, chapterNumber });
}

export function updateChapterReadStatus(chapterId: number, page: number): Promise<void> {
    return invoke("chapter_page_update", { id: chapterId, page });
}

export function getChapterOrderings(comicId: number): Promise<ChapterOrdering[]> {
    return invoke("chapter_orderings", { comicId });
}

export function insertChapterOrdering(ordering: ChapterOrdering): Promise<void> {
    return invoke("insert_ordering", { ordering })
}

export function deleteChapterOrdering(id: number): Promise<void> {
    return invoke("delete_ordering", { id })
}