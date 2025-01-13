import { invoke } from "@tauri-apps/api/core";
import type { EntrySchema, Pagination } from "../../types";

export type CommandResponse<T> = {
  data: T | null;
  error: string | null;
};

export async function getEntries(
  page: number = 1,
  per_page: number = 10,
  pinned?: boolean,
  s?: string,
  sort = "last"
) {
  const response: CommandResponse<Pagination> = await invoke("read_entries", {
    page,
    per_page,
    pinned,
    s,
    sort,
  });

  if (response.error) {
    throw response.error;
  }

  return response.data!;
}

export async function createEntry(content: string, pinned: boolean = false) {
  const response: CommandResponse<EntrySchema> = await invoke("create_entry", {
    content,
    pinned,
  });

  if (response.error) {
    throw response.error;
  }

  return response.data!;
}

export async function deleteEntry(id: number) {
  const response: CommandResponse<void> = await invoke("delete_entry", { id });
  if (response.error) {
    throw response.error;
  }
}

export async function putEntry(id: number, content?: string, pinned?: boolean) {
  const response: CommandResponse<EntrySchema> = await invoke("update_entry", {
    id,
    content,
    pinned,
  });

  if (response.error) {
    throw response.error;
  }

  return response.data!;
}
