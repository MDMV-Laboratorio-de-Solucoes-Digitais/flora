# Tasks: 071 — remover stub morto AuthService

- **T001** excluir `flora-core/src/services/auth_service.rs`. *(FR-001, SC-003)*
- **T002** `flora-core/src/services/mod.rs` — remover `mod`/`pub use`. *(FR-001, SC-001)*
- **T003** `flora-api/src/state/app_state.rs` — remover import/campo/init/literal. *(FR-002, SC-001)*
- **T004** 7 construtores de teste — remover imports e inicializadores, preservando `ServiceUnavailable`/`AuthProvider`. *(FR-003, FR-004, SC-001)*
- **T005** verificar `grep` zero e compilação/testes. *(SC-001, SC-002)*
- **T006** `STATE.md`, `analyze.md`, `converge.md`. *(FR-005)*
- **T007** `validate.md`, `release.md`, gates, PR, merge, higiene. *(SC-002, SC-003)*

## Convenção de commit

Um arquivo por commit, mensagem conventional em pt-BR, paths explícitos; nunca `git add -A`/`.`.
