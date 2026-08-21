-- =========================================================================
-- Flora Workspace — Development Seed Data
-- Run with: PGPASSWORD=flora psql -h 127.0.0.1 -U flora -d flora -f dev-seed.sql
-- =========================================================================

-- Use a transaction so we can rollback on error
BEGIN;

-- -------------------------------------------------------------------------
-- 1. ORGANIZATION
-- -------------------------------------------------------------------------
INSERT INTO organizations (id, name, slug, settings)
VALUES (
    '11111111-1111-1111-1111-111111111111'::uuid,
    'Acme Corporation',
    'acme-corp',
    '{"theme": "dark", "locale": "pt-BR"}'::jsonb
)
ON CONFLICT (slug) DO UPDATE SET
    name = EXCLUDED.name,
    settings = EXCLUDED.settings
RETURNING id;

-- -------------------------------------------------------------------------
-- 2. USERS
-- -------------------------------------------------------------------------
-- Alice (admin)
INSERT INTO users (id, email, display_name, avatar_url, profile, is_active)
VALUES (
    '22222222-2222-2222-2222-222222222222'::uuid,
    'alice@acme.test',
    'Alice Admin',
    'https://api.dicebear.com/7.x/avataaars/svg?seed=alice',
    '{"department": "Engineering", "title": "Tech Lead"}'::jsonb,
    true
)
ON CONFLICT (email) DO UPDATE SET
    display_name = EXCLUDED.display_name,
    profile = EXCLUDED.profile,
    is_active = EXCLUDED.is_active
RETURNING id;

-- Bob (member)
INSERT INTO users (id, email, display_name, avatar_url, profile, is_active)
VALUES (
    '33333333-3333-3333-3333-333333333333'::uuid,
    'bob@acme.test',
    'Bob Builder',
    'https://api.dicebear.com/7.x/avataaars/svg?seed=bob',
    '{"department": "Engineering", "title": "Senior Engineer"}'::jsonb,
    true
)
ON CONFLICT (email) DO UPDATE SET
    display_name = EXCLUDED.display_name,
    profile = EXCLUDED.profile,
    is_active = EXCLUDED.is_active
RETURNING id;

-- Carol (viewer)
INSERT INTO users (id, email, display_name, avatar_url, profile, is_active)
VALUES (
    '44444444-4444-4444-4444-444444444444'::uuid,
    'carol@acme.test',
    'Carol Viewer',
    'https://api.dicebear.com/7.x/avataaars/svg?seed=carol',
    '{"department": "Design", "title": "UX Designer"}'::jsonb,
    true
)
ON CONFLICT (email) DO UPDATE SET
    display_name = EXCLUDED.display_name,
    profile = EXCLUDED.profile,
    is_active = EXCLUDED.is_active
RETURNING id;

-- -------------------------------------------------------------------------
-- 3. ROLES (with permission arrays matching flora-core Permission enum)
-- -------------------------------------------------------------------------
-- Admin role (all permissions)
INSERT INTO roles (id, organization_id, name, permissions, description, is_builtin)
VALUES (
    '55555555-5555-5555-5555-555555555555'::uuid,
    '11111111-1111-1111-1111-111111111111'::uuid,
    'Admin',
    '[
        "OrganizationRead", "OrganizationWrite", "OrganizationAdmin",
        "WorkspaceRead", "WorkspaceWrite", "WorkspaceAdmin",
        "ChannelRead", "ChannelWrite", "ChannelAdmin",
        "MessageRead", "MessageWrite", "MessageAdmin",
        "TaskRead", "TaskWrite", "TaskAdmin",
        "FileRead", "FileWrite", "FileAdmin",
        "SearchRead", "SearchAdmin",
        "NotificationRead", "NotificationWrite", "NotificationAdmin",
        "UserRead", "UserWrite", "UserAdmin",
        "RoleRead", "RoleWrite", "RoleAdmin",
        "MembershipRead", "MembershipWrite", "MembershipAdmin"
    ]'::jsonb,
    'Full administrative access to the organization',
    true
)
ON CONFLICT DO NOTHING;

-- Member role (read/write on most resources, no admin)
INSERT INTO roles (id, organization_id, name, permissions, description, is_builtin)
VALUES (
    '66666666-6666-6666-6666-666666666666'::uuid,
    '11111111-1111-1111-1111-111111111111'::uuid,
    'Member',
    '[
        "OrganizationRead",
        "WorkspaceRead", "WorkspaceWrite",
        "ChannelRead", "ChannelWrite",
        "MessageRead", "MessageWrite",
        "TaskRead", "TaskWrite",
        "FileRead", "FileWrite",
        "SearchRead",
        "NotificationRead", "NotificationWrite",
        "UserRead",
        "RoleRead",
        "MembershipRead"
    ]'::jsonb,
    'Standard member with read/write access to workspaces, channels, tasks, files',
    true
)
ON CONFLICT DO NOTHING;

-- Viewer role (read-only)
INSERT INTO roles (id, organization_id, name, permissions, description, is_builtin)
VALUES (
    '77777777-7777-7777-7777-777777777777'::uuid,
    '11111111-1111-1111-1111-111111111111'::uuid,
    'Viewer',
    '[
        "OrganizationRead",
        "WorkspaceRead",
        "ChannelRead",
        "MessageRead",
        "TaskRead",
        "FileRead",
        "SearchRead",
        "NotificationRead",
        "UserRead",
        "RoleRead",
        "MembershipRead"
    ]'::jsonb,
    'Read-only access to all resources',
    true
)
ON CONFLICT DO NOTHING;

-- -------------------------------------------------------------------------
-- 4. MEMBERSHIPS (link users to organization with roles)
-- -------------------------------------------------------------------------
-- Alice = Admin
INSERT INTO memberships (user_id, organization_id, role_id, metadata)
SELECT
    '22222222-2222-2222-2222-222222222222'::uuid,
    '11111111-1111-1111-1111-111111111111'::uuid,
    id,
    '{"invited_by": "system", "joined_via": "seed"}'::jsonb
FROM roles WHERE name = 'Admin' AND organization_id = '11111111-1111-1111-1111-111111111111'::uuid
ON CONFLICT (user_id, organization_id) DO UPDATE SET
    role_id = EXCLUDED.role_id,
    metadata = EXCLUDED.metadata;

-- Bob = Member
INSERT INTO memberships (user_id, organization_id, role_id, metadata)
SELECT
    '33333333-3333-3333-3333-333333333333'::uuid,
    '11111111-1111-1111-1111-111111111111'::uuid,
    id,
    '{"invited_by": "system", "joined_via": "seed"}'::jsonb
FROM roles WHERE name = 'Member' AND organization_id = '11111111-1111-1111-1111-111111111111'::uuid
ON CONFLICT (user_id, organization_id) DO UPDATE SET
    role_id = EXCLUDED.role_id,
    metadata = EXCLUDED.metadata;

-- Carol = Viewer
INSERT INTO memberships (user_id, organization_id, role_id, metadata)
SELECT
    '44444444-4444-4444-4444-444444444444'::uuid,
    '11111111-1111-1111-1111-111111111111'::uuid,
    id,
    '{"invited_by": "system", "joined_via": "seed"}'::jsonb
FROM roles WHERE name = 'Viewer' AND organization_id = '11111111-1111-1111-1111-111111111111'::uuid
ON CONFLICT (user_id, organization_id) DO UPDATE SET
    role_id = EXCLUDED.role_id,
    metadata = EXCLUDED.metadata;

-- -------------------------------------------------------------------------
-- 5. WORKSPACES
-- -------------------------------------------------------------------------
INSERT INTO workspaces (id, organization_id, name, description)
VALUES
    ('88888888-8888-8888-8888-888888888888'::uuid,
     '11111111-1111-1111-1111-111111111111'::uuid,
     'Main Workspace',
     'Primary workspace for Acme Corporation'),
    ('99999999-9999-9999-9999-999999999999'::uuid,
     '11111111-1111-1111-1111-111111111111'::uuid,
     'Design Workspace',
     'Design team collaboration space')
ON CONFLICT DO NOTHING;

-- -------------------------------------------------------------------------
-- 6. CHANNELS
-- -------------------------------------------------------------------------
INSERT INTO channels (id, workspace_id, organization_id, name, type)
VALUES
    ('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa'::uuid,
     '88888888-8888-8888-8888-888888888888'::uuid,
     '11111111-1111-1111-1111-111111111111'::uuid,
     'general',
     'Public'),
    ('bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb'::uuid,
     '88888888-8888-8888-8888-888888888888'::uuid,
     '11111111-1111-1111-1111-111111111111'::uuid,
     'random',
     'Public'),
    ('cccccccc-cccc-cccc-cccc-cccccccccccc'::uuid,
     '88888888-8888-8888-8888-888888888888'::uuid,
     '11111111-1111-1111-1111-111111111111'::uuid,
     'engineering',
     'Private'),
    ('dddddddd-dddd-dddd-dddd-dddddddddddd'::uuid,
     '99999999-9999-9999-9999-999999999999'::uuid,
     '11111111-1111-1111-1111-111111111111'::uuid,
     'design-general',
     'Public')
ON CONFLICT DO NOTHING;

-- -------------------------------------------------------------------------
-- 7. MESSAGES (sample conversations)
-- -------------------------------------------------------------------------
INSERT INTO messages (id, channel_id, organization_id, sender_id, content, thread_id)
VALUES
    -- General channel messages
    ('eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee'::uuid,
     'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa'::uuid,
     '11111111-1111-1111-1111-111111111111'::uuid,
     '22222222-2222-2222-2222-222222222222'::uuid,
     'Welcome to Acme Corporation! 🎉 This is the general channel.',
     NULL),
    ('ffffffff-ffff-ffff-ffff-ffffffffffff'::uuid,
     'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa'::uuid,
     '11111111-1111-1111-1111-111111111111'::uuid,
     '33333333-3333-3333-3333-333333333333'::uuid,
     'Thanks Alice! Excited to be here.',
     NULL),
    -- Thread reply
    ('11111111-2222-3333-4444-555555555555'::uuid,
     'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa'::uuid,
     '11111111-1111-1111-1111-111111111111'::uuid,
     '44444444-4444-4444-4444-444444444444'::uuid,
     'Looking forward to collaborating!',
     'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee'::uuid),
    -- Random channel
    ('22222222-3333-4444-5555-666666666666'::uuid,
     'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb'::uuid,
     '11111111-1111-1111-1111-111111111111'::uuid,
     '33333333-3333-3333-3333-333333333333'::uuid,
     'Anyone up for coffee? ☕',
     NULL)
ON CONFLICT DO NOTHING;

-- -------------------------------------------------------------------------
-- 8. TASKS
-- -------------------------------------------------------------------------
INSERT INTO tasks (id, workspace_id, organization_id, creator_id, assignee_id, title, description, status)
VALUES
    ('33333333-4444-5555-6666-777777777777'::uuid,
     '88888888-8888-8888-8888-888888888888'::uuid,
     '11111111-1111-1111-1111-111111111111'::uuid,
     '22222222-2222-2222-2222-222222222222'::uuid,
     '33333333-3333-3333-3333-333333333333'::uuid,
     'Set up CI/CD pipeline',
     'Configure GitHub Actions for automated testing and deployment',
     'InProgress'),
    ('44444444-5555-6666-7777-888888888888'::uuid,
     '88888888-8888-8888-8888-888888888888'::uuid,
     '11111111-1111-1111-1111-111111111111'::uuid,
     '22222222-2222-2222-2222-222222222222'::uuid,
     '44444444-4444-4444-4444-444444444444'::uuid,
     'Design new dashboard UI',
     'Create Figma mockups for the analytics dashboard redesign',
     'Todo'),
    ('55555555-6666-7777-8888-999999999999'::uuid,
     '99999999-9999-9999-9999-999999999999'::uuid,
     '11111111-1111-1111-1111-111111111111'::uuid,
     '44444444-4444-4444-4444-444444444444'::uuid,
     '33333333-3333-3333-3333-333333333333'::uuid,
     'Write API documentation',
     'Document all REST endpoints with OpenAPI/Swagger specs',
     'Done')
ON CONFLICT DO NOTHING;

-- -------------------------------------------------------------------------
-- 9. NOTIFICATIONS (sample)
-- -------------------------------------------------------------------------
INSERT INTO notifications (id, organization_id, user_id, event_type, target_id, target_url, is_read)
VALUES
    ('66666666-7777-8888-9999-000000000000'::uuid,
     '11111111-1111-1111-1111-111111111111'::uuid,
     '33333333-3333-3333-3333-333333333333'::uuid,
     'Mention',
     'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee'::uuid,
     '/channels/general/messages/eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee',
     false),
    ('77777777-8888-9999-0000-111111111111'::uuid,
     '11111111-1111-1111-1111-111111111111'::uuid,
     '44444444-4444-4444-4444-444444444444'::uuid,
     'Assignment',
     '33333333-4444-5555-6666-777777777777'::uuid,
     '/tasks/33333333-4444-5555-6666-777777777777',
     false)
ON CONFLICT DO NOTHING;

-- -------------------------------------------------------------------------
-- 10. SESSIONS (placeholder - real sessions created via auth flow)
-- -------------------------------------------------------------------------
-- Not seeding sessions; they're created dynamically via /auth/callback

-- -------------------------------------------------------------------------
-- COMMIT
-- -------------------------------------------------------------------------
COMMIT;

-- -------------------------------------------------------------------------
-- VERIFICATION QUERIES (run separately to verify)
-- -------------------------------------------------------------------------
-- SELECT o.name, o.slug, u.email, u.display_name, r.name as role
-- FROM organizations o
-- JOIN memberships m ON m.organization_id = o.id
-- JOIN users u ON u.id = m.user_id
-- JOIN roles r ON r.id = m.role_id
-- WHERE o.slug = 'acme-corp';
--
-- SELECT w.name, c.name as channel, c.type
-- FROM workspaces w
-- JOIN channels c ON c.workspace_id = w.id
-- WHERE w.organization_id = '11111111-1111-1111-1111-111111111111'::uuid;
--
-- SELECT t.title, t.status, u.display_name as assignee
-- FROM tasks t
-- LEFT JOIN users u ON u.id = t.assignee_id
-- WHERE t.organization_id = '11111111-1111-1111-1111-111111111111'::uuid;