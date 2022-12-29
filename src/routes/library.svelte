<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { each } from "svelte/internal";
    import type { Comic } from "../entities/Comic";

    let comics = invoke("all_comics").then((v) => v as Comic[]);
</script>

{#await comics}
    loading comics
{:then cs}
    <ul>
        {#each cs as c}
            <li>{c.name}</li>
        {/each}
    </ul>
{/await}
