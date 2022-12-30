<script lang="ts">
	import Router from "svelte-spa-router";
	import Test from "./routes/test.svelte";
	import Cbzviewer from "./routes/cbzviewer.svelte";
	import Library from "./routes/library.svelte";
	import ComicDetail from "./routes/comic_detail.svelte";

	const routes = {
		"/": Library,
		"/detail/:id": ComicDetail,
		"/reader": Cbzviewer,
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
