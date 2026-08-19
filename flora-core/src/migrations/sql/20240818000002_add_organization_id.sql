-- Migration: add organization_id column to tenant‑scoped tables
-- Table: messages
ALTER TABLE messages ADD COLUMN organization_id UUID NOT NULL;
CREATE INDEX idx_messages_organization_id ON messages (organization_id);

-- Table: tasks
ALTER TABLE tasks ADD COLUMN organization_id UUID NOT NULL;
CREATE INDEX idx_tasks_organization_id ON tasks (organization_id);

-- Table: files
ALTER TABLE files ADD COLUMN organization_id UUID NOT NULL;
CREATE INDEX idx_files_organization_id ON files (organization_id);

-- Table: channels
ALTER TABLE channels ADD COLUMN organization_id UUID NOT NULL;
CREATE INDEX idx_channels_organization_id ON channels (organization_id);
