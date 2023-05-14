<script lang="ts">
    import type { Chapter } from "../entities/Chapter";
    import InplaceEdit from "./InplaceEdit.svelte";

    export let chapter: Chapter;
    export let toggleChecked: (c: Chapter) => void;

    let status = chapter.read.toString();
    let read = chapter.read.toString();

    $: if (read && status != read) {

        if (parseInt(read) <= chapter.pages) {
            console.log("hello", read);
        } else {
            console.log("world", read)
            read = chapter.pages.toString();
        }

        console.log("world", read)
        status = read;
    }

    function getChapterLink(c: Chapter): string {
        const page = c.read == c.pages || c.read == 0 ? 1 : c.read;

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