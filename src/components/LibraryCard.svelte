<script lang="ts">
    import { deleteLibrary, updateLibrary } from "../api/settings";
    import { open } from "@tauri-apps/api/dialog";
    import type { LibraryConfig } from "../entities/LibraryConfig";
    import InplaceTextEdit from "./InplaceTextEdit.svelte";
    import IconButton from "./IconButton.svelte";
    import { TrashIcon } from "svelte-feather-icons";
    import { reloadSettings } from "../store";

    export let library: LibraryConfig;

    let timer: NodeJS.Timeout;

    /*$: {
        console.log("hello");
        clearTimeout(timer);
        timer = setTimeout(() => updateLibrary(library), 500);
    }*/

    $: library && debounceUpdate();

    function debounceUpdate() {
        clearTimeout(timer);
        timer = setTimeout(() => updateLibrary(library), 500);
    }

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

<div class="card">
    <InplaceTextEdit
        class="name"
        bind:value={library.name}
        placeholder="Name"
    />
    <IconButton class="delete" on:click={deletelib}><TrashIcon /></IconButton>
    <InplaceTextEdit
        class="path"
        bind:value={library.path}
        placeholder="Path"
    />
    <button class="browse" on:click={browse}>Browse</button>
</div>

<style>
    .card {
        width: 100%;
        display: grid;
        grid-template-columns: repeat(5, 1fr);
        grid-template-rows: repeat(2, 1fr);
        grid-column-gap: 0px;
        grid-row-gap: 0px;

        box-shadow: 5px 5px 5px 3px rgba(0, 0, 0, 0.1);
    }

    .card :global(.name) {
        grid-area: 1 / 1 / 2 / 4;
        font-size: 2rem;
    }
    .card :global(.delete) {
        grid-area: 1 / 5 / 2 / 6;
    }
    .card :global(.path) {
        grid-area: 2 / 1 / 3 / 5;
        font-size: 1.2rem;
    }
    .browse {
        grid-area: 2 / 5 / 3 / 6;
    }
</style>
