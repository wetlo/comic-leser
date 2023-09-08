<script>
    import { Divider } from "attractions";
    import {
        MenuIcon,
        HomeIcon,
        SettingsIcon,
        AlertTriangleIcon,
    } from "svelte-feather-icons";
    import LibrarySelection from "./LibrarySelection.svelte";
</script>

<div class="navbar flex column space-between">
    <div>
        <h1 class="flex row v-center"><MenuIcon />ComicLazer</h1>
        <Divider />
        <h2>
            <a href="#/" data-tooltip="Go to your library">
                <HomeIcon />
                Library
            </a>
        </h2>
        <h2 data-tooltip="Go to your settings">
            <SettingsIcon />
            Settings
        </h2>
        <h2>
            <a href="#/test" data-tooltip="Go to the Test page">
                <AlertTriangleIcon />
                Test Page
            </a>
        </h2>
    </div>
    <LibrarySelection />
</div>

<div class="main-content">
    <slot />
</div>

<style>
    .navbar {
        position: fixed;
        top: 0;
        left: 0;
        width: 20vw;
        height: 100vh;
    }

    h1 {
        column-gap: 0.5em;
    }

    .main-content {
        margin-left: 20vw;
        width: calc(100% - 20vw);
    }

    a:hover {
        text-decoration: none;
    }

    a {
        color: inherit;
    }
</style>
