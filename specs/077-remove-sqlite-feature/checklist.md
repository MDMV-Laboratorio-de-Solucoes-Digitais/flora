# Quality Checklist: 077 — remoção da feature `sqlite` em flora-core

> Itens gerados sem marcações; `[ ]` = aguardando revisão de qualidade de requisitos, **não** implementação.

## Requisitos

- [x] FR-001 — remoção de `sqlite = ["sqlx/sqlite"]` de `[features]` em `flora-core/Cargo.toml:61` especificada?
- [x] FR-002 — preservação integral de `default = []`, `test = []` e `dev-mint` especificada?
- [x] FR-003 — `Cargo.toml:143` (sqlx 0.9 com `postgres`, sem `sqlite`) e `.env.example:81` inalterados?
- [x] FR-004 — proibição de segredos, `.tokens/`, dependências novas e mudanças em gate/scripts/workflows especificada?

## Critérios de sucesso

- [x] SC-001 — ausência da chave `sqlite` em `[features]` de `flora-core/Cargo.toml` verificável?
- [x] SC-002 — features restantes byte-idênticas, sem feature default nova, verificável?
- [x] SC-003 — varredura determinística com zero consumidores de `sqlite` verificável?
- [x] SC-004 — diff de 1 linha em `flora-core/Cargo.toml` + 1 aresta em `Cargo.lock` + artefatos 077, gate PASS e `cargo metadata --locked --offline` exit 0 verificável?

## Portarias

- [x] R1 — `go (scoped)` em `decision.md` antes de `specify`?
- [x] R3 — zero perguntas carry-forward, registradas em `clarify.md`?
- [x] R4 e R7 — rastreabilidade registrada, sem execução indevida?
- [x] Nenhuma invocação de `/speckit.validate` ou `/speckit.release`?
- [x] Fronteira — `Cargo.toml` raiz, `.env.example`, código Rust, workflows, gate/scripts, `deny.toml`, ADRs, constituição, STATE, worktree 073, issues e leg RBAC fora de escopo?
- [x] `[NV]` — extensões de arquivo não varridas e necessidade offline real separadas da evidência determinística?
