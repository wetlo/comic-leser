<script lang="ts">
    import Navbar from "../components/Navbar.svelte";
    import type { Chapter } from "../entities/Chapter";
    import type { Comic } from "../entities/Comic";
    import { CheckIcon, PlayIcon } from "svelte-feather-icons";
    import { getComicWithChapters, updateChapterReadStatus } from "../api/api";

    export let params: { id: string };

    let comicPromise = getComicWithChapters(parseInt(params.id));
    let checked: Chapter[] = [];

    $: console.log(checked);

    function setRead() {
        checked.forEach((c) =>
            updateChapterReadStatus(c.id, c.pages)
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
            <div class="banner-container">
                <img alt="cover" src="comic://localhost/{encodeURIComponent(
                    comic.chapters[0].path
                )}?page=1" />   
            </div>
            
            <div class="banner">
                <span>
                    <span class="flex space-between v-flex-end">
                        <h1>{comic.name}</h1>
                        <h2>{comic.chapter_read} / {comic.chapter_count}</h2>
                    </span>

                    <div class="operations flex flex-end">
                        <button on:click={setRead} data-tooltip="Set marked chapters as read">
                            <CheckIcon />
                        </button>
                        <a href={getContinueLink(comic)} data-tooltip="Continue from the last read chapter">
                            <PlayIcon />
                        </a>
                    </div>
                </span>
            </div>
        </header>

        <div class="chapters">
            <!-- <div class="operations flex flex-end">
                <a href={getContinueLink(comic)}>Continue</a>
                <button on:click={setRead}>Read</button>
            </div> -->
            <table cellspacing="0">
                <thead>
                    <th />
                    <th class="left">Name</th>
                    <th>Pages</th>
                </thead>
                {#each comic.chapters as c}
                    <tr>
                        <td class="check">
                            <input
                                type="checkbox"
                                on:change={() => toggleChecked(c)}
                            />
                        </td>
                        <td class="left">
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
        position: sticky;
        top: 0;
        width: 100%;
        height: 30vh;
        
        z-index: 5;
    }

    .banner-container > img {
        width: 100%;
        margin-top: -50%;
    }

    .banner-container {
        border-radius: 10px 10px 0 0;
        width: 100%;
        height: 100%;
        overflow: hidden;
    }

    .banner {
        width: 100%;
        height: 100%;
        position: absolute;
        bottom: 0;

        border-radius: 10px 10px 0 0;
        background: rgb(0,0,0);
        /* TODO: improve this gradient maybe */
        background: linear-gradient(0deg, rgba(0,0,0,1) 0%, rgba(0,0,0,0.75) 60%, rgba(0,0,0,0.40) 100%);

        z-index: 100000;
    }

    .banner > span {
        color: white;
        position: absolute;
        bottom: 0;
        left: 0;

        margin: 0 20px;
        width: calc(100% - 40px);
    }

    span > h1, span >h2 {
        margin-bottom: 0.1rem;
    }

    .chapters {
        border: solid;
        border-color: #111;
        border-radius: 0 0 10px 10px;
    }

    .operations {
        display: flex;
        justify-content: flex-end;
        align-items: center;
        padding: 5px;
        padding-top: 0;
        gap: 10px;
    }

    .operations > * {
        color: white;
    }

    .operations > button {
        border: none;
        background-color: transparent;
        cursor: pointer;
    }

    .operations > button:hover {
        text-decoration: underline;
    }

    table {
        width: 100%;
    }

    thead {
        position: sticky;
        top: 30vh;
        background-color: #171717;
    }

     td, th, td > a {
        color: lightgray;
    }

    td, th {
        padding: 10px;
    }

    th {
        text-decoration: underline;
    }

    tr:nth-child(even) {
        background-color: #111;
    }

    .left {
        text-align: left;
    }
</style>
