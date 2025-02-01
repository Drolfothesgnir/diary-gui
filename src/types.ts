export type EntrySchema = {
  id: number;
  content: string;
  created_at: string;
  updated_at: string | null;
  pinned: boolean;
};

export type Pagination = {
  entries: EntrySchema[];
  has_next: boolean;
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
};

export enum PinnedFilter {
  ALL = "ALL",
  PINNED = "PINNED",
  UNPINNED = "UNPINNED",
}

export enum EditorState {
  CLOSED = 0,
  NEW = 1,
  EDIT = 2,
}

export enum SortOrder {
  ASC = "ASC",
  DESC = "DESC",
}
