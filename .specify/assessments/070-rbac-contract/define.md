# Define: 070 — contrato RBAC real no lugar do stub

## Problema

O contrato de `/roles` é uma asserção vazia. Uma regressão em autorização (listar papéis de outro tenant, criar papel sem `RoleWrite`, modificar papel built-in) passaria despercebida pela suíte, apesar dos testes unitários puros das funções de decisão.

## Requisitos funcionais

- **FR-001**: substituir o stub por módulo `#[cfg(test)] mod db` com `db_state()` fail-closed (sem env → `bail!`), seguindo `workspace.rs`.
- **FR-002**: contrato `db_rbac_list_scoped_to_session_org` — 200, papel próprio presente, papel de outra org ausente.
- **FR-003**: contrato `db_rbac_create_requires_role_write_and_uses_session_org` — 201 para admin, corpo com OrganizationId da sessão; 403 para membro somente-leitura.
- **FR-004**: contrato `db_rbac_builtin_immutable_custom_patchable` — 403 ao PATCH de built-in, 200 ao PATCH de role customizada.
- **FR-005**: rastrear execução em `STATE.md` e artefatos SDD, um arquivo por commit.

## Critérios de sucesso

- **SC-001**: `cargo test -p flora-tests contract::rbac::db_ -- --ignored --test-threads=1` com Postgres descartável → todos passam; sem env → falha fechada (não `Ok(())`).
- **SC-002**: gates do alvo verdes (autoritativo local/`--remote`, fmt/clippy/deny/build/test); a contagem de legs `db-tests` não muda (nenhum piso alterado).
- **SC-003**: diff restrito a `flora-tests/src/contract/rbac.rs` + artefatos + `STATE.md`; zero mudança em código de produção, workflows ou migrations.

## Fora de escopo

Mudanças em handlers, migrations, ADRs, constituição, required checks; contratos para assign/revoke (podem vir em ciclo seguinte); execução em produção.
