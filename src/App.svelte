<script lang="ts">
	import { open } from "@tauri-apps/api/dialog";
	export let name: string;

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

<main>
	<h1>Hello {name}!</h1>
	{#await imgProm}
		<button on:click={choose}> Choose image </button>
	{:then img}
		<p>{img}</p>
		<img src={img} alt="System" />
	{/await}
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
