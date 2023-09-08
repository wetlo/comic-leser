import { invoke } from "@tauri-apps/api";
import type { Settings } from "../entities/Settings";
import type { LibraryConfig } from "../entities/LibraryConfig";

export async function getSettings(): Promise<Settings> {
    return invoke("get_settings")
}

export async function addLibrary(lib: LibraryConfig): Promise<void> {
    return invoke("add_library", { lib })
}

export async function selectLibrary(path: string): Promise<void> {
    return invoke("select_library", { path })
}

export async function deleteLibrary(path: string): Promise<void> {
    return invoke("delete_library", { path })
}