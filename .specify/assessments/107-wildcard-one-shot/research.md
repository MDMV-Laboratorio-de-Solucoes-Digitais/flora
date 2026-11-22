# Research: 107 — wildcard one-shot

- Baseline `df8c1e18` (dev após PR #352). Fontes: código local, ADRs 0001–0006, migration `20240818000001_init_schema.sql`, `check_gate_integrity.py:37-47,122-160`.
- Verificado: `Config` é imutável em `AppState.config` (`app_state.rs:24-29`); flag em memória não sobrevive restart.
- Verificado: não existe tabela de config global; singleton aditiva é a opção mais segura (`bootstrap.rs:29` já usa `pg_advisory_xact_lock`).
- Verificado: papéis administrativos são `Admin` (`bootstrap.rs:302-304`) e `Owner` (`rbac_service.rs:18,149-160`), built-in; `roles.organization_id == memberships.organization_id` é garantido por FK.
- Verificado: gate compara contagens exatas de testes `#[ignore]` (bootstrap 13, auth 23) — nenhum teste novo ignorado é permitido.
- [NV]: staging real, credenciais e restart multiréplica.
