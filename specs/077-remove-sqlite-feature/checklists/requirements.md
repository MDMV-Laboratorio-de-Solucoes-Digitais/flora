# Requirements Quality Checklist — 077

> `[x]` = requisito revisado quanto à qualidade, **não** implementação concluída. Revisão em 2026-11-19T19:44Z.

## Completude

- [x] FR-001 define a remoção da linha `sqlite = ["sqlx/sqlite"]` de `[features]` em `flora-core/Cargo.toml:61`.
- [x] FR-002 preserva integralmente `default = []`, `test = []` e `dev-mint` (`flora-core/Cargo.toml:60,62-63`).
- [x] FR-003 mantém `Cargo.toml:143` (sqlx 0.9 com `postgres`, sem `sqlite`) e `.env.example:81` inalterados.
- [x] FR-004 proíbe segredos, credenciais, `.tokens/`, dependências novas e mudanças em gate/scripts/workflows.

## Clareza e consistência

- [x] SC-001 é verificável: `flora-core/Cargo.toml` não contém mais a chave `sqlite` em `[features]`.
- [x] SC-002 é verificável: `default = []` e as demais features permanecem byte-idênticas, sem feature default nova.
- [x] SC-003 é verificável: varredura determinística segue com zero consumidores que referenciam `sqlite`.
- [x] SC-004 é verificável: diff restrito a `flora-core/Cargo.toml` (1 linha), `Cargo.lock` (1 aresta `cc`) e artefatos 077; gate PASS e `cargo metadata --locked --offline` exit 0.
- [x] IDs explícitos e individuais: FR-001, FR-002, FR-003, FR-004 e SC-001, SC-002, SC-003, SC-004 — sem intervalos.
- [x] Evidência determinística dos manifests/config distinguida de `[NV]` para extensões de arquivo não varridas e necessidade offline real (ADR de projeto, §4 linha 52).

## Edge cases

- [x] E se a linha já estiver ausente? FR-001 é idempotente: nada a remover e SC-001 já satisfeito.
- [x] E se existirem `sqlite_path`/`is_sqlite()` em `database.rs`? Permanecem FORA de escopo: código Rust não é tocado.
- [x] E se `Cargo.lock` ainda contiver `sqlx-sqlite` como dependência opcional? Verificação por `cargo metadata --locked --offline` na implementação, sem editar o lock; resultado registrado quando aplicável.
- [x] E se houver risco de DB/rede/segredos? Nenhum comando de DB é executado; a fronteira de paths permanece explícita.

## Portarias

- [x] R1 `go (scoped)` registrado em `decision.md` (2026-11-19T19:42Z) antes da escrita.
- [x] R3 zero perguntas carry-forward, registradas em `clarify.md`.
- [x] R4 e R7 respeitados na rastreabilidade, sem invocar `/speckit.validate` nem `/speckit.release`.
- [x] Fora de escopo: `Cargo.toml` raiz, `.env.example`, código Rust, workflows, gate/scripts, `deny.toml`, ADRs, constituição, STATE, worktree 073, issues e leg RBAC.
