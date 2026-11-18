# Progresso — Flora (estado verificado)

> **Atualizado:** 2026-11-18T02:16Z · **Âncora:** `github/dev@94fcf19ca`
> **Conteúdo:** fatos locais verificados por comando. Estado remoto/CI é **[NV]**.

## Âncora e gate

- `dev` local = `github/dev@94fcf19ca` (merge da PR #273, ciclo 055). `main` remoto em `82ee437f…`.
- Gate determinístico autoritativo: `python3 scripts/ci/check_gate_integrity.py` → **PASS**, 4 required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`), **8 pisos / 50 total**. ADR-0003 rege merge deliberado após CI verde.
- Worktrees/stashes/branches de feature: zero no início deste ciclo; ledger append-only em `.specify/queue/concurrency.jsonl` (runtime, gitignored).

## Contagens verificadas (por `find`, 2026-11-18T02:10Z)

| Artefato | Contagem |
|---|---:|
| `specs/` | 46 diretórios / 196 arquivos |
| `.specify/assessments/` | 43 diretórios / 170 arquivos |
| `.specify/validations/` | 42 diretórios / 43 arquivos |
| `.specify/releases/` | 42 diretórios / 42 arquivos |
| `.specify/bugs/` | 4 diretórios / 12 arquivos |
| `docs/research/` | 135 arquivos (índice em `docs/research/index.md`) |
| `docs/adr/` | 6 arquivos |

`.specify/feature.json` aponta `specs/051-dead-invite-ui` (ciclo fechado); identidade de ciclo é resolvida por inventário/`--dry-run`, nunca pelo nome do branch.

## O que funciona (evidência local)

- API Axum em `flora-api` com Postgres 18; rotas montadas e autenticação por sessão no Postgres (`jti` verificado), OIDC com JWKS, FTS do Postgres autoritativa.
- Suíte Rust: fmt/clippy/deny/build/test verdes (105 passed / 0 failed / 51 ignored no último run completo).
- Frontend Svelte 5: `bun run check` 0/0, lint ok, 42/42 testes, build ok.
- `db-tests` com 8 legs cobrindo 50 contratos DB, fail-closed por env e com piso de auth 13.

## Dívidas registradas

- ADR-0003 ainda cita "43" (seis legs) — correção depende de aprovação de maintainer: **[NV]**, escalada.
- Fase: `001–009` sem validate/release; `025` sem assessment; `035–038` sem artefatos SDD; `plan.md` ausente em `025–042`; `analyze.md`/`converge.md` ausentes antes de `047`.
- `index.md` na raiz ausente (fora do escopo de `docs/`).
- Falso positivo local: `cargo test --ignored` sem env reporta passes (CI já fail-closed; correção local exigiria 9 helpers/50 chamadores).

## NV

PRs/CI/proteção/merge/releases ao vivo, execução de runners e intenção futura do dono sobre RBAC/RLS/convites: **[NV]**. Nenhuma afirmação remota é feita sem verificação ao vivo neste documento.
