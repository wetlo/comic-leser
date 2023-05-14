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

    function toggleChecked(c: Chapter) {
        let idx = checked.indexOf(c);

        // add chapter when not checked
        // and remove when checked
        idx > -1 ? checked.splice(idx, 1) : checked.push(c);
        checked = checked;
    }

    function getContinueLink(co: Comic): string {
        var cont = co.chapter_read + 1;
        
        if (cont >= co.chapters.length) cont = 1;

        const chap = co.chapters.find((c) => c.chapter_number == cont);

        // if you didn't begin reading the chapter begin with the first page
        return `#/reader/${co.id}/${cont}/${chap?.read || 1}`;
    }

    function getChapterLink(c: Chapter): string {
        let page: string | number = "";
        if (c.read < c.pages) page = c.read || 1;
        console.log(page);

        return `#/reader/${c.comic_id}/${c.chapter_number}/${page}`;
    }

    function onKeyDown(e: KeyboardEvent): void {
        switch (e.key) {
            case "Escape":
                history.back();
                break;
        }
    }
</script>

<svelte:window on:keydown="{onKeyDown}" />

<Navbar>
    {#await comicPromise}
        Loading comic
    {:then comic}
        <header>
            <img alt="cover" src="comic://localhost/{encodeURIComponent(
                comic.chapters[0].path
            )}?page=1" />
            <div class="banner">
                <span class="flex space-between v-center">
                    <h1>{comic.name}</h1>
                    <h2>{comic.chapter_read} / {comic.chapter_count}</h2>
                </span>
            </div>

        </header>
        <div class="chapters">
            <div class="operations flex flex-end">
                <a href={getContinueLink(comic)}>Continue</a>
                <button on:click={setRead}>Read</button>
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
                                on:change={() => toggleChecked(c)}
                            />
                        </td>
                        <td>
                            <a href={getChapterLink(c)}>{c.name}</a>
                        </td>
                        <td>{c.read} / {c.pages}</td>
                    </tr>
                {/each}
            </table>
        </div>
    {/await}
</Navbar>

<style>

    header {
        position: relative;
        width: 100%;
        height: 30vh;
        overflow: hidden;
        border-radius: 10px 10px 0 0;
    }

    header > img {
        width: 100%;
        margin-top: -50%;
    }

    .banner {
        width: 100%;
        height: 100%;
        position: absolute;
        bottom: 0;

        background: rgb(0,0,0);
        /* TODO: improve this gradient maybe */
        background: linear-gradient(0deg, rgba(0,0,0,1) 0%, rgba(0,0,0,0.75) 60%, rgba(0,0,0,0.40) 100%); 
    }

    .banner > span {
        color: white;
        position: absolute;
        bottom: 0;

        margin: 0 20px;
        width: calc(100% - 40px);
    }

    .chapters {
        border: solid;
        border-radius: 0 0 10px 10px;
    }

    .operations {
        background-color: #333333;
        color: lightgray;
        display: flex;
        justify-content: flex-end;
        align-items: center;
        padding: 5px;
        gap: 10px;
    }

    .operations > * {
        color: lightgray;
    }

    .operations > button {
        border: none;
        background-color: transparent;
        cursor: pointer;
    }

    .operations > button:hover {
        text-decoration: underline;
    }

    td > a {
        color: #333333;
    }
</style>
