-- 017-B: add workspace scoping for file records (additive, Seed/VPS safe)

ALTER TABLE files
    ADD COLUMN IF NOT EXISTS workspace_id UUID;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'files_workspace_id_fkey'
    ) THEN
        ALTER TABLE files
            ADD CONSTRAINT files_workspace_id_fkey
            FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE SET NULL;
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_files_workspace_id ON files(workspace_id);
CREATE INDEX IF NOT EXISTS idx_files_organization_workspace_id ON files(organization_id, workspace_id);
