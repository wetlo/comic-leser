<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Navbar from "../components/Navbar.svelte";
    import type { Chapter } from "../entities/Chapter";
    import type { Comic } from "../entities/Comic";

    export let params: { id: string };
    let comicPromise = invoke("comic_with_chapters", {
        id: parseInt(params.id),
    }).then((v) => v as Comic | null);

    let checked: Chapter[] = [];

    $: console.log(checked);

    function setRead() {
        checked.forEach((c) =>
            invoke("chapter_page_update", { id: c.id, page: c.pages })
        );
        comicPromise = comicPromise;
    }
</script>

<Navbar>
    {#await comicPromise}
        Loading comic
    {:then comic}
        <h1>{comic.name}</h1>
        <div class="chapters">
            <div class="operations flex flex-end">
                <button on:click={setRead}>read</button>
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
                            <input
                                type="checkbox"
                                on:change={() => {
                                    let idx = checked.indexOf(c);

                                    // add chapter when not checked
                                    // and remove when checked
                                    idx > -1
                                        ? checked.splice(idx, 1)
                                        : checked.push(c);
                                    checked = checked;
                                }}
                            />
                        </td>
                        <td>
                            <a
                                href="#/reader/{c.comic_id}/{c.chapter_number}/{c.read <
                                c.pages
                                    ? c.read
                                    : ''}">{c.name}</a
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
