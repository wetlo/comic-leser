import { invoke } from "@tauri-apps/api";
import type { Settings } from "../entities/Settings";
import type { LibraryConfig } from "../entities/LibraryConfig";

export async function getSettings(): Promise<Settings> {
    return invoke("get_settings")
}

export async function addLibrary(lib: LibraryConfig): Promise<void> {
    return invoke("add_library", { lib })
}

export async function selectLibrary(id: number): Promise<void> {
    return invoke("select_library", { id })
}

export async function deleteLibrary(id: number): Promise<void> {
    return invoke("delete_library", { id })
}

export async function updateLibrary(lib: LibraryConfig): Promise<void> {
    return invoke("update_library", { lib })
}