<script lang="ts">
    export let value: number;
    export let pattern: string | undefined = undefined;

    let val = value;
    let text = value.toString();
    $: text = value.toString();
    let width = "2ch";
    $: width = text.length + "ch";

    $: {
        val = parseInt(text) || 0;
        text = val.toString();
    }

    $: value = val;
</script>

<input
    style="width: {width}"
    type="text" bind:value={text} {pattern}
/>

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