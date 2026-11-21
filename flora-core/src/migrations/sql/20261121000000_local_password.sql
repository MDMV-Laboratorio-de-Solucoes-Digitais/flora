-- Local password authentication fallback (cycle 096).
-- Existing OIDC-only users keep NULL hashes and cannot use password login.
ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash TEXT;
ALTER TABLE users ADD COLUMN IF NOT EXISTS password_updated_at TIMESTAMPTZ;
