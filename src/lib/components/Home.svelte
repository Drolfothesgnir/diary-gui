<script lang="ts">
  import EntryList from "./EntryList.svelte";
  import LoadMore from "./LoadMore.svelte";

  import type { EntrySchema } from "../../types";

  export let entries: EntrySchema[];
  export let hasNext: boolean;
  export let onPinClick: (entry: EntrySchema) => void;
  export let onRemoveClick: (id: number) => void;
  export let onEditClick: (entry: EntrySchema) => void;
  export let onLoadMoreClick: () => void;
</script>

<EntryList {entries} {onRemoveClick} {onEditClick} {onPinClick} />
{#if hasNext}
  <LoadMore onClick={onLoadMoreClick} />
{/if}
