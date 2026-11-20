-- Enforce idempotent unread notification dedup (022, Seed/Postgres-first).
-- Prevents duplicate unread rows for the same (user_id, event_type, target_id),
-- eliminating TOCTOU/race duplicates from best-effort producers.
CREATE UNIQUE INDEX IF NOT EXISTS idx_notifications_unread_user_event_target_unique
    ON notifications (user_id, event_type, target_id)
    WHERE is_read = false;
