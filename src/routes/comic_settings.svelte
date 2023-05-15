<script lang="ts">
    import {
        getChapterOrderings,
        getComic,
        insertChapterOrdering,
    } from "../api/api";
    import type { ChapterOrdering } from "../entities/ChapterOrdering";
    import type { Comic } from "../entities/Comic";

    export let params: { id: string };
    const comicId = +params.id;

    let orderPromise = getChapterOrderings(comicId);

    let orderings: ChapterOrdering[] = [];

    function setOrderings(os: ChapterOrdering[]): string {
        orderings = os;
        return "";
    }

    function addEmptyOrder(comic: Comic, orderings: ChapterOrdering[]) {
        const rank = orderings[orderings.length - 1]?.rank || 0;
        const order: ChapterOrdering = {
            id: 0, // invalid id for typescript
            rank: rank + 1,
            regex: "",
            comic_id: comic.id,
        };
        insertChapterOrdering(order);

        orderPromise = getChapterOrderings(comicId);
    }
</script>

{#await getComic(comicId)}
    <p>Loading</p>
{:then c}
    {#await orderPromise then os}
        {setOrderings(os)}
    {/await}

    <h1>{c.name}</h1>

    {#each orderings as o}
        <p>{o.regex} - {o.rank}</p>
    {/each}

    <button on:click={() => addEmptyOrder(c, orderings)}>Add</button>
{/await}
