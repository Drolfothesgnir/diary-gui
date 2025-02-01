import { invoke } from "@tauri-apps/api/core";
import {
  PinnedFilter,
  SortOrder,
  type EntrySchema,
  type Pagination,
} from "../../types";

export type CommandResponse<T> = {
  data: T | null;
  error: string | null;
};

// page: Option<i64>,
// per_page: Option<i64>,
// sort: Option<SortOrder>,
// pinned: Option<bool>,
// substring: Option<String>,

export async function getEntries(
  page: number = 1,
  per_page: number = 10,
  pinned_val: PinnedFilter = PinnedFilter.ALL,
  search_val: string = "",
  sort = SortOrder.DESC
) {
  let pinned: boolean | undefined;
  if (pinned_val === PinnedFilter.PINNED) {
    pinned = true;
  } else if (pinned_val === PinnedFilter.UNPINNED) {
    pinned = false;
  }

  let substring: string | undefined;
  if (search_val.trim().length > 0) {
    substring = search_val;
  }

  const response: CommandResponse<Pagination> = await invoke("read_entries", {
    page,
    per_page,
    pinned,
    substring,
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

export async function dumpEntries() {
  const response: CommandResponse<void> = await invoke("dump_entries");

  if (response.error) {
    throw response.error;
  }

  return response.data!;
}
