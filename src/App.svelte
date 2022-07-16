<script lang="ts">
	import Router from "svelte-spa-router";
	import Imageviewer from "./routes/imageviewer.svelte";
	import Test from "./routes/test.svelte";
	import Cbzviewer from "./routes/cbzviewer.svelte";

	const routes = {
		"/": Cbzviewer,
		"/img": Imageviewer,
		"/test": Test,
	};
</script>

<main>
	<Router {routes} />
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
