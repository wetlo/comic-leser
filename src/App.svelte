<script lang="ts">
	import Router from "svelte-spa-router";
	import Test from "./routes/test.svelte";
	import Cbzviewer from "./routes/cbzviewer.svelte";
	import Library from "./routes/library.svelte";
	import ComicDetail from "./routes/comic_detail.svelte";
	import ComicSettings from "./routes/comic_settings.svelte";
    import Settings from "./routes/settings.svelte";

	const routes = {
		"/": Library,
		"/detail/:id": ComicDetail,
		"/detail/:id/settings": ComicSettings,
		"/reader/:comic/:chapter/:page?": Cbzviewer,
		"/settings": Settings,
		"/test": Test,
	};
</script>

<main>
	<Router {routes} />
</main>

<style>
	main {
		text-align: center;
		/* padding: 1em; */
		max-width: 240px;
		/* margin: 0 auto; */
	}

	:root {
		background-color: #171717;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
