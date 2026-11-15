-- Dev-only bootstrap for Flora cycle 008 smoke (no OIDC).
-- Inserts/refreshes a pre-selection session for seeded Alice so a matching
-- JWT (see docs/dev-smoke.md) can call POST /api/v1/auth/select-organization.
--
-- Prereqs: migrations applied and dev-seed.sql loaded.
-- Safe to re-run (upserts fixed session id/jti).

BEGIN;

INSERT INTO sessions (
    id,
    user_id,
    organization_id,
    jti,
    refresh_token_id,
    client_ip,
    user_agent,
    expires_at,
    last_activity_at,
    is_active,
    created_at,
    updated_at
)
VALUES (
    '018f0000-0000-7000-8000-000000000010'::uuid,
    '22222222-2222-2222-2222-222222222222'::uuid, -- Alice (dev-seed.sql)
    '00000000-0000-0000-0000-000000000000'::uuid, -- nil org (pre-selection)
    '018f0000-0000-7000-8000-000000000001',
    NULL,
    '127.0.0.1',
    'dev-smoke',
    NOW() + INTERVAL '365 days',
    NOW(),
    TRUE,
    NOW(),
    NOW()
)
ON CONFLICT (jti) DO UPDATE SET
    id = EXCLUDED.id,
    user_id = EXCLUDED.user_id,
    organization_id = EXCLUDED.organization_id,
    refresh_token_id = NULL,
    client_ip = EXCLUDED.client_ip,
    user_agent = EXCLUDED.user_agent,
    expires_at = EXCLUDED.expires_at,
    last_activity_at = NOW(),
    is_active = TRUE,
    updated_at = NOW();

COMMIT;
