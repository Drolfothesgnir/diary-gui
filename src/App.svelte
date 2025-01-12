<script lang="ts">
  import { onMount } from "svelte";

  import { addEntry, editEntry } from "./lib/store/allEntriesStore";
  import Editor from "./lib/components/Editor.svelte";

  import Header from "./lib/components/Header.svelte";
  import Main from "./lib/components/Main.svelte";
  import NewEntry from "./lib/components/NewEntry.svelte";
  import {
    closeEditor,
    editId,
    editorContent,
    EditorState,
    editorState,
    startNewEntry,
  } from "./lib/store/editorStore";

  async function saveEntry(content: string) {
    if ($editorState === EditorState.NEW) {
      await addEntry(content);
    } else if ($editorState === EditorState.EDIT && $editId !== null) {
      await editEntry($editId, content);
    }
    closeEditor();
  }

  const handleKeyDown = (e: KeyboardEvent) => {
    switch (e.key.toLowerCase()) {
      case "т":
      case "n": {
        if ($editorState === EditorState.CLOSED) {
          e.preventDefault();
          startNewEntry();
        }
        break;
      }

      case "escape": {
        if ($editorState > EditorState.CLOSED) {
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

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  });
</script>

<Header />
<Main>huy</Main>
{#if $editorState > 0}
  <Editor
    initialContent={$editorContent}
    onContentSave={saveEntry}
    onCloseClick={closeEditor}
  />
{:else}
  <NewEntry onClick={startNewEntry} />
{/if}
