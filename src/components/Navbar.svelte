<script>
    import { Divider } from "attractions";
    import { MenuIcon } from "svelte-feather-icons";
</script>

<div class="navbar">
    <h1 class="flex row v-center"><MenuIcon />ComicLazer</h1>
    <Divider />
    <h2>Settings</h2>
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
</style>
