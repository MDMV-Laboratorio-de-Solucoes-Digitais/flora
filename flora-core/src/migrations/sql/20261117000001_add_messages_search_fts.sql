-- 019: Postgres FTS support for message search (Seed/VPS, PG-first fallback)
ALTER TABLE messages
    ADD COLUMN IF NOT EXISTS search_tsv tsvector
    GENERATED ALWAYS AS (to_tsvector('simple', coalesce(content, ''))) STORED;

CREATE INDEX IF NOT EXISTS idx_messages_search_tsv
    ON messages USING GIN (search_tsv)
    WHERE is_deleted = false;
