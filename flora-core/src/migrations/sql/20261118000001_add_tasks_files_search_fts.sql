-- 020: Postgres FTS support for task and file metadata search (Seed/VPS, PG-first fallback)
ALTER TABLE tasks
    ADD COLUMN IF NOT EXISTS search_tsv tsvector
    GENERATED ALWAYS AS (
        to_tsvector('simple', coalesce(title, '') || ' ' || coalesce(description, ''))
    ) STORED;

CREATE INDEX IF NOT EXISTS idx_tasks_search_tsv
    ON tasks USING GIN (search_tsv)
    WHERE is_deleted = false;

ALTER TABLE files
    ADD COLUMN IF NOT EXISTS search_tsv tsvector
    GENERATED ALWAYS AS (
        to_tsvector('simple', coalesce(name, '') || ' ' || coalesce(file_type, ''))
    ) STORED;

CREATE INDEX IF NOT EXISTS idx_files_search_tsv
    ON files USING GIN (search_tsv)
    WHERE is_deleted = false;
