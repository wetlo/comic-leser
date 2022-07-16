<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";

    let page = 1;

    let cbzProm: Promise<string> = new Promise(() => {
        while (true) {} // halt the promise
    });

    let choose = () =>
        (cbzProm = open({
            multiple: false,
            filters: [
                {
                    name: "Comic",
                    extensions: ["cbz"],
                },
            ],
        })
            .then((s) => (Array.isArray(s) ? s[0] : s)))

    function onKeypress(e: KeyboardEvent) {
	switch (e.key) {
	    case 'ArrowLeft':
		window.scrollTo(0, 0);
		page += 1;
		break;
	    case 'ArrowRight':
		window.scrollTo(0, 0);
		page -= 1;
		break;
	}
    }
</script>

<svelte:window
    on:keydown={onKeypress}
/>

{#await cbzProm}
    <button on:click={choose}>Choose comic</button>
{:then cbz}
    <p>{cbz}</p>
    <img alt="comic page" src={`comic://localhost${cbz}?page=${page}`} />
{/await}

