-- N3: formaliza a chave composta usada como alvo da FK de memberships.
-- `id` já é PK, portanto a restrição é aditiva e não muda dados.
ALTER TABLE roles
    ADD CONSTRAINT uq_roles_org_id UNIQUE (organization_id, id);
