<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { each } from "svelte/internal";
    import Navbar from "../components/Navbar.svelte";
    import type { Comic } from "../entities/Comic";

    async function get_comics() {
        const cs = (await invoke("all_comics")) as Comic[];

        for (let i = 0; i < cs.length; i++) {
            const c = cs[i];
            c.chapters = [
                await invoke("chapter", { comicId: c.id, chapterNumber: 1 }),
            ];
        }

        console.log(cs);
        return cs;
    }
</script>

<Navbar>
    {#await get_comics()}
        loading comics
    {:then cs}
        <div class="flex v-center comics">
            {#each cs as c}
                <a href="#/detail/{c.id}" class="card">
                    <img
                        alt="{c.name} cover"
                        src="comic://localhost/{encodeURIComponent(
                            c.chapters[0].path
                        )}?page=1"
                    />
                    <div class="info flex space-between v-center">
                        <p>{c.name}</p>
                        <p>{c.chapter_read}/{c.chapter_count}</p>
                    </div>
                </a>
            {/each}
        </div>
    {/await}
</Navbar>

<style>
    .card {
        position: relative;
        width: 8vw;
    }

    .info {
        width: 100%;
        background-color: rgba(0, 0, 0, 0.4);
        position: absolute;
        bottom: 0;
    }

    .comics {
        flex-wrap: wrap;
        column-gap: 1em;
        row-gap: 1em;
    }

    img {
        width: 100%;
    }

    p {
        text-align: start;
        color: white;
    }
</style>
