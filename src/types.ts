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
};
