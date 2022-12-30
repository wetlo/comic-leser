<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Chapter } from "../entities/Chapter";

    //import { querystring } from "svelte-spa-router";
    interface Params {
        comic: string;
        chapter: string;
    }

    export let params: Params;

    const comicId = parseInt(params.comic);
    let chapterNumber = parseInt(params.chapter);

    let chapter: Chapter | undefined;
    let wentBack: boolean = false;
    let page = 1;

    $: invoke("chapter", { comicId, chapterNumber }).then((c) => {
        chapter = c as Chapter;
    });

    $: {
        page = wentBack ? chapter.pages : page;
        // reset wentBack after the chapter has been loaded and pagenumber has been updated
        if (chapterNumber == chapter?.chapter_number) wentBack = false;
    }

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
</script>

<svelte:window on:keydown={onKeypress} />

{#if chapter}
    <p>{chapter.path}</p>
    <img
        alt="comic page"
        src={`comic://localhost${chapter.path}?page=${page}`}
    />
{:else}
    loading
{/if}
