<script lang="ts">
    import { MenuIcon, TrashIcon } from "svelte-feather-icons";
    import type { ChapterOrdering } from "../entities/ChapterOrdering";
    import IconButton from "./IconButton.svelte";
    import InplaceTextEdit from "./InplaceTextEdit.svelte";
    import { updateChapterOrdering } from "../api/api";

    export let ordering: ChapterOrdering;
    export let handleDelete: () => void;

    let regex = ordering.regex;

    $: if (ordering.regex != regex) {
        updateChapterOrdering(ordering);
        regex = ordering.regex;
    }
</script>

<div class="flex space-between v-center row">
    <MenuIcon />
    <div class="regex">
        <InplaceTextEdit bind:value={ordering.regex} />
    </div>
    <span />
    <span>{ordering.rank}</span>

    <IconButton on:click={handleDelete}>
        <TrashIcon />
    </IconButton>
</div>

<style>
    .regex {
    }

    .row {
        display: grid;
        grid-template-columns: 10% 40% 40% 5% 5%;

        max-width: 50em;
        margin: 0 auto;
    }

    .row > * {
        align-self: center;
    }

    .row:nth-child(even) {
        background-color: #111;
    }
</style>
