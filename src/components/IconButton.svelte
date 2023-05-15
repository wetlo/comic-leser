<script lang="ts">
    export let tooltip: string | undefined = undefined;
</script>

<button on:click data-tooltip={tooltip}>
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
