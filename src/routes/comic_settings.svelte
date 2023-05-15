<script lang="ts">
    import { dndzone } from "svelte-dnd-action";
    import { TrashIcon } from "svelte-feather-icons";
    import {
        deleteChapterOrdering,
        getChapterOrderings,
        getComic,
        insertChapterOrdering,
        updateChapterOrdering,
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

    type DnDEvent<T> = CustomEvent<DndEvent<ChapterOrdering>> & {
        target: EventTarget & T;
    };
    function handleDndConsider<T>(e: DnDEvent<T>) {
        orderings = e.detail.items.map((o, i) => ({ ...o, rank: i + 1 }));
    }
    function handleDndFinalize<T>(e: DnDEvent<T>) {
        handleDndConsider(e);
        console.log(orderings);
        // update them all, well there shouldn't be too many orderings?
        orderings.forEach(updateChapterOrdering);
    }
</script>

{#await getComic(comicId)}
    <p>Loading</p>
{:then c}
    {#await orderPromise then os}
        {setOrderings(os)}
    {/await}

    <h1>{c.name}</h1>

    <section
        use:dndzone={{ items: orderings }}
        on:consider={handleDndConsider}
        on:finalize={handleDndFinalize}
    >
        {#each orderings as o (o.id)}
            <p>
                <span>{o.regex} - {o.rank}</span>
                <button on:click={() => deleteOrdering(o)}>
                    <TrashIcon />
                </button>
            </p>
        {/each}
    </section>

    <button on:click={addEmptyOrder}>Add</button>
{/await}
