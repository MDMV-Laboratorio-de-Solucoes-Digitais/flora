# Release: 077 — remoção da feature `sqlite` em flora-core

**Ciclo**: 077 · **Data**: 2026-11-19T19:52Z · **Branch**: `feature/077-remove-sqlite-feature` · **Baseline**: `dev@cc421e42d`
**Tipo**: higiene de manifest (remoção de feature inerte). **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `flora-core/Cargo.toml:61` — linha `sqlite = ["sqlx/sqlite"]` removida de `[features]`; `default = []`, `test = []` e `dev-mint` permanecem byte-idênticos.
- `Cargo.lock` — 1 aresta removida (`cc` de `libsqlite3-sys`); resolução coerente confirmada por `cargo metadata --locked --offline`.
- Artefatos completos: assess (5), spec, clarify, plan, checklist/tasks, analyze (2 execuções), converge append-only, validate R4, release R7.

## SemVer

**Proposta:** patch `0.2.7` → `0.2.8` (remoção de superfície inerte, sem mudança de API, comportamento ou dependência ativa). Nenhuma tag criada.

## Changelog (FR / T)

- `FR-001` linha 61 removida; `FR-002` `default = []`, `test = []` e `dev-mint` preservados; `FR-003` `Cargo.toml:143` (sqlx postgres-only) e `.env.example:81` intactos; `FR-004` zero segredos, credenciais, dependências novas ou mudanças em gate/scripts/workflows.
- `SC-001`..`SC-004` verificados por `tomllib`, `grep`, `cargo tree`, `cargo check`, `check_gate_integrity.py`, `cargo metadata --locked --offline` e auditoria de diff (2 arquivos).
- `T001`..`T004` executados; verificador isolado `SqliteVerifier` **PASS-WITH-NITS**, nit (drift do lock) fechado nos artefatos.
- Nenhum bug de `.specify/bugs/` acionado; leg RBAC permanece bloqueada por maintainer (fora do ciclo).

## Evidência

`.specify/validations/077-remove-sqlite-feature/validate.md` · `specs/077-remove-sqlite-feature/converge.md` · `specs/077-remove-sqlite-feature/analyze.md`.

## Rollback

Reverter, em `dev/`, os commits que tocaram exatamente os dois paths — `flora-core/Cargo.toml` (reintroduz `sqlite = ["sqlx/sqlite"]` na linha 61) e `Cargo.lock` (restaura a aresta `cc` de `libsqlite3-sys`) — com novo commit de reversão, sem force-push ou reescrita de histórico. Os artefatos do ciclo 077 são documentação e podem ser revertidos em bloco. Nenhum outro path é modificado: `Cargo.toml` raiz, `.env.example`, código Rust, workflows, gate/scripts, `deny.toml`, ADRs, constituição, STATE, DB/rede e segredos permanecem intactos.

## [NV]

Extensões de arquivo não varridas, necessidade offline real (ADR de projeto, §4 linha 52), CI ao vivo, Forgejo, aprovações e merge: **[NV]**.
