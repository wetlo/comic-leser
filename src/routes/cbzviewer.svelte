<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { Loading } from "attractions";
    import type { Chapter } from "../entities/Chapter";

    //import { querystring } from "svelte-spa-router";
    interface Params {
        comic: string;
        chapter: string;
        page?: string;
    }

    export let params: Params;

    const comicId = parseInt(params.comic);
    let chapterNumber = parseInt(params.chapter);

    let chapter: Chapter | undefined;
    let chapterPromise: Promise<Chapter>;
    let wentBack: boolean = false;
    // begin on the page if it is given as a parameter
    let page = params.page ? parseInt(params.page) : 1;

    // get the new chapter when the number updates
    $: chapterPromise = invoke("chapter", { comicId, chapterNumber }).then(
        (c) => c as Chapter
    );

    $: {
        page = wentBack ? chapter.pages : page;
        // reset wentBack after the chapter has been loaded and pagenumber has been updated
        if (chapterNumber == chapter?.chapter_number) wentBack = false;
    }

    // update the read status when we progress on a chapter
    // TODO: maybe merge with other reactive thingy
    // make sure the chapter is already loaded
    $: if (chapterNumber == chapter?.chapter_number && page > chapter?.read)
        invoke("chapter_page_update", { id: chapter.id, page });

    function onKeypress(e: KeyboardEvent) {
        switch (e.key) {
            case "ArrowLeft":
                window.scrollTo(0, 0);
                page += 1;

                if (page > chapter.pages) {
                    page = 1;
                    chapterNumber += 1;
                }
                break;
            case "ArrowRight":
                page -= 1;
                if (page > 0 || chapterNumber > 1)
                    window.scrollTo(0, document.body.scrollHeight);
                if (page <= 0) {
                    if (chapterNumber > 1) {
                        chapterNumber -= 1;
                        wentBack = true;
                    }
                    page = 1;
                }
                break;
        }
    }

    // TODO: find replacement for this hack
    function setChapter(c: Chapter) {
        chapter = c;
        return "";
    }
</script>

<svelte:window on:keydown={onKeypress} />

{#await chapterPromise}
    <Loading />
{:then chapter}
    {setChapter(chapter)}

    <div class="hide-zone" />
    <span class="header">
        <div class="flex space-around">
            <span>{chapter.name}</span>
            <span>{page}/{chapter.pages}</span>
        </div>
    </span>

    <img
        alt="comic page"
        src={`comic://localhost/${encodeURIComponent(
            chapter.path
        )}?page=${page}`}
    />

    <img
        alt="comic page"
        src={`comic://localhost/${encodeURIComponent(chapter.path)}?page=${
            page + 1
        }`}
        class="cache"
    />
{/await}

<style>
    img {
        max-width: 100%;
    }

    .cache {
        position: fixed;
        top: 50vh;
        left: 50vw;
        z-index: -100;
        height: 1px;
        width: 1px;
    }

    .header {
        display: grid;
        margin-bottom: 2px;

        grid-template-rows: 0fr;
        transition: grid-template-rows 200ms;
    }

    .header > div {
        overflow: hidden;
    }

    .hide-zone {
        position: absolute;
        width: 100vw;
        height: 20vh;
        z-index: 5;
    }

    .hide-zone:hover + .header {
        grid-template-rows: 1fr;
    }
</style>
