<script lang="ts">
    import { compute_rest_props } from "svelte/internal";

    export let tooltip: string | undefined = undefined;
</script>

<button on:click data-tooltip={tooltip} class={$$props.class}>
    <slot />
</button>

<style>
    button {
        border: none;
        color: inherit;
        background: transparent;
        cursor: pointer;
    }

    button:focus {
        border: none;
        background: transparent;
    }
</style>
