<script lang="ts">
    import InplaceEdit from "../components/InplaceNumberEdit.svelte";

    let val = 0;
</script>

<h1>This is a second page</h1>
<InplaceEdit bind:value={val} />
