<script lang="ts">
    import { getChapterOrderings, getComic } from "../api/api";

    export let id: number;

    let comicPromise = Promise.all([getComic(id), getChapterOrderings(id)]);
</script>

{#await comicPromise}
    <p>Loading</p>
{:then [c, orderings]}
    <h1>{c.name}</h1>

    {#each orderings as o}
        <p>{o.regex} - {o.rank}</p>
    {:else}
        <p>Empty</p>
    {/each}
{/await}
