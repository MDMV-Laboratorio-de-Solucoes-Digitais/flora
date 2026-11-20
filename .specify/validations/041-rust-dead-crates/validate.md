# Validation: 041 — Rust dependency debloat

**Data:** 2026-11-17 · **Baseline:** `dev@ccd6d656` · **Tipo:** deleção pura de deps,
sem mudança de comportamento.

## Gates determinísticos (antes → depois)

| Gate | Baseline | Depois | Resultado |
|---|---|---|---|
| `cargo check --workspace` | OK (0.63s cached) | OK (145 crates, 1m33s) | PASS |
| `cargo fmt --all -- --check` | — | limpo | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | — | "No issues found" | PASS |
| `cargo deny check bans licenses sources` | — | bans ok, licenses ok, sources ok | PASS |
| `cargo test --workspace` | 46 passed / 43 ignored | 46 passed / 43 ignored | PASS (idêntico) |
| `cargo vet check` | — | "Vetting Succeeded (424 exempted)" + aviso de prune | PASS |

## Delta do grafo

`Cargo.lock`: **4898 → 4529 linhas (−369)**, 34 pacotes removidos, **nenhum pacote
novo**. O `git diff Cargo.lock | grep '^+'` mostra 9 linhas `+ "bitflags",` — não são
dependências adicionadas, mas **desambiguação** do Cargo: ao remover `bcrypt` (que
trazia `bitflags 1.x`), resta uma única versão e o Cargo troca `"bitflags 2.13.1"` por
`"bitflags"` em 9 pacotes. Nenhuma crate nova entrou no grafo. Removidos: argon2, bcrypt,
dotenv, rust-i18n{,-macro,-support}, password-hash, blake2, blowfish, cipher, inout,
ahash, annotate-snippets, arraydeque, base62, bstr, crossbeam-{deque,epoch},
encoding_rs_io, glob, globset, globwalk, granit-parser, ignore, itertools,
nohash-hasher, normpath, serde-saphyr, triomphe, unicode-width, flora-files, flora-tasks.
Preservados: `dotenvy` (sqlx), `jsonwebtoken/rust_crypto`.

## Remoção auditável

- `Cargo.toml`: −2 members, −5 workspace deps. `flora-core/Cargo.toml`: −4 deps.
- `flora-tasks/`, `flora-files/` deletados (2 arquivos cada, folhas órfãs).
- `supply-chain/config.toml`: −6 exemptions; `dotenvy` (linha 396) preservada.
- `grep -rnE 'argon2|bcrypt|dotenv|rust_i18n|flora_tasks|flora_files' --include='*.rs'`
  → vazio antes e depois.

## Auditoria author-blind

Ver `/tmp/opencode/dark-factory-80-20/c041-04-author-blind-audit.md`. O verificador
confirmou independentemente: fmt/clippy/deny/test (46 passed/43 ignored)/vet todos
verdes; exatamente 6 exemptions removidas; `dotenvy` preservada; nenhum usage órfão;
as 9 linhas `+ "bitflags"` são desambiguação (bcrypt saiu do grafo).

**F-1 (P1) — falso positivo por concorrência.** O auditor reportou um fresh
`cargo check --workspace` com falha em `flora-api/src/routes/rbac.rs`. A falha ocorreu
porque o verificador rodou **concorrentemente com edições em andamento** daquele arquivo
(ciclo 042), capturando um estado intermediário inválido. Reexecutado o fresh check com
target isolado (`CARGO_TARGET_DIR=/tmp/opencode/c042-fresh-target`) sobre o estado final:
**330 crates compilados, exit 0 (3m19s)**. Nenhum defeito real no 041.

Lição de processo registrada: verificadores author-blind não devem rodar enquanto o autor
edita os mesmos arquivos; a auditoria deve ser disparada sobre um snapshot congelado.

## NV

- Deltas de memória/CPU em runtime não medidos (só tamanho do grafo/config).
- Forgejo/remote CI [NV]. Merge via ADR-0003 sob 4 required checks verdes.

## Fora do ciclo

RbacService real (c041-02) e RLS/role least-privilege (c041-03) — decisões de
escopo/segurança do SE, mantidas `[NV]`; não bloqueiam este debloat.
