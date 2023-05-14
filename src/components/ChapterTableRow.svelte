<script lang="ts">
    import type { Chapter } from "../entities/Chapter";

    export let chapter: Chapter;
    export let toggleChecked: (c: Chapter) => void;

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
    <td class="text-left">
        <a href={getChapterLink(chapter)}>{chapter.name}</a>
    </td>
    <td>{chapter.read} / {chapter.pages}</td>
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
</style>