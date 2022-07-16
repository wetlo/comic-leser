<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    //import { link } from "svelte-spa-router";

    let imgProm: Promise<string> = new Promise(() => {
        while (true) {} // halt the promise
    });

    let choose = () =>
        (imgProm = open({
            multiple: false,
            filters: [
                {
                    name: "Image",
                    extensions: ["png", "jpg"],
                },
            ],
        })
            .then((s) => (Array.isArray(s) ? s[0] : s))
            .then((s) => "image://localhost" + s));
</script>

<h1>Hello World!</h1>
<a href="#/test">To the second page</a>
{#await imgProm}
    <button on:click={choose}> Choose image </button>
{:then img}
    <p>{img}</p>
    <img src={img} alt="System" />
{/await}

<style>
    h1 {
        color: #ff3e00;
        text-transform: uppercase;
        font-size: 4em;
        font-weight: 100;
    }
</style>
