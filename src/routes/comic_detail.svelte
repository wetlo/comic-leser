<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Comic } from "../entities/Comic";

    export let params: { id: string };
    let comicPromise = invoke("comic_with_chapters", {
        id: parseInt(params.id),
    }).then((v) => v as Comic | null);
</script>

{#await comicPromise}
    Loading comic
{:then comic}
    <h1>{comic.name}</h1>
    <table>
        <tr>
            <th>Path</th>
            <th>Read Status</th>
        </tr>
        {#each comic.chapters as c}
            <tr>
                <td>
                    <a href="#/reader/{c.comic_id}/{c.chapter_number}"
                        >{c.path}</a
                    >
                </td>
                <td>{c.read} / {c.pages}</td>
            </tr>
        {/each}
    </table>
{/await}
