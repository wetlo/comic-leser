<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import { querystring } from "svelte-spa-router";
    let params = new URLSearchParams($querystring);
    let path = params.get("path");

    let page = 1;

    let choose = () =>
        open({
            multiple: false,
            filters: [
                {
                    name: "Comic",
                    extensions: ["cbz"],
                },
            ],
        })
            .then((s) => (Array.isArray(s) ? s[0] : s))
            .then((p) => (path = p));

    function onKeypress(e: KeyboardEvent) {
        switch (e.key) {
            case "ArrowLeft":
                window.scrollTo(0, 0);
                page += 1;
                break;
            case "ArrowRight":
                window.scrollTo(0, 0);
                page -= 1;
                break;
        }
    }
</script>

<svelte:window on:keydown={onKeypress} />

{#if path}
    <p>{path}</p>
    <img alt="comic page" src={`comic://localhost${path}?page=${page}`} />
{:else}
    <button on:click={choose}>Choose comic</button>
{/if}
