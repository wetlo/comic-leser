<script lang="ts">
    import { deleteLibrary, updateLibrary } from "../api/settings";
    import { open } from "@tauri-apps/api/dialog";
    import type { LibraryConfig } from "../entities/LibraryConfig";
    import InplaceTextEdit from "./InplaceTextEdit.svelte";
    import IconButton from "./IconButton.svelte";
    import { TrashIcon } from "svelte-feather-icons";
    import { reloadSettings } from "../store";

    export let library: LibraryConfig;

    $: updateLibrary(library);

    async function browse() {
        const selected = await open({
            directory: true,
            multiple: false,
        });

        if (typeof selected == "string") {
            library.path = selected;
        }
    }

    async function deletelib() {
        // todo: add confirmation screen
        await deleteLibrary(library.id);
        reloadSettings();
    }
</script>

<div>
    <InplaceTextEdit bind:value={library.name} placeholder="Name" />
    <InplaceTextEdit bind:value={library.path} placeholder="Path" />
    <button on:click={browse}>Browse</button>
    <IconButton on:click={deletelib}><TrashIcon /></IconButton>
</div>
