<script lang="ts">
    import InplaceEdit from "../components/InplaceEdit.svelte";

    let val = 0

    $: console.log(val);
</script>

<h1>This is a second page</h1>
<InplaceEdit type="number" bind:value="{val}"/>
