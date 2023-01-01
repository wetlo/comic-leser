<script>
    import { Divider } from "attractions";
    import { MenuIcon } from "svelte-feather-icons";
</script>

<div class="flex row">
    <div class="navbar">
        <h1 class="flex row v-center"><MenuIcon />ComicLazer</h1>
        <Divider />
        <h2>Settings</h2>
    </div>
    <div class="main-content">
        <slot />
    </div>
</div>

<style>
    .navbar {
        position: static;
        width: 20vw;
        height: 100vh;
    }

    .main-content {
        width: 100%;
    }
</style>
