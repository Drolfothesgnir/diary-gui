import { invoke } from "@tauri-apps/api/core";
import type { EntrySchema } from "../../types";

export async function getEntries(
  page: number = 1,
  per_page: number = 10,
  pinned?: boolean,
  s?: string,
  sort = "last"
) {
  const response: { items: EntrySchema[]; has_next: boolean } = await invoke(
    "read_entries",
    {
      page,
      per_page,
      pinned,
      s,
      sort,
    }
  );

  return response;
}

export async function createEntry(content: string, pinned?: boolean) {
  const response: EntrySchema = await invoke("create_entry", {
    content,
    pinned,
  });
  return response;
}

export async function deleteEntry(id: number) {
  await invoke("delete_entry", { id });
}

export async function putEntry(id: number, content?: string, pinned?: boolean) {
  const response: EntrySchema = await invoke("update_entry", {
    id,
    content,
    pinned,
  });
  return response;
}
