# Progresso — Flora (estado verificado)

> **Atualizado:** 2026-11-18T02:16Z · **Âncora:** `github/dev@94fcf19ca`
> **Conteúdo:** fatos locais verificados por comando. Estado remoto/CI é **[NV]**.

## Âncora e gate

- `dev` local = `github/dev@94fcf19ca` (merge da PR #273, ciclo 055) — **[NV]** quanto ao estado remoto ao vivo. `main` remoto em `82ee437f…` segundo ref de tracking local; estado vivo **[NV]**.
- Gate determinístico autoritativo: `python3 scripts/ci/check_gate_integrity.py` → **PASS**, 4 required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`), **8 pisos / 50 total**. ADR-0003 rege merge deliberado após CI verde.
- Worktrees/stashes/branches de feature: zero no checkout compartilhado no início deste ciclo (conforme releitura de estado); esta é uma worktree de feature. Ledger append-only em `.specify/queue/concurrency.jsonl` (runtime, gitignored, ausente nesta worktree).

## Contagens verificadas (por `find`, 2026-11-18T02:23Z; já inclui os artefatos do ciclo 056)

| Artefato | Contagem |
|---|---:|
| `specs/` | 47 diretórios / 202 arquivos |
| `.specify/assessments/` | 44 diretórios / 175 arquivos |
| `.specify/validations/` | 42 diretórios / 43 arquivos |
| `.specify/releases/` | 42 diretórios / 42 arquivos |
| `.specify/bugs/` | 4 diretórios / 12 arquivos |
| `docs/research/` | 135 arquivos (índice em `docs/research/index.md`) |
| `docs/adr/` | 6 arquivos |

`.specify/feature.json` apontava `specs/051-dead-invite-ui` (ciclo fechado) na última releitura do checkout compartilhado; o arquivo é machine-local/gitignored e **não existe nesta worktree** — **[NV]** para o valor atual. Identidade de ciclo é resolvida por inventário/`--dry-run`, nunca pelo nome do branch.

## O que funciona (evidência local)

- API Axum em `flora-api` com Postgres 18; rotas montadas e autenticação por sessão no Postgres (`jti` verificado), OIDC com JWKS, FTS do Postgres autoritativa.
- Suíte Rust: fmt/clippy/deny/build/test verdes (105 passed / 0 failed / 51 ignored no último run completo).
- Frontend Svelte 5: `bun run check` 0/0, lint ok, 42/42 testes, build ok.
- `db-tests` com 8 legs cobrindo 50 contratos DB, fail-closed por env e com piso de auth 13.

## Dívidas registradas

- ADR-0003 (`:19`) e ADR-0006 (`:10,52`) ainda citam "43" (seis legs) — correção depende de aprovação de maintainer: **[NV]**, escalada.
- Fase: `001–009` sem validate/release; `025` sem assessment; `035–038` sem artefatos SDD; `plan.md` ausente em `025–042`; `analyze.md`/`converge.md` ausentes antes de `047`.
- `index.md` na raiz ausente (fora do escopo de `docs/`).
- Falso positivo local: `cargo test --ignored` sem env reporta passes (CI já fail-closed; correção local exigiria 9 helpers/50 chamadores).

## NV

PRs/CI/proteção/merge/releases ao vivo, execução de runners e intenção futura do dono sobre RBAC/RLS/convites: **[NV]**. Nenhuma afirmação remota é feita sem verificação ao vivo neste documento.

## Contagens verificadas (por `find`, 2026-11-18T03:51Z; âncora `262ac042f`)

Atualização append-only após os ciclos 057–059. A nota de 02:23Z permanece
integralmente como registro histórico.

| Artefato | Contagem |
|---|---:|
| `specs/` | 50 diretórios / 224 arquivos |
| `.specify/assessments/` | 47 diretórios / 192 arquivos |
| `.specify/validations/` | 46 diretórios / 47 arquivos |
| `.specify/releases/` | 46 diretórios / 46 arquivos |
| `.specify/bugs/` | 4 diretórios / 12 arquivos |
| `docs/research/` (sem `index.md`) | 135 arquivos |
| `docs/adr/` | 6 arquivos |

Gate autoritativo: `check_gate_integrity.py` PASS — 4 required checks, 8 pisos / 50
total (derivado de `FLOORS`). PR/CI/proteção/merge ao vivo: **[NV]**. ADR-0003/0006
(texto 43) e N3 (FK composto) permanecem escalados **[NV]**, sem alteração.

## Correção de contagens — 2026-11-18T03:52Z (append-only)

A tabela de 03:51Z foi calculada antes de os artefatos deste ciclo entrarem na
worktree. Contagens reais após a inclusão (e após o merge em `dev`):

| Artefato | Contagem |
|---|---:|
| `specs/` | 51 diretórios / 230 arquivos |
| `.specify/assessments/` | 48 diretórios / 197 arquivos |
| `.specify/validations/` | 46 diretórios / 47 arquivos |
| `.specify/releases/` | 46 diretórios / 46 arquivos |
| `.specify/bugs/` | 4 diretórios / 12 arquivos |
| `docs/research/` (sem `index.md`) | 135 arquivos |
| `docs/adr/` | 6 arquivos |

Os números de 03:51Z permanecem como registro do estado anterior à inclusão
dos artefatos do ciclo 060. Estado remoto/CI: **[NV]**.

## Estado verificado — 2026-11-18T15:53:43Z (ciclo 067; append-only)

Contagens obtidas por `find` nesta worktree (já incluem os artefatos do ciclo 067):

| Artefato | Contagem |
|---|---:|
| `specs/` | 58 diretórios |
| `.specify/assessments/` | 55 diretórios |
| `.specify/validations/` | 53 diretórios |
| `.specify/releases/` | 53 diretórios |
| `.specify/bugs/` | 4 diretórios |
| `docs/research/` | 138 arquivos (2 com prefixo `2026-11-18`) |
| `docs/adr/` | 6 arquivos |

Gate autoritativo do alvo: `check_gate_integrity.py` **PASS** — 4 required checks,
8 pisos / 51 total; `--remote` confirma required contexts em `dev` e `main`.
A cifra "50" em notas anteriores estava desatualizada (o piso `workspace` passou
de 2 para 3 no ciclo 064); o histórico permanece intacto.

Remoto observado ao vivo nesta sessão: PR **#285** merged em `9d5f6656f` com
6 checks verdes; nenhum PR aberto em `github`. Não é mais afirmação **[NV]**
para aquele merge específico. Status de CI futuro, Forgejo e produção: **[NV]**.

Entrega deste ciclo: `docs/research/2026-11-18-067-c6-canary-rls-design.md` —
desenho de canário RLS com rollback, sem habilitação em produção (C6, autorizado).
Nenhum código, workflow, migration, ADR ou constituição alterado.

Dívida registrada (não forjada): `docs/research/index.md` cobre até a entrada 046;
as demais entradas do diretório seguem pendentes de indexação retroativa.

## Estado verificado — 2026-11-19T01:58Z (ciclo 068; append-only)

Contagens obtidas por `find` nesta worktree (comandos e saídas em
`.specify/assessments/068-verified-docs-refresh/research.md`):

| Artefato | Contagem |
|---|---:|
| `specs/` | 58 diretórios / 280 arquivos |
| `.specify/assessments/` | 55 diretórios / 232 arquivos |
| `.specify/validations/` | 54 diretórios |
| `.specify/releases/` | 54 diretórios |
| `.specify/bugs/` | 4 diretórios |
| `docs/research/` | 137 arquivos sem `index.md` (138 com ele) |
| `docs/adr/` | 6 arquivos |

Gate autoritativo do alvo: `python3 scripts/ci/check_gate_integrity.py` → **PASS**
(4 required checks, 8 pisos / 51 total); `--remote` confirma os required contexts
em `dev` e `main` (strict, enforce_admins, sem required reviews). ADR-0003 rege
merge deliberado após CI verde observada ao vivo.

Âncora local `dev@cbad031b5` igual a `github/dev@cbad031b5` (0/0 divergência);
nenhum PR aberto no `github`; quatro execuções recentes em `dev` com
`completed/success` observadas via `gh` às 01:55Z. Status de CI futuro, Forgejo,
produção e aprovações: **[NV]**.

Este ciclo corrigiu apenas documentação de estado e um comentário obsoleto em
`flora-api/src/routes/rbac.rs` (handlers reais desde o ciclo 066). Nenhum
workflow, gate, migration, ADR, constituição ou produção foi alterado. A dívida
de indexação retroativa 047+ em `docs/research/index.md` permanece registrada.

### Correção append-only — 2026-11-19T02:10Z (ciclo 068)

A nota das `01:58Z` registrou o snapshot obtido **antes** da materialização dos
artefatos do ciclo 068. Após a criação deles, `find` na worktree final conta:
`specs/` **59 diretórios / 287 arquivos**, `.specify/assessments/` **56 diretórios /
237 arquivos**, `.specify/validations/` **55 diretórios**, `.specify/releases/`
**55 diretórios**, `docs/research/` **137 arquivos** (sem `index.md`). As cifras
anteriores permanecem como registro histórico do momento da medição; nenhuma
linha foi reescrita.

## Estado verificado — 2026-11-19T04:22:55Z (ciclo 072; append-only)

- **Âncora de início:** `dev@cdf50eaff`; gate do alvo `python3 scripts/ci/check_gate_integrity.py` PASS às `04:14:31Z` — `gate-integrity: PASS`, 4 checks, `all 8 db contract floors intact (51 total)`, exit 0.
- **Contagens por `find` neste checkout:** `specs/` 61 diretórios, assessments 58, validations 57, releases 57, bugs 4, `docs/research/` 137 arquivos sem `index.md` (138 com índice). Intervalo 001–070.
- **Ciclo 069:** índice de research verificado 137 arquivos / 137 links / `missing=[]`; dívida 047+ estava fechada por evidência, não reaberta.
- **Ciclo 070:** 3 contratos DB-gated de RBAC aprovados em R4, release `0.2.3` proposta; o CI ainda não os executa (sem leg `rbac`) — exige aprov maintainer, **[NV]**.
- As notas 01:58Z e 02:10Z acima são histórico; nenhuma linha foi reescrita. **[NV]** para CI futuro, Forgejo, produção e aprovações.

### Correção append-only — 2026-11-19T04:23:44Z (ciclo 072)

O snapshot `04:22:55Z` foi medido antes dos artefatos 072. Estado final:
`specs/` 62, assessments 59, validations 58, releases 58, bugs 4, research
137 sem `index.md` (138 com índice). Notas 01:58Z/02:10Z e a nota 04:22:55Z
permanecem intactas; apenas acréscimos.
