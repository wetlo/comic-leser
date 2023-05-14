<script>
    import { Divider } from "attractions";
    import { MenuIcon, HomeIcon, SettingsIcon, AlertTriangleIcon } from "svelte-feather-icons";
</script>

<div class="navbar">
    <h1 class="flex row v-center"><MenuIcon />ComicLazer</h1>
    <Divider />
    <h2><a href="#/" data-tooltip="Go to your library">
        <HomeIcon />
        Library
    </a></h2>
    <h2 data-tooltip="Go to your settings">
        <SettingsIcon/>
        Settings
    </h2>
    <h2><a href="#/test" data-tooltip="Go to the Test page">
        <AlertTriangleIcon />
        Test Page
    </a></h2>
</div>

<div class="main-content">
    <slot />
</div>

<style>
    .navbar {
        position: fixed;
        width: 20vw;
        height: 100vh;
    }

    .navbar > h1 {
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
