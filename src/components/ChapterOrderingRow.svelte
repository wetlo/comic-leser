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

<tr>
    <td>
        <MenuIcon />
    </td>

    <td>
        <InplaceTextEdit bind:value={ordering.regex} />
    </td>
    <td />
    <td>{ordering.rank}</td>

    <td>
        <IconButton on:click={handleDelete}>
            <TrashIcon />
        </IconButton>
    </td>
</tr>

<style>
    tr:nth-child(even) {
        background-color: #111;
    }

    td {
        padding: 0;
    }
</style>
