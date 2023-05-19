<script lang="ts">
    import { dndzone } from "svelte-dnd-action";
    import { PlusIcon, TrashIcon } from "svelte-feather-icons";
    import {
        deleteChapterOrdering,
        getChapterOrderings,
        getComic,
        insertChapterOrdering,
        updateChapterOrdering,
    } from "../api/api";
    import type { ChapterOrdering } from "../entities/ChapterOrdering";
    import type { Comic } from "../entities/Comic";
    import ChapterOrderingRow from "../components/ChapterOrderingRow.svelte";
    import InplaceTextEdit from "../components/InplaceTextEdit.svelte";
    import IconButton from "../components/IconButton.svelte";

    export let params: { id: string };
    const comicId = +params.id;

    let orderPromise = getChapterOrderings(comicId);

    let orderings: ChapterOrdering[] = [];

    let newOrderingRegex = "";

    function setOrderings(os: ChapterOrdering[]): string {
        orderings = os;
        return "";
    }

    function addEmptyOrder() {
        const rank = orderings[orderings.length - 1]?.rank || 0;
        const order: ChapterOrdering = {
            id: 0, // invalid id for typescript
            rank: rank + 1,
            regex: newOrderingRegex,
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

    <div class="table">
        <h2 class="text-left">Chapter Orderings</h2>
        <table cellspacing="0">
            <thead>
                <th />
                <th style="text-align: left;">Regex</th>
                <th />
                <th>#</th>
                <th />
            </thead>
            <tbody
                use:dndzone={{ items: orderings }}
                on:consider={handleDndConsider}
                on:finalize={handleDndFinalize}
            >
                {#each orderings as o (o.id)}
                    <ChapterOrderingRow
                        ordering={o}
                        handleDelete={() => deleteOrdering(o)}
                    />
                {/each}
            </tbody>
            <tr>
                <td />
                <td>
                    <InplaceTextEdit
                        bind:value={newOrderingRegex}
                        class="comic-settings-next-regex"
                        placeholder="Regex for new ordering"
                    />
                </td>
                <td />
                <td />
                <td>
                    <IconButton
                        on:click={addEmptyOrder}
                        tooltip="add new ordering"
                    >
                        <PlusIcon />
                    </IconButton>
                </td>
            </tr>
        </table>
    </div>
{/await}

<style>
    .table {
        display: grid;
        grid-template-rows: auto auto;
        margin: 0 auto;
        max-width: 50em;
        width: 100%;
    }

    table {
        /* max-width: 50em; */
        width: 100%;
    }

    thead {
        border: solid gray 10px;
    }

    :global(.comic-settings-next-regex) {
        border-bottom: solid 1px !important;
    }
</style>
