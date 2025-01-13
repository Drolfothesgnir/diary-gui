import { writable, get } from "svelte/store";
import {
  getEntries,
  createEntry,
  deleteEntry,
  putEntry,
} from "../utils/requests";
import type { EntrySchema } from "../../types";

// Writable store for entries
export const entries = writable<EntrySchema[]>([]);

// Pagination state (if needed)
export const page = writable<number>(1);
export const perPage = writable<number>(10);
export const hasNext = writable<boolean>(true);

// Function to load entries (and update the store)
export async function loadEntries(pinned?: boolean) {
  const currentPage = get(page);
  const currentPerPage = get(perPage);

  const pagination = await getEntries(currentPage, currentPerPage, pinned);
  const { entries: items, has_next } = pagination;

  entries.update((currentEntries) => currentEntries.concat(items));
  hasNext.set(has_next);
  page.update((n) => n + 1); // Increment page number after loading
}

export async function initialFetch(
  page_i = 1,
  per_page?: number,
  pinned?: boolean,
  s?: string,
  sort = "DESC"
) {
  const currentPerPage = per_page || get(perPage);
  const pagination = await getEntries(page_i, currentPerPage, pinned, s, sort);

  const { entries: items, has_next } = pagination;

  entries.update(() => items);
  hasNext.set(has_next);
  page.update(() => 2);
}

// Function to add a new entry
export async function addEntry(content: string, pinned?: boolean) {
  const entry = await createEntry(content, pinned);
  entries.update((currentEntries) => [entry, ...currentEntries]); // Add new entry to the beginning
}

// Function to edit an entry
export async function editEntry(id: number, content: string) {
  const entry = await putEntry(id, content);
  entries.update((currentEntries) => {
    const index = currentEntries.findIndex((entry) => entry.id === id);
    const updatedEntries = [...currentEntries];
    updatedEntries[index] = entry;
    return updatedEntries;
  });
}

// Function to delete an entry
export async function removeEntry(id: number) {
  await deleteEntry(id);
  entries.update((currentEntries) =>
    currentEntries.filter((entry) => entry.id !== id)
  );
}

// Function to toggle pinned status
export async function togglePinEntry(id: number, pinned: boolean) {
  const entry = await putEntry(id, undefined, pinned);
  entries.update((currentEntries) => {
    const index = currentEntries.findIndex((entry) => entry.id === id);
    const updatedEntries = [...currentEntries];
    updatedEntries[index] = entry;
    return updatedEntries;
  });
}
