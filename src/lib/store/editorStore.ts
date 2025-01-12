import { writable, get } from "svelte/store";
import type { EntrySchema } from "../../types";
import { entries } from "./allEntriesStore";

export enum EditorState {
  CLOSED = 0,
  NEW = 1,
  EDIT = 2,
}

export const editorState = writable<EditorState>(EditorState.CLOSED);
export const editId = writable<number | null>(null);
export const editorContent = writable<string>("");

export function startNewEntry() {
  editorState.update(() => EditorState.NEW);
}

export function startEntryEdit(entry: EntrySchema) {
  editId.update(() => entry.id);
  editorState.update(() => EditorState.EDIT);
  editorContent.update(
    () => get(entries).find(({ id }) => id === entry.id)!.content
  );
}

export function closeEditor() {
  editId.update(() => null);
  editorState.update(() => EditorState.CLOSED);
  editorContent.update(() => "");
}
