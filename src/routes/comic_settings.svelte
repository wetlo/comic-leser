<script lang="ts">
    import { TrashIcon } from "svelte-feather-icons";
    import {
        deleteChapterOrdering,
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

    function addEmptyOrder() {
        const rank = orderings[orderings.length - 1]?.rank || 0;
        const order: ChapterOrdering = {
            id: 0, // invalid id for typescript
            rank: rank + 1,
            regex: "",
            comic_id: comicId,
        };

        orderPromise = insertChapterOrdering(order).then((_) =>
            getChapterOrderings(comicId)
        );
    }

    function deleteOrdering(o: ChapterOrdering): void {
        orderPromise = deleteChapterOrdering(o.id).then((_) =>
            getChapterOrderings(comicId)
        );
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
        <p>
            <span>{o.regex} - {o.rank}</span>
            <button on:click={() => deleteOrdering(o)}>
                <TrashIcon />
            </button>
        </p>
    {/each}

    <button on:click={addEmptyOrder}>Add</button>
{/await}
