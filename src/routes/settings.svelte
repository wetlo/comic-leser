<script lang="ts">
    import { PlusIcon } from "svelte-feather-icons";
    import { addLibrary, getSettings } from "../api/settings";
    import IconButton from "../components/IconButton.svelte";
    import Navbar from "../components/Navbar.svelte";
    import InplaceTextEdit from "../components/InplaceTextEdit.svelte";
    import type { LibraryConfig } from "../entities/LibraryConfig";
    import LibraryCard from "../components/LibraryCard.svelte";
    import { reloadSettings, settings } from "../store";

    function addDummyLibrary() {
        const lib: LibraryConfig = {
            id: 0,
            name: "",
            path: "",
        };

        addLibrary(lib);
        reloadSettings();
    }
</script>

<Navbar>
    <h1>Settings</h1>

    {#if $settings == null}
        Loading settings
    {:else}
        <h2>Libraries</h2>

        <div class="library-editor">
            {#each $settings.libraries as l}
                <LibraryCard bind:library={l} />
            {/each}
            <IconButton on:click={addDummyLibrary}><PlusIcon /></IconButton>
        </div>
    {/if}
</Navbar>

<style>
    .library-editor {
        margin: 0 auto;
    }
</style>
