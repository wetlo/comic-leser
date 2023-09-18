<script lang="ts">
    import { Divider } from "attractions";
    import { getSettings, selectLibrary } from "../api/settings";
    import type { LibraryConfig } from "../entities/LibraryConfig";
    import { push } from "svelte-spa-router";
    import { reloadSettings, settings } from "../store";

    let showmenu: boolean = false;

    $: className = showmenu ? "show" : "";

    function switchLibrary(library: LibraryConfig): void {
        selectLibrary(library.id);
        reloadSettings(); // TODO: maybe add those in the api
        //console.log($settings);
        showmenu = false;
        push("/");
    }

    $: selected_library = $settings?.libraries.find(
        (l) => l.id == $settings.selected_library
    );
</script>

<div>
    <Divider />
    {#if $settings == null}
        Loading settings
    {:else}
        <div class="pop-up-container">
            <button
                class="flex row space-between"
                on:click={() => (showmenu = !showmenu)}
            >
                <h3>{selected_library?.name}</h3>
            </button>

            <div class="selector {className}">
                <h4>Switch library</h4>
                <Divider />

                {#each $settings.libraries as l}
                    <button
                        class="library-card"
                        on:click={() => switchLibrary(l)}>{l.name}</button
                    >
                {/each}
            </div>
            <button
                class="outside-click {className}"
                on:click={() => (showmenu = false)}
            />
        </div>
    {/if}
</div>

<style>
    * {
        text-align: left;
    }

    button,
    button:focus {
        background: none;
        color: inherit;
        border: none;
        /* padding: inherit; */
        font: inherit;
        cursor: pointer;
        outline: inherit;
    }

    .pop-up-container {
        position: relative;
    }

    .selector {
        display: none;
        position: absolute;
        bottom: 100%;
        width: 20rem;

        margin: 1.5rem;
        padding: 1rem;
        padding-top: 0.3rem;

        border-radius: 5px;
        box-shadow: 0 2px 10px 0 rgba(0, 0, 0, 0.9);
        z-index: 10;
    }

    .show {
        display: block !important;
    }

    .outside-click {
        display: none;
        position: fixed;
        top: 0;
        left: 0;
        bottom: 0;
        right: 0;
    }

    .library-card {
        padding: 0.5rem;
        width: 100%;
    }

    .library-card:hover {
        background-color: #222222;
    }
</style>
