<script lang="ts">
    export let value: string;
    export let pattern: string | undefined = undefined;

    let width = "2ch";
    $: width = value.length + "ch";
</script>

<input style="width: {width}" type="text" bind:value {pattern} />

<style>
    input {
        padding: 0;
        min-width: 1ch;

        background-color: transparent;
        color: inherit;
        border: none;
        resize: none;
    }

    input:focus {
        border: none;
        border-bottom: solid 1px;
        color: inherit;
        outline: none;
    }
</style>
