<script lang="ts">
    export let value: string;
    export let pattern: string | undefined = undefined;
    export let widthAdjusting: boolean = false;
    export let placeholder: string = "";

    let width = "2ch";
    $: if (widthAdjusting) width = value.length + "ch";
</script>

{#if widthAdjusting}
    <input
        style="width: {width}"
        type="text"
        bind:value
        {pattern}
        {placeholder}
        class={$$props.class}
    />
{:else}
    <input
        type="text"
        bind:value
        {pattern}
        {placeholder}
        class={$$props.class}
    />
{/if}

<style>
    input {
        padding: 0;
        min-width: 1ch;
        width: 100%;

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

    input::placeholder {
        color: #555;
    }
</style>
