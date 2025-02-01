<script lang="ts">
  import { onMount } from "svelte";
  import Editor from "../lib/components/Editor.svelte";

  import Header from "../lib/components/Header.svelte";
  import Main from "../lib/components/Main.svelte";
  import NewEntry from "../lib/components/NewEntry.svelte";
  import Home from "$lib/components/Home.svelte";
  import {
    createEntry,
    deleteEntry,
    dumpEntries,
    getEntries,
    putEntry,
  } from "$lib/utils/requests";
  import {
    EditorState,
    PinnedFilter,
    SortOrder,
    type EntrySchema,
  } from "../types";
  import type { ChangeEventHandler, EventHandler } from "svelte/elements";

  // dumping status
  let dumpingInProcess = false;

  // pagination and filter state
  let page = 1;
  let per_page = 10;
  let has_next = true;
  let pinned = PinnedFilter.ALL;
  let search = "";
  let sort = SortOrder.DESC;

  // editor state
  let editorState: EditorState = EditorState.CLOSED;
  let editId: number | null = null;
  let editorContent: string = "";

  // entries
  let entries: EntrySchema[] = [];

  const addEntry = async (content: string, pinned?: boolean) => {
    const entry = await createEntry(content, pinned);
    entries = [entry, ...entries];
  };

  // Function to edit an entry
  const editEntry = async (id: number, content: string) => {
    const entry = await putEntry(id, content);
    const index = entries.findIndex((entry) => entry.id === id);
    const updatedEntries = [...entries];
    updatedEntries[index] = entry;
    entries = updatedEntries;
  };

  const loadEntries = async () => {
    const pagination = await getEntries(page, per_page, pinned, search, sort);
    page += 1;
    has_next = pagination.has_next;
    entries = entries.concat(pagination.entries);
  };

  const startNewEntry = () => {
    editorState = EditorState.NEW;
  };

  const startEntryEdit = (entry: EntrySchema) => {
    editId = entry.id;
    editorState = EditorState.EDIT;
    editorContent = entries.find(({ id }) => id === entry.id)!.content;
  };

  const togglePinEntry = async (id: number, pinned: boolean) => {
    const entry = await putEntry(id, undefined, pinned);
    const index = entries.findIndex((entry) => entry.id === id);
    const updatedEntries = [...entries];
    updatedEntries[index] = entry;
    entries = updatedEntries;
  };

  const alterEntryPinnedField = async (entry: EntrySchema) => {
    await togglePinEntry(entry.id, !entry.pinned);
  };

  const removeEntry = async (id: number) => {
    await deleteEntry(id);
    entries = entries.filter((entry) => entry.id !== id);
  };

  const closeEditor = () => {
    editId = null;
    editorState = EditorState.CLOSED;
    editorContent = "";
  };

  const onFilterFormSubmit: EventHandler<SubmitEvent, HTMLFormElement> = () => {
    page = 1;
    entries = [];
    loadEntries();
  };

  const searchChangeHandler: ChangeEventHandler<HTMLInputElement> = (e) => {
    search = e.currentTarget.value;
  };

  const pinnedChangeHandler = (pinnedFilter: PinnedFilter) => {
    pinned = pinnedFilter;
  };

  const sortOrderChangeHandler = (sortOrder: SortOrder) => {
    sort = sortOrder;
  };

  const onDumpEntriesClick = async () => {
    dumpingInProcess = true;
    await dumpEntries();
    dumpingInProcess = false;
  };

  const saveEntry = async (content: string) => {
    if (editorState === EditorState.NEW) {
      await addEntry(content);
    } else if (editorState === EditorState.EDIT && editId !== null) {
      await editEntry(editId, content);
    }
    closeEditor();
  };

  const clearFilters = () => {
    page = 1;
    per_page = 10;
    has_next = true;
    pinned = PinnedFilter.ALL;
    search = "";
    sort = SortOrder.DESC;
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    switch (e.key.toLowerCase()) {
      case "т":
      case "n": {
        if (editorState === EditorState.CLOSED) {
          e.preventDefault();
          startNewEntry();
        }
        break;
      }

      case "escape": {
        if (editorState > EditorState.CLOSED) {
          closeEditor();
        }
        break;
      }

      default:
        break;
    }
  };

  onMount(() => {
    document.addEventListener("keydown", handleKeyDown);
    loadEntries();

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  });
</script>

<Header
  onClearClicked={clearFilters}
  onSearchChanged={searchChangeHandler}
  onSortOrderChange={sortOrderChangeHandler}
  onPinnedChange={pinnedChangeHandler}
  onSubmit={onFilterFormSubmit}
  {sort}
  {pinned}
  {search}
/>
<Main>
  <Home
    {entries}
    hasNext={has_next}
    onEditClick={startEntryEdit}
    onLoadMoreClick={loadEntries}
    onPinClick={alterEntryPinnedField}
    onRemoveClick={removeEntry}
  />
</Main>
{#if editorState > 0}
  <Editor
    initialContent={editorContent}
    onContentSave={saveEntry}
    onCloseClick={closeEditor}
  />
{:else}
  <NewEntry
    onNewEntryClick={startNewEntry}
    dumpEntriesDisabled={dumpingInProcess}
    {onDumpEntriesClick}
  />
{/if}
