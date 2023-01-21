<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Navbar from "../components/Navbar.svelte";
    import type { Comic } from "../entities/Comic";

    export let params: { id: string };
    let comicPromise = invoke("comic_with_chapters", {
        id: parseInt(params.id),
    }).then((v) => v as Comic | null);
</script>

<Navbar>
    {#await comicPromise}
        Loading comic
    {:then comic}
        <h1>{comic.name}</h1>
        <div class="chapters">
            <div class="operations flex flex-end">
                <button>read</button>
            </div>
            <table>
                <thead>
                    <th />
                    <th>Name</th>
                    <th>Read Status</th>
                </thead>
                {#each comic.chapters as c}
                    <tr>
                        <td>
                            <input type="checkbox" />
                        </td>
                        <td>
                            <a href="#/reader/{c.comic_id}/{c.chapter_number}"
                                >{c.name}</a
                            >
                        </td>
                        <td>{c.read} / {c.pages}</td>
                    </tr>
                {/each}
            </table>
        </div>
    {/await}
</Navbar>

<style>
    .chapters {
        border-radius: 10px;
        border: solid;
    }

    .operations {
        background-color: #333333;
        color: lightgray;
    }

    a {
        color: #333333;
    }
</style>
