-- 023: make Postgres FTS match hyphenated/dotted/underscored identifiers by their parts.
-- The 'simple' text-search config treats e.g. "quarterly-report-xyz.pdf" as a single
-- token, so sub-token queries ("report") could not match. Generated columns hold
-- derived data only; dropping and re-adding them loses no source data.

DROP INDEX IF EXISTS idx_messages_search_tsv;
ALTER TABLE messages DROP COLUMN IF EXISTS search_tsv;
ALTER TABLE messages
    ADD COLUMN search_tsv tsvector
    GENERATED ALWAYS AS (
        to_tsvector(
            'simple',
            replace(replace(replace(coalesce(content, ''), '-', ' '), '.', ' '), '_', ' ')
        )
    ) STORED;
CREATE INDEX idx_messages_search_tsv
    ON messages USING GIN (search_tsv)
    WHERE is_deleted = false;

DROP INDEX IF EXISTS idx_tasks_search_tsv;
ALTER TABLE tasks DROP COLUMN IF EXISTS search_tsv;
ALTER TABLE tasks
    ADD COLUMN search_tsv tsvector
    GENERATED ALWAYS AS (
        to_tsvector(
            'simple',
            replace(replace(replace(coalesce(title, ''), '-', ' '), '.', ' '), '_', ' ')
            || ' '
            || replace(replace(replace(coalesce(description, ''), '-', ' '), '.', ' '), '_', ' ')
        )
    ) STORED;
CREATE INDEX idx_tasks_search_tsv
    ON tasks USING GIN (search_tsv)
    WHERE is_deleted = false;

DROP INDEX IF EXISTS idx_files_search_tsv;
ALTER TABLE files DROP COLUMN IF EXISTS search_tsv;
ALTER TABLE files
    ADD COLUMN search_tsv tsvector
    GENERATED ALWAYS AS (
        to_tsvector(
            'simple',
            replace(replace(replace(coalesce(name, ''), '-', ' '), '.', ' '), '_', ' ')
            || ' '
            || replace(replace(replace(coalesce(file_type, ''), '-', ' '), '.', ' '), '_', ' ')
        )
    ) STORED;
CREATE INDEX idx_files_search_tsv
    ON files USING GIN (search_tsv)
    WHERE is_deleted = false;
