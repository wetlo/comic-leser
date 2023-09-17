import { writable } from "svelte/store";
import type { Settings } from "./entities/Settings";
import { getSettings } from "./api/settings";
import type { Comic } from "./entities/Comic";
import { getAllComics, getChapterByNumber } from "./api/comic";



export const settings = writable<Settings | null>()
export const comics = writable<Comic[]>([])


export function reloadSettings(): void {
    getSettings().then(s => settings.set(s))
}
export async function reloadComics(): Promise<void> {
    const cs = await getAllComics();

    for (let i = 0; i < cs.length; i++) {
        const c = cs[i];
        c.chapters = [await getChapterByNumber(c.id, 1)];
    }

    comics.set(cs)
}

// initialize stores
reloadSettings();
reloadComics();

// TODO check for comic update events
