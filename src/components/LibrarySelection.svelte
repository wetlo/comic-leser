<script lang="ts">
    import { Divider } from "attractions";
    import { getSettings } from "../api/settings";

    const settingsPromise = getSettings();
</script>

<div>
    <Divider />
    {#await settingsPromise}
        what?
    {:then settings}
        <div class="flex row space-between">
            <h3>{settings.libraries[settings.selected_library].name}</h3>
        </div>
    {/await}
</div>
