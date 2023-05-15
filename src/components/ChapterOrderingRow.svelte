<script lang="ts">
    import { TrashIcon } from "svelte-feather-icons";
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

<div class="flex space-between">
    <div class="regex">
        <InplaceTextEdit bind:value={ordering.regex} />
    </div>
    <span>{ordering.rank}</span>
    <IconButton on:click={handleDelete}>
        <TrashIcon />
    </IconButton>
</div>

<style>
    .regex {
        max-width: 40vw;
    }
</style>
