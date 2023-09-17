<script lang="ts">
    import Navbar from "../components/Navbar.svelte";
    import { getAllComics, getChapterByNumber } from "../api/comic";
    import { comics } from "../store";

    /*async function get_comics() {
        const cs = await getAllComics();

        for (let i = 0; i < cs.length; i++) {
            const c = cs[i];
            c.chapters = [await getChapterByNumber(c.id, 1)];
        }

        console.log(cs);
        return cs;
    }*/
</script>

<Navbar>
    <div class="flex v-center comics">
        {#each $comics as c}
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
        {:else}
            no comics
        {/each}
    </div>
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
