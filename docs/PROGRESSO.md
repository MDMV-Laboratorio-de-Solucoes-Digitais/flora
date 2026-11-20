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

## Estado verificado — 2026-11-19T23:12Z (ciclo 083; append-only)

- **Âncora:** base local `dev@33d174095` no snapshot 23:11Z; o cabeçalho histórico `dev@94fcf19ca` (`:3`) permanece intacto. Estado remoto, PRs, CI ao vivo e produção: **[NV]**.
- **Gate determinístico local:** `python3 scripts/ci/check_gate_integrity.py` → **PASS** (4 required checks), evidência de `2026-11-19T23:11Z`. CI futuro: **[NV]**.
- **Contagens por `find` (`2026-11-19T23:11Z`, neste checkout):** `specs/` 72, `.specify/assessments/` 69, `.specify/validations/` 68, `.specify/releases/` 68, `.specify/bugs/` 4, `docs/research/` 138 arquivos `.md` (137 sem `index.md`), `docs/adr/` 6.
- **Escopo:** apenas apêndices datados no EOF de `docs/PROGRESSO.md` e `docs/research/index.md`; nenhum outro arquivo alterado.
- **Histórico:** notas `:3`, `:151`, `:159` e `:159-164` intactas; cifras antigas são do momento da medição.

## Estado verificado — 2026-11-20 (ciclo 085; append-only)

- **Âncora:** branch `feature/085-local-first-seed`, commit local `38184fa50`; gate determinístico `python3 scripts/ci/check_gate_integrity.py` → **PASS** (4 required checks, 8 pisos / 51), medido neste checkout.
- **Contagens por `find` (2026-11-20, neste checkout):** `specs/` **75** diretórios, `.specify/assessments/` **72**, `.specify/validations/` **70**, `.specify/releases/` **70**, `.specify/bugs/` **4**, `docs/research/` **137** arquivos sem `index.md` (**138** com ele).
- **Ciclo 085 concluído com evidência local:** clippy, contratos e vitest validados; E2E real `POST /api/v1/bootstrap` → `201`, segunda chamada → `409`, `GET /api/v1/tasks` com token → `200`. Contrato da rota pública: `specs/085-local-first-seed/contracts/bootstrap.md`; nota de arquitetura: `docs/architecture/2026-11-20-085-sem-html-stale-bootstrap.md`.
- **Exceção FR-012:** o bootstrap não verifica posse do e-mail; aplica-se somente a instalação vazia (zero organizações não-sistema). Registrada em `docs/governance/2026-11-20-085-fr-012-bootstrap-sem-verificacao-email.md`, na spec e no contrato.
- **Histórico:** cifras anteriores (59/61/62/69/72 etc.) e todas as notas 2026-11-18/19 permanecem intactas — são registros do momento da medição; nenhuma linha foi reescrita. Checklist `security.md` segue com 12 itens reviewer-owned pendentes, sem marcação `[x]`.
- **[NV]** CI futuro, Forgejo, produção e aprovações ao vivo: reconsultar antes de qualquer merge.

### Revalidação determinística — 2026-11-20T09:34Z

- Gate autoritativo PASS; `gh pr checks 309` mostra 11 `pass` + 1 `skipping`, sem pendências; `reviewDecision` vazio e zero reviews. Proteção de `dev`: 4 required checks, `enforce_admins=true`, nenhum review obrigatório.
- Contagens recontadas: specs 75, assessments 72, validations **71**, releases **71**, bugs 4, research 137 sem `index.md` (138 com ele). Cifras 70/70 anteriores preservadas como registro do snapshot 03:27Z.
- Não existe documento literal “GDD” no repo; o checklist de qualidade `specs/085-local-first-seed/checklists/security.md` permanece reviewer-owned (12 itens `[ ]`), conforme semaphore de implement. Nenhum marcador foi alterado.
- Scripts SDD da factory em `ee71d8db` com `--root .`: os 4 continuam `FAIL` por dívida legada em 75 specs (baseline da hub, não aplicável ao alvo). Nenhum registro novo do ciclo 085; gate do alvo PASS. **[NV]** produção/Forgejo/aprovação de maintainer.

### Decisão do SE — 2026-11-20 (append-only)

- SE substituiu a regra de merge com aprovador distinto/revisão deliberada por: CI/CD GREEN basta para merge autônomo em `dev/`; se não inspirar confiança, o 80/20 imediato é reforçar o CI/CD até inspirar. Emenda em `docs/adr/0003-autonomous-merge-under-deterministic-gates.md` (Status `Accepted`), prevalecendo sobre cláusulas anteriores de aprovação.
- Âncora: `feature/085-local-first-seed` @ `2a9b7e76b`; PR #309. As notas "sem merge" das `09:34Z` permanecem como histórico, não autoritativas. Nenhum código/gate/workflow tocado.

### Merge autônomo #309 — 2026-11-20T09:49:31Z

- PR #309 mesclado em `dev/` sob a regra CI/CD GREEN; merge commit `f350b2774`. Gate `check_gate_integrity.py` PASS no HEAD mesclado; 4 required checks veridos ao vivo antes do merge; `dev` local alinhado. Sem auto-merge/force-push/deleção de branch. **[NV]** produção/Forgejo/aprovações fora do gate.

### Ciclo 086 — leg RBAC no CI — 2026-11-20T10:30Z

- Reforço de CI exigido pela nova regra: 3 contratos RBAC antes invisíveis no CI passaram a rodar no job `db-tests` (`run_leg rbac contract::rbac::db::db_ 3`).
- `check_gate_integrity.py` agora verifica 9 legs / 54 pisos; gate PASS; leg local 3/3; workspace tests verdes. R4 aprovado; R7 `0.4.0→0.4.1`. **[NV]** CI do PR, merge, produção/Forgejo.

## Estado verificado — 2026-11-20T10:50Z (ciclo 087; append-only)

- **Âncora:** `.specify/feature.json` → `specs/087-bootstrap-ci-leg`; branch `feature/087-bootstrap-ci-leg` @ `367a50e79`. Estado remoto/CI ao vivo: **[NV]**; cabeçalho histórico `:3` e todas as notas anteriores intactas.
- **P0 bootstrap fora do gate:** 4 contratos `#[ignore]` em `flora-tests/src/contract/bootstrap.rs` nunca rodavam no CI. Agora cobertos por `run_leg bootstrap contract::bootstrap::tests::db_ 4` em `rust.yml` e `"bootstrap": 4` em `check_gate_integrity.py` — 2 arquivos / 2 linhas alteradas (`+1`/arquivo, `0−`), `git diff --check` limpo.
- **Filtro corrigido após regex mismatch:** `...::db_bootstrap_` casava 0 testes e era rejeitado pela regex do guarda (`:118`); trocado por `contract::bootstrap::tests::db_` (4 testes, regex existente preservada).
- **Evidência:** gate `all 10 db contract floors intact (58 total)` exit 0; `--list` → 4; leg `--ignored --test-threads=1` → 4 passed/4. R4 aprovado; R7 registrado. Nenhum validation/release tocado.
- **Contagens (snapshot 2026-11-20T10:50Z):** `specs/` 76, `.specify/assessments/` 73, `.specify/validations/` 72, `.specify/releases/` 72, `.specify/bugs/` 4, `docs/research/` 137 sem `index.md` (138 com ele). Apêndice append-only; cifras antigas permanecem como registro histórico do momento da medição.
- **[NV]** CI do PR, merge, produção/Forgejo e aprovações ao vivo: reconsultar antes de qualquer decisão de merge.

## Estado verificado — 2026-11-20T11:06Z (ciclo 088; append-only)

- **Âncora:** `.specify/feature.json` → `specs/088-dynamic-gate-oracle`; branch `feature/088-dynamic-gate-oracle` @ `cbc885383`. Estado remoto/CI/PR ao vivo: **[NV]**; cabeçalho histórico `:1` e todas as notas anteriores intactas.
- **Causa raiz recorrente:** o guarda `scripts/ci/check_gate_integrity.py` era um espelho entre `FLOORS` e as legs `run_leg` do `rust.yml`: duas listas escritas à mão, sem visão da workspace de testes. Os `#[ignore]` esquecidos em 086 (RBAC) e 087 (bootstrap) passaram batido por esse motivo.
- **Oráculo dinâmico:** `LEG_RE` extrai as 10 legs `run_leg <label> <filter> <floor>`; `cargo test -p flora-tests -- --ignored --list` descobre os testes (stdout); contagem por `startswith(filter_)`, exatamente uma leg por label e piso igual à contagem. Divergência (rc != 0, lista vazia, label desconhecido, piso/contagem diferentes, leg duplicada) falha fechado.
- **Evidência local:** gate `all 10 db contract floors intact (58 total)` → exit 0; `python3 -m py_compile` → exit 0; diff 1 arquivo (`scripts/ci/check_gate_integrity.py`, `+59/−5`). R4/R7 em materialização no snapshot 11:06Z; nenhum validation/release existente tocado.
- **Contagens (snapshot 11:06Z):** `specs/` 77, `.specify/assessments/` 74, `.specify/validations/` 73, `.specify/releases/` 73, `.specify/bugs/` 4, `docs/research/` 137 sem `index.md` (138 com ele). `find` no checkout confirma 78/75 (incluindo spec/assessment 088 em curso), 73/73/4 e research 138 = 137 + `index.md`. Apêndice append-only; cifras antigas permanecem como registro do momento da medição.
- **[NV]** CI do PR, merge, produção/Forgejo e aprovações ao vivo: reconsultar antes de qualquer decisão de merge.

## Estado verificado — 2026-11-20 (ciclo 089; append-only)

- **Âncora:** branch `feature/089-forgejo-mirror-sync` @ `15203ee78`; `.specify/feature.json` → `specs/089-forgejo-mirror-sync`. Cabeçalho `:1` e todas as notas 2026-11-18/19/20 intactas. Registro canônico: `docs/governance/2026-11-20-089-decisoes-governanca.md` (novo). Estado remoto/Forgejo/CI ao vivo: **[NV]**.
- **Gate autoritativo (decisão explícita do SE):** os 4 scripts SDD da factory (`check_sdd_completeness.py`, `check_converge_append_only.py`, `check_task_id_uniqueness.py`, `check_work_order_dag.py`) **não** são gate do alvo — falham por dívida legada de baseline da hub (75 specs), não são regressão. Gate é `python3 scripts/ci/check_gate_integrity.py` (oráculo dinâmico 088, 10 legs / 58 pisos). Nenhum script SDD promovido a required check.
- **Branch protection (escopo mínimo):** PAT fine-grained `Administration: read` restrito ao repo `flora`; `GITHUB_TOKEN` não serve (sem escopo Admin); secret sugerido `FLORA_BRANCH_PROTECTION_TOKEN`. Nenhuma credencial copiada/exposta. **[NV]** provisionamento e leitura ao vivo.
- **Advisories:** bump `rustls` `0.23.43→0.23.46` resolve 4 advisories canônicos (`Cargo.lock` em `0.23.43`). Recomendação registrada; **não promover gate ainda**. **[NV]** aplicação do bump.
- **Deploy Flora→Coolify (ligação MDMV-Autonoma):** app `Flora-Staging`, deployment `44djh8p8owth5buhmcvgpilb`, commit `15203ee78`, health `200`, rollback image `b1dafae3…`. Fora do gate de merge (ADR-0003). **[NV]** relatório via mensageria cross-session — reconsultar estado real antes de promoção/rollback.
- **Ciclo 089 — espelho Forgejo:** origem `https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git`; `origin/dev` = `f12a328c` é ancestral de `dev` (fast-forward); job de espelho **não** é required check (ADR-0003).
- **Escopo:** apenas apêndices datados em `STATE.md`/`PROGRESSO.md` e o novo documento de governança; nenhum validation/release/código/workflow/gate/migration/ADR/constituição alterado; nenhuma linha reescrita. **[NV]** remoto Forgejo, job de espelho e proteção ao vivo.

### Execução verificada do espelho — 2026-11-20T13:02Z

- Workflow `.github/workflows/mirror-to-forgejo.yml` criado e parseado com sucesso; evento `push` em `dev`/`main`/tags `v*`, `permissions: {}`, segredos via `env`, guarda de ancestralidade e push sem force.
- Espelho sincronizado agora: `f12a328c4..15203ee78 dev -> dev` (exit 0, fast-forward). **[NV]** secrets e próximas execuções do job.

### Ciclo 090 — bump rustls 0.23.46 — 2026-11-20T15:05Z

- Atualização cirúrgica de `Cargo.lock` (`rustls 0.23.43→0.23.46`) e da exemption cargo-vet correspondente; manifest inalterado. Fecha 4 advisories canônicos (2 medium, 2 low), sem critical/high.
- Gates locais verdes: gate 10 legs/58, `cargo test --workspace`, `cargo deny check bans licenses sources`, `cargo vet --locked` e `git diff --check`. R4 aprovado; R7 `0.4.4→0.4.5`. **[NV]** CI do PR/merge/produção.

## Estado verificado — 2026-11-20 (ciclo 091; append-only)

- **Âncora:** `.specify/feature.json` → `specs/091-governance-ci-audit`; branch `feature/091-governance-ci-audit` @ `673c687ea`. Cabeçalho `:1`, notas 2026-11-18/19/20 e o registro 089 intactos; apenas acréscimo no EOF.
- **Auditoria viva PASS em dev/main:** `python3 scripts/ci/check_gate_integrity.py --remote` → **PASS** (`dev: live required contexts match`; `main: live required contexts match`). 4 required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`), *strict*, `enforce_admins=true`, sem required reviews; gate local PASS (oráculo dinâmico 088, 10 legs / 58 pisos). Zero required checks novos. Leitura via `gh` com PAT `Administration: read` restrito a `flora`; secret `FLORA_BRANCH_PROTECTION_TOKEN` registrado, sem credenciais expostas. **[NV]** reconsulta futura.
- **Workflow de auditoria não required:** `.github/workflows/branch-protection-audit.yml` — `schedule` (`23 4 * * *`) + `workflow_dispatch`, `permissions: {}`, job `branch-protection-audit` **advisory (não required)** com `GH_TOKEN` = `FLORA_BRANCH_PROTECTION_TOKEN`, executando `check_gate_integrity.py --remote`. YAML válido; fora do conjunto de gates obrigatórios (ADR-0003).
- **Constituição v1.4.0 aprovada SE:** emenda `1.3.0 → 1.4.0` (2026-11-20) com migration plan documentation-only — Svelte 5 canônico, sem SQLite, ADR-0001/0002 para mecanismos diferidos, ADR-0003 para merges; rollback = v1.3.0. Nenhuma alteração de código/config/workflow/gate/migration/secret.
- **Deploy de produção [NV]:** só a app `Flora-Staging` existe no Coolify; faltam app de produção, FQDN e revisão. Nenhum deploy de produção reivindicado; fora do gate de merge (ADR-0003). Reconsultar ao vivo antes de promoção/rollback.
- **Advisories:** `apps/frontend/package-lock.json` → **25 advisories (7 high)**. Recomendação: **não promover** gate de advisories; **triar/limpar** os findings. Nenhum check obrigatório novo; **[NV]** triagem e recontagem.
- **Decisão de relatórios:** conforme `docs/governance/2026-11-20-091-relatorios-sem-scripts-sdd.md`, os scripts SDD da factory **não** constam destes relatórios — menção apenas em documento de decisão/governança. Única autoridade de gate: `check_gate_integrity.py` (10 legs / 58 pisos).
- **Histórico/escopo:** cifras e notas anteriores preservadas; nenhum validation/release, workflow, gate, ADR, constituição, código, PR, merge, tag ou produção alterado; nenhuma credencial copiada. **[NV]** CI futuro, proteção ao vivo, deploy, Forgejo e aprovações.

## Estado verificado — 2026-11-20 (ciclo 092; append-only)

- **Âncora:** `.specify/feature.json` → `specs/092-npm-lock-sync`; branch `feature/092-npm-lock-sync` @ `bf00caf5c`. Cabeçalho `:1` e todas as notas 2026-11-18/19/20 intactas; apenas acréscimo no EOF. Registro canônico: `docs/governance/2026-11-20-092-package-lock-sync.md` (novo).
- **Sincronização do lockfile derivado:** `apps/frontend/bun.lock` é canônico para CI; `apps/frontend/package-lock.json` foi regenerado com `npm install --package-lock-only --ignore-scripts` em `apps/frontend`, sem tocar o manifesto. Evidência local: `git diff -- apps/frontend/package-lock.json` → `+116/−0`, com `shadcn-svelte 1.8.1` acrescentado à root `devDependencies` e entradas transitivas incluídas; `git diff -- apps/frontend/package.json` vazio.
- **Advisories:** baseline 091 de 25 alerts do Dependabot (7 high) reenumerada após a regeneração; `npm audit --package-lock-only --json` ao vivo → **21 advisories (5 high, 15 moderate, 1 low, 0 critical)**. Fontes distintas, sem comparação estrita. Decisão 089/091 reafirmada de **não promover** advisories a required check; nenhum gate alterado.
- **Promoção dev→main / produção:** não executadas e fora do gate de merge (ADR-0003). **[NV]** estado ao vivo de PR, CI e Coolify; sem app/FQDN/revisão de produção confirmados (somente `Flora-Staging` em 091). Não promover sem verificação ao vivo.
- **Escopo:** apêndices datados em `STATE.md`/`PROGRESSO.md` e nova nota de governança; nenhum validation/release, workflow, gate, ADR, constituição, código, PR, merge, tag ou produção alterado; nenhuma credencial exposta. **[NV]** CI, proteção, deploy, Forgejo e aprovações ao vivo.

## Estado verificado — 2026-11-20 (ciclo 093; append-only)

- **Âncora:** branch `feature/093-sveltekit-security-upgrade` @ `b426e4f81`; `.specify/feature.json` → `specs/093-sveltekit-security-upgrade`. Cabeçalho `:1` e todas as notas 2026-11-18/19/20 intactas; apenas acréscimo no EOF.
- **PR #318 (dev→main, produção) bloqueado:** lido ao vivo via `gh` às 19:56Z — `OPEN`/`BLOCKED`; `dependency-review` `fail` no run `44806804503` (`fail-on-severity: moderate`) por 4 advisories em `@sveltejs/kit@2.70.3`: `GHSA-3r8g-jcqq-3p5j` (high) e `GHSA-mw6w-2fmv-pvw3`, `GHSA-rqp5-r3wg-vg35`, `GHSA-6cf5-gc4q-fm3x` (moderate). Detalhes e demais checks em `docs/governance/2026-11-20-093-sveltekit-upgrade.md`.
- **Upgrade 093:** `kit 3.1.1` / `adapter-node 6.0.5` / `svelte 5.57.1` (pares upstream: `vite ^8.0.12`, `vite-plugin-svelte ^7`, `typescript ^6`; adapter exige `kit ^3`). Manifesto local já reflete essas versões no snapshot deste ciclo; lockfiles em sincronização. **[NV]** resultado do `dependency-review` após o bump.
- **Produção:** app e DNS prontos; deploy condicionado à promoção de `main` (PR #318), fora do gate de merge (ADR-0003). Nenhum deploy/promoção executado; **[NV]** estado ao vivo de Coolify, FQDN e revisão.
- **Advisories:** decisão 089/091/092 mantida — **sem promoção** de checagem de advisories a required check. Gate segue `python3 scripts/ci/check_gate_integrity.py` com os 4 required checks da ADR-0003; `dependency-review` permanece required e a correção é via versões corrigidas, não por flexibilização. Scripts SDD fora de gate e de relatórios (089/091).
- **Escopo:** apêndice datado; nenhum validation/release, workflow, gate, ADR, constituição, código, PR, merge, tag ou produção alterado por esta nota; nenhuma credencial exposta. **[NV]** CI, proteção, deploy, Forgejo e aprovações ao vivo.

### Promoção e deploy — 2026-11-20T20:38Z

- PR #318 mesclado; `main=761539b9f`. 4 required checks verdes no head promovido; espelho Forgejo de `main` sincronizado. App produção `9q3wuuwkur2kl27py0si785n`, FQDN `https://flora.mdmv.qzz.io`, DNS ativo. Deploy em curso; health e commit efetivo **[NV]** até confirmação.

## Estado verificado — 2026-11-20 (ciclo 094; append-only)

- **Âncora:** branch `feature/094-visao-so-pme` @ `fd25d8f8b`; `.specify/feature.json` → `specs/094-visao-so-pme`. Cabeçalho `:1` e todas as notas 2026-11-18/19/20 intactas; apenas acréscimo no EOF. Registro canônico: `docs/governance/2026-11-20-094-visao-so-pme-integracoes.md` (novo).
- **Produção observada:** app `9q3wuuwkur2kl27py0si785n`, FQDN `https://flora.mdmv.qzz.io`, `/health` **200**, deploy **finished 21:23:59Z** no commit `cee7b0cda`. A nota de 20:38Z registrou "deploy em curso, health [NV] até confirmação" no momento da medição; permanece como histórico. Fora do gate de merge (ADR-0003).
- **Forgejo:** refs do espelho sincronizados; job `mirror` informativo, não required (ADR-0003).
- **Advisories de runtime:** **0**. Decisão 089/091/092/093 reafirmada: sem promoção a required check. Gate inalterado: `python3 scripts/ci/check_gate_integrity.py` (oráculo dinâmico 088), 4 required checks da ADR-0003.
- **Diretriz SO PME:** nota de governança NÃO-normativa, sem emenda constitucional, gate, workflow, migration ou código. Mapa: Coolify/Forgejo implementados e verificados; Grafana+VictoriaMetrics/Uptime Kuma deferidos; agents Autonoma/dark-factory como ligação documental; marketing/monetization/seller ausentes no alvo **[NV]**.
- **Escopo:** apêndice datado; nenhum validation/release, workflow, gate, ADR, constituição, código, PR, merge, tag ou deploy alterado por esta nota; nenhuma credencial exposta. **[NV]** CI futuro, proteção, deploy, Forgejo e aprovações ao vivo: reconsultar antes de agir.

## Estado verificado — 2026-11-20 (ciclo 095; append-only)

- **Âncora:** branch `feature/095-frontend-deploy` @ `29b34aefe`; `.specify/feature.json` → `specs/095-frontend-deploy`. Cabeçalho `:1` e todas as notas 2026-11-18/19/20 intactas; apenas acréscimo no EOF. Registro canônico: `docs/governance/2026-11-20-095-frontend-deploy.md` (novo).
- **404 na raiz:** `https://flora.mdmv.qzz.io/` → **404** (`/tasks` e `/login` também), enquanto localmente `/` e `/tasks` → **200**. Causa raiz: FQDN servia a API Rust, não o frontend. Evidências: `/health` **200**, `/metrics` **200** (`flora_api_up 1`), `/llms.txt` **200** (`# Flora Workspace (flora-api)`); `Dockerfile:5,9,11` só builda/copia/executa `flora-api`; `.github/workflows/docker-staging.yml:34` `context: .` sem `file:`; `apps/frontend/Dockerfile` não usado no deploy.
- **Manifesto do frontend:** `apps/frontend/build/server/manifest.js:33-34` `pattern: /^\/$/` e `:89-90` `pattern: /^\/tasks\/?$/` — rotas presentes no build; `/(auth)` é grupo interno, não URL. A hipótese de rota ausente foi descartada: o defeito é de publicação/roteamento.
- **Decisão:** imagem `flora-web` a partir de `apps/frontend/Dockerfile`, publicada por workflow manual; `flora.mdmv.qzz.io` para o frontend; API em `flora-api.mdmv.qzz.io`; `FLORA_API_BASE_URL` gera `config.js` no entrypoint; CORS da API (`FLORA_APP__CORS_ORIGINS`) deve incluir a origem do frontend. Deploy fora do gate de merge (ADR-0003); nada disso executado por esta nota.
- **[NV] até deploy final:** imagem, app/DNS da API, CORS em produção e medição pós-deploy. Próximos passos de validação: gate `check_gate_integrity.py` + `git diff --check`; build local do container e `GET /`, `/tasks`, `/config.js`; publicar via workflow manual; apontar FQDN para `flora-web`; validar API e CORS no FQDN novo; medir `/`, `/tasks`, `/health` e fluxo no browser; registrar nova nota append-only. Rollback = reapontar FQDN para a imagem anterior, sem force-push.
- **Escopo:** apêndice datado; nenhum validation/release, workflow, gate, ADR, constituição, código, PR, merge, tag ou deploy alterado por esta nota; nenhuma credencial exposta. **[NV]** CI futuro, proteção, Coolify, Forgejo e aprovações ao vivo: reconsultar antes de agir.
