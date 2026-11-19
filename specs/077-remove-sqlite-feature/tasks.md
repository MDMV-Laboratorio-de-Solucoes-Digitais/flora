# Tasks: 077 — remoção da feature `sqlite` em flora-core

- **T001** Editar `flora-core/Cargo.toml`: remover a linha 61 `sqlite = ["sqlx/sqlite"]` de `[features]`, preservando `default = []` (`:60`), `test = []` (`:62`) e `dev-mint` (`:63`). *(FR-001, FR-002, SC-001, SC-002)*
- **T002** Manter a fronteira intacta: `Cargo.toml:143` (sqlx 0.9 com `postgres`, sem `sqlite`) e `.env.example:81` inalterados; zero segredos, `.tokens/`, dependências novas ou mudanças em gate/scripts/workflows; `sqlite_path`/`is_sqlite()` em `database.rs` fora de escopo. *(FR-003, FR-004, SC-003, SC-004)*
- **T003** Gates determinísticos: parse de `[features]` via `tomllib`, `grep` por `sqlite`, `cargo metadata --locked --offline`, `cargo tree --offline`, `check_gate_integrity.py`, `cargo check -p flora-core --offline` e `git diff --name-only`. *(FR-001, FR-003, FR-004, SC-001, SC-002, SC-003, SC-004)*
- **T004** Fechamento do ciclo: registrar `[NV]` para extensões de arquivo não varridas e necessidade offline real (ADR de projeto, §4 linha 52), confirmar o diff (`Cargo.toml` 1 linha + `Cargo.lock` 1 aresta + artefatos 077) e preparar rollback por reversão.

Rollback: reverter `flora-core/Cargo.toml` e `Cargo.lock` ao estado anterior; nenhum outro path é tocado (workflows, gate/scripts, `Cargo.toml` raiz, `.env.example` e código Rust permanecem intactos).

Convenção: um arquivo por commit; Conventional Commits em pt-BR; paths explícitos; nunca `git add -A`/`.`. Sem `/speckit.validate` nem `/speckit.release`.
