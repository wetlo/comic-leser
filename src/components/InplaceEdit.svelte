<script lang="ts">
    import type { HTMLInputTypeAttribute } from 'svelte/elements';
    export let type: HTMLInputTypeAttribute;

    export let value: any;
</script>

<input {type} {value} on:input={e => (value = e.currentTarget.value)}/>

<style>
    input {
        background-color: transparent;
        color: inherit;
        border: none;
    }

    input:focus {
        border: none;
        border-bottom: solid 1px;
        color: inherit;
        outline: none;
    }

    /* TODO: does this even work on all platforms?? */
    input[type=number]::-webkit-inner-spin-button,
    input[type=number]::-webkit-outer-spin-button { 
        -webkit-appearance: none; 
        margin: 0; 
    }
</style>