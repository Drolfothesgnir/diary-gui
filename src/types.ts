export type EntrySchema = {
  id: number;
  content: string;
  created_at: string;
  updated_at: string | null;
  pinned: boolean;
};
