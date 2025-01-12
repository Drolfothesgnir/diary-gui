<script lang="ts">
  import type { EntrySchema } from "../../types";
  import Entry from "./Entry.svelte";

  export let entries: EntrySchema[];
  export let onRemoveClick: (id: number) => void;
  export let onEditClick: (entry: EntrySchema) => void;
  export let onPinClick: (entry: EntrySchema) => void;
</script>

<ul class="entryList">
  {#each entries as entry}
    <Entry
      {entry}
      onRemoveClick={() => onRemoveClick(entry.id)}
      onEditClick={() => onEditClick(entry)}
      onPinClick={() => onPinClick(entry)}
    />
  {/each}
</ul>

<style lang="scss">
  .entryList {
    width: 50%;
    max-width: 600px;
    list-style: none;
  }
</style>
