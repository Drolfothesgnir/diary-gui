<script lang="ts">
  import { onMount } from "svelte";
  import EntryList from "./EntryList.svelte";
  import LoadMore from "./LoadMore.svelte";
  import {
    entries,
    loadEntries,
    removeEntry,
    togglePinEntry,
    hasNext,
    initialFetch,
  } from "../store/allEntriesStore";
  import { startEntryEdit } from "../store/editorStore";
  import type { EntrySchema } from "../../types";

  async function alterEntryPinnedField(entry: EntrySchema) {
    await togglePinEntry(entry.id, !entry.pinned);
  }

  onMount(() => {
    initialFetch();
  });
</script>

<EntryList
  entries={$entries}
  onRemoveClick={removeEntry}
  onEditClick={startEntryEdit}
  onPinClick={alterEntryPinnedField}
/>
{#if $hasNext}
  <LoadMore onClick={() => loadEntries()} />
{/if}
