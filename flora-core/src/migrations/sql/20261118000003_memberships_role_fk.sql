-- N3: garante no banco que uma membership só pode referenciar um papel
-- da própria organização. Fail-closed: se houver dados inválidos, a
-- migration aborta em vez de corromper ou remover registros.
DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM memberships m
        LEFT JOIN roles r ON r.id = m.role_id
        WHERE r.id IS NULL
    ) THEN
        RAISE EXCEPTION 'N3: memberships with orphan role_id; cleanup requires SE decision';
    END IF;

    IF EXISTS (
        SELECT 1
        FROM memberships m
        JOIN roles r ON r.id = m.role_id
        WHERE r.organization_id <> m.organization_id
    ) THEN
        RAISE EXCEPTION 'N3: cross-tenant membership/role pair; cleanup requires SE decision';
    END IF;
END $$;

ALTER TABLE memberships
    ADD CONSTRAINT fk_memberships_role_org
    FOREIGN KEY (organization_id, role_id)
    REFERENCES roles (organization_id, id)
    ON DELETE RESTRICT;
