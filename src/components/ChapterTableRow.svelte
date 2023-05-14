<script lang="ts">
    import type { Chapter } from "../entities/Chapter";
    import InplaceEdit from "./InplaceEdit.svelte";

    export let chapter: Chapter;
    export let toggleChecked: (c: Chapter) => void;

    let status = chapter.read.toString();
    let read = chapter.read.toString();

    $: if (status != read) {
        console.log(read);
        status = read;
    }

    function getChapterLink(c: Chapter): string {
        let page: string | number = "";
        if (c.read < c.pages) page = c.read || 1;
        console.log(page);

        return `#/reader/${c.comic_id}/${c.chapter_number}/${page}`;
    }
</script>

<tr>
    <td class="check">
        <input
            type="checkbox"
            on:change={() => toggleChecked(chapter)}
        />
    </td>
    <td class="text-left name">
        <a href={getChapterLink(chapter)}>{chapter.name}</a>
    </td>
    <td class="pages"><InplaceEdit type="number" bind:value={read}/> / {chapter.pages}</td>
</tr>

<style>
    tr:nth-child(even) {
        background-color: #111;
    }

    td {
        padding: 10px;
    }

    td, td > a {
        color: white;
    }

    .check {
        width: 5%;
    }

    .name {
        width: 70%;
    }

    .pages {
        width: 25%;
    }
</style>