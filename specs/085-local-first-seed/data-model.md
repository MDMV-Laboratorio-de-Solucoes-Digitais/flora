# Data Model: First Seed Journey local one-time bootstrap

**Feature**: 085-local-first-seed | **Date**: 2026-11-20T01:33:00Z

Todas as entidades já existem no modelo Seed; esta feature apenas define a transição de "instalação vazia" para "instalação inicializada".

## Organization

- **id** (`Uuid`): gerado com UUIDv7.
- **name** (`String`): nome informado no formulário, 1–255 caracteres após trim.
- **slug** (`String`): derivado do nome (lowercase, não-alfanumérico → `-`, hífens colapsados, bordas removidas); único no sistema.
- **settings** (`Json`): padrão de quotas do `Organization::new`.
- **created_at / updated_at**: timestamp da criação.
- **Regra**: primeira organização só pode ser criada quando a contagem de organizações não-sistema (`id <> 00000000-0000-0000-0000-000000000000`) for zero sob `pg_advisory_xact_lock`.

## User (administrador)

- **id** (`Uuid`): UUIDv7.
- **email** (`String`): lowercase, formato válido, único.
- **display_name** (`String`): `Admin`.
- **oidc_subject**: `NULL` — bootstrap não tem IdP (FR-012).
- **is_active**: `true`.
- **Regra**: e-mail já existente recusa a operação sem gravar nada.

## Role

- **id** (`Uuid`): UUIDv7.
- **organization_id**: organização recém-criada.
- **name**: `Admin`.
- **permissions**: `["*"]` — todas as permissões conforme `Role::has_permission`.
- **is_builtin**: `true` — papel não removível.
- **Regra**: criado na mesma transação da organização.

## Membership

- **user_id / organization_id / role_id**: chave composta ligando o administrador ao papel Admin.
- **metadata**: `{}`.
- **Regra**: única associação criada no bootstrap.

## Workspace

- **id** (`Uuid`): UUIDv7.
- **organization_id**: organização recém-criada.
- **name**: `Main Workspace`.
- **description**: `NULL`.
- **Regra**: um workspace inicial por instalação, criado na mesma transação.

## Session

- **id** (`Uuid`): UUIDv7.
- **user_id / organization_id**: administrador e nova organização.
- **jti** (`String`): UUIDv7 em texto, único.
- **expires_at**: `now + session_ttl_secs`.
- **is_active**: `true`.
- **client_ip / user_agent**: capturados da requisição.
- **Regra**: inserida na mesma transação; o JWT devolvido carrega o mesmo `jti`, `sub`, `organization_id` e `email`.

## State Transitions

```text
empty (0 organizations)
  -- POST /bootstrap (valid + lock acquired) --> initialized (6 objects + active session)
initialized
  -- any further POST /bootstrap --> 409, no state change
```

Não há update, delete reexecução ou "reset" nesta feature.
