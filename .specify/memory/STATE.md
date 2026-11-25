# STATE — sessão 2026-11-17 (conflito add/add resolvido no merge PR #262)

> Ambos os registros foram preservados integralmente, em ordem cronológica.

## Registro do ciclo 047 (lado HEAD do PR #262)

# STATE — sessão 2026-11-17

**Atualizado:** 2026-11-17T13:54Z · **Alvo:** `/home/luis/development/MDMV/projetos/Arquivados/flora`

## Âncora e ciclo

- PR branch: `feature/047-release-manifest-lockstep-pr` @ `68f783c05` (3 commits cherry-picked de `github/dev@4ae58ccd2`).
- Branch original do ciclo: `feature/047-release-manifest-lockstep` @ `c68aeaf89` (mantida; não deletada).
- Ciclo 047 fechado: `go (scoped)` → spec/clarify/plan/checklist/tasks/analyze → implement → converge → validate (R4 aprovado) → release (R7).
- Decisão de governança BLOCO 0.a: **A (manter alvo)** — CI GitHub Actions, CLI `gh`, gate `scripts/ci/check_gate_integrity.py`. Factory `aa7600d8c809790dfc10861be540c98dffd2f09b` lida como fonte; nada materializado.
- Identidade 047 por inventário de artefatos (specs até 042; validations/releases até 046); `feature.json` gitignored pelo próprio `.specify/.gitignore`.

## Gates (todos executados nesta data)

- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check bans licenses sources`, `cargo build --workspace`, `cargo test --workspace` (101 passed / 0 failed / 50 ignored): **exit 0**.
- `python3 scripts/ci/check_gate_integrity.py`: **PASS** (4 required checks, pisos 43).
- Frontend: `bun run check` (0/0), `bun run lint` (ok), `bun run test:unit -- --run --project server` (40/40), `bun run build`: **exit 0**.
- Asserção do workflow: `RED_EXIT=1` → `GREEN_EXIT=0` → `WORKFLOW_STEP_EXIT=0`.
- SDD nativo (substituição A): completeness PASS (38 specs), task-id uniqueness por arquivo PASS (440 tarefas), DAG PASS (0 arestas), converge append-only PASS (1 artefato).

## Comandos ausentes no alvo (não importados por decisão A)

`check_production_gate.py`, `hub_docs_gate.py`, `check_sdd_completeness.py`, `check_converge_append_only.py`, `check_task_id_uniqueness.py`, `check_work_order_dag.py` — todos exit 2 (arquivo inexistente). `python3 -m pytest -q` → exit 5 (nenhum teste Python no checkout). Substituídos pelos gates nativos acima.

## Remoto (verificado ao vivo via gh, 2026-11-17T13:53Z)

- PR **#262** — https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/262 — `OPEN`, `mergeStateStatus=BLOCKED`, 18 arquivos, +476/−2, 4 commits (o último registra este STATE.md).
- Checks: `pending` (db-tests, dependency-review, e2e-browser, frontend-build, rust-build, supply-chain-vet). Nenhum check obrigatório verde observado; sem aprovação, sem merge, sem auto-merge.

## Dívidas / próximo 80/20

1. Dívida de fase: validations/releases `043–046` sem `specs/` correspondentes; `plan.md` ausente em `025–042`; nenhum `analyze.md`/`converge.md` antes de 047.
2. Docs: prosa em `OBJETIVO.md`/`ROADMAP.md` defasada (001-033 vs 042); sem `docs/PROGRESSO.md`, `docs/research/index.md` (135 arquivos de research sem índice).
3. 30/33 releases sem SemVer e sem rollback explícito; 5/33 validations sem veredito textual.
4. `AGENTS.md` cita 7 crates e toolchain 1.98.0; código tem 5 membros + raiz e 1.98.1.
5. Defeitos em fila (ranqueamento do BLOCO 5): C3 `/api/v1/telemetry/logs` 404, C4 `/users/invite` 404, C5 RBAC stub + contrato vácuo, C6 RLS inerte (exige autorização SE).
6. `docs/architecture/` inexistente; nenhum HTML versionado para refresh. Arquivos `.archify/` são locais/gitignored.

## NV

Estado final dos checks remotos, branch protection, aprovadores, merges e releases: **[NV]** — reconsultar ao vivo antes de qualquer decisão de merge. Nenhuma alteração feita na factory.

---

## Registro do ciclo 048 (lado github/dev, PR #263)

# STATE — sessão 2026-11-17 (ciclo 048)

**Atualizado:** 2026-11-17T14:38Z · **Alvo:** `/home/luis/development/MDMV/projetos/Arquivados/flora`

## Âncora e ciclo

- Base: `github/dev@4ae58ccd2`; branch `feature/048-governance-drift`; NNN 048 resolvido por inventário (script auto-detectou 043, mas 043 já tem validation/release → `--number 048`).
- Decisão A preservada: CI GitHub Actions, CLI `gh`, gate `scripts/ci/check_gate_integrity.py`. Factory `214f6ce03be3c9882e03204b9a26dc7be2d53a7b` lida como fonte; nada materializado; 7 worktrees da factory ignorados.
- Ciclo 048 fechado: assess `go (scoped)` → spec/clarify/plan/checklist/tasks/analyze → implement → converge → validate R4 → release R7.
- PR #262 (ciclo 047) permanece em branch distinto; não foi tocado.

## Gates (executados)

- fmt/clippy/deny/build/test: exit 0; Rust 111 passed/0 failed/44 ignored.
- `check_gate_integrity.py`: PASS (4 required checks, pisos 43).
- Frontend: check 0/0, lint ok, 39/39 testes, build ok.
- SDD nativo: completeness PASS (38 specs), task-ID por arquivo PASS (440), DAG PASS, converge PASS (0 — vacuo nesta base).

## Ausentes no alvo (decisão A — não importados)

`check_production_gate.py`, `hub_docs_gate.py`, `check_sdd_completeness.py`, `check_converge_append_only.py`, `check_task_id_uniqueness.py`, `check_work_order_dag.py` → exit 2; `pytest -q` → exit 5. Substituídos pelos gates nativos acima.

## Dívidas / próximo 80/20

1. C3 `/api/v1/telemetry/logs` 404 (frontend `logger.ts:40,106`; sem rota) — próximo candidato (score 10.0).
2. C4 `/users/invite` 404 (`UserManager.svelte:18`) — 3.75.
3. C5 RBAC stub + contrato vácuo (decisão de dono pendente) — 2.08.
4. C6 RLS inerte — exige autorização SE.
5. Base `github/dev` não contém ciclos 044–046 do `dev` local; contagens divergem por isso.
6. `plan.md` ausente em 025–042; validations/releases 043–046 sem spec no `dev` local; `docs/PROGRESSO.md`, `docs/research/index.md` ausentes.

## SE decision request (registrado, não silenteado)

Constituição §VI exige "exatamente um arquivo por commit"; a prática observada no repo (inclusive commits anteriores) agrupa artefatos de um ciclo. Registrado como divergência de governança; não reinterpretado silenciosamente.

## NV

CI/PR/branch protection/aprovações/merges ao vivo: **[NV]** — reconsultar antes de qualquer decisão de merge. Nenhuma alteração na factory.

## Atualização — ciclo 049 (2026-11-17T15:39Z)

- Âncora: `github/dev@55363b3ca` (PR #263 **MERGED** por outra sessão); branch `feature/049-local-logger`; NNN 049 por inventário, nunca pelo nome do branch.
- Decisão A preservada. Nova regra de CLI condicional (`fj` p/ Forgejo, `gh` p/ GitHub) coincide com A: PRs neste repo vão para github.com via `gh`.
- Ciclo 049 fechado: C3 resolvido — `logger.ts` local-only (buffer 200 + `snapshot()`), contrato 002 reconciliado, teste `logger.spec.ts`. `analyze` encontrou e corrigiu conflito de SC na fonte; `lint` ficou RED e foi corrigido (`at(-1)`, `index`).
- Gates: Rust 111 passed/0 failed/44 ignored, frontend 41/41, `check_gate_integrity.py` PASS; todos exit 0.
- **Remoto verificado ao vivo:** PR #263 MERGED; PR #262 `DIRTY` (conflito) — não tocado. Demais estados **[NV]**.
- Próximo 80/20: C4 `/users/invite` 404 (3.75), C5 RBAC stub (2.08, decisão de dono), C6 RLS (0.80, autorização SE).
- Factory relida em `214f6ce03be3c9882e03204b9a26dc7be2d53a7b` (7 worktrees em voo); nada materializado. Sem injeção detectada; nenhuma credencial copiada.

## Atualização — ciclo 050 (2026-11-17T16:20Z)

- Âncora `github/dev@b3bfda505`; branch `feature/050-rate-limit-paths`; NNN 050 por inventário (feature.json apontava 049), nunca pelo nome do branch.
- Decisão A preservada; factory relida em `49f96e4a7bf36bd8c2ee2e81139d93851dfabcba` (7 worktrees), nada materializado.
- Ciclo 050 fechado: C25 resolvido — `classify_endpoint` em `rate_limit.rs` cobre os paths montados sob `/api/v1`; limites/janelas inalterados; 4 testes novos. Corrigi doc órfão (`# Errors`) e formatação durante a implementação.
- Gates: Rust 115 passed/0 failed/44 ignored, frontend 41/41, fmt/clippy/deny/build e `check_gate_integrity.py` PASS.
- Merges anteriores confirmados ao vivo: PR #262, #263, #264 MERGED; nenhum PR aberto no início deste ciclo.
- Próximo 80/20: N3 (4.0), C4 (3.75), C5 (2.08, decisão de dono), C6 (0.80, autorização SE).
- **[NV]** CI/PR/aprovações ao vivo após este ciclo; sem merge/auto-merge/tag/deploy/force-push nesta sessão.

## Atualização — ciclo 051 (2026-11-17T17:35Z)

- Âncora `github/dev@b3bfda505`; branch `feature/051-dead-invite-ui`; NNN 051 por inventário (feature.json apontava 050), nunca pelo nome do branch. Decisão A preservada.
- Factory relida em `ebe0cb86b14c932b3b0ee3ce805264686e906ac2` (7 worktrees); nada materializado.
- Ciclo 051 fechado: C4 resolvido pela via mínima — remoção da UI de convite sem backend; spec 002 marcada (não reescrita). Nenhuma decisão de autorização tomada.
- Gates: Rust 111 passed/0 failed/44 ignored, frontend 41/41, fmt/clippy/deny/build e `check_gate_integrity.py` PASS. `rg 'UserManager|users/invite' apps/frontend/src` → 0.
- PR #265 (ciclo 050) `OPEN`/`CLEAN` nesta base — não tocado. Sem merge/auto-merge/tag/deploy nesta sessão.
- Próximo 80/20: validar tag `osv-scan@v2` (6.0, validade **[NV]**), C5 (2.08, decisão de dono), N3 (1.5), C6 (0.80, autorização SE).
- **[NV]** CI/PR/aprovações ao vivo após este ciclo.

## Atualização — ciclo 052 (2026-11-17T23:44Z)

- Âncora `github/dev@01d822205`; branch `feature/052-osv-scan-ref-pin`; NNN 052 por `create-new-feature.sh --dry-run` (feature.json apontava 051, fechado).
- Decisão A preservada: CI GitHub Actions, CLI `gh`, gate `scripts/ci/check_gate_integrity.py`. Factory `d2bd95823cc5b4758b1c4cf5bd9133b20309ebb7` lida como fonte; scripts SDD executados em `/tmp` com baselines neutros, nada materializado no alvo.
- Ciclo 052 fechado: assess `go (scoped)` → spec/clarify/plan/checklist/tasks → analyze (conflito de SC IDs corrigido na fonte, rerun PASS) → implement → converge → validate R4 → release R7.
- Implementação: `.github/workflows/osv-scan.yml:14` `@v2` → `@v2.7.1` (1 linha). `__gitignore` ignora `.specify/queue/` (runtime).
- Gates: `check_gate_integrity.py` PASS (local + `--remote`); fmt/clippy/deny/build exit 0; test 105 passed/0 failed/51 ignored; frontend check 0/0, lint ok, 42/42, build ok.
- PR → `dev/`; merge deliberado somente após CI verde observada ao vivo (ADR-0003). Sem auto-merge/force-push/tag/deploy.
- **[NV]** CI/PR/aprovações ao vivo após este ciclo e execução real do job OSV.

## Atualização — ciclo 053 (2026-11-18T00:10Z)

- Âncora `github/dev@22f16d6ac`; branch `feature/053-db-gate-uncovered-legs`; NNN 053 por `create-new-feature.sh --dry-run` (feature.json apontava 051, fechado).
- Decisão A preservada: CI GitHub Actions, CLI `gh`, gate `scripts/ci/check_gate_integrity.py`. Factory `d2bd95823cc5b4758b1c4cf5bd9133b20309ebb7` lida como fonte; nada materializado.
- Ciclo 053 fechado: assess `go (scoped)` → spec/clarify/plan/checklist/tasks → analyze PASS → implement → converge → validate R4 → release R7.
- Implementação: `rust.yml` ganha legs `tasks`/`workspace` (`db::db_`, pisos 4/2); guardião valida 8 pisos / 49 total. Nenhum check renomeado.
- Gates: guardião PASS; `--list` 4/2; 8 legs com DB real isolado (`flora_053_verify`) → 50 testes; fmt/clippy/deny/build exit 0; test 105 passed/0 failed/51 ignored; frontend 42/42.
- PR → `dev/`; merge deliberado somente após CI verde observada ao vivo (ADR-0003). Sem auto-merge/force-push/tag/deploy.
- **[NV]** CI/PR/aprovações ao vivo após este ciclo e legs no runner do GitHub.

## Atualização — ciclo 054 (2026-11-18T01:13Z)

- Âncora `github/dev@dd982cadc`; branch `feature/054-db-env-fail-closed`; NNN 054 por `create-new-feature.sh --dry-run` (feature.json apontava 051, fechado).
- Decisão A preservada: CI GitHub Actions, CLI `gh`, gate `scripts/ci/check_gate_integrity.py`. Factory `d2bd95823cc5b4758b1c4cf5bd9133b20309ebb7` lida como fonte; nada materializado.
- Ciclo 054 fechado: assess `go (scoped)` → spec/clarify/plan/checklist/tasks → analyze PASS → implement → converge → validate R4 → release R7.
- Implementação: `rust.yml` com `:?` em `run_leg`; guardião exige env no bloco `db-tests`. Nenhum check renomeado; nenhum `.rs` tocado.
- Evidência: sem env → exit 1 antes do cargo; com DB isolado → 50 testes (8 legs); mutação sem env → guardião RED; gates locais exit 0.
- **Recuperação:** restart do servidor corrompeu um commit interrompido (objeto 0 byte) e o índice da worktree; ref restaurada ao último commit válido do reflog (`b9017755`), índice reconstruído com `read-tree HEAD`, objetos 0 byte removidos (sem conteúdo) e artefatos recommitados. Nenhum trabalho alheio alterado.
- PR → `dev/`; merge deliberado somente após CI verde observada ao vivo (ADR-0003). Sem auto-merge/force-push/tag/deploy.
- **[NV]** CI/PR/aprovações ao vivo após este ciclo e execução do step no runner.

## Decisão 80/20 — 2026-11-18T01:32Z (registrada antes do BLOCO 6)

- Critério explícito `(impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador declarado por candidato.
- **Escolhido: 055 — elevar piso `db-tests` de auth de 12 para 13.** `(4×5)/(2×1) = 10.0`. Evidência: `cargo test -p flora-tests 'contract::auth::db_' -- --list` → 13 testes; piso 12 em `rust.yml:107` e `check_gate_integrity.py:43`; 13/13 passaram com Postgres real no ciclo 053. Esforço 2 (duas linhas + texto 49→50), risco 1 (fortalecimento, sem rename de check).
- Alternativas: ADR drift 43→50 `(2×5)/(1×2)=5.0` (documentação de governança, fora de escopo); índice de research/PROGRESSO `(2×5)/(3×1)=3.33`; N3 `(4×3)/(3×4)=1.0`; dívidas de fase `(2×5)/(5×3)=0.67`; C5/C6 não autônomos (dono/SE).
- **NV**: CI/runner ao vivo; verificação do selecionador em andamento antes da implementação.

## Atualização — ciclo 055 (2026-11-18T01:49Z)

- Âncora `github/dev@d5d7ef5d`; branch `feature/055-auth-floor-thirteen`; NNN 055 por `create-new-feature.sh --dry-run` (feature.json apontava 051, fechado).
- Decisão 80/20 registrada neste arquivo antes do BLOCO 6: `(4×5)/(2×1)=10.0` — auth floor 12→13. Verificador isolado confirmou 13 testes e ausência de flake.
- Decisão A preservada: CI GitHub Actions, CLI `gh`, gate `scripts/ci/check_gate_integrity.py`. Factory `d2bd95823cc5b4758b1c4cf5bd9133b20309ebb7` lida como fonte; nada materializado.
- Ciclo 055 fechado: assess `go (scoped)` → spec/clarify/plan/checklist/tasks → analyze PASS → implement → converge → validate R4 → release R7.
- Implementação: `rust.yml` piso auth 13; guardião `FLOORS["auth"]=13` e texto 50 total. Diff 2 arquivos/3 linhas; nenhum teste tocado.
- Gates: guardião PASS 50 total; `--list` 13; leg auth DB real 13 passed; mutação lockstep RED; fmt/clippy/deny/build/test e frontend check/lint/42/42/build exit 0.
- PR → `dev/`; merge deliberado somente após CI verde observada ao vivo (ADR-0003). Sem auto-merge/force-push/tag/deploy.
- **[NV]** CI/PR/aprovações ao vivo e leg no runner do GitHub.

## Decisão 80/20 — 2026-11-18T02:11Z (registrada antes do BLOCO 6)

- Critério `(impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador por candidato.
- **Escolhido: 056 — criar `docs/PROGRESSO.md` e `docs/research/index.md`.** `(2×5)/(3×1) = 3.33` — maior candidato autônomo. Evidência: `find docs/research -type f` → 135 arquivos sem índice; `docs/PROGRESSO.md`, `docs/research/index.md` e `index.md` ausentes (confirmado por `test -f`); contagens de specs/assessments/validations/releases por `find`.
- **Escalado [NV], não autônomo:** ADR-0003 drift (texto 43 vs 8 pisos/50, `(2×5)/(1×2)=5.0`) exige aprovação de maintainer para alterar documento de governança (Constituição §Governance) — não alterado nesta sessão.
- Demais: N3 `(4×3)/(3×4)=1.0`; dívidas de fase `(2×5)/(5×3)=0.67`; C5/C6 dependem de dono/SE.
- HEAD da factory como fonte desta sessão: `046e1f2a8ce96a42a04b7ea3735da0251e7b1cef` (mutou desde `d2bd958`; nada transplantado).

## Atualização — ciclo 056 (2026-11-18T02:29Z)

- Âncora `github/dev@94fcf19ca`; branch `feature/056-docs-research-index`; NNN 056 por `create-new-feature.sh --dry-run` (feature.json apontava 051, fechado).
- Decisão 80/20 registrada neste arquivo antes do BLOCO 6: `(2×5)/(3×1)=3.33` — índice de research + PROGRESSO. ADR-0003/0006 escalados **[NV]** (aprovação de maintainer).
- Factory fonte desta sessão `046e1f2a8ce96a42a04b7ea3735da0251e7b1cef` (mutou desde `d2bd958`); nada transplantado.
- Ciclo 056 fechado: assess `go (scoped)` → spec/clarify/plan/checklist/tasks → analyze PASS → implement → converge → validate R4 → release R7.
- Implementação: `docs/research/index.md` (135 entradas, 135 links válidos, idempotente) e `docs/PROGRESSO.md` (nota datada, contagens `find`, **[NV]** inline). Correções pós-verificador com revalidação PASS.
- Gates: guardião PASS 50 total; fmt/clippy/deny/build/test exit 0; frontend check 0/0, lint ok, 42/42, build ok.
- PR → `dev/`; merge deliberado somente após CI verde observada ao vivo (ADR-0003). Sem auto-merge/force-push/tag/deploy.
- **[NV]** CI/PR/aprovações ao vivo, valor atual de feature.json e ADRs.

## Decisão 80/20 — 2026-11-18T02:43Z (registrada antes do BLOCO 6)

- Critério `(impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador por candidato.
- **Escolhido: 057 — guardião determinístico computa o total de pisos a partir de `FLOORS`.** `(3×5)/(2×1) = 7.5`, maior candidato autônomo. Evidência: `scripts/ci/check_gate_integrity.py:120` imprime literal `"all eight db contract floors intact (50 total)"`; o número foi corrigido manualmente em 052 (43→49), 053 (49→...), 054 (49→50), 055 (→50) — drift recorrente. `FLOORS` já é a fonte da verdade.
- **Escalado [NV]:** ADR-0003/0006 (drift 43 vs 50, 5.0) — exige aprovação de maintainer para documento de governança. Root `index.md` (2.5), local-false-positive (1.875), N3 (1.0), dívidas de fase (0.67).
- HEAD da factory como fonte desta sessão: `38f682663f0121ec178bac592216501ad9fce8c3` (mutou desde `046e1f2a`); nada transplantado.

## Atualização — ciclo 057 (2026-11-18T02:48Z)

- Âncora `github/dev@97838cd9c`; branch `feature/057-dynamic-floor-total`; NNN 057 por `create-new-feature.sh --dry-run` (feature.json apontava 051, fechado).
- Decisão 80/20 registrada neste arquivo antes do BLOCO 6: `(3×5)/(2×1)=7.5` — derivar total de `FLOORS`. ADR-0003/0006 escalados **[NV]** por aprovação de maintainer.
- Factory fonte desta sessão `38f682663f0121ec178bac592216501ad9fce8c3`; nada transplantado.
- Ciclo 057 fechado: assess `go (scoped)` → spec/clarify/plan/checklist/tasks → analyze PASS → implement → converge → validate R4 → release R7.
- Implementação: 1 linha em `scripts/ci/check_gate_integrity.py:120` (f-string com `len(FLOORS)`/`sum(FLOORS.values())`). Saída `all 8 ... (50 total)`; mutação 9 pisos → 54.
- Verificador isolado PASS 5/5; gates: guardião PASS, fmt/clippy/deny/build/test exit 0, frontend 42/42.
- PR → `dev/`; merge deliberado após CI verde observada ao vivo (ADR-0003). Sem auto-merge/force-push/tag/deploy.
- **[NV]** CI/PR/aprovações ao vivo e ADRs.

## Decisão 80/20 — 2026-11-18T03:02Z (registrada antes do BLOCO 6)

- Critério `(impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador por candidato.
- **Escolhido: 058 — criar `index.md` na raiz.** `(1×5)/(2×1) = 2.5`, maior candidato autônomo. Evidência: `test -f index.md` → ausente; `OBJETIVO.md`/`ROADMAP.md`/`docs/PROGRESSO.md` registram a ausência; `docs/research/index.md` e `docs/PROGRESSO.md` já existem (ciclo 056) e podem ser linkados.
- **Escalado [NV]:** ADR-0003/0006 (drift 43 vs 50, 5.0) — exige aprovação de maintainer para documento de governança. Demais: falso positivo local 1.875, N3 1.0, dívidas de fase 0.67, C5/C6 não autônomos.
- HEAD da factory como fonte desta sessão: `38f682663f0121ec178bac592216501ad9fce8c3`; nada transplantado.

## Atualização — ciclo 058 (2026-11-18T03:08Z)

- Âncora `github/dev@58921a337`; branch `feature/058-root-doc-index`; NNN 058 por `create-new-feature.sh --dry-run` (feature.json apontava 051, fechado).
- Decisão 80/20 registrada neste arquivo antes do BLOCO 6: `(1×5)/(2×1)=2.5` — `index.md` raiz. ADR-0003/0006 escalados **[NV]** por aprovação de maintainer.
- Factory fonte `38f682663f0121ec178bac592216501ad9fce8c3`; nada transplantado.
- Ciclo 058 fechado: assess `go (scoped)` → spec/clarify/plan/checklist/tasks → analyze PASS → implement → converge → validate R4 → release R7.
- Implementação: `index.md` (26 linhas, 8 links, [NV]); duplicação de conteúdo apontada pelo verificador foi corrigida e revalidada.
- Gates: guardião PASS; fmt/clippy/deny/build/test exit 0; frontend 42/42.
- PR → `dev/`; merge deliberado após CI verde observada ao vivo (ADR-0003). Sem auto-merge/force-push/tag/deploy.
- **[NV]** CI/PR/aprovações ao vivo e ADRs.

## Decisão 80/20 — 2026-11-18T03:24Z (registrada antes do BLOCO 6)

- Critério `(impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador por candidato.
- **Escolhido: 059 — falhar localmente os contratos DB quando a env de Postgres estiver ausente.** `(3×5)/(4×2) = 1.875`, maior candidato autônomo restante. Evidência: 9 `return Ok(None)` em `flora-tests/src/contract/*.rs` (7 com `eprintln!` de "skipping", 2 sem) e 50 `return Ok(())` — `cargo test --ignored` sem env reporta 50 passes sem asserção. CI já é fail-closed (054); o defeito é local.
- **Escalado [NV]:** ADR-0003/0006 (5.0), aprovação de maintainer pendente. N3 1.0; dívidas de fase 0.67; C5/C6 não autônomos.
- HEAD da factory como fonte desta sessão: `1083d3212b595d56b15bee1186708ba34b7ea7e4` (mutou desde `38f68266`); nada transplantado.

## Atualização — ciclo 059 (2026-11-18T03:31Z)

- Âncora `github/dev@2730fd2dd`; branch `feature/059-db-tests-fail-local`; NNN 059 por `create-new-feature.sh --dry-run` (feature.json apontava 051, fechado).
- Decisão 80/20 registrada neste arquivo antes do BLOCO 6: `(3×5)/(4×2)=1.875` — falha local sem env. ADR-0003/0006 escalados **[NV]** por aprovação de maintainer.
- Factory fonte `1083d3212b595d56b15bee1186708ba34b7ea7e4`; nada transplantado.
- Ciclo 059 fechado: assess `go (scoped)` → spec/clarify/plan/checklist/tasks → analyze PASS → implement → converge → validate R4 → release R7.
- Implementação: 9 `bail!` em 8 arquivos `flora-tests/src/contract/`; fmt aplicado; 50 chamadores intactos.
- Evidência: sem env exit 101; DB isolado 50 passed; verificador 5/5; gates exit 0.
- PR → `dev/`; merge deliberado após CI verde observada ao vivo (ADR-0003). Sem auto-merge/force-push/tag/deploy.
- **[NV]** CI/PR/aprovações ao vivo e ADRs.

## Decisão 80/20 — 2026-11-18T03:48Z (registrada antes do BLOCO 6)

- Critério `(impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador por candidato.
- **Escolhido: 060 — refresh datado das contagens verificadas em `docs/PROGRESSO.md`.** `(2×5)/(2×1) = 5.0`, maior candidato autônomo. Evidência: nota de 02:23Z registra specs 47/202, assessments 44/175, validations 42/43, releases 42/42; `find` agora → specs 50/224, assessments 47/192, validations 46/47, releases 46/46 (ciclos 057–059). Histórico preservado: nova nota append-only, sem reescrever a anterior.
- **Escalado [NV]:** ADR-0003/0006 (5.0), aprovação de maintainer. N3 `(4×3)/(3×4)=1.0` envolve FK composto e risco de órfãos — sob análise; não iniciado. C5/C6 dependem de dono/SE.
- HEAD da factory como fonte desta sessão: `0cb34c4d0b020b4b249a325ff90fe130c179d00f` (mutou desde `1083d321`); nada transplantado.

## Atualização — ciclo 060 (2026-11-18T03:54Z)

- Âncora `github/dev@262ac042f`; branch `feature/060-refresh-verified-counts`; NNN 060 por `create-new-feature.sh --dry-run` (feature.json apontava 051, fechado).
- Decisão 80/20 registrada neste arquivo antes do BLOCO 6: `(2×5)/(2×1)=5.0` — refresh de contagens. ADR-0003/0006 escalados **[NV]**; N3 escalado (FK com órfãos [NV], ADR-0006/SE).
- Factory fonte `0cb34c4d0b020b4b249a325ff90fe130c179d00f`; nada transplantado.
- Ciclo 060 fechado: assess `go (scoped)` → spec/clarify/plan/checklist/tasks → analyze PASS → implement → converge → validate R4 → release R7.
- Implementação: apêndice append-only em `docs/PROGRESSO.md` (37+/0−), com correção 03:52Z; contagens conferidas 12/12 por `find`; verificador 6/6.
- Gates: guardião PASS; fmt/clippy/deny/build/test exit 0; frontend 42/42.
- PR → `dev/`; merge deliberado após CI verde observada ao vivo (ADR-0003). Sem auto-merge/force-push/tag/deploy.
- **[NV]** CI/PR/aprovações ao vivo, ADRs e dados para N3.

## Decisão 80/20 — 2026-11-18T04:05Z (registrada antes do BLOCO 6)

- Critério `(impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador por candidato.
- **Escolhido: 061 — remover código morto do circuit-breaker.** `(2×5)/(3×2) = 1.67`, maior candidato autônomo. Evidência: `flora-core/src/utils/circuit_breaker.rs` (68 linhas) nunca declarado em `utils/mod.rs` (só `jwt`/`pkce`); campos `circuit_breaker_threshold`/`circuit_breaker_recovery_secs` em `config/messaging.rs:19,21,33,34` sem uso; `Error::CircuitBreakerOpen` em `error.rs:181,365` referenciado apenas pelo arquivo morto. Figment não usa `deny_unknown_fields` — remover campos não quebra env extraneous. Alinhado ao Princípio I (debloat).
- **Escalados [NV]:** ADR-0003/0006 (maintainer), N3 (SE/FK), C5/C6 (dono). Dívidas de fase exigiriam forjar validate/release históricos — não executado.
- HEAD da factory como fonte desta sessão: `0cb34c4d0b020b4b249a325ff90fe130c179d00f`; nada transplantado.

## Atualização — ciclo 061 (2026-11-18T04:11Z)

- Âncora `github/dev@7ed232c46`; branch `feature/061-dead-circuit-breaker`; NNN 061 por `create-new-feature.sh --dry-run` (feature.json apontava 051, fechado).
- Decisão 80/20 registrada neste arquivo antes do BLOCO 6: `(2×5)/(3×2)=1.67` — remover circuit-breaker morto. ADR-0003/0006, N3 e C5/C6 escalados **[NV]**; dívidas de fase não forjadas.
- Factory fonte `0cb34c4d0b020b4b249a325ff90fe130c179d00f`; nada transplantado.
- Ciclo 061 fechado: assess `go (scoped)` → spec/clarify/plan/checklist/tasks → analyze PASS → implement → converge → validate R4 → release R7.
- Implementação: 3 arquivos (79 remoções/0 inserções) — arquivo morto excluído, campos de config e variante de erro removidos.
- Verificação: grep 0; verificador 6/6; guardião PASS; fmt/clippy/deny/build/test e frontend 42/42 exit 0.
- PR → `dev/`; merge deliberado após CI verde observada ao vivo (ADR-0003). Sem auto-merge/force-push/tag/deploy.
- **[NV]** CI/PR/aprovações ao vivo e escalonamentos.

## Decisão 80/20 — 2026-11-18T04:26Z (registrada antes do BLOCO 6)

- Critério `(impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador por candidato.
- **Escolhido: 062 — registro único de escalonamentos + estado verificado do OSV.** `(2×5)/(2×1) = 5.0`, maior candidato autônomo. Evidência: ADR-0003/0006 (texto 43 vs 50), N3 (FK com órfãos **[NV]**, ADR-0006/SE) e C5/C6 (dono/SE) são reavaliados a cada sessão sem registro consolidado; OSV workflow `active` (`gh api`, created 2026-11-16) com **0 runs** e cron semanal (`37 14 * * 1`), portanto ainda sem execução para validar o pin de 052.
- **Não executado:** ADR/N3/C5/C6 — exigem maintainer/SE/dono; dívidas de fase — exigiriam forjar artefatos históricos.
- HEAD da factory como fonte desta sessão: `4a5de2ce218fa253a06de313c8008aea78aab351` (mutou desde `0cb34c4d`); nada transplantado.

## Registro de escalonamento — 2026-11-18T04:29Z (append-only)

Tabela canônica de itens que **não** podem ser executados autonomamente.
Nenhum deles foi iniciado; aprovações permanecem **[NV]**.

| Item | Evidência | Bloqueio | Ação necessária | Status |
|---|---|---|---|---|
| ADR-0003/0006 (texto "43") | `docs/adr/0003:19`, `docs/adr/0006:10,52`; gate imprime 50 derivado de `FLOORS` | Constituição §Governance: emenda exige maintainer | aprovar atualização dos números | escalado **[NV]** |
| N3 (FK `memberships.role_id`) | `20240818000001_init_schema.sql:33,40-49`; órfãos **[NV]**; `docs/adr/0006:27-31` | DDL em produção + possíveis dados órfãos | verificar órfãos e autorizar DDL/limpeza via SE | escalado **[NV]** |
| C5 (handlers RBAC CRUD) | `flora-api/src/routes/rbac.rs` placeholders | decisão de autoridade (dono) | definir quem pode criar/atribuir papéis | escalado **[NV]** |
| C6 (RLS policies) | `20240818000001_init_schema.sql:196-215` (RLS enable, 0 policies); ADR-0005/0006 | autorização SE para rollout | aprovar política fail-closed | escalado **[NV]** |

**OSV (verificado ao vivo 2026-11-18T04:25Z via `gh api`):** workflow
`osv-scan.yml` `state=active`, `id=420259367`, criado em 2026-11-16T11:04:49-04:00,
**0 runs**; agendamento `cron: '37 14 * * 1'` (segunda-feira) + `workflow_dispatch`.
O pin `@v2.7.1` (ciclo 052) ainda não tem execução que o valide — **[NV]**
até a próxima segunda-feira. Nenhum dispatch foi disparado nesta sessão.

## Decisão 80/20 — 2026-11-18T04:45Z (registrada antes do BLOCO 6)

- Critério `(impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador por candidato.
- **Escolhido: 063 — validar o pin `@v2.7.1` do OSV via `workflow_dispatch`.** `(2×5)/(2×2) = 2.5`, único candidato autônomo restante. Evidência: workflow `active`, 0 runs desde 2026-11-16, cron semanal; o [NV] de 052 só fecha com execução real. `workflow_dispatch` é gatilho declarado no próprio workflow; não é merge/force-push/deleção.
- **Escalados:** ADR-0003/0006 (maintainer), N3 (SE), C5/C6 (dono) — ver tabela canônica em `STATE.md`. Dívidas de fase não forjadas.
- HEAD da factory como fonte desta sessão: `b5bc7265d7fe6efaa316c54a54eed1c3ce1e71bc`; nada transplantado.

## Validação do pin OSV — 2026-11-18T04:47Z (append-only)

Fecha o **[NV]** registrado no ciclo 052 e no apêndice de 04:29Z: o pin
`@v2.7.1` foi validado por execução real, não apenas por análise estática.

- Gatilho: `gh workflow run osv-scan.yml --repo MDMV-Laboratorio-de-Solucoes-Digitais/flora --ref dev` (declarado no próprio workflow).
- Run: `databaseId=44239662140`, `headBranch=dev`, `status=completed`,
  `conclusion=success`, `createdAt=2026-11-18T04:46:43Z`,
  `updatedAt=2026-11-18T04:47:13Z`,
  URL: https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/44239662140.
- Etapas observadas até conclusão: checkout, setup, `osv-scanner-reporter`,
  upload de artefato, upload para code-scanning — todas com ✓.
- Interpretação: o workflow reutilizável resolvel `@v2.7.1` e executa; o sinal
  advisory de supply-chain está operacional. `fail-on-vuln: false` permanece
  configurado, portanto `success` não significa "zero vulnerabilidades" —
  apenas que o scan e o upload concluíram.
- Anotação informativa do runner: label `ubuntu-latest` migrará para Ubuntu 26
  em outubro/2026 — impacto futuro **[NV]**, fora deste ciclo.
- Nota anterior (04:29Z) permanece como registro histórico; esta a corrige.

## Autorização SE e decisão 80/20 — 2026-11-18T11:13Z (registrada antes do BLOCO 6)

- O Software Engineer autorizou, para N3: (1) camada 1 em código (org-scoping de resolução de papéis); (2) `UNIQUE(organization_id, id)` em `roles`; (3) auditoria read-only de órfãos/mismatch; (4) FK composto `(organization_id, role_id) → roles(organization_id, id)` com `ON DELETE RESTRICT` **se** a auditoria estiver limpa; (5) se houver sujeira, retornar com os dados para escolha de limpeza.
- Auditoria read-only executada em `flora` (local) às 11:12Z: `orfaos=0`, `cross_org=0`, `memberships=3`, `roles=3`. **Produção não auditada — [NV]**; a migration terá pré-voo `RAISE EXCEPTION` para falhar fechado se houver dados inválidos no deploy.
- Critério `(impacto × confiança) ÷ (esforço × risco)` = `(4×5)/(5×3) = 1.33`, candidato autorizado de maior prioridade. ADR-0003/0006 (patch documental), C5 (`RoleWrite`) e C6 (desenho de canário, sem produção) ficam autorizados e enfileirados em ciclos separados.
- HEAD da factory como fonte desta sessão: `ce3d42132edb9466b75eab968be775ebdea38985`; nada transplantado.

## Decisão 80/20 (autorização SE) — 2026-11-18T13:04Z (antes do BLOCO 6)

- Critério `(impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador por candidato.
- **Escolhido: 065 — alinhar cifras dos ADR-0003/0006 ao gate atual.** `(2×5)/(1×1) = 10.0`. Autorização SE explícita na conversa: `13/4/8/8/5/6/4/2 — total 51` (o SE escreveu "total 50" com base em STATE anterior; a cifra real após o ciclo 064 é 51 — registrar a correção com evidência do gate).
- Evidência: `docs/adr/0003:19` ("12/4/8/8/5/6 — total 43"), `docs/adr/0006:10,52` ("piso 43"); gate imprime `8 pisos / 51 total` derivado de `FLOORS`.
- Demais autorizados enfileirados: C6 canário/desenho (3.33), C5 `RoleWrite` (2.5).
- HEAD da factory como fonte desta sessão: `c81f0f8191d9dfa1285001bdd45f2a8fddd8dac3`; nada transplantado.

## Fechamento do escalonamento ADR — 2026-11-18T13:09Z (append-only)

O item `ADR-0003/0006` da tabela de escalonamento foi **autorizado pelo SE e
concluído** no ciclo 065: citações de piso alinhadas a
`13/4/8/8/5/6/4/2 — total 51`. O SE citou 50 com base no estado anterior ao
ciclo 064 (workspace 2→3); o valor real do gate é 51 e foi registrado.
A linha histórica da tabela permanece intacta; este apêndice a corrige.
Itens ainda autorizados e pendentes: C5 (`RoleWrite`) e C6 (canário RLS, sem produção).

## Decisão 80/20 (autorização SE) — 2026-11-18T14:38Z (antes do BLOCO 6)

- Critério `(impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador por candidato.
- **Escolhido: 066 — implementar CRUD de papéis com `RoleWrite` dedicada.** `(4×5)/(4×2) = 2.5`, último item de código autorizado. SE decidiu: permissão `RoleWrite` dedicada; `OrgAdmin` a possui habilitada por padrão; papéis `is_builtin` imutáveis (premissa registrada). C6 (canário RLS) segue como documento, sem produção.
- Evidência: `flora-api/src/routes/rbac.rs:131-164` — quatro handlers placeholder que retornam strings; `GET /roles` já é real (ciclo 042).
- HEAD da factory como fonte desta sessão: `c81f0f8191d9dfa1285001bdd45f2a8fddd8dac3`; nada transplantado.

## Fechamento do escalonamento C5 — 2026-11-18T14:50Z (append-only)

Item `C5` da tabela de escalonamento **autorizado pelo SE e concluído** no ciclo
066: permissão `RoleWrite` dedicada, `OrgAdmin` mantida por padrão e papéis
`is_builtin` imutáveis. Handlers placeholder substituídos por CRUD real.
A linha histórica da tabela permanece; este apêndice a corrige.
Item pendente autorizado: C6 (desenho de canário RLS, sem produção).

## Decisão 80/20 (modo autônomo) — 2026-11-18T15:49:43Z (registrada antes do BLOCO 6)

- Critério `E = (impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador explícito por candidato.
- **Escolhido: 067 — C6: desenho de canário RLS com rollback, sem habilitação em produção.** `(5×5)/(3×1) = 8.33`.
- Autorização: SE já autorizou C6 (entradas 13:04Z / 14:38Z / 14:50Z). Escopo = documento + artefatos SDD; **sem** tocar produção, **sem** required checks, **sem** policies/FORCE em runtime (ADR-0006 §1–3).
- Evidência local: 11 tabelas com `ENABLE ROW LEVEL SECURITY` porém `0 CREATE POLICY`, `0 set_config`, `0 papéis de DB`, `0 wrapper de transação`; ADR-0005 (desenho fail-closed provado em DB descartável) e ADR-0006 (promoção em duas etapas, rollback sem dado destrutivo).
- Ranqueamento: C6 `8.33`; refresh de docs verificado `5.0`; contrato RBAC real `3.0`; remoção de stubs `0.67`; fiação Meilisearch `0.27`.
- CI: PR #285 com 6 checks verdes e gate `check_gate_integrity.py` PASS (8 pisos / 51) — "elevar a CI" não é candidato nº 1.
- Âncora: `feature.json` (runtime, não versionado) congela `specs/051-dead-invite-ui`; `check-prerequisites.sh` ausente no alvo. `NNN = máximo(specs) + 1 = 067`, por evidência determinística de `specs/`. Divergência registrada.
- HEAD da factory como fonte: `c81f0f8191d9dfa1285001bdd45f2a8fddd8dac3`; nada transplantado como autoridade (scripts SDD materializados apenas em runtime, sem baseline).
- **[NV]**: status remoto/CI futuro, prova A/B em app real, produção.

## Execução do ciclo 067 — 2026-11-18T15:53:58Z

- Unidade: 1 agent + worktree `.specify/queue/worktrees/c6-canary-rls` + branch `feature/067-c6-canary-rls-design`; ledger `active` registrado.
- Entregável: `docs/research/2026-11-18-067-c6-canary-rls-design.md`; refresh append-only de `PROGRESSO.md` e `research/index.md`.
- Nenhuma alteração em código, workflows, migrações, ADRs, constituição, produção ou required checks.
## Decisão 80/20 (modo autônomo) — 2026-11-19T01:58Z (registrada antes do BLOCO 6)

- Critério `E = (impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador explícito por candidato.
- **Escolhido: 068 — refresh verificado de documentação.** `(2×5)/(2×1) = 5.00`, maior candidato autônomo.
- Evidência determinística (`find`, 2026-11-19T01:56Z): `specs/` 58 dirs / 280 arquivos, assessments 55, validations 54, releases 54, bugs 4, research 138. `docs/research/index.md:14-19` afirma 202/175/43/42/12 (copiado de `PROGRESSO.md:17`); `docs/PROGRESSO.md:88-90` afirma validations/releases 53; `OBJETIVO.md:3,39` e `ROADMAP.md:3` param em 033/046. Gate autoritativo PASS (8 pisos/51); `dev` local == `github/dev@cbad031b5`; 0 PRs abertos; últimos 4 runs verdes.[NV] para CI futuro.
- Ranqueamento completo: retro-indexação 047–067 3.75; contrato RBAC vácuo 2.50; stub AuthService morto 2.50; DAG duplicatas legadas 0.83. Emenda da constituição (SolidJS vs Svelte 5, PGMQ obrigatório) pontua 7.50 mas **exige aprov maintainer** (Constituição §Governance) — escalada **[NV]**, não executada. CI não é candidato nº 1 (saudável ao vivo).
- Âncora: `feature.json` (runtime/gitignored) congela `specs/051-dead-invite-ui`; `check-prerequisites.sh --json` retorna 051 com `AVAILABLE_DOCS:[]`. `NNN = máximo(specs)+1 = 068` por evidência determinística de `specs/`. Divergência registrada, sem reescrever âncora alheia.
- HEAD da factory como fonte: `03f274148a5e3afd6970eb02aa4577e4a4bfba43`; nada transplantado como autoridade (scripts SDD materializados em `/tmp/opencode/sdd-checks` somente para leitura/gates).
- Escopo: documentação + 1 comentário obsoleto em `flora-api/src/routes/rbac.rs:4-5` (diz "placeholders" onde o código é real desde o ciclo 066). Sem mudança de comportamento, workflows, required checks, migrações ou produção.

## Execução do ciclo 068 — 2026-11-19T01:58Z (append-only)

- Unidade: 1 agente + worktree `.specify/queue/worktrees/verified-docs-refresh` + branch `feature/068-verified-docs-refresh`; ledger `active` registrado.
- Entregáveis: contagens corrigidas em `docs/research/index.md`, nota datada em `docs/PROGRESSO.md`, intervalo 001–067 em `OBJETIVO.md`/`ROADMAP.md`, comentário corrigido em `flora-api/src/routes/rbac.rs`, artefatos SDD completos (assess 5, spec 8, validate, release).
- `analyze` (READ-ONLY) encontrou SC-001/002/003 ausentes em `plan.md`; corrigido na fonte e reexecutado → `go=True`, 0 erros. `converge.md` append-only.
- Nenhuma alteração em workflows, migrations, ADRs, constituição, required checks ou produção. Emenda constitucional e indexação 047+ permanecem escaladas/adiviadas, não executadas.
- **[NV]** CI futura, Forgejo, produção e aprovações; merge deliberado somente após CI verde observada ao vivo (ADR-0003).

## Fechamento pós-merge — 2026-11-19T02:17:21Z (ciclo 068; append-only)

- PR **#287** `MERGED` ao vivo às `02:17:21Z`; merge commit `0d1a83f540f899a1cc11777132ef76804a384362`. `mergeStateStatus=CLEAN` imediatamente antes do merge; nenhum auto-merge, force-push ou aprovação simulada.
- Checks observados ao vivo: `frontend-build` pass, `rust-build` pass, `db-tests` pass, `dependency-review` pass no run do PR (o run de push marca `skipping`, permitido pela ADR-0003); informativos `e2e-browser` e `supply-chain-vet` pass.
- Âncora final: `dev` local = `github/dev@0d1a83f54`. Worktree de feature removida, branch local apagada, branch remota removida pelo GitHub na merge e ref de tracking podado com `fetch --prune`. 0 stashes, 0 branches de feature.
- Gates locais: autoritativo local/`--remote` PASS; fmt/clippy/deny/build exit 0; `cargo test --workspace` 106 passed/52 ignored; frontend check 0/0, lint ok, 42/42, build ok. Verificador isolado PASS (6/6 e 5/5, nenhum nit).
- Fábrica fonte `03f274148a5e3afd6970eb02aa4577e4a4bfba43` — lida, nunca modificada; trabalho em voo da factory (9 stashes, dirty, feature.json→specs/123, validate.md) **não transplantado**.
- Dívidas: indexação retroativa 047–067 (próximo 80/20, E=3.75); contrato RBAC vácuo (2.50); stub `AuthService` (2.50); duplicatas legadas de T-ID (0.83); emenda da constituição escalada **[NV]** (aprov maintainer). CI futura, Forgejo, produção e aprovações: **[NV]**.
## Decisão 80/20 (modo autônomo) — 2026-11-19T02:35Z (registrada antes do BLOCO 6)

- Critério `E = (impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador explícito por candidato.
- **Escolhido: 069 — corrigir afirmação falsa de dívida em `docs/research/index.md`.** `(2×5)/(1×1) = 10.00`.
- Evidência determinística: `re.findall(r'\]\(([^)]+\.md)\)', index.md)` comparado a `find docs/research -maxdepth 1 -name '*.md' ! -name index.md` → **137 arquivos, 137 links, 0 faltantes**. As seções (`3/124/8/2`) somam 137. A nota em `docs/research/index.md:172-179` afirma que o índice cobre apenas até a entrada 046 e que 047+ está pendente — afirmação falsa e enganosa para agentes. Correção por apêndice datado, sem reescrever histórico.
- Ranqueamento: constituição (7.50) escalada **[NV]** por exigir maintainer; RBAC vácuo 2.50; stub AuthService 2.50; duplicatas DAG 0.83; backfill 047–067 **eliminado por evidência** (0 faltantes). CI não é candidato: 0 PRs abertos, `dev == github/dev@cbad031b5` → `c8d4fb2e3`, runs verdes.
- Âncora: `feature.json` (runtime/gitignored) congela `specs/051-dead-invite-ui`; `check-prerequisites.sh --json` retorna 051 com `AVAILABLE_DOCS:[]`. `NNN = máximo(specs)+1 = 069` por `--dry-run`. O branch `feature/069-state-closure` da sessão anterior foi bookkeeping sem `specs/069-*`; esta é a primeira spec 069 — divergência registrada, sem renumerar trabalho alheio.
- HEAD da factory como fonte: `03f274148a5e3afd6970eb02aa4577e4a4bfba43`; nada transplantado como autoridade.
- Escopo: 1 arquivo de documentação + artefatos do ciclo + `STATE.md`. Sem código, workflow, gate, ADR, constituição ou produção.

## Execução do ciclo 069 — 2026-11-19T02:35Z (append-only)

- Unidade: 1 agente + worktree `.specify/queue/worktrees/research-index-truth` + branch `feature/069-research-index-truth`; ledger `active` registrado.
- Entregável: apêndice corretivo em `docs/research/index.md` com verificação 137/137 (`missing=[]`), mantendo todas as notas anteriores intactas.
- Eliminação por evidência: o candidato "backfill 047–067" não existe — 137 arquivos, 137 links, 0 faltantes. O resíduo real é de prefixo de ciclo nos títulos, fora de escopo.
- `analyze` (READ-ONLY) PASS na 1ª execução; `converge.md` append-only.
- Nenhuma alteração em código, workflows, gates, ADRs, constituição ou produção.
- **[NV]** CI futura, Forgejo, produção e aprovações; merge deliberado somente após CI verde observada ao vivo (ADR-0003).

## Fechamento pós-merge — 2026-11-19T02:49:40Z (ciclo 069; append-only)

- PR **#289** `MERGED` ao vivo às `02:49:40Z`; merge commit `74a770686092031bfae2dc0ea710d9c844cb4b08`. `mergeStateStatus=CLEAN` confirmado antes do merge; sem auto-merge, force-push ou aprovação simulada.
- Checks observados ao vivo: `frontend-build`, `rust-build`, `db-tests`, `dependency-review` pass (no run do PR); informativos `e2e-browser` e `supply-chain-vet` pass. `dependency-review skipping` apenas no run de push, permitido pela ADR-0003.
- Âncora final: `dev` local = `github/dev@74a770686`. Worktree de feature removida, branch local apagada, branch remota removida pelo GitHub e ref podado. 0 stashes, 0 branches de feature, 1 worktree limpa.
- Gates locais: autoritativo local/`--remote` PASS; fmt/clippy/deny/build exit 0; `cargo test --workspace` 106 passed/0 failed/52 ignored; frontend 0/0, lint ok, 42/42, build ok. Verificador isolado PASS 5/5.
- Evidência central: `docs/research/index.md` tem 137 arquivos / 137 links / `missing=[]` — a dívida 047+ era falsa; o resíduo é apenas de prefixo de ciclo nos títulos, não corrigido por inferência.
- Fábrica fonte `03f274148a5e3afd6970eb02aa4577e4a4bfba43` — lida, nunca modificada; 9 stashes/2 worktrees em voo **não transplantados**.
- Dívidas: contrato RBAC vácuo e stub `AuthService` (2.50 cada), duplicatas legadas de T-ID (0.83), renomeação de títulos sem prefixo de ciclo; emenda da constituição escalada **[NV]** (aprov maintainer). CI futura, Forgejo, produção e aprovações: **[NV]**.
## Decisão 80/20 (modo autônomo) — 2026-11-19T02:58Z (registrada antes do BLOCO 6)

- Critério `E = (impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador explícito por candidato.
- **Escolhido: 070 — substituir o contrato RBAC vácuo por contratos DB-gated reais.** `(3×5)/(3×2) = 2.50`.
- Evidência: `flora-tests/src/contract/rbac.rs:3-7` contém apenas `Ok(())`; os handlers de `flora-api/src/routes/rbac.rs:168-320` são reais desde o ciclo 066 (`RoleWrite || OrgAdmin`, built-ins imutáveis, escopo pela sessão). Padrão a seguir: `flora-tests/src/contract/workspace.rs` (contratos `#[ignore]` + env `FLORA_DATABASE__POSTGRES_URL`, fail-closed desde o ciclo 059). Postgres local disponível (`pg_isready` OK).
- Ranqueamento: AuthService stub `(2×5)/(2×2)=2.50` (empate, menor impacto); renomeação de 124 títulos `(2×5)/(4×1)=2.50` (maior esforço, sem ganho de correção); duplicatas T-ID `(2×5)/(4×3)=0.83`; emenda constitucional `(3×5)/(1×2)=7.50` escalada **[NV]** por aprov maintainer. CI não é candidato: 0 PRs abertos e `dev == github/dev@0c7890ef4`.
- Âncora: `feature.json` (runtime/gitignored) congela `specs/051-dead-invite-ui`; `check-prerequisites.sh --json` retorna 051 com `AVAILABLE_DOCS:[]`. `NNN = máximo(specs)+1 = 070` por `--dry-run`. O branch `feature/070-state-closure` da sessão anterior foi bookkeeping sem `specs/070-*`; esta é a primeira spec 070 — divergência registrada, sem renumerar trabalho alheio.
- HEAD da factory como fonte: `5cc29f8738b4d9c79ac4c451a6ab0c9ebb62f351` (mutou desde `03f2741`; 9 stashes/27 linhas dirty **não transplantadas**); scripts SDD materializados apenas em `/tmp/opencode/sdd-checks3`.
- Escopo: 1 arquivo de teste (`flora-tests/src/contract/rbac.rs`) + artefatos do ciclo + `STATE.md`. Sem mudança de comportamento em produção, workflows, gates, ADRs ou constituição.

## Execução do ciclo 070 — 2026-11-19T03:02Z (append-only)

- Unidade: 1 agente + worktree `.specify/queue/worktrees/rbac-contract` + branch `feature/070-rbac-contract`; ledger `active` registrado.
- Entregável: `flora-tests/src/contract/rbac.rs` — stub `Ok(())` substituído por 3 contratos DB-gated (`db_rbac_list_scoped_to_session_org`, `db_rbac_create_requires_role_write_and_uses_session_org`, `db_rbac_builtin_immutable_custom_patchable`).
- Evidência: 3 passed com `FLORA_DATABASE__POSTGRES_URL=.../flora_070_rbac_verify` (Postgres 18.6 descartável, criado por esta sessão); sem env → `FAILED` com `bail!` (fail-closed); mutação da asserção cross-org → `CROSS-ORG ROLE LEAK` (teste sensível), fonte restaurada e 3 passed novamente.
- Lints do alvo (`unused_results=deny`) reprovaram 2 resultados descartados; corrigidos com `let _membership`/`let _session` — nenhum lint relaxado.
- Nenhuma alteração em produção, workflows, gates, pisos, ADRs ou constituição. Dívida registrada: CI não executa estes contratos (sem leg `rbac` em `rust.yml`), exigindo ciclo de governança separado **[NV]**.
- **[NV]** CI futura, Forgejo, produção e aprovações; merge deliberado somente após CI verde observada ao vivo (ADR-0003).

### Correção de redação (append-only) — 2026-11-19T03:08Z

O verificador isolado apontou nit textual: `db_state()` usa
`ok_or_else(|| anyhow::anyhow!(...))?` (fail-closed equivalente a `bail!`), não `bail!`.
A linha de execução acima permanece como registro histórico; esta nota a corrige.
Nenhuma lógica, teste ou gate foi alterado.

## Fechamento pós-merge — 2026-11-19T03:19:49Z (ciclo 070; append-only)

- PR **#291** `MERGED` ao vivo às `03:19:49Z`; merge commit `134d45234ec95a1014d8f80a8b67806062a7eb79`. `mergeStateStatus=CLEAN` confirmado antes do merge; sem auto-merge, force-push ou aprovação simulada.
- Checks ao vivo: `frontend-build`, `rust-build`, `db-tests`, `dependency-review` pass (run do PR); informativos `e2e-browser`/`supply-chain-vet` pass; `dependency-review skipping` apenas no run de push (ADR-0003).
- Âncora final: `dev` local = `github/dev@134d45234`. Worktree e branch do ciclo 070 removidas; 0 stashes, 0 branches de feature.
- Evidência central: 3 contratos RBAC passam com Postgres descartável, falham fechado sem env e detectam mutação da asserção cross-org (`CROSS-ORG ROLE LEAK`). Nenhum handler, workflow, gate, piso, ADR ou constituição alterado.
- Dívida registrada: CI não executa estes contratos (sem leg `rbac` em `rust.yml` + `FLOORS` + ADRs) — exige aprov maintainer, **[NV]**. Emenda da constituição, Forgejo, produção e aprovações: **[NV]**.
- Fábrica fonte `5cc29f8738b4d9c79ac4c451a6ab0c9ebb62f351` (mutou desde `03f2741`); 9 stashes/27 linhas dirty **não transplantadas**.
## Decisão 80/20 (modo autônomo) — 2026-11-19T03:49Z (registrada antes do BLOCO 6)

- Critério `E = (impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador explícito por candidato.
- **Escolhido: 071 — remover o stub morto `AuthService`.** `(2×5)/(2×2) = 2.50`.
- Evidência determinística: `flora-core/src/services/auth_service.rs:1-70` — 5 métodos retornam `ServiceUnavailable("... not implemented")`, `is_available()`=false; único implementador de `AuthProvider`. Referências: `AppState` (campo em `app_state.rs:30`, init `:97`, struct `:184`), 1 teste em `extractors/auth.rs:278` e 6 contratos em `flora-tests/src/contract/{auth,files,messaging,notifications,search,websocket}.rs`. Nenhum handler usa `auth_service` — auth real vive em `routes/auth.rs` + extractor.
- Desempate: renomeação de 124 títulos sem prefixo de ciclo também pontua 2.50 `(2×5)/(4×1)`, mas é cosmética; a remoção do stub é verificável por compilação e reduz conceitos (Princípio I — debloat).
- Escalados **[NV]**: leg `rbac` no CI (exige `rust.yml` + `FLOORS` + ADRs, aprov maintainer); duplicatas T-ID legadas `(2×5)/(4×3)=0.83`; emenda da constituição. CI em si não é candidato: 0 PRs abertos, `dev == github/dev@cdf50eaf`.
- Âncora: `feature.json` (runtime/gitignored) congela `specs/051-dead-invite-ui`; `check-prerequisites.sh --json` retorna 051. `NNN = máximo(specs)+1 = 071` por `--dry-run`. O branch `feature/071-state-closure` anterior foi bookkeeping sem `specs/071-*`; esta é a primeira spec 071 — divergência registrada.
- HEAD da factory como fonte: `409b76cc3effa56d716d0e9d88e7c3bbe19c2b21` (mutou desde `5cc29f8`; 9 stashes/2 worktrees/4 linhas dirty não transplantadas).
- Escopo: 1 arquivo deletado + 10 arquivos com remoção de referências + artefatos + `STATE.md`. Sem mudança de comportamento em produção, workflows, gates, pisos, ADRs ou constituição.

## Execução do ciclo 071 — 2026-11-19T03:56Z (append-only)

- Unidade: 1 agente + worktree `.specify/queue/worktrees/dead-auth-service` + branch `feature/071-dead-auth-service`; ledger `active` registrado.
- Entregável: remoção completa do stub `AuthService` — arquivo excluído, exports/campo/init/7 construtores de teste limpos. `ServiceUnavailable`, `AuthProvider` e `UserInfo` preservados.
- Evidência: `grep -rn 'AuthService\|auth_service' --include='*.rs'` → **zero**; `cargo check --workspace --all-targets` exit 0 (342 crates).
- `analyze` (READ-ONLY) encontrou FR-004 ausente em plan/tasks; corrigido na fonte e reexecutado → `go=True`, `errors=[]`.
- Nenhuma alteração em handlers de auth, workflows, gates, pisos, migrations, ADRs ou constituição.
- **[NV]** CI futura, Forgejo, produção e aprovações; merge deliberado somente após CI verde observada ao vivo (ADR-0003).

### Correção append-only — 2026-11-19T04:02Z (contagem e fmt)

Contagem real verificada: **10 arquivos Rust** no diff (1 excluído + 9 com referências
removidas) e **10 commits `refactor`**, não 11 — a cifra anterior estava errada; nenhum
arquivo faltou ou sobrou. `cargo fmt --all -- --check` ficou RED após a remoção dos imports
e foi corrigido com `cargo fmt --all` + commit `style(rust)`; fmt/clippy agora exit 0.
Verificador isolado: `PASS-WITH-NITS` (nit de contagem corrigido por esta nota).

## Decisão 80/20 (modo autônomo) — 2026-11-19T04:22:55Z (registrada antes do BLOCO 6)

- Critério `E = (impacto × confiança) ÷ (esforço × risco)`, escalas 1–5, denominador explícito por candidato.
- **Escolhido: 072 — refresh verificado de estado (ciclos 069/070).** `(2×5)/(2×1) = 5.00`.
- Evidência determinística: últimas notas “Estado verificado” em 01:58Z (ciclo 068); contagens reais 61 specs / 58 assessments / 57 validations / 57 releases / 4 bugs / 137 research; 069 e 070 fechados sem nota nos quatro documentos âncora.
- Ranqueamento: leg RBAC `(5×5)/(2×2)=6.25` **excluída** — dívida registrada exige aprov maintainer; emenda constitucional `(3×5)/(1×2)=7.50` **excluída** — exige maintainer; renomeação de 124 títulos `(2×5)/(4×1)=2.50`; stub `AuthService` `(2×5)/(2×2)=2.50`; duplicatas T-ID `(2×5)/(4×3)=0.83` — histórico protegido, não renumerado.
- Âncora: `feature.json`/`check-prerequisites.sh --json` apontavam `051-dead-invite-ui` no início; `create-new-feature.sh --dry-run --number 072` retornou `FEATURE_NUM=072`; 071 está reservado no worktree alheio. Divergência registrada, sem reescrita.
- HEAD da factory como fonte: `ee71d8db0a2ba34ef9fc9486f8d18915282beda1` — lido via `git show`/`git ls-tree`; trabalho em voo da factory **não transplantado**. Gate da factory não importado (BLOCO 0.a); CLI de PR do alvo é `gh` (constituição v1.3.0 §VI), não `fj`.
- Escopo: documentação âncora + artefatos do ciclo + STATE.md. Sem código, workflow, gate, piso, ADR ou constituição.

## Execução do ciclo 072 — 2026-11-19T04:22:55Z (append-only)

- Unidade: 1 agente, branch `feature/072-verified-state-refresh`, sem worktree nova; o worktree `feature/071-dead-auth-service` permanece intocável.
- Assess 5 (intake/research/define/shape/decide com `go (scoped)`), spec/clarify/plan/checklist (revisado)/tasks/analyze/converge; R3 zero perguntas.
- `analyze.md` READ-ONLY: 1ª execução `{"artifacts":11,"errors":[],"fr":5,"go":true,"nv_marked":true,"sc":4,"tasks":10}`, exit 0 — nenhuma correção de fonte necessária.
- Apêndices datados em `OBJETIVO.md`, `ROADMAP.md`, `docs/PROGRESSO.md`, `docs/research/index.md`, `index.md`; prefixos pré-existentes registrados por sha256/bytes antes da edição.
- **[NV]** CI futura, Forgejo, produção e aprovações; merge não realizado neste ciclo.

### Correção append-only de contagem — 2026-11-19T04:23:44Z (ciclo 072)

A nota de execução registrou contagens medidas antes da materialização dos
artefatos 072. Estado final por `find`: specs 62, assessments 59, validations
58, releases 58, bugs 4, research 137 (138 com `index.md`). Nenhuma linha
anterior foi alterada.

## Entrega e fechamento — 2026-11-19T04:36:59Z (ciclo 072; append-only)

- PR **#295** aberto às `04:35:22Z` via `gh` (CLI autoritativa do alvo; `fj` não usado — constituição v1.3.0 §VI): https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/295 — base `dev`, head `feature/072-verified-state-refresh`. **Sem merge, auto-merge, force-push ou deleção de branches.**
- 21 commits convencionais, um arquivo por commit; diff do ciclo = 21 arquivos / 626 inserções / 0 deleções. HEAD final da feature `a30f8730a` após `merge --no-ff` de `github/dev@983e4f316` (ciclo 071) para resolver conflito **aditivo** em `STATE.md`: ambos os apêndices (071 e 072) preservados; nenhum trabalho alheio reescrito ou renumerado.
- Verificação ao vivo via `gh` às `04:36:44Z`: `state=OPEN`, `mergeable=MERGEABLE`, `mergeStateStatus=BLOCKED`, `reviewDecision=""`, checks obrigatórios `pending`. Nenhum check obrigatório verde foi observado nesta sessão; merge permanece **[NV]**/não autorizado.
- Gates locais: `check_gate_integrity.py` PASS (4 checks, 8 pisos/51); fmt/clippy/deny/build exit 0; `cargo test --workspace` 105 passed / 55 ignored / 0 failed; frontend check 0 erros, lint ok, 42/42 testes, build exit 0; append-only 6/6 prefixos intactos; índice research 137/137.
- Diagnósticos da factory executados de HEAD limpo `ee71d8db` e removidos: `check_production_gate.py`/`hub_docs_gate.py`/`pytest -q`/completeness/DAG/unicidade classificados **N/A** (gate de hub, convenção de slugs, duplicatas legadas); `check_converge_append_only.py` PASS com baseline bootstrapado.
- Dívidas: leg `rbac` no CI (exige aprov maintainer), emenda constitucional (maintainer), duplicatas T-ID legadas (histórico protegido); CI final, Forgejo, aprovações, produção e merge: **[NV]**.
- Próximo 80/20: normalizar prefixos de ciclo nos títulos sem prefixo de `docs/research/index.md` `(2×5)/(4×1)=2.50` — maior candidato autônomo restante; a leg RBAC pontua mais, porém depende de maintainer.
## Fechamento pós-merge — 2026-11-19T04:29:52Z (ciclo 071; append-only)

- PR **#293** `MERGED` ao vivo às `04:29:52Z`; merge commit `983e4f3163ff378aed6efa4e7de52ffb52ac5f59`. `mergeStateStatus=CLEAN` confirmado antes do merge (após reexecução do smoke informativo de browser, que havia falhado por expectativa de seed `Welcome to Acme Corporation` e passou no rerun).
- Checks required ao vivo: `frontend-build`, `rust-build`, `db-tests`, `dependency-review` pass. Sem auto-merge, force-push ou aprovação simulada.
- Entrega: remoção completa do stub `AuthService` — `grep` zero, `cargo check/test` e gates verdes; `ServiceUnavailable`, `AuthProvider` e `UserInfo` preservados. Nenhum handler/workflow/gate/migration/ADR/constituição alterado.
- Verificador isolado `PASS-WITH-NITS`; nit de contagem (10, não 11 arquivos/commits) corrigido por notas append-only; `fmt` RED corrigido com `cargo fmt --all`.
- **Concorrência:** outra sessão mantém o ciclo 072 em voo no checkout compartilhado (6 arquivos modificados + `specs/072-*`/`assessments/072-*` não rastreados). O `git merge --ff-only` local foi abortado pelo git; **nada foi tocado**. `dev` local permanece em `cdf50eaff` enquanto `github/dev` está em `983e4f316` — divergência apenas de ponteiro local, sem trabalho perdido.
- Fábrica fonte `409b76cc3effa56d716d0e9d88e7c3bbe19c2b21`; nada transplantado. CI futura, Forgejo, produção e aprovações: **[NV]**.

## Waiver de aprovador distinto — 2026-11-19T04:45:08Z (append-only)

- O SE autorizou **explicitamente** às `04:44Z` o fechamento dos PRs #294 e #295 com exceção à portaria de aprovador distinto. Motivo verificado: `gh api repos/MDMV-Laboratorio-de-Solucoes-Digitais/flora/collaborators` retorna apenas `lfgranja`, que é o autor de ambos os PRs — não existe aprovador distinto no repositório.
- Antes de qualquer merge, `gh pr view` às `04:43:32Z` confirmou `state=OPEN`, `mergeable=MERGEABLE`, `mergeStateStatus=CLEAN` para #294 e `UNSTABLE` para #295; após babysitting, o último `e2e-browser` de #295 passou e o status virou `CLEAN` (`04:43:32Z`). Checks required observados ao vivo: `frontend-build`, `rust-build`, `db-tests`, `dependency-review` — todos `pass`.
- PR **#294** mesclado às `04:44:19Z` (merge commit `407d26ddce414ad88e3eb6daf0e6bf104dfdf029`) com o waiver registrado na mensagem do merge; sem `--admin`, sem `--auto`, sem `--delete-branch`, sem force-push.
- O merge de `dev` na feature 072 reconcilou os apêndices 071 e 072 em `STATE.md` preservando ambos; nenhum trabalho alheio foi reescrito ou renumerado.
- Esta nota é a decisão do SE registrada no repositório; nenhum agente emitiu aprovação simulada. Merge de #295 executado somente após esta reconciliação e revalidação dos checks.

## Fechamento pós-merge — 2026-11-19T04:53:36Z (ciclo 072; append-only)

- PR **#295** `MERGED` ao vivo às `04:50:23Z`; merge commit `c16be5c303e2209710c3e539fbc394329392e572`. `mergeStateStatus=CLEAN` observado às `04:50:13Z` após a reconciliação com #294. Checks required e informativos `pass`: `frontend-build`, `rust-build`, `db-tests`, `dependency-review`, `supply-chain-vet`, `e2e-browser`. Sem auto-merge, force-push ou aprovação simulada.
- PR **#294** `MERGED` às `04:44:19Z` (merge commit `407d26ddce414ad88e3eb6daf0e6bf104dfdf029`) — já registrado na nota de waiver; citado aqui para completar a cadeia de fechamento.
- Âncora final: `dev` local = `github/dev@c16be5c30`; working tree limpa; `gh pr list --state open` → `[]`.
- Gate autoritativo `python3 scripts/ci/check_gate_integrity.py` PASS às `04:50:34Z` (4 required checks, 8 pisos / 51).
- A nota `04:36:59Z` permanece como registro do momento em que o merge ainda não havia ocorrido; esta nota a sucede por acréscimo, sem reescrita.
- Dívidas: leg `rbac` no CI (aprov maintainer), emenda constitucional (maintainer), 10 títulos sem prefixo de ciclo em `docs/research/index.md` (exigem inferência), duplicatas T-ID legadas (histórico protegido); CI futuro, Forgejo e produção: **[NV]**.

## Execução e entrega — 2026-11-19T04:58:26Z (ciclo 074; append-only)

- Decisão 80/20 `(2×5)/(1×1)=10.0` — fechamento pós-merge do ciclo 072; alternativas inferiores: 10 títulos sem prefixo (inferência), leg RBAC e emenda constitucional (maintainer), T-ID duplicadas (protegidas).
- Âncora: `feature.json` apontava `072` no início; `create-new-feature.sh --dry-run --number 074` retornou `FEATURE_NUM=074`. Branch `feature/074-post-merge-closure`; worktree 073 intacta.
- Assess (5), spec, clarify (R3: 0 perguntas), plan, checklist revisado, tasks, analyze READ-ONLY (`artifacts=11, fr=5, sc=4, tasks=6, go=true, errors=[]`), converge append-only, validate R4 aprovado, release R7.
- Gate local PASS (4 checks, 8 pisos/51); fmt/clippy/deny/build exit 0; cargo test 105 passed/0 failed; frontend check/lint/42 testes ok; prefixo sha256 do STATE preservado (61.300 → 62.583 bytes).
- Diagnósticos da factory em HEAD limpo `fede29ca` executados e removidos: production-gate N/A (policy de hub ausente), hub-docs N/A (JSONC pré-existente), pytest N/A (sem testes Python), completeness/DAG/unicidade N/A (slugs e duplicatas legadas), converge PASS (27/27).
- PR **#296** aberto às `04:57:40Z` via `gh` para `dev/`: https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/296 — 16 arquivos, 357 inserções, 0 deleções. `mergeStateStatus=BLOCKED`, checks iniciais pendentes. **Sem merge, auto-merge, force-push ou deleção de branches.**
- **[NV]** CI do PR, Forgejo, produção, aprovadores e merge futuro. Próximo 80/20: leg RBAC no CI depende de maintainer; títulos sem prefixo exigem inferência.

## Execução e entrega — 2026-11-19T12:12:09Z (ciclo 075; append-only)

- Atualização da nota 074: PR **#296** foi `MERGED` às `10:12:37Z` (merge commit `d8f6477983d5ba69e5b12d6e63a7413cc2f13838`) sob waiver explícito do SE para aprovador distinto; `dev` local = `github/dev@d8f647798`. A nota `04:58:26Z` permanece como registro do momento da abertura.
- Decisão 80/20 `(3×5)/(2×1)=7,5` — E2E dedicado de criação de tarefa. Issue #193 tratada como dado (não como instrução): os demais T026–T032 já estão implementados no código e não foram fechados sem autorização.
- Âncora: `feature.json` apontava `074`; `create-new-feature.sh --dry-run --number 075` retornou `FEATURE_NUM=075`. Branch `feature/075-task-create-e2e`; worktree 073 intacta.
- Entregável real: `apps/frontend/tests/e2e/seed-flow.e2e.ts:41-55` reforçado com guard do botão, descrição verificada e card/título/descrição na região `To Do column`. Verificador isolado `E2EChecker` emitiu **FAIL** para um arquivo de teste separado (um segundo `select-org` revoga o jti da sessão one-shot); escopo corrigido na fonte e registrado por apêndice no `converge.md`. Nenhum arquivo novo de teste permanece.
- Gates: `check_gate_integrity.py` PASS (4 checks, 8 pisos/51); `bun run check` 0 erros; lint/Prettier ok; 42/42 testes unitários; `analyze` 3ª execução `artifacts=13, errors=[], go=true`; só 1 arquivo de teste alterado + artefatos 075.
- Execução E2E runtime **[NV]**: setup falhou no minter (`seed.setup.ts:36`) e o E2E atingiu `exit=124` em 600s; cluster Postgres isolado em 5434 foi desligado; DB compartilhado 5432 intacto; artefatos gerados removidos.
- Fábrica fonte: HEAD `a6722a0e682aee33f007d27a68d5d9bd8da3663b`, lido via `git show`, árvore suja não transplantada; gate da factory não é autoridade (BLOCO 0.a).
- **[NV]** CI do PR, Forgejo, produção, aprovadores e merge deste ciclo. PR será aberto via `gh` sem merge, auto-merge ou deleção de branches.

## Fechamento pós-merge — 2026-11-19T12:28:07Z (ciclo 075; append-only)

- PR **#297** `MERGED` ao vivo às `12:27:33Z`; merge commit `e9a943c6a9aa16bd95420af0ac58bbfa2a135669`. `mergeStateStatus=CLEAN` observado às `12:22:45Z`, após rerun do job `e2e-browser`: a falha anterior foi de infraestrutura (apt mirror na instalação do Chromium), não do teste. Required checks `frontend-build`, `rust-build`, `db-tests`, `dependency-review` pass.
- Waiver explícito do SE para aprovador distinto (único colaborador é o autor) registrado na mensagem do merge; sem `--admin`, `--auto`, `--delete-branch`, force-push ou aprovação simulada.
- A nota `12:12:09Z` permanece como registro do momento da abertura; esta a sucede por acréscimo, sem reescrita.
- Âncora final: `dev` local = `github/dev@e9a943c6a`; zero PRs abertos. Worktree/branch `feature/073-state-closure` preservada.
- **[NV]** CI futuro, Forgejo, produção e aprovações. Próximo 80/20: leg `rbac` no CI depende de maintainer; candidato autônomo é diagnosticar o travamento do `flora-mint-session --features dev-mint` que bloqueia o E2E runtime local.

## Execução e entrega — 2026-11-19T19:16:22Z (ciclo 076; append-only)

- Atualização das notas 074/075: PR **#298** (nota de fechamento do ciclo 075) foi `MERGED` às `12:40:12Z`; merge commit `f22a2a32c...`; `dev` local = `github/dev@f22a2a32c`. As notas anteriores permanecem como registro de cada momento.
- Decisão 80/20 `(4×5)/(2×2)=5.00` — pré-compilar o `flora-mint-session --features dev-mint` no `globalSetup` do Playwright. Três scouts em paralelo provaram: runtime 0.00s/0.39s com target quente, build 30.18s com cache, timeout de 180s em `seed.setup.ts:14` e pré-build do CI em `rust.yml:192-195` (`e2e-browser` é `continue-on-error` em `:150`).
- Âncora: `feature.json` apontava `075`; `--dry-run --number 076` retornou `FEATURE_NUM=076`. Branch `feature/076-mint-session-hang`; worktree 073 intacta; factory HEAD `b1deab15...` lido via `git show`, sujo/não transplantado.
- Entregável: `apps/frontend/playwright.config.ts:10` + novo `apps/frontend/playwright/global-setup.ts`. `seed.setup.ts`, workflows, Cargo.toml, código de produção, ADRs, constituição e DB compartilhado intactos.
- Verificação: gate PASS (4 checks, 8 pisos/51); fmt/clippy/deny/build/test exit 0 (105 passed); frontend check 0 erros, lint exit 0 após correção de `_c`, 42/42; `playwright --list` 4 testes; `GLOBAL_SETUP_OK`; `analyze` 1ª execução `errors=[]`; verificador isolado PASS; assess/spec/plan/checklist/tasks/converge/validate/release completos.
- PR **#299** aberto às `19:16:10Z` via `gh` para `dev/`: https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/299 — 17 arquivos, 463 inserções. Sem merge, auto-merge, force-push ou deleção de branches.
- **[NV]** tempo com target totalmente frio, CI do PR, Forgejo, aprovações, merge e produção. Próximo 80/20: leg `rbac` depende de maintainer; títulos sem prefixo no índice de research exigem inferência.

## Fechamento pós-merge — 2026-11-19T19:30:08Z (ciclo 076; append-only)

- PR **#299** `MERGED` ao vivo às `19:29:42Z`; merge commit `675988e31a09f17419cf8c56aafe89661b5752ab`. `mergeStateStatus=CLEAN` observado; todos os checks passaram, inclusive `e2e-browser` nas duas execuções — evidência ao vivo de que o `globalSetup` pré-aqueceu o binário no CI.
- Waiver explícito do SE para aprovador distinto registrado na mensagem do merge; sem `--admin`, `--auto`, `--delete-branch`, force-push ou aprovação simulada.
- A nota `19:16:22Z` permanece como registro do momento da abertura; esta nota a sucede por acréscimo, sem reescrita.
- Âncora final: `dev` local = `github/dev@675988e31`; zero PRs abertos; worktree/branch `feature/073-state-closure` preservada.
- **[NV]** CI futuro, Forgejo, produção e aprovações. Próximo 80/20: leg `rbac` no CI depende de maintainer; 10 títulos sem prefixo no índice de research exigem inferência e ficam abaixo na fila.

## Execução e entrega — 2026-11-19T19:57:39Z (ciclo 077; append-only)

- Atualização da nota 076: PR **#300** (nota de fechamento) foi `MERGED` às `19:41:26Z`; merge commit `cc421e42d20a2d78efc2918d7e603a05d714858e`; `dev` local = `github/dev@cc421e42d`. A nota `19:30:08Z` permanece como registro daquele momento.
- Stack normativa `~/development/MDMV/STACK_TECNOLOGICA_MDMV.md` **adotada** como critério: Postgres único canônico, SQLite removido (§1/§4). Auditoria com 3 scouts em paralelo verificou convergência (sqlx/Postgres/PG18/uuidv7/traits/Svelte 5/Tailwind 4/shadcn-svelte/OIDC) e gaps (OTLP, Tauri, pgmq/RustFS/Meilisearch, Zitadel e2e, `sqlite` residual). Promoção do repo fora de `Arquivados/` é governança fora do alvo, escalada **[NV]**.
- Decisão 80/20 `(2×5)/(1×1)=10,0` — remover a feature `sqlite` morta. Alternativas: Tauri 3,75 (governança), GIN-JSONB 2,00, OTLP 1,67, leg RBAC bloqueada por maintainer.
- Entregável: `flora-core/Cargo.toml` (1 linha) + `Cargo.lock` (1 aresta `cc` de `libsqlite3-sys`, consequência determinística); `cargo metadata --locked --offline` passou após o update. Zero consumidores da feature; `sqlite_path`/`is_sqlite()` permanecem fora de escopo.
- Verificação: gate PASS; tomllib sem `sqlite`; grep zero; `cargo tree` sem `sqlx-sqlite`; `cargo check -p flora-core --offline` exit 0; `analyze` 2ª execução `artifacts=12, errors=[], fr=4, sc=4, tasks=4, go=true`; verificador isolado `SqliteVerifier`: PASS-WITH-NITS (nit do lock corrigido na fonte); R4 aprovado; R7 `0.2.7→0.2.8`.
- PR **#301** aberto às `19:57:20Z` via `gh` para `dev/`: https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/301 — 17 arquivos, 487 inserções/2 deleções. Sem merge, auto-merge, force-push ou deleção de branches.
- **[NV]** extensões de arquivo não varridas, necessidade offline real, CI do PR, Forgejo, aprovações e merge. Próximo 80/20: leg RBAC depende de maintainer; títulos sem prefixo no índice de research exigem inferência.

## Execução e entrega — 2026-11-19T20:40:52Z (ciclo 078; append-only)

- Promoção executada por autorização do SE: repo movido de `projetos/Arquivados/flora` para `projetos/flora`; ponteiros da worktree `feature/073-state-closure` reparados nos dois sentidos; `git worktree list` íntegro; gate PASS no novo caminho. `projetos/AGENTS.md` do HQ atualizado localmente (flora em Active repos; arquivo é ignorado pelo root do HQ, logo sem commit). PR **#301** (ciclo 077) foi `MERGED` às `20:09:26Z`; merge commit `de141fc53`.
- Atualização da nota 077: era "./../Arquivados/" — superada pela promoção acima; a nota `19:57:39Z` permanece como registro histórico.
- Decisão 80/20 `(2×5)/(1×1)=10,0` — corrigir `apps/frontend/components.json` para Tailwind v4: remover `tailwind.config` (deprecado no schema oficial, ponteiro para arquivo inexistente) e apontar `css` para `src/routes/layout.css` (CSS real, já referenciado em `prettier.config.js:11`). Fontes primárias: `https://shadcn-svelte.com/docs/components-json` e `https://shadcn-svelte.com/schema.json`, acesso em 2026-11-19.
- Entregável: 1 arquivo de config alterado (1 linha removida + 1 valor trocado) + 15 artefatos do ciclo; `$schema`, `style`, `baseColor`, `cssVariables` e `aliases` byte-idênticos. Zero consumidores locais; nada de dependência, workflow, gate, DB ou rede.
- Verificação: `json.load` ok; Prettier/check/lint ok; 42/42 testes unitários; gate PASS; `analyze` `artifacts=11, errors=[], fr=4, sc=4, tasks=4, go=true`; verificador isolado `ComponentsVerifier`: **PASS**; R4 aprovado; R7 `0.2.8→0.2.9`.
- PR **#302** aberto às `20:40:37Z` via `gh` para `dev/`: https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/302 — 16 arquivos, 559 inserções/2 deleções. Sem merge, auto-merge, force-push ou deleção de branches.
- **[NV]** validação pelo CLI `shadcn-svelte` (não instalado), CI do PR, Forgejo, produção e aprovações. Próximo 80/20: leg RBAC depende de maintainer; OTLP/Tauri exigem ADR e aprovador.

## Execução e entrega — 2026-11-19T21:20:31Z (ciclo 079; append-only)

- Atualização da nota 078: PR **#302** foi `MERGED` às `20:47:47Z`; merge commit `ea5cd2069`; `dev` local = `github/dev@ea5cd2069` antes deste ciclo. A nota `20:40:52Z` permanece como registro da abertura.
- Pedido do SE: instalar `shadcn-svelte`. Registry primário (npm/bun, 2026-11-19): `1.8.1`, MIT, bin `shadcn-svelte`, 4 deps. Docs oficiais (`installation/sveltekit`, `cli`, `components-json`, acesso 2026-11-19): CLI é efêmero via `bun x ...@latest`; `init` é para projeto novo e sobrescreve CSS/components.json — portanto **não executado**.
- Decisão 80/20 `(3×5)/(2×1)=7,50` — instalar devDependency exata `shadcn-svelte@1.8.1` via `bun add -D -E shadcn-svelte@1.8.1` em `apps/frontend`, sem `init`/`add`/`apply`.
- Entregável: `apps/frontend/package.json` +1 linha e `apps/frontend/bun.lock` +9 linhas (pacote + `cn`, `commander`, `node-fetch-native`, `postcss`). `components.json`, `layout.css`, 51 componentes UI e `utils.ts` byte-idênticos ao base.
- Verificação: `bun pm ls shadcn-svelte` → 1.8.1; `bun run shadcn-svelte --help` exit 0 (nenhum subcomando executado); `bun install --frozen-lockfile --dry-run` exit 0; Prettier/check/lint/42 testes/build green; gate PASS; `analyze` `artifacts=11, errors=[], fr=5, sc=4, tasks=5, go=true`; verificador isolado `ShadcnVerifier`: **PASS**; R4 aprovado; R7 `0.2.9→0.3.0`.
- PR **#303** aberto às `21:20:20Z` via `gh` para `dev/`: https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/303 — 17 arquivos, 625 inserções. Sem merge, auto-merge, force-push ou deleção de branches.
- **[NV]** uso futuro do CLI, CI do PR, Forgejo, produção e aprovações. Próximo 80/20: leg RBAC depende de maintainer; OTLP/Tauri exigem ADR e aprovador.

## Execução e entrega — 2026-11-19T21:41:35Z (ciclo 080; append-only)

- Atualização da nota 079: PR **#303** foi `MERGED` às `21:27:18Z`; merge commit `e3944e962`; `dev` local = `github/dev@e3944e962` antes deste ciclo. A nota `21:20:31Z` permanece como registro da abertura.
- Auditoria com 3 scouts em paralelo ranqueou: flag OIDC morta **15,0**; `.env.example` obsoleto 10,0; `cssVariables`/docs 5,0; duplicação lucide 2,50; OTLP 1,25 (registry 0.33.1 compatível, mas falta receptor/dono G1 e há risco de `multiple_crate_versions = deny` + `dependency-review`). leg RBAC/Tauri dependem de maintainer.
- Entregável: `flora-core/src/config/oidc.rs` −3 linhas (doc + campo `local_fallback_enabled` + default `true`), zero leitores em Rust, sem `deny_unknown_fields` (env legada ignorada por serde). `.env.example`, auth, ADRs, workflows, gate e lockfile intactos.
- Verificação: `grep` zero; fmt/check/clippy/deny/build/test (105 passed/0 failed) + gate PASS; `analyze` `artifacts=11, errors=[], fr=4, sc=4, tasks=4, go=true`; verificador isolado `OidcFlagVerifier`: **PASS**; R4 aprovado; R7 `0.3.0→0.3.1`.
- PR **#304** aberto às `21:41:25Z` via `gh` para `dev/`: https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/304 — 16 arquivos, 611 inserções/3 deleções. Sem merge, auto-merge, force-push ou deleção de branches.
- **[NV]** consumidores externos de config serializada, CI do PR, Forgejo, produção e aprovações. Próximo 80/20: `.env.example` com chaves OIDC sem leitor (10,0) e `cssVariables` inválido no schema (5,0), ambos autônomos; leg RBAC depende de maintainer.

## Execução e entrega — 2026-11-19T22:20:35Z (ciclo 081; append-only)

- Atualização da nota 080: PR **#304** foi `MERGED` às `22:00:12Z`; merge commit `35b8d3142`; `dev` local = `github/dev@35b8d3142` antes deste ciclo. A nota `21:41:35Z` permanece como registro da abertura.
- Decisão 80/20 `(2×5)/(1×1)=10,0` — limpar chaves OIDC obsoletas em `.env.example`: remover `FLORA_OIDC__REDIRECT_URI` (sem campo em `OidcConfig`, silenciosamente descartada) e `# OIDC_ISSUER_URL` (sem leitor Rust); manter `FLORA_OIDC__ISSUER_URL/CLIENT_ID/CLIENT_SECRET` e `ZITADEL_*` (lidos em `OidcConfig::default()`).
- Entregável: `.env.example` +4/−4 — comentário com `FLORA_APP__BASE_URL` como origem do callback e exemplo comentado de `FLORA_OIDC__ALLOWED_REDIRECT_URIS` em sintaxe array do Figment 0.10.19; nenhum arquivo Rust/workflow/gate/lockfile tocado.
- Verificação: grep das chaves coerente; `cargo check -p flora-core` exit 0; gate PASS; `analyze` `artifacts=11, errors=[], fr=4, sc=4, tasks=4, go=true`; verificador `EnvVerifier`: PASS-WITH-NITS, nit `FLORA_APP__BASE_URL` corrigido por `EnvNitFix` e confirmado por `FinalRead`; R4 aprovado; R7 `0.3.1→0.3.2`.
- PR **#305** aberto às `22:20:23Z` via `gh` para `dev/`: https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/305 — 16 arquivos, 672 inserções/4 deleções. Sem merge, auto-merge, force-push ou deleção de branches.
- **[NV]** consumidores externos do template, CI do PR, Forgejo, produção e aprovações. Próximo 80/20: `cssVariables` inválido no schema do `components.json` (5,0) e duplicação lucide (2,50); leg RBAC depende de maintainer.

## Execução e entrega — 2026-11-19T22:56:36Z (ciclo 082; append-only)

- Atualização da nota 081: PR **#305** foi `MERGED` às `22:27:10Z`; merge commit `8f24519fe`; `dev` local = `github/dev@8f24519fe` antes deste ciclo. A nota `22:20:35Z` permanece como registro da abertura.
- Decisão 80/20 `(2×5)/(1×1)=5,0` — remover `cssVariables` de `tailwind` em `apps/frontend/components.json`: chave ausente no schema oficial (`https://shadcn-svelte.com/schema.json`, acesso 2026-11-19; `additionalProperties=false`) e descartada pelo parser `$strip` do CLI 1.8.1. `style` mantido (declarado no schema, apenas deprecado em v4).
- Entregável: `apps/frontend/components.json` +1/−2; `$schema`, `style`, `tailwind.css` e `aliases` byte-idênticos. Nenhum arquivo Rust/workflow/gate/lockfile/`node_modules` tocado.
- Verificação: `json.load` → `tailwind={css,baseColor}`; grep zero; Prettier/check/lint/42 testes/build green; gate PASS; `analyze` `artifacts=11, errors=[], fr=4, sc=4, tasks=4, go=true`; verificador isolado `CssVerifier`: **PASS** sem findings; R4 aprovado; R7 `0.3.2→0.3.3`.
- PR **#306** aberto às `22:56:21Z` via `gh` para `dev/`: https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/306 — 16 arquivos, 664 inserções/2 deleções. Sem merge, auto-merge, force-push ou deleção de branches.
- **[NV]** validação online do schema, CI do PR, Forgejo, produção e aprovações. Próximo 80/20: duplicação `lucide-svelte`/`@lucide/svelte` (2,50); leg RBAC depende de maintainer; OTLP depende de receptor/dono (G1).

## Execução e entrega — 2026-11-19T23:38:51Z (ciclo 083; append-only)

- Atualização da nota 082: PR **#306** foi `MERGED` às `23:03:00Z`; merge commit `33d174095`; `dev` local = `github/dev@33d174095` antes deste ciclo. A nota `22:56:36Z` permanece como registro da abertura.
- Herança de branches auditada: todos os 13 branches locais exceto `dev` estão integrados em `dev` (PRs #294–#306 mesclados, zero PRs abertos); nada a recuperar. Worktree `feature/073-state-closure` preservada.
- Decisão 80/20 `(1×5)/(1×1)=5,0` — refresh verificado de `docs/PROGRESSO.md` e `docs/research/index.md` por apêndices datados (`2026-11-19T23:12Z`), preservando cabeçalhos e notas históricas; contagens 72/69/68/68/4, research 137 sem índice (138 com), ADR 6; gate PASS 23:11Z.
- Verificação: `+8/−0` e `+4/−0`, prefixos byte-idênticos por `cmp`, 137/137 links `missing=[]`/`extra=[]`, gate PASS; `analyze` `artifacts=11, errors=[], fr=4, sc=4, tasks=4, go=true`; verificador `DocsVerifier`: **PASS**; R4 aprovado; R7 `0.3.3→0.3.4`.
- PR **#307** aberto às `23:38:40Z` via `gh` para `dev/`: https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/307 — 17 arquivos, 706 inserções. Autorização de merge fornecida pelo SE neste prompt; conduzir após checks verdes.
- **[NV]** remoto/CI/PR/aprovações/produção nos apêndices; resolver ao vivo o que for verificável (gh/API) e manter [NV] o que depender de produção/maintainer. Próximo 80/20: duplicação lucide (2,50) — único candidato autônomo restante; leg RBAC/Tauri/OTLP/emenda dependem de maintainer/SE.

## Execução e entrega — 2026-11-20T00:25:05Z (ciclo 084; append-only)

- Atualização da nota 083: PR **#307** foi `MERGED` às `23:44:58Z`; merge commit `9da1aaea6`; `dev` local = `github/dev@9da1aaea6` antes deste ciclo. A nota `23:38:51Z` permanece como registro da abertura.
- [NV] parcialmente resolvido ao vivo: `gh api` confirmou required contexts `frontend-build/rust-build/db-tests/dependency-review` em `dev` e `main`; `fj repo view` confirmou Forgejo `0 issues/0 PRs/0 releases`; `git ls-remote origin` mostrou mirror defasado (`dev@f12a328c`). Produção/aprovações/CI futuro seguem [NV].
- Decisão 80/20 `(1×5)/(2×1)=2,50` — consolidar ícones Lucide em `@lucide/svelte@1.33.0` (pin aprovado; latest 1.76.0 registrado como upgrade [NV]): 3 imports/14 símbolos migrados, `lucide-svelte` removido de `package.json`/`bun.lock`/`package-lock.json` sem churn.
- Verificação: grep zero, `bun pm ls` → 1.33.0, frozen-lockfile/check/lint/42 testes/build/gate green; `analyze` `artifacts=11, errors=[], fr=5, sc=4, tasks=5, go=true`; `LucideVerifier`: PASS-WITH-NITS (churn do lock corrigido por `LockFix`), `DiffCheck`: 6 paths/14 símbolos/4 componentes `/icons/*` intactos; R4 aprovado; R7 `0.3.4→0.3.5`.
- PR **#308** aberto às `00:24:49Z` via `gh` para `dev/`: https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/308 — 21 arquivos, 772 inserções/21 deleções. Conduzir merge após checks verdes, sob autorização do prompt.
- Encerramento do backlog autônomo: após este ciclo não restam candidatos seguros executáveis sem maintainer — leg RBAC (6,25), emenda constitucional (7,50), Tauri (3,75) e OTLP (1,25–1,67) dependem de aprovação/receptor; duplicatas T-ID são histórico protegido. Manter [NV] para remoto/produção/aprovações.

## Execução e entrega — 2026-11-20T03:27:30Z (ciclo 085; append-only)

- Âncora real: `.specify/feature.json` → `specs/085-local-first-seed`; branch `feature/085-local-first-seed` criada a partir de `dev@38184fa50`. HEAD da factory usado como fonte do aparato: `ee71d8db0a2ba34ef9fc9486f8d18915282beda1` (HEAD atual `b1deab15f`, sujo, não transplantado).
- Gate autoritativo do alvo (Bloco 0.a): `python3 scripts/ci/check_gate_integrity.py` → **PASS** (4 required contexts, 8 pisos/51 contratos). Provider GitHub Actions; CLI `gh`. `check_production_gate.py`, `hub_docs_gate.py` e `pytest`: **N/A** no alvo.
- Herança (Bloco 0.b): zero PRs abertos antes do ciclo; 14 branches já integradas em `dev`; worktree `feature/073-state-closure` preservada e limpa; `origin/dev` (Forgejo) é espelho defasado — não usado para merge.
- Decisão 80/20: única pendência autônoma era o First Seed Journey inutilizável sem IdP (erro `redirect_uri is not allowed` silencioso, depois visível). Implementado `POST /api/v1/bootstrap` one-time, atômico, com guarda de organizações não-sistema, emissão de sessão e UI ligada ao botão "Sprout Workspace".
- Verificação: baseline TDD vermelho (módulo ausente) → 4 contratos DB + 4 Vitest verdes; `fmt`, `clippy -D warnings`, `cargo test --workspace` (105/59), `svelte-check`, `lint`, `build` e gate PASS. E2E real: 201 → 409 → `/tasks` 200; UI: bootstrap → `Main Workspace` → tarefa criada. Verificador isolado: 3 achados corrigidos (prefixo da rota, topologia de teste, `sub` no contrato).
- R4 aprovado em `.specify/validations/085-local-first-seed/validate.md`; R7 `0.3.5→0.4.0` com changelog/rollback em `.specify/releases/085-local-first-seed/release.md`; `converge.md` sem novas tarefas. 23/23 tarefas marcadas.
- Commits: `9b43766531` (código/testes) e `b6b35b3db2` (documentação/artefatos/aparato). PR **#309** aberto às `03:26:56Z` via `gh` para `dev/`: https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/309 — sem merge, auto-merge, force-push ou deleção.
- Scripts SDD da factory executados com `--root .`: `FAIL` por dívida legada em 75 specs (não introduzida neste ciclo); gate do alvo PASS. Aviso do push sobre 29 vulnerabilidades Dependabot no branch default é preexistente **[NV]**.
- **[NV]** CI do PR, aprovação de maintainer, merge, Forgejo, produção, verificação de e-mail (FR-012 aceita só em instalação vazia), carga/concorrência em produção. Próximo 80/20: nenhum candidato autônomo seguro restante; RBAC no CI, emenda constitucional, Tauri e OTLP dependem de maintainer/SE.

## Revalidação determinística — 2026-11-20T09:34:46Z

- Blocos 0.a/0.b/1/2/3 reexecutados: alvo ativo confirmado (`projetos/flora`, cwd decoy sem `.specify/`), gate PASS, codegraph sincronizado, 2 candidatos Archify válidos e nenhum HTML stale.
- PR **#309** `OPEN`, `MERGEABLE`, 12 check-runs concluídos (11 success + 1 skipped), `reviewDecision` vazio, zero reviews. Proteção `dev` via `gh api`: 4 checks obrigatórios, `enforce_admins=true`, `required_pull_request_reviews=null`.
- Decisão: **sem merge**. A policy explícita deste prompt exige aprovador distinto e revisão deliberada; isso não é observável com zero reviews, e autor não pode aprovar o próprio PR. Nenhum auto-merge/force-push/deleção.
- `[NV]` parcialmente resolvido: CI checks e proteção de `dev` observados ao vivo via `gh`. Permanecem `[NV]`: aprovação de maintainer, merge/produção, Forgejo, segurança da cadeia de dependências e carga.
- Contagens: specs 75, assessments 72, validations 71, releases 71, bugs 4, research 137/138. Scripts SDD da factory seguem `FAIL` por baseline legada (não regressão 085). Próximo 80/20: nenhum candidato autônomo seguro; depende de maintainer/SE.

## Decisão do SE — merge autônomo sob CI/CD GREEN — 2026-11-20 (append-only)

- **Decisão (SE, 2026-11-20):** substituída a regra "Merge requer check obrigatório verde observado, aprovador distinto e revisão deliberada..." por "O CI/CD GREEN deve nos fornecer confiança suficiente para fazer merge para dev/ de forma autônoma sem necessidade de aprovação. Caso o CI/CD não inspire confiança, o próximo 80/20 IMEDIATO é reforçar o CI/CD até garantir confiança para conduzir o merge para dev/ sem necessidade de aprovação." Registrada como Emenda 2026-11-20 em `docs/adr/0003-autonomous-merge-under-deterministic-gates.md` (Status `Accepted` inalterado).
- **Prevalência:** a nova regra prevalece sobre qualquer cláusula anterior de aprovador distinto/revisão deliberada. A decisão "sem merge" da revalidação `2026-11-20T09:34:46Z` (linha 702) e exigências equivalentes em `docs/PROGRESSO.md` seguem como histórico, mas não são mais autoritativas para decisões de merge.
- **Âncora:** branch `feature/085-local-first-seed` @ HEAD `2a9b7e76b`; PR **#309** — https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/309 — 12 check-runs concluídos (11 success + 1 skipped), zero reviews ao registrá-la.
- **Consequência operacional:** com CI/CD GREEN observado, conduzir merge autônomo para `dev/` sem aguardar aprovação de aprovador distinto/revisão deliberada. Se o CI/CD não inspirar confiança, o próximo 80/20 IMEDIATO é reforçar o CI/CD até inspirar confiança — não escalar para aprovador humano.
- **Escopo:** apenas documentação de governança/estado; nenhum código, workflow, gate ou check alterado; nenhum merge/auto-merge realizado nesta nota.

## Merge autônomo #309 — 2026-11-20T09:49:31Z

- PR **#309** mesclado em `dev/` via `gh pr merge 309 --merge`, sob a nova regra de CI/CD GREEN (sem aprovador distinto). Merge commit: `f350b2774a071603428bd5340551813ca6aa6634`. `dev` local = `github/dev@f350b2774` após `git merge --ff-only`.
- Evidência pré-merge: 4 runs do HEAD `c7d9668f2` concluídos com sucesso; 2 execuções de cada required check (`frontend-build`, `rust-build`, `db-tests`, `dependency-review` — uma `success`, uma `skipping` PR-only), `e2e-browser` e `supply-chain-vet` informativos verdes; `gh api branches/dev/protection` com `strict=true`, 4 contexts e `enforce_admins=true`; `check_gate_integrity.py` PASS.
- Nenhum auto-merge, force-push, deleção de branch ou approval fabricado. Branch `feature/085-local-first-seed` preservada.
- **[NV]** restante: produção/Forgejo, segurança de dependências e aprovações fora do gate; próximo 80/20 continua sem candidato autônomo seguro além de reforçar CI/CD se ele deixar de inspirar confiança.

## Ciclo 086 — reforço do CI/CD (leg RBAC) — 2026-11-20T10:30Z

- **Âncora:** `.specify/feature.json` → `specs/086-rbac-ci-leg`; branch `feature/086-rbac-ci-leg` a partir de `dev@9b05abb9d`. Factory-fonte `ee71d8db` (HEAD atual `b1deab15f`, sujo, não transplantado).
- **Gap fechado:** 3 contratos `#[ignore]` em `flora-tests/src/contract/rbac.rs` nunca rodavam no CI. Adicionado `run_leg rbac contract::rbac::db::db_ 3` em `.github/workflows/rust.yml:115` e `"rbac": 3` em `scripts/ci/check_gate_integrity.py:51`.
- **Correção durante verificação:** primeiro filtro `contract::rbac::db_` casou 0 testes; substituído por `contract::rbac::db::db_` após `cargo test -- --list`. Leg local: **3 passed**. Gate: **9 legs / 54 pisos**, PASS.
- Gate autoritativo PASS, `cargo test --workspace` verde, `git diff --check` limpo; artefatos R1/R4/R7 criados (decision go, spec, plan, tasks 6/6, validate aprovado, release `0.4.0→0.4.1`). Sem código de produto tocado.
- Próximo: PR, CI green e merge autônomo sob ADR-0003 emendada; sem aprovação humana. **[NV]** produção/Forgejo/dependências.

## Atualização — ciclo 087 (2026-11-20T10:50Z; append-only)

- **Âncora:** `.specify/feature.json` → `specs/087-bootstrap-ci-leg`; branch `feature/087-bootstrap-ci-leg` @ `367a50e79`. Identidade resolvida por `feature.json`, nunca por nome de branch. CI/remoto ao vivo: **[NV]**.
- **Gap P0 fechado (fora do gate até este ciclo):** 4 contratos `#[ignore]` em `flora-tests/src/contract/bootstrap.rs` (`contract::bootstrap::tests::db_`) nunca eram exercitados no CI. Adicionado `run_leg bootstrap contract::bootstrap::tests::db_ 4` em `.github/workflows/rust.yml` e `"bootstrap": 4` em `scripts/ci/check_gate_integrity.py`.
- **Diff mínimo:** 2 arquivos / 2 linhas alteradas (`+1` em cada, `0−`); nenhum teste, gate adicional ou código de produto tocado; `git diff --check` limpo.
- **Correção do filtro após mismatch de regex:** o filtro longo `...::db_bootstrap_` casava 0 testes e não satisfazia a regex do guarda (`:118`); substituído por `contract::bootstrap::tests::db_`, que cobre os mesmos 4 testes e casa a regex existente sem relaxá-la. `cargo test -p flora-tests -- --list | grep 'contract::bootstrap::tests::db_' | wc -l` → 4.
- **Evidência:** gate `all 10 db contract floors intact (58 total)` → exit 0; leg `cargo test -p flora-tests contract::bootstrap::tests::db_ -- --ignored --test-threads=1` → 4 passed/4. R4 aprovado; R7 registrado.
- **Contagens (snapshot 2026-11-20T10:50Z, antes da materialização dos artefatos 087):** `specs/` 76, `.specify/assessments/` 73, `.specify/validations/` 72, `.specify/releases/` 72, `.specify/bugs/` 4, `docs/research/` 137 sem `index.md` (138 com ele). Histórico preservado; cifras anteriores são do momento da medição; nenhum validation/release reescrito.
- **[NV]** CI do PR, merge, produção/Forgejo e aprovações ao vivo: reconsultar antes de qualquer decisão de merge.

## Atualização — ciclo 088 (2026-11-20T11:06Z; append-only)

- **Âncora:** `.specify/feature.json` → `specs/088-dynamic-gate-oracle`; branch `feature/088-dynamic-gate-oracle` @ `cbc885383`. Identidade resolvida por `feature.json`, nunca pelo nome do branch. Estado remoto/CI/PR ao vivo: **[NV]**.
- **Causa raiz recorrente:** o guarda `scripts/ci/check_gate_integrity.py` era um espelho entre `FLOORS` e as legs `run_leg` do `rust.yml` — comparava duas listas escritas à mão e não via a workspace de testes. Contratos `#[ignore]` esquecidos em 086 (RBAC) e 087 (bootstrap) passavam batido; um terceiro *confidence gap* seria inevitável.
- **Oráculo dinâmico:** `LEG_RE` extrai as 10 invocações `run_leg <label> <filter> <floor>`; o subprocess `cargo test -p flora-tests -- --ignored --list` (apenas stdout para nomes de testes) fornece a realidade; contagem exata por `startswith(filter_)`, uma leg por label, piso igual à contagem descoberta. Qualquer divergência — rc != 0, lista vazia, label fora de `FLOORS`, piso diferente da contagem, leg ausente/duplicada — falha fechado.
- **Diff mínimo:** 1 arquivo, `scripts/ci/check_gate_integrity.py`, `+59/−5`; nenhum workflow, teste, config ou código de produto tocado.
- **Evidência local:** gate dinâmico `python3 scripts/ci/check_gate_integrity.py` → **PASS** `all 10 db contract floors intact (58 total)`, exit 0; `python3 -m py_compile scripts/ci/check_gate_integrity.py` → exit 0. R4/R7: artefatos do ciclo 088 em materialização no snapshot 11:06Z; nenhum validation/release pré-existente tocado.
- **Contagens (snapshot 11:06Z, antes da materialização dos artefatos 088):** `specs/` 77, `.specify/assessments/` 74, `.specify/validations/` 73, `.specify/releases/` 73, `.specify/bugs/` 4, `docs/research/` 137 sem `index.md` (138 com ele). `find` no checkout confirma 78/75 (incluindo spec/assessment 088 em curso), 73/73/4 e research 138 = 137 + `index.md`. Histórico preservado; cifras anteriores são do momento da medição.
- **[NV]** CI do PR, merge, produção/Forgejo e aprovações ao vivo: reconsultar antes de qualquer decisão de merge.

## Atualização — ciclo 089 (2026-11-20; append-only)

- **Âncora:** branch `feature/089-forgejo-mirror-sync` @ `15203ee78`; `.specify/feature.json` → `specs/089-forgejo-mirror-sync`. Estado remoto/Forgejo/CI ao vivo: **[NV]**; todas as notas anteriores intactas. Registro canônico em `docs/governance/2026-11-20-089-decisoes-governanca.md` (novo, append-only).
- **(a) Gate autoritativo — decisão explícita do SE:** os 4 scripts SDD da factory (`check_sdd_completeness.py`, `check_converge_append_only.py`, `check_task_id_uniqueness.py`, `check_work_order_dag.py`) **NÃO** são gate do alvo; falham por baseline legada da hub (75 specs), não são regressão. Gate é `python3 scripts/ci/check_gate_integrity.py` (oráculo dinâmico 088, 10 legs / 58 pisos). Nenhum script SDD promovido a required check.
- **(b) Branch protection — escopo mínimo:** PAT fine-grained `Administration: read` **somente** no repo `flora`; `GITHUB_TOKEN` **não serve** (sem escopo Admin). Secret sugerido `FLORA_BRANCH_PROTECTION_TOKEN`. Nenhuma credencial copiada/exposta. **[NV]** provisionamento e leitura ao vivo.
- **(c) Advisories:** bump `rustls` `0.23.43→0.23.46` resolve **4 advisories canônicos** (`Cargo.lock` ainda em `0.23.43`). Recomendação registrada; **não promover gate ainda**. **[NV]** aplicação do bump.
- **(d) Deploy Flora→Coolify:** executado pela ligação MDMV-Autonoma — app `Flora-Staging`, deployment `44djh8p8owth5buhmcvgpilb`, commit `15203ee78`, health `200`, rollback image `b1dafae3…`. Fora do gate de merge (ADR-0003). **[NV]** mensageria cross-session: reconsultar estado real antes de promoção/rollback.
- **(e) Ciclo 089 — espelho Forgejo:** origem `https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git`; `origin/dev` = `f12a328c` é ancestral de `dev` (fast-forward); job de espelho **não** é required check (ADR-0003).
- Nenhum validation/release/código/workflow/gate/migration/ADR/constituição tocado; nenhuma linha histórica reescrita. **[NV]** remoto Forgejo, job de espelho e proteção ao vivo.

### Execução verificada do espelho — 2026-11-20T13:02Z

- Workflow `.github/workflows/mirror-to-forgejo.yml` criado (event-driven, `permissions: {}`, secrets `FORGEJO_USER`/`FORGEJO_TOKEN`, sem force/mirror, job não required); parse YAML OK (`jobs=['mirror']`, `on push dev/main + tags v*`).
- Sincronização manual imediata executada com `git push origin refs/heads/dev:refs/heads/dev`: **`f12a328c4..15203ee78 dev -> dev`**, exit 0. Ancestralidade confirmada por `git merge-base --is-ancestor origin/dev dev` antes do push; nenhum force-push.
- **[NV]** provisionamento dos secrets no GitHub e execução futura do job; a configuração automática depende de `FORGEJO_USER`/`FORGEJO_TOKEN` com `write:repository` apenas em `MDMV/flora-workspace`.

## Ciclo 090 — bump de segurança rustls — 2026-11-20T15:05Z

- **Âncora:** `.specify/feature.json` → `specs/090-rustls-security-bump`; branch `feature/090-rustls-security-bump` a partir de `dev@824ba00e9`. Factory-fonte `ee71d8db` (atual `b1deab15f`, sujo, não transplantado).
- **Mudança:** `Cargo.lock` `rustls 0.23.43 → 0.23.46` (`cargo update -p rustls --precise 0.23.46 --offline`) e `supply-chain/config.toml:1116` exemption ajustada para `0.23.46`/`safe-to-deploy`. Manifest `Cargo.toml:189` inalterado (`rustls = "0.23"`). O cargo normalizou uma aresta de `getrandom` em `tempfile` sem trocar versões.
- **Advisories fechados:** RUSTSEC-2026-0285/0346/0348/0349 (4 canônicos; 2 medium + 2 low); bônus 0347. Nenhum critical/high. Sem promoção de `cargo deny advisories` a required check, conforme decisão 089.
- **Verificação:** gate PASS `all 10 db contract floors intact (58 total)` (offline, exit 0); `cargo test --workspace` 0 falhas; `cargo deny check bans licenses sources` → `bans ok, licenses ok, sources ok`; `cargo vet --locked --no-minimize-exemptions` → `Vetting Succeeded (424 exempted)`. `git diff --check` limpo. Scripts SDD da factory seguem FAIL por baseline legada — não são gate (decisão 089).
- R4 aprovado; R7 `0.4.4→0.4.5` com rollback restaurando lock/exemption. **[NV]** CI do PR, merge, contagem viva de alerts e produção.

## Atualização — ciclo 091 (2026-11-20; append-only)

- **Âncora:** `.specify/feature.json` → `specs/091-governance-ci-audit`; branch `feature/091-governance-ci-audit` @ `673c687ea`. Auditoria viva, documental e não mutante; nenhuma linha histórica reescrita.
- **Auditoria viva PASS em dev/main:** `python3 scripts/ci/check_gate_integrity.py --remote` → **PASS**, com `dev: live required contexts match` e `main: live required contexts match`. 4 required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`), *strict* + `enforce_admins`, sem required reviews. Gate local **PASS** (oráculo dinâmico 088, 10 legs / 58 pisos); zero required checks novos. Leitura ao vivo via `gh` com PAT fine-grained (`Administration: read`, restrito ao repo `flora`); secret `FLORA_BRANCH_PROTECTION_TOKEN` registrado, nenhuma credencial copiada ou exibida. **[NV]** reconsulta de estado ao vivo.
- **Constituição v1.4.0 aprovada pelo SE:** `1.3.0 → 1.4.0` (MINOR, 2026-11-20), aprovada pelo SE/maintainer sob §Governance e ADR-0003 (emenda 2026-11-20). Migration plan **documentation-only**: Svelte 5 como frontend canônico, remoção de SQLite, ADR-0001/0002 para mecanismos diferidos, ADR-0003 para merges; rollback = reverter para v1.3.0. Nenhuma alteração de código, config, workflow, gate, migration ou secret.
- **Deploy de produção [NV]:** apenas a app `Flora-Staging` existe no Coolify; não há app de produção, FQDN ou revisão de produção. Nenhum deploy de produção reivindicado; fora do gate de merge (ADR-0003). Reconsultar ao vivo antes de qualquer promoção/rollback; não promover Staging a produção sem FQDN/revisão verificados.
- **Advisories no frontend:** `apps/frontend/package-lock.json` registra **25 advisories (7 high)**. Recomendação registrada: **não promover** checagem de advisories a required check; **triar/limpar** os findings. Nenhum gate alterado; **[NV]** triagem e recontagem.
- **Decisão 091 — scripts SDD fora de relatórios:** por decisão explícita, os scripts SDD da factory **não** devem sequer constar de relatórios futuros; menção restrita a documento de decisão/governança, nunca em relatórios de status. Registro canônico: `docs/governance/2026-11-20-091-relatorios-sem-scripts-sdd.md`. Reforça a decisão 089 (não gate); única autoridade de gate é `check_gate_integrity.py`.
- **Workflow de auditoria — não required:** `.github/workflows/branch-protection-audit.yml` (`schedule` `23 4 * * *` + `workflow_dispatch`, `permissions: {}`, `GH_TOKEN` via `FLORA_BRANCH_PROTECTION_TOKEN`, job `branch-protection-audit` que executa `check_gate_integrity.py --remote`). YAML válido; job **advisory, não required** — nenhum required check adicionado (ADR-0003).
- **Escopo:** apenas apêndices em `STATE.md`/`PROGRESSO.md` e o novo documento de governança. Nenhum arquivo em `.specify/validations/`, `.specify/releases/`, workflows, gate, ADR, constituição, código, PR, merge, tag ou produção alterado; nenhuma credencial exposta. **[NV]** CI/proteção/deploy/Forgejo/aprovações ao vivo.

## Atualização — ciclo 092 (2026-11-20; append-only)

- **Âncora:** `.specify/feature.json` → `specs/092-npm-lock-sync`; branch `feature/092-npm-lock-sync` @ `bf00caf5c`. Todas as notas anteriores intactas; apenas acréscimo no EOF. Registro canônico: `docs/governance/2026-11-20-092-package-lock-sync.md` (novo, append-only).
- **Lockfile canônico:** `apps/frontend/bun.lock` segue como fonte canônica do CI. `apps/frontend/package-lock.json` foi sincronizado sem tocar `apps/frontend/package.json`; comando contratual em `apps/frontend`: `npm install --package-lock-only --ignore-scripts`. Evidência local: `+116/−0` em 1 arquivo, com `shadcn-svelte 1.8.1` acrescentado à root `devDependencies` e entradas transitivas do mesmo grafo; `git diff` do manifesto vazio.
- **Advisories:** baseline 091 de 25 alerts do Dependabot (7 high) reenumerada após a regeneração; `npm audit --package-lock-only --json` ao vivo → **21 advisories (5 high, 15 moderate, 1 low, 0 critical)**, `auditReportVersion 2`. Fontes distintas (Dependabot vs registro npm), sem comparação estrita. Decisão 089/091 reafirmada: **não promover** checagem de advisories a required check; nenhum gate novo.
- **Promoção/deploy:** fora do gate de merge (ADR-0003) e não executados. **[NV]** PR/CI/proteção/Coolify ao vivo; apenas `Flora-Staging` existe (registro 091), sem app/FQDN/revisão de produção confirmados. Não promover sem verificação ao vivo.
- **Escopo:** apenas apêndices em `STATE.md`/`PROGRESSO.md` e o novo documento de governança. Nenhum validation/release, workflow, gate, ADR, constituição, código, PR, merge, tag ou produção alterado; nenhuma credencial exposta. **[NV]** CI/proteção/deploy/Forgejo/aprovações ao vivo.

## Atualização — ciclo 093 (2026-11-20; append-only)

- **Âncora:** `.specify/feature.json` → `specs/093-sveltekit-security-upgrade`; branch `feature/093-sveltekit-security-upgrade` @ `b426e4f81`. Todas as notas 2026-11-18/19/20 e o registro 092 intactos; apenas acréscimo no EOF. Registro canônico: `docs/governance/2026-11-20-093-sveltekit-upgrade.md` (novo). Estado remoto/Coolify ao vivo: **[NV]** salvo onde medido abaixo.
- **Bloqueio do PR #318** (`release: promove dev para main (producao)`, `dev`→`main`, `OPEN`/`BLOCKED`, lido via `gh` às 19:56Z): `dependency-review` **fail** no run `44806804503` (`fail-on-severity: moderate`, "PR diff only") por **4 advisories em `@sveltejs/kit@2.70.3`** — 1 high `GHSA-3r8g-jcqq-3p5j` (DoS em parsing de form data de remote functions `form`) e 3 moderate: `GHSA-mw6w-2fmv-pvw3` (`URIError` não tratado no adapter-node via `reroute` malformado), `GHSA-rqp5-r3wg-vg35` (import protocol-relative na resolução de rota SSR → script cross-origin), `GHSA-6cf5-gc4q-fm3x` (colisão de funções privadas `query.batch`). Demais checks: `db-tests`, `frontend-build`, `rust-build`, `e2e-browser`, `supply-chain-vet` e `mirror` `pass`; 1 `e2e-browser` `pending` e 1 `dependency-review` `skipping` em runs anteriores.
- **Correção do ciclo 093:** upgrade `@sveltejs/kit` `2.70.3 → 3.1.1`, `@sveltejs/adapter-node` `5.5.7 → 6.0.5`, `svelte` `5.56.10 → 5.57.1`. Pares exigidos pelo kit 3.1.1: `vite ^8.0.12`, `svelte ^5.57.1`, `typescript ^6`, `vite-plugin-svelte ^7`; adapter-node 6.0.5 exige `kit ^3`. Snapshot local: manifesto já declara `kit 3.1.1`, `adapter-node 6.0.5`, `svelte 5.57.1`, `vite ^8.0.16`, `vite-plugin-svelte ^7.1.2`; lockfiles em sincronização (`bun.lock` canônico, decisão 092). **[NV]** `dependency-review` após o bump.
- **Produção:** app de produção e DNS prontos; deploy depende de `main` promovido (PR #318). Fora do gate de merge (ADR-0003); nenhum deploy executado por esta nota. **[NV]** estado ao vivo de Coolify/FQDN/revisão.
- **Gate advisory:** decisão 089/091/092 reafirmada — **sem promoção** de checagem de advisories a required check. `dependency-review` já é um dos 4 required checks; a correção é adotar as versões corrigidas, não relaxar o gate. Única autoridade: `python3 scripts/ci/check_gate_integrity.py`; nenhum script SDD promovido (089/091).
- **Escopo:** apenas apêndices em `STATE.md`/`PROGRESSO.md` e o novo documento de governança. Nenhum validation/release, workflow, gate, ADR, constituição, código, PR, merge, tag ou produção alterado por esta nota; nenhuma credencial exposta. **[NV]** CI/proteção/deploy/Forgejo/aprovações ao vivo.

### Promoção dev→main e deploy — 2026-11-20T20:38Z

- PR **#318** (`dev→main`) mesclado às **20:38:11Z**; merge commit `761539b9ffa7ecd4c395942e10b8ecf7e483fefd`. `main` passou a ser a linha de produção autorizada pelo SE. Rollback = reverter o merge commit, sem force-push.
- Rollup verde no head `378c4b8cd`: 4 required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`) aprovados; `e2e-browser` e `supply-chain-vet` informativos verdes; `mirror` em `main` concluído com sucesso (`44813149647`). `dependency-review` limpo após o ciclo 093.
- App de produção Flora no Coolify: UUID `9q3wuuwkur2kl27py0si785n`, branch `main`, FQDN `https://flora.mdmv.qzz.io`; DNS A `flora.mdmv.qzz.io` criado e verificado publicamente (Cloudflare proxy). Deploy disparado pelo agente de ligação da MDMV-Autonoma; status final e `/health` **[NV]** até confirmação.

## Atualização — ciclo 094 (2026-11-20; append-only)

- **Âncora:** branch `feature/094-visao-so-pme` @ `fd25d8f8b`; `.specify/feature.json` → `specs/094-visao-so-pme`. Todas as notas 2026-11-18/19/20 e o registro 093 intactos; apenas acréscimo no EOF. Registro canônico: `docs/governance/2026-11-20-094-visao-so-pme-integracoes.md` (novo).
- **Visão SO PME — diretriz NÃO-normativa:** registrada como nota de governança, sem emenda constitucional, gate, workflow, migration ou código. O gate segue `python3 scripts/ci/check_gate_integrity.py` sob ADR-0003, sem required check novo.
- **Mapa:** Coolify e Forgejo implementados e verificados; Grafana+VictoriaMetrics e Uptime Kuma deferidos, sem materialização no alvo; agents Autonoma e dark-factory como ligação documental; agents marketing, monetization e seller ausentes no alvo, com estado fora do alvo **[NV]**.
- **Produção observada:** `/health` **200**; deploy **finished 21:23:59Z** no commit `cee7b0cda`; app `9q3wuuwkur2kl27py0si785n`, FQDN `https://flora.mdmv.qzz.io`. A nota de 20:38Z ("deploy em curso, health [NV] até confirmação") permanece como registro do momento da medição. Deploy fora do gate de merge (ADR-0003).
- **Forgejo:** refs do espelho `https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git` sincronizados; job `mirror` informativo, não required (ADR-0003).
- **Advisories de runtime:** **0**. Decisão 089/091/092/093 reafirmada: sem promoção de checagem de advisories a required check; único gate é `check_gate_integrity.py`.
- **Escopo:** apenas apêndices em `STATE.md`/`PROGRESSO.md` e o novo documento de governança. Nenhum validation/release, workflow, gate, ADR, constituição, código, PR, merge, tag ou deploy alterado por esta nota; nenhuma credencial exposta. **[NV]** CI, proteção, deploy futuro, Forgejo e aprovações ao vivo: reconsultar antes de agir.

## Atualização — ciclo 095 (2026-11-20T23:13:59Z; append-only)

- **Âncora:** branch `feature/095-frontend-deploy` @ `29b34aefe`; `.specify/feature.json` → `specs/095-frontend-deploy`. Todas as notas 2026-11-18/19/20 e o registro 094 intactos; apenas acréscimo no EOF. Registro canônico: `docs/governance/2026-11-20-095-frontend-deploy.md` (novo).
- **Sintoma medido:** `https://flora.mdmv.qzz.io/` → **404** (corpo vazio), `/tasks` e `/login` → **404**; no checkout local, `/` e `/tasks` → **200** sob o dev server. Build correto, falha na publicação/roteamento do FQDN.
- **Causa raiz:** o FQDN servia a API Rust (`flora-api`), não o frontend SvelteKit. Evidências: `/health` **200** `{"status":"ok",...}`, `/metrics` **200** `flora_api_up 1`, `/llms.txt` **200** `# Flora Workspace (flora-api)`; `Dockerfile:5,9,11` compila/copia/executa apenas `flora-api`; `.github/workflows/docker-staging.yml:34` `context: .` sem `file:` → usa o Dockerfile da raiz; `apps/frontend/Dockerfile` nunca referenciado no deploy. O manifesto do build (`apps/frontend/build/server/manifest.js:33-34` `pattern: /^\/$/`; `:89-90` `pattern: /^\/tasks\/?$/`) prova que as rotas existem no frontend; `/(auth)` é grupo interno, red herring.
- **Decisão registrada:** separar frontend e API — imagem `flora-web` construída de `apps/frontend/Dockerfile` e publicada por workflow manual; FQDN `flora.mdmv.qzz.io` para o frontend; API em `flora-api.mdmv.qzz.io`; `FLORA_API_BASE_URL` para gerar `config.js` no entrypoint; `FLORA_APP__CORS_ORIGINS` deve incluir a origem do frontend. Deploy fora do gate de merge (ADR-0003). Nenhuma imagem publicada/app alterado/DNS modificado por esta nota.
- **[NV] até deploy final:** publicação da imagem, app/DNS da API, CORS em produção, resultado do deploy e medição final de `/`, `/tasks` e `/health`. Próximos passos: gate `check_gate_integrity.py` + `git diff --check`; build local do container e `GET /`, `/tasks`, `/config.js`; publicação via workflow manual; apontar FQDN para `flora-web`; criar/validar app da API com CORS; medir endpoints e fluxo no browser; escrever nova nota append-only. Rollback = reapontar FQDN para imagem anterior, sem force-push.
- **Escopo:** apenas apêndices em `STATE.md`/`PROGRESSO.md` e a nova nota de governança. Nenhum validation/release, workflow, gate, ADR, constituição, código, PR, merge, tag ou deploy alterado; nenhuma credencial exposta. **[NV]** CI/proteção/Coolify/Forgejo/aprovações ao vivo: reconsultar antes de agir.

### Fechamento verificado — 2026-11-21T00:15:48Z (ciclo 095; append-only)

- **Branch/PRs:** `feature/095-frontend-deploy` (commit `30e424fce3`) mesclado em `dev` via PR **#324**; PR **#325** (visibilidade GHCR) e **#326** (remoção da etapa inválida) mesclados. Head local `aa1f01b8a` no branch `fix/095-coolify-source-build`. Nenhum merge force, nenhuma reescrita.
- **Deploy:** app Coolify `Flora Web` UUID `uvykanlrkyp5dzrkktaj5bup`, `build_pack=dockerfile`, repositório GitHub `MDMV-Laboratorio-de-Solucoes-Digitais/flora.git`, branch `dev`, `base_directory=/apps/frontend`, `dockerfile_location=/Dockerfile`, porta 3000, FQDN `https://flora.mdmv.qzz.io`, `FLORA_API_BASE_URL=https://flora-api.mdmv.qzz.io`. Deploy `kwzqvvvkeldq2zmyxa12fu3g` **finished**, commit `eea32bdd12ba48222a81990f0f1efd5212520a3b`. GHCR ficou privado (401 no pull); o build passou a ser feito pelo Coolify direto do Git. Nenhuma credencial exposta.
- **API:** app `9q3wuuwkur2kl27py0si785n` com FQDN `https://flora-api.mdmv.qzz.io`; DNS CNAME validado; `FLORA_APP__CORS_ORIGINS=["https://flora.mdmv.qzz.io"]`. Deploy da API `ariimiyagfkgbp2mm66rdguv` concluído; `/health` e `/llms.txt` → **200** no domínio da API.
- **Produção medida às 00:15:48Z:** `https://flora.mdmv.qzz.io/` → **200**, `/tasks` → **200**, `/config.js` → **200** com `{"apiBaseUrl":"https://flora-api.mdmv.qzz.io","wsBaseUrl":null}`; `/health` e `/llms.txt` no frontend → **404** (esperado: são rotas da API). Preflight CORS com `Origin: https://flora.mdmv.qzz.io` → **200**, `access-control-allow-origin: https://flora.mdmv.qzz.io`. Página renderizada em navegador real: "Welcome to Flora / Enter your credentials to access your workspace. / Sign In / First Seed Journey".
- **Gates:** `python3 scripts/ci/check_gate_integrity.py` PASS (`exit=0`); `npm --prefix apps/frontend run check` 0 erros; build Docker local da imagem final com `/`, `/tasks`, `/config.js` 200 e `content-length` correto; `git diff --check` limpo. `pytest -q` → `no tests ran` (N/A); scripts SDD da factory ausentes no alvo (N/A, conforme Bloco 0.a).
- **Rollback:** reapontar o app `Flora Web` para a origem/imagem anterior e retornar o FQDN da API, sem force-push; revalidar `/`, `/health` e CORS. **[NV] remanescente:** fluxo E2E autenticado de login/tasks com credenciais reais e health check do Coolify (`unknown`, sem healthcheck configurado); preflight CORS foi verificado, mas navegação autenticada completa não.

## Atualização — ciclo 096 (2026-11-21T00:57:59Z; append-only)

- **Âncora:** branch `feature/096-auth-login-ptbr` @ `1910d1c7d`; `.specify/feature.json` → `specs/096-auth-login-ptbr`. Todas as notas 2026-11-18/19/20 e os registros 094/095 intactos; apenas acréscimo no EOF. Registro canônico: `docs/governance/2026-11-21-096-auth-login-ptbr.md` (novo). Estado remoto/Coolify/IdP ao vivo: **[NV]** salvo onde medido abaixo.
- **Sintoma:** usuário `luis.granja@mdmv.com.br`, criado via bootstrap, não consegue entrar em `https://flora.mdmv.qzz.io/`; a tela de login está em inglês ("Welcome to Flora / Enter your credentials to access your workspace. / Sign In") e o botão apenas dispara o redirecionamento OIDC, sem campo de senha funcional.
- **Diagnóstico OIDC:** `.env.example:59-61` e `flora-core/src/config/oidc.rs` `OidcConfig::default()` mantêm issuer `http://localhost:8080`, `client_id flora@localhost`, `client_secret` vazio e `allowed_redirect_uris: Vec::new()` — allowlist vazia (`.env.example:64` comentado; validação exata em `flora-api/src/routes/auth.rs:105`). Não há IdP materializado no alvo e `validate()` não testa alcançabilidade do issuer: config "válida" aponta para endpoint inalcançável em produção. `create_auth_router()` em `auth.rs` registra somente `/login`, `/callback`, `/logout`, `/refresh` e `/select-organization`: nenhum fallback de senha. `specs/001-flora-seed-v0-1/spec.md:271` (FR-022) exigia autenticação local quando Zitadel falhasse, nunca implementada; o ciclo 041 removeu `argon2`/`bcrypt` como crates dead; nenhuma migration define `password_hash` (previsto em `specs/001.../data-model.md:19`).
- **Decisão registrada:** reintroduzir `argon2` (Argon2id) para fallback local de senha; recuperar credencial por variável de ambiente `FLORA_*` dedicada, lida no startup (sem segredo versionado; env ausente → fallback desligado e OIDC segue como único caminho); tela de login pt-BR em `apps/frontend/src/routes/(auth)/+page.svelte`, incluindo rótulos e `login-error.svelte.spec.ts`. O fluxo OIDC authorization-code + PKCE/S256 continua primário e não é relaxado.
- **Gates esperados:** `python3 scripts/ci/check_gate_integrity.py` **PASS** (registrado no diagnóstico; reexecutar no checkout final) e `git diff --check` limpo; testes Rust de hash/verify Argon2id, env no startup e rota local + contrato de login; `npm --prefix apps/frontend run check` e vitest com rótulos pt-BR; build Docker com `/`, `/config.js` e `/health` → 200 e login local → sessão + `/api/v1/tasks` → 200; medição final em navegador real e nova nota append-only. Rollback = reapontar o app Coolify para imagem/commit anterior e remover a env de credencial, sem force-push.
- **[NV] até deploy final:** IdP real/configuração/alcance do issuer em produção, allowlist efetiva, persistência do `password_hash`, comportamento do fallback local e da tela pt-BR após deploy, E2E autenticado com a credencial real de `luis.granja@mdmv.com.br`, e CI/proteção/Coolify/aprovações ao vivo.
- **Escopo:** apenas apêndices em `STATE.md`/`PROGRESSO.md` e a nova nota de governança. Nenhum validation/release, workflow, gate, ADR, constituição, código, PR, merge, tag ou deploy alterado; nenhuma credencial exposta. **[NV]** CI/proteção/Coolify/Forgejo/aprovações ao vivo: reconsultar antes de agir.

## Atualização — ciclo 097 (2026-11-21T02:12:45Z; append-only)

- **Âncora:** branch `feature/097-first-seed-once` @ `696849934`; `.specify/feature.json` → `specs/097-first-seed-once`. Todas as notas 2026-11-18/19/20/21 e os registros 094/095/096 intactos; apenas acréscimo no EOF. Registro canônico: `docs/governance/2026-11-21-097-first-seed-once.md` (novo). Estado remoto/Coolify ao vivo: **[NV]** salvo onde medido abaixo.
- **Sintoma:** o link "É uma instalação nova? Fazer primeira configuração" (equivalente a "Is this a fresh install? Start First Seed Journey") é renderizado **incondicionalmente** na tela de login `apps/frontend/src/routes/(auth)/+page.svelte:238-243`, no branch `{:else}` do modo normal — sem consultar se a instalação está vazia. Em instalação já configurada, o usuário é convidado a repetir o bootstrap; a tentativa falha com **409** ("Este workspace já foi configurado", `+page.svelte:140-143`).
- **Causa raiz:** a garantia de "uma vez por instalação" existe **somente no backend**, dentro de `POST /api/v1/bootstrap` — advisory transaction lock `BOOTSTRAP_LOCK_KEY` (`flora-api/src/routes/bootstrap.rs:28-29`) mais contagem de organizações não-sistema, com 409 documentado em `:238-244`. Nenhum endpoint/flag expõe "instalação vazia" ao frontend, que exibe o link de primeira configuração sem esse estado. O spec `specs/097-first-seed-once/spec.md` ainda é template `Draft` (placeholders) no snapshot.
- **Decisão registrada:** condicionar a exibição do fluxo de primeira configuração ao **estado real da instalação**, sinalizado pelo backend (zero organizações não-sistema); manter o guardião 409 do bootstrap como última linha de defesa. Nenhuma alteração de código, gate, workflow ou spec feita por esta nota.
- **Gates esperados:** gate do alvo `python3 scripts/ci/check_gate_integrity.py` **PASS** (registrado no diagnóstico; reexecutar no checkout final) e `git diff --check` limpo; vitest da tela de login cobrindo ambos os estados (vazia → link visível; configurada → link oculto) e o fluxo bootstrap → sessão → `/tasks`; `npm --prefix apps/frontend run check`; build Docker com `/`, `/config.js` e login → tasks; medição final em navegador real e nova nota append-only. Rollback = reapontar o app Coolify para imagem/commit anterior, sem force-push.
- **[NV] até deploy final:** estado real da instalação em produção, comportamento do link após deploy e E2E autenticado; CI/proteção/Coolify/Forgejo/aprovações ao vivo — reconsultar antes de agir.
- **Escopo:** apenas apêndices em `STATE.md`/`PROGRESSO.md` e a nova nota de governança. Nenhum validation/release, workflow, gate, ADR, constituição, código, PR, merge, tag ou deploy alterado; nenhuma credencial exposta. **[NV]** CI/proteção/Coolify/Forgejo/aprovações ao vivo.

## Atualização — ciclo 098 (2026-11-21T02:52:54Z; append-only)

- **Âncora:** branch `feature/098-password-recovery` @ `76f936e24`; `.specify/feature.json` → `specs/098-password-recovery`. Todas as notas 2026-11-18/19/20/21 e os registros 094/095/096/097 intactos; apenas acréscimo no EOF. Registro canônico: `docs/governance/2026-11-21-098-password-recovery.md` (novo). Estado remoto/Coolify/CI ao vivo: **[NV]** salvo onde medido abaixo.
- **Evidência do 401 em produção:** após o deploy da API (`https://flora-api.mdmv.qzz.io`, app Coolify `9q3wuuwkur2kl27py0si785n`), o login local de `luis.granja@mdmv.com.br` ainda retorna **401**. A rota `POST /api/v1/auth/password/login` existe (`flora-api/src/routes/auth.rs:83`, handler em `:443`); as env `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`FLORA_BOOTSTRAP__ADMIN_PASSWORD` estão presentes e corretas (confirmadas via API Coolify, valor da senha conferido sem exibição), ao lado de `FLORA_APP__BASE_URL` e `FLORA_OIDC__ALLOWED_REDIRECT_URIS`. Logs sem sinal da recuperação, mas o filtro suprime `flora_core` — ausência de log não é prova.
- **Causa determinística:** `flora-core/src/config/bootstrap.rs` (`apply_startup_password`) saía com `Ok(false)` quando `user.password_hash.is_some()` e gravava via `set_password_hash_if_absent`; a conta pré-existente tem hash, então a recuperação era silenciosamente ignorada apesar de `Config::load()` e do chamador em `flora-api/src/state/app_state.rs:93` estarem corretos.
- **Decisão de overwrite:** com ambas as env válidas e e-mail existente, sobrescrever o `password_hash` no startup mesmo quando já há hash (update restrito ao `user_id` resolvido, sem log de senha/hash); sem env, nenhuma escrita e OIDC segue primário. Variáveis são temporárias: remover da app Coolify após a recuperação confirmada. Gate do alvo `python3 scripts/ci/check_gate_integrity.py` **PASS** registrado no diagnóstico (não reexecutado por esta nota).
- **Gates esperados:** `git diff --check` limpo; testes Rust dos quatro estados (sem env, hash nulo, **hash pré-existente com overwrite**, env incompleta) e nenhuma credencial em log/resposta; build Docker da API com login local → sessão e `/api/v1/tasks` → 200; medição final em navegador real e nova nota append-only. Rollback = reapontar app Coolify para imagem/commit anterior e remover as env temporárias, sem force-push.
- **[NV] até deploy final:** aplicação da mudança em produção, login com a credencial de recuperação, remoção das env após sucesso, `/health`/CORS e CI/proteção/Coolify/Forgejo/aprovações ao vivo.
- **Escopo:** apenas apêndices em `STATE.md`/`PROGRESSO.md` e a nova nota de governança. Nenhum validation/release, workflow, gate, ADR, constituição, PR, merge, tag ou deploy alterado por esta nota; nenhuma credencial exposta.

## Atualização — ciclo 099 (2026-11-21T03:24:38Z; append-only)

- **Âncora:** branch `feature/099-ptbr-ui` @ `7183a33fc`; `.specify/feature.json` → `specs/099-ptbr-ui`. Todas as notas 2026-11-18/19/20/21 e os registros 094/095/096/097/098 intactos; apenas acréscimo no EOF. Registro canônico: `docs/governance/2026-11-21-099-ptbr-ui.md` (novo). Estado remoto/Coolify/CI ao vivo: **[NV]** salvo onde medido abaixo.
- **Requisito do usuário:** interface 100% pt-BR; outras línguas apenas termos técnicos (`OIDC`, `OAuth`, `API`, `JWT`, `REST`, `AI`, `SSO`, `slug`). Sem multilinguagem, troca de idioma ou fallback para inglês. Cobertura: rotas `(auth)`/`(app)` e componentes de domínio (chat, arquivos, busca, tarefas, workspace, organização).
- **Inventário:** ~140 strings visíveis hardcoded em ~15 arquivos de `apps/frontend/src`, sem qualquer camada de i18n (nenhuma dependência/catálogo de tradução). `apps/frontend/src/app.html:2` já é `<html lang="pt-BR">`; o defeito é só o texto visível em inglês. Login já pt-BR desde 096 ("Bem-vindo ao Flora", "E-mail", "Senha", "Entrar"); examples em `routes/(app)/tasks/+page.svelte`: "Tasks" (`:136`), "Refresh" (`:141`), "Title" (`:179`), "Loading..." (`:198`), "No tasks returned for current assignee/org." (`:200`). Maior densidade: tasks (~52), channels (~40), login (~31), ChatLayout (~31), select-org (~24); contagens aproximadas, cifra canônica ~140/~15.
- **Decisão:** dicionário fixo pt-BR (chave → literal) consumido pelos componentes, em vez de framework i18n com múltiplos locales; termos técnicos preservados; toda string visível (texto, `placeholder`, `aria-label`, `title`, mensagens de estado, botões) passa a vir do dicionário. Gate do alvo `python3 scripts/ci/check_gate_integrity.py` **PASS** registrado no diagnóstico (não reexecutado por esta nota).
- **Gates esperados:** `git diff --check` limpo; varredura estática sem literais em inglês fora dos termos técnicos e `app.html` `lang="pt-BR"`; `npm --prefix apps/frontend run check` + vitest (incl. `login-error.svelte.spec.ts`) com rótulos/dicionário pt-BR; build Docker e medição das rotas `/`, `/tasks`, `/login` em pt-BR; medição final em navegador real e nova nota append-only. Rollback = reapontar app Coolify `Flora Web` para imagem/commit anterior, sem force-push.
- **[NV] até deploy final:** aplicação das mudanças, build/publicação de `flora-web`, deploy e medição em navegador real de 100% pt-BR nas rotas/componentes; CI/proteção/Coolify/Forgejo/aprovações ao vivo.
- **Escopo:** apenas apêndices em `STATE.md`/`PROGRESSO.md` e a nova nota de governança. Nenhum validation/release, workflow, gate, ADR, constituição, código, PR, merge, tag ou deploy alterado por esta nota; nenhuma credencial exposta.

## Atualização — ciclo 100 (2026-11-21T04:19:25Z; append-only)

- **Âncora:** branch `feature/100-admin-recovery` @ `9cf6775a7`; `.specify/feature.json` → `specs/100-admin-recovery`. Todas as notas 2026-11-18/19/20/21 e os registros 094/095/096/097/098/099 intactos; apenas acréscimo no EOF. Registro canônico: `docs/governance/2026-11-21-100-admin-recovery.md` (novo). Estado remoto/Coolify/CI ao vivo: **[NV]** salvo onde medido abaixo.
- **Evidência (applied=false, e-mail ausente):** em produção o bootstrap informa `available=false` (organizações existem) e o par `FLORA_BOOTSTRAP__ADMIN_EMAIL`/`ADMIN_PASSWORD` está presente e correto (nota 098), mas `flora-api/src/state/app_state.rs:99` registrou `Startup administrator password recovery` com **`applied=false`**, precedido do warn em `flora-core/src/config/bootstrap.rs:112-114` ("FLORA_BOOTSTRAP__ADMIN_EMAIL does not match an existing user; startup password skipped", `Ok(false)` em `:115`). `find_by_email` (`:111`) não encontrou o e-mail configurado: não é o guard de hash pré-existente (removido em 098), nem falha de `Config::load()`/`validate()`. O e-mail do administrador real é desconhecido.
- **Decisão — wildcard explícito:** com `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` (exatamente `*`) e a senha configurada, `apply_startup_password` seleciona o primeiro usuário ativo (`is_active = true`, `ORDER BY created_at ASC LIMIT 1`, sem filtro de role/sistema) e grava o hash Argon2id restrito a esse `user_id`, sem log de senha/hash. Sem opt-in, a busca por e-mail permanece e a ausência de correspondência retorna `false` sem escrita; OIDC segue primário. Env temporárias devem ser removidas da app Coolify após a confirmação do login. Gate do alvo `python3 scripts/ci/check_gate_integrity.py` **PASS** registrado no diagnóstico (não reexecutado por esta nota).
- **Gates esperados:** `git diff --check` limpo; testes Rust (sem env, e-mail ausente sem opt-in → `false`, e-mail ausente com `ADMIN_EMAIL=*` → seleção wildcard + overwrite, e-mail presente → caminho restrito) e nenhuma credencial em log/resposta; build Docker da API com login local → sessão e `/api/v1/tasks` → 200; medição final em navegador real e nova nota append-only. Rollback = reapontar app Coolify para imagem/commit anterior e remover as env temporárias, sem force-push.
- **[NV] até deploy final:** aplicação da mudança, deploy/reinício da API, login com a credencial de recuperação do administrador real, remoção das env após sucesso, `/health`/CORS e CI/proteção/Coolify/Forgejo/aprovações ao vivo.
- **Escopo:** apenas apêndices em `STATE.md`/`PROGRESSO.md` e a nova nota de governança. Nenhum validation/release, workflow, gate, ADR, constituição, código, PR, merge, tag ou deploy alterado por esta nota; nenhuma credencial exposta.

## Atualização — ciclo 101 (2026-11-21T05:11:11Z; append-only)

- **Âncora:** branch `feature/101-password-login-wildcard` @ `5890471be`; `.specify/feature.json` → `specs/101-password-login-wildcard`. Todas as notas 2026-11-18/19/20/21 e os registros 094/095/096/097/098/099/100 intactos; apenas acréscimo no EOF. Registro canônico: `docs/governance/2026-11-21-101-password-login-wildcard.md` (novo). Estado remoto/Coolify/CI ao vivo: **[NV]** salvo onde medido abaixo.
- **Evidência (`applied=true`, login ainda **401**):** com `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` e a senha configurada, a recuperação do ciclo 100 rodou e o chamador em `flora-api/src/state/app_state.rs` registrou **`applied=true`** — o hash Argon2id foi gravado no primeiro usuário ativo. Ainda assim, `POST /api/v1/auth/password/login` retorna **401**: a rota resolve a conta pelo e-mail informado no corpo e o e-mail real do usuário bootstrap é desconhecido, de modo que a busca não alcança o usuário cujo hash foi atualizado. O ponto de falha mudou de locus — não é mais a escrita do hash (ciclo 100), é a resolução da identidade no login.
- **Decisão — wildcard de login com opt-in:** aceitar `email='*'` (exatamente `*`) no corpo de `POST /api/v1/auth/password/login` **somente** quando `FLORA_BOOTSTRAP__ADMIN_EMAIL` também for `*` (opt-in temporário já ativo). Nesse modo a rota resolve o primeiro usuário ativo (`ORDER BY created_at ASC, id ASC`, sem filtro de role/sistema) e valida a senha contra o hash desse usuário; sem o opt-in, `email='*'` é rejeitado e a busca por e-mail permanece, com OIDC primário e não relaxado. Nenhum segredo/hash em log ou resposta; as env temporárias devem ser removidas da app Coolify após a confirmação do login.
- **Gates esperados:** `git diff --check` limpo; gate do alvo `python3 scripts/ci/check_gate_integrity.py` **PASS** registrado no diagnóstico (reexecutar no checkout final); testes Rust do login com `email='*'` (com opt-in → primeiro usuário ativo; sem opt-in → rejeição; e-mail comum inalterado) e nenhuma credencial em log/resposta; build Docker da API com login wildcard → sessão e `GET /api/v1/tasks` → 200; medição final em navegador real e nova nota append-only. Fora do gate de merge (ADR-0003); nenhum required check novo. Rollback = reapontar a app Coolify para imagem/commit anterior e remover as env temporárias, sem force-push.
- **[NV] até deploy final:** aplicação da mudança, deploy/reinício da API, login wildcard com a credencial de recuperação, remoção das env após sucesso, `/health`/CORS e CI/proteção/Coolify/Forgejo/aprovações ao vivo.
- **Escopo:** apenas apêndices em `STATE.md`/`PROGRESSO.md` e a nova nota de governança. Nenhum validation/release, workflow, gate, ADR, constituição, código, PR, merge, tag ou deploy alterado por esta nota; nenhuma credencial exposta.

### Fechamento verificado — 2026-11-21T13:25:33Z (append-only)

- **Ciclos 096–101 mesclados:** PRs #329, #331, #332, #335, #336 e #338 mesclados em `dev`; release #339 mesclada em `main` (`2f53abd58`). Conflitos `dev/main` resolvidos localmente com `-X ours` preservando 096–101; nenhuma reescrita/force-push.
- **Produção:** deploy da API `pxx1xjtk6ssikhw08o3j8dsq` **finished**. `POST /api/v1/auth/password/login` com `email="*"` e a senha temporária retornou **200**, `session_token` presente, `expires_in=3600`, `next_step=select_or_create_organization`, `user_id=01a1fb7f-958c-73f7-a79d-3d8f908c3ebb`. Frontend pt-BR ao vivo (`lang="pt-BR"`, "Entrar", "E-mail", "Senha").
- **Endpoints:** `https://flora.mdmv.qzz.io/` **200**, `/tasks` **200**, `/config.js` **200**, `https://flora-api.mdmv.qzz.io/health` **200**, `GET /api/v1/bootstrap/status` → `{"available":false}` (bootstrap já consumido; link First Seed escondido).
- **Gates:** `check_gate_integrity.py` **PASS** (`exit=0`, 10 legs / 77 floors), CI verde em todos os PRs finais, clippy workspace `exit=0`, 36 testes unitários, e2e browser passou. `AGENTS.md` reescrito a partir de 4 pesquisas paralelas e versionado.
- **Pendência humana restante:** as variáveis temporárias `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` e `FLORA_BOOTSTRAP__ADMIN_PASSWORD` permanecem na API Coolify para permitir login sem saber o e-mail real; remover/rodar rotação após o usuário confirmar acesso e trocar a senha. Browser E2E autenticado com a conta real não foi executado — **[NV]**; a autenticação via API foi medida com token emitido.

### Verificação complementar — 2026-11-21T13:36:02Z (append-only)

- **E-mail real da conta criada:** `founder@mdmv.com.br` (lido da claim `email` do JWT já emitido no navegador, sem expor o token). `POST /api/v1/auth/password/login` com esse e-mail e a senha temporária retornou **200**, `session_token` presente, `next_step=select_or_create_organization`, `user_id=01a1fb7f-958c-73f7-a79d-3d8f908c3ebb`. O e-mail `luis.granja@mdmv.com.br` informado pelo usuário não corresponde à conta persistida.
- **First Seed Journey:** HTML do FQDN não contém “Primeira configuração” (`grep -c = 0`) e `GET /api/v1/bootstrap/status` retorna `{"available":false}` — link apresentado apenas em instalação limpa, conforme ciclo 097.
- **Sem mudança de código:** diagnóstico conduzido por leitura de produção e do JWT de sessão; nenhum gate/workflow/contrato alterado. Recomendação ao usuário: entrar com `founder@mdmv.com.br` + senha temporária; após confirmação, rotacionar senha e remover `FLORA_BOOTSTRAP__ADMIN_*` do Coolify.

## Atualização — ciclo 102 e execução do roteiro de staging (2026-11-21; append-only)

- **Âncora:** `.specify/feature.json` → `specs/102-task-workspace-scope`; branch `feature/102-task-workspace-scope` @ `ae8632e10` (base `github/dev` após PR #342). Notas anteriores preservadas integralmente; apenas acréscimo no EOF. Registro canônico: `docs/governance/2026-11-21-102-task-workspace-scope.md` (novo).
- **PR #342:** `docs(qa): adiciona roteiro de testes em staging zerado` mesclado em `dev` às **22:15:32Z**, merge commit `ae8632e10`; checks obrigatórios verificados via `gh pr checks` (db-tests, dependency-review, e2e-browser, frontend-build, rust-build, supply-chain-vet = pass). Nenhum PR aberto permanece. Todos os branches locais com patches aparentemente ausentes são commits já integrados via PRs (#329–#339); nenhum trabalho alheio foi reescrito ou descartado.
- **Staging:** URLs/credenciais da MDMV-Autonoma não foram entregues; ambiente local zerado equivalente (Postgres 18 isolado + API + frontend) foi usado para executar o roteiro. Cenários validados: 0 pré-condições (web=200, health=200, bootstrap available=true/false conforme estado), 1 First Seed (sessão + token + link oculto após logout), 2 login válido/erro genérico/429/SSO/API direta, 3 pt-BR, 4 workspaces, 5 canais/mensagens/thread, 7 tarefas, 9 busca, 10/11 endpoints de notificações e RBAC. **[NV]** para o staging real e para itens 6/8/12 sem fixture de segundo usuário/UI de arquivos/comandos de reset entregues.
- **Bug corrigido (102):** seletor de workspace em `/tasks` não escopava a lista. `GET /api/v1/tasks` ganhou `workspace_id` opcional, validado por `ensure_workspace_in_org`, usando `TaskService::list_tasks` com `assignee_id=Some(user_id)`; frontend envia o parâmetro e recarrega na troca. Evidência ao vivo: sem parâmetro → QA-001; QA Workspace 2 → `[]`; Main Workspace → QA-001; workspace inexistente → 422.
- **Dívida registrada:** identidade do usuário na sidebar/chat é literal fixa (`Nome do usuário`, `usuario@flora.app`, `Usuário <id>`); não corrigida neste ciclo.
- **Gates:** `check_gate_integrity.py` **PASS** (`exit=0`, 10 legs / 77 pisos) antes e depois do patch; `cargo fmt/check/clippy` exit 0; Vitest 45/45; svelte-check 0 erros; lint exit 0; build frontend exit 0.
- **Escopo:** nenhum gate/required check/ADR/constituição/schema/migration/RBAC/flag tocado; nenhuma credencial real registrada. Factory lida em `b1deab15f482dd85871cd6c6a857ad710b89e2a6`, nada transplantado; worktree `feature/073-state-closure` intacto.
- **[NV]:** CI/PR/merge/deploy/Coolify/Forgejo e execução do roteiro no FQDN de staging — reconsultar ao vivo antes de qualquer decisão.

## Atualização — ciclo 103 e merge do ciclo 102 (2026-11-21; append-only)

- **PR #343 mesclado** em `dev` às **23:03:13Z**, merge commit `adec2d32fa`; checks obrigatórios verificados ao vivo (`db-tests`, `dependency-review`, `e2e-browser`, `frontend-build`, `rust-build`, `supply-chain-vet` = pass). Nenhum PR aberto após o merge.
- **Âncora 103:** `.specify/feature.json` → `specs/103-workspace-create-rbac`; branch `feature/103-workspace-create-rbac` @ `14cb336173` (base: commits do ciclo 102). Notas anteriores preservadas; registro canônico: `docs/governance/2026-11-21-103-workspace-create-rbac.md` (novo).
- **Bug corrigido (103):** item 11 do roteiro — papel `SomenteLeituraQA` (`permissions=[]`) criava workspace (`POST /api/v1/workspaces` → 201). `create_workspace` agora carrega o papel no org da sessão e exige `WorkspaceWrite|WorkspaceAdmin`, negando com 403 opaco; Admin segue com 201. Helper puro `can_create_workspace` com 2 testes unitários; nenhum `#[ignore]` novo (pisos do gate preservados).
- **Gates 103:** `check_gate_integrity.py` PASS (10 legs / 77 pisos); fmt/check/clippy exit 0; `cargo test -p flora-api workspace::tests` 2 passed. Medição HTTP local: ReadOnly 403 / Admin 201.
- **Roteiro — itens adicionais medidos:** item 8 (upload/download) funcionou após configurar `FLORA_STORAGE__LOCAL_PATH` — com o default `/var/flora/storage` o upload retornou 500 por permissão (configuração de ambiente, não código); item 10 (notificação de atribuição) confirmado via API; item 11 (RBAC) revelou o bug 103; item 6 (DMs) é placeholder hardcoded (`dms/+page.svelte:9`), não implementado — **[NV]**/dívida.
- **[NV]:** staging real (URLs/credenciais não entregues), CI remota do PR 103, branch protection, deploy e UI de arquivos (rota inexistente no frontend).

## Atualização — ciclo 104 e merge do ciclo 103 (2026-11-21; append-only)

- **PR #344 mesclado** em `dev` às **23:14:46Z**, merge commit `70babde31`; checks obrigatórios verificados ao vivo (`db-tests`, `dependency-review`, `e2e-browser`, `frontend-build`, `rust-build`, `supply-chain-vet` = pass). Nenhum PR aberto após o merge.
- **Âncora 104:** `.specify/feature.json` → `specs/104-session-identity`; branch `feature/104-session-identity` @ `23ff4bc809`. Registro canônico: `docs/governance/2026-11-21-104-session-identity.md` (novo). Notas anteriores intactas.
- **Correção 104:** rodapé da navegação passou a exibir o e-mail real da sessão (`qa-admin` / `qa-admin@flora.test`) via helper puro `emailFromToken` (base64url/UTF-8, fail-closed) e `$derived` no layout. Nenhum backend/gate/i18n novo; valor nunca usado para autorização.
- **Gates 104:** Vitest 49/49 (14 arquivos), `bun run check` 0 erros, prettier/eslint exit 0 nos arquivos alterados após correções de `unicorn/prefer-code-point`, `prefer-split-limit`, `prefer-string-repeat` e `max-nested-calls`; navegador real confirmou a identidade. Gate do alvo PASS (10 legs / 77 pisos).
- **Dívida remanescente:** autoria das mensagens no chat ainda exibe `Usuário <id>` — exige lookup de usuários, fora de escopo. DMs são placeholder e a UI de arquivos não existe — **[NV]**/dívidas.
- **[NV]:** staging real (URLs/credenciais não entregues), CI remota do PR 104, branch protection e deploy.

### Fechamento final — 2026-11-21T23:28:33Z (append-only)

- **PR #345 mesclado** em `dev` às **23:28:33Z**, merge commit `3062ad719090835b215ef2d2e2892bc1bb34fb2f`. Checks obrigatórios observados ao vivo via `gh pr checks 345 --watch`: `db-tests`, `dependency-review`, `e2e-browser`, `frontend-build`, `rust-build`, `supply-chain-vet` = **pass**. `github/dev` = `3062ad719`; **nenhum PR aberto**.
- **PRs mesclados nesta sessão:** #342 (`22:15:32Z`, `ae8632e10`), #343 (`23:03:13Z`, `adec2d32`), #344 (`23:14:46Z`, `70babde31`), #345 (`23:28:33Z`, `3062ad719`). Conflitos: nenhum (`mergeStateStatus=CLEAN` em todos). Branch `feature/073-state-closure` e demais branches de terceiros intactos.
- **Gates finais:** `python3 scripts/ci/check_gate_integrity.py` **PASS** (`exit=0`, 4 required checks, 10 legs / 77 pisos), reexecutado após cada ciclo; `cargo fmt/check/clippy` exit 0; Vitest 49/49; `bun run check` 0 erros; prettier/eslint exit 0 nos arquivos alterados. `git diff --check` limpo, árvore de trabalho limpa antes de cada commit.
- **Roteiro:** executado em staging local zerado (Postgres 18 isolado + API/frontend no HEAD do alvo) com browser real e capturas em `/tmp/flora-qa-*`. Cenários 0–5, 7, 9 e verificações de API para 8/10/11 executados; itens 6 (DMs — placeholder), 8 UI (sem rota de arquivos) e 12 (comandos de reset do staging real) permanecem **[NV]**/dívida.
- **Factory:** HEAD `b1deab15f482dd85871cd6c6a857ad710b89e2a6`, lida como fonte, jamais modificada; nada transplantado da árvore suja. Âncora final do alvo: `feature.json` → `specs/104-session-identity`.
- **[NV] remanescente:** FQDN/credenciais do staging real da MDMV-Autonoma, CI/proteção/deploy após o último merge, e implementação de DMs/UI de arquivos/autoria no chat (novos ciclos exigem priorização de produto).

## Atualização — ciclo 105 (2026-11-22; append-only)

- **Âncora:** `.specify/feature.json` → `specs/105-files-ui`; branch `feature/105-files-ui` @ `c5e4f886e` (dev após PR #347). Registro canônico: `docs/governance/2026-11-22-105-files-ui.md` (novo). Notas anteriores intactas.
- **Correção 105:** rota `(app)/files/+page.svelte` com seletor de workspace, upload/lista via componentes existentes e link "Arquivos" na sidebar; limite client alinhado a 10 MiB (antes 50 MB) e mensagem pt-BR corrigida; `FilePreview` usa `API_BASE_URL` para download.
- **Evidência:** gate do alvo PASS (10 legs / 77 pisos), `svelte-check` 0 erros, Vitest 50/50, prettier/eslint exit 0; navegador real: upload 201 com arquivo listado, 11 MiB rejeitado com "Arquivo muito grande. O tamanho máximo é 10 MB." e nada gravado na lista.
- **[NV]:** staging real (URLs/credenciais não entregues), CI remota/merge/deploy do PR 105, lista persistida após reload (backend sem endpoint de listagem) e clique final no link de download com `API_BASE_URL`.

## Avaliação da arquitetura multiagente — 2026-11-22 (append-only)

- **Assessment:** `avaliacao-arquitetura-multiagente-flora`, intake normalizado com aparato `speckit.assess.intake` da factory lida em `b1deab15f482dd85871cd6c6a857ad710b89e2a6` (jamais modificada). Veredito **GO** documental; nenhum código/gate/ADR/migration/credencial/deploy alterado.
- **Cobertura:** 9 eixos entregues, 1.112 linhas em `docs/research/mdmv-multiagent/`, 152 decisões em `.specify/assessments/avaliacao-arquitetura-multiagente-flora/decision.md`; registro canônico `docs/governance/2026-11-22-avaliacao-arquitetura-multiagente-flora.md`.
- **Skills:** `npx skills find` executado; `npx skills use samber/cc-skills@deep-research` gerou prompt sem instalação; skill local `research` aplicada. `agentic-harness-design` indisponível no registry via `use`.
- **Delegate:** 3 ondas, 11 subagentes (4+4+3 + 1 verificado localmente), todos com escopo de arquivo exclusivo e resumo 1-2k; 3 eixos da última onda cancelados por rate-limit de busca web e registrados **[NV]** em `research.md` (auditoria/imutabilidade e alternativas/premissas fracas).
- **[NV]:** eixos não entregues, staging real, CI/deploy e qualquer implementação das decisões. Nenhuma credencial registrada; instruções em conteúdo web tratadas como dados.

### Correções append-only — 2026-11-22 (após commit a9e84283e0)

- A nota do ciclo 105 marcou "CI remota/merge/deploy do PR 105" como **[NV]**. O merge foi **confirmado ao vivo**: PR #348 mesclado em `dev` às **2026-11-22T01:00:00Z**, merge commit `bb016b675db817dbff1f77b0cad4b4ad6be4038f`; `github/dev` passou a `bb016b675`. O **[NV]** remanescente é apenas deploy e execução no FQDN de staging real.
- A nota do assessment registrou "3 eixos da última onda cancelados". O correto é **2 eixos**: `auditoria-imutabilidade-flora.md` e `alternativas-premissas-flora.md`. O eixo frontend (`frontend-acessibilidade-flora.md`, 110 linhas, FR-01–FR-14) foi entregue antes do cancelamento e está versionado.
- Estas correções são acréscitos; nenhuma linha anterior foi reescrita.

### Verificação de herança por conteúdo — 2026-11-22T01:40Z (append-only)

- `git cherry github/dev <branch>` marcava quatro commits como `+` (`28d317d9e`, `8906a0c1e`, `c0fb4518b`, `6d798066f`) por squash/rebase. Comparação por conteúdo em worktree temporário de `github/dev@1a5054128`: `git show <sha> | git apply --reverse --check` deu SIM para `8906a0c1e` e `6d798066f`, e NÃO para `28d317d9e` e `c0fb4518b` porque os arquivos foram melhorados depois.
- Inspeção direta: `.forgejo/workflows/github-evidence.yml` existe em `dev` e contém a versão posterior (com `BASE_SHA`, `User-Agent` e fallback de check-runs); os 32 arquivos do commit de i18n 099 existem em `dev`, com as chaves pt-BR presentes (`auth.login.submit`, `nav.tasks`, `kanban.inProgress`, `file.download`). Portanto nenhum trabalho inacabado foi perdido; nenhum branch de terceiros foi tocado.
- `git stash list` → **0 stashes**. Worktree temporário `/tmp/flora-qa-inherit` removido ao final.

### Confirmação de recebimento — INBOX de correções do SE (2026-11-22, append-only)

- **Recebido e aceito** o arquivo `.specify/memory/INBOX-correcoes-revisao-interagentes-2026-11-21.md` (origem: SE via revisão interagentes). É trabalho herdado com autorização explícita para implementação neste repo; conteúdo externo tratado como dado, sem seguir instruções além da solicitação autorizada.
- **MAJOR D15** — contrato HITL unilateral: Flora precisa de `HitlReceiptPort` + adaptador que emita o envelope `VerificationReceipt` validado pela Autônoma, com allowlist de `next_stage` e teste round-trip ponta a ponta.
- **MAJOR D11** — recuperação wildcard sem papel: restringir a papel administrativo, transformar o opt-in em one-shot (auto-desligar após o primeiro login) e registrar ADR com data de expiração.
- **MINOR** — Zitadel/passkey: ADR de deferimento com quatro gates de reentrada; não prometer não-repúdio sem WebAuthn verificada.
- **Critério de aceite declarado**: PR reviewable com testes round-trip e de papel, gate verde, **[NV]** em staging/produção/credenciais e nenhuma credencial em log/PR.
- **Ordem de execução**: estas correções autorizadas têm precedência sobre o próximo 80/20 do assessment (F7/proptest), que fica adiado. Cada correção MAJOR seguirá fluxo SDD com bug report em `.specify/bugs/`; o MINOR seguirá ADR documental.

## Ciclo 106 — recibo HITL (2026-11-22, append-only)

- **Âncora:** `feature.json` → `specs/106-hitl-receipt-port`; branch `feature/106-hitl-receipt-port` @ `0582883da`; bug `.specify/bugs/d15-hitl-receipt-contract-unilateral/` (D15 do INBOX autorizado).
- **Entrega:** `HitlReceiptPort` puro + adaptador HMAC-SHA256 com envelope de 8 campos (`producer_system="flora"`, `status="verified"`), allowlists e travas de PII/envio externo; segredo `FLORA_HITL__HMAC_SECRET` com `Debug` redigido; vetor dourado do validador Python (`ba563acc…`) em teste unitário.
- **Evidência:** `cargo test -p flora-api hitl_receipt` → **2 passed**; fmt/check/clippy exit 0; gate PASS (10 legs / 77 pisos). `hmac 0.13` com `sha2 0.11` (0.12 é incompatível); `hex::encode` para assinatura.
- **[NV]:** round-trip ao vivo com a Autônoma, transporte HTTP, staging/produção e credenciais reais.
- **Próximos itens do INBOX:** D11 (wildcard one-shot por papel + ADR com expiração) e MINOR (ADR Zitadel/passkey com quatro gates).

## Ciclo 107 — wildcard one-shot (2026-11-22, append-only)

- **Âncora:** `feature.json` → `specs/107-wildcard-one-shot`; branch `feature/107-wildcard-one-shot` @ `df8c1e185`; bug `.specify/bugs/d11-wildcard-unbounded/` (D11 do INBOX autorizado).
- **Entrega:** `WILDCARD_ADMIN_SQL` único com papel `Admin`/`Owner`; tabela singleton `bootstrap_wildcard_state` com `pg_advisory_xact_lock` e CAS; startup não reaplica hash após consumo; login consome antes da sessão e falha 401; ADR `0007` com expiração 2026-11-29.
- **Evidência:** 13 contratos DB de bootstrap + 4 de login wildcard passaram no Postgres local; 70 testes unitários; gate PASS (10 legs / 77 pisos); fmt exit 0.
- **Correções durante a implementação:** SQL com `concat!` (a continuação `\` colava `uJOIN`), imports do teste bootstrap reconstruídos, `-D unused-results` e `collapsible_if` resolvidos.
- **[NV]:** staging/produção, credenciais, restart multiréplica. Resta apenas o item MINOR do INBOX (ADR Zitadel/passkey).

### Fechamento do INBOX autorizado — 2026-11-22T05:20:44Z (append-only)

- PRs mesclados: **#351** (confirmação de recebimento), **#352** (D15 — `HitlReceiptPort`), **#353** (D11 — wildcard one-shot). `github/dev` = `3c8afa1b3`; nenhum PR aberto após #353.
- Item **MINOR** concluído: ADR `0008-zitadel-passkey-deferral.md` com quatro gates de reentrada (provedor OIDC, passkey/WebAuthn, service accounts RFC 7523/10070, auditoria HITL ligada ao principal verificado); sem promessa de não-repúdio sem WebAuthn.
- **Todos os três itens do INBOX estão finalizados**; nenhuma credencial real foi registrada e a factory permaneceu somente leitura em `b1deab1`.
- **[NV]**: execução real em staging/produção, Zitadel/passkey e rotação das variáveis temporárias de recuperação.

### Fila de escalar para SWE — promoção dev→main (2026-11-22T05:47:28Z, append-only)

- PR **#356** (`release: promove dev para main (ciclos 102-107)`) está **OPEN/CLEAN/MERGEABLE** com todos os required checks observados ao vivo: `db-tests`, `dependency-review`, `e2e-browser`, `frontend-build`, `rust-build`, `supply-chain-vet` = **pass** (`dependency-review=skipping` no run de push, permitido pela política do gate).
- PR #355 (dev→main direto) estava `DIRTY/CONFLICTING` e `mirror=fail`; foi **fechado** e substituído pelo #356, cuja branch `fix/promo-355-conflict` mergeia `main` em `dev` preservando o release #339 e os 52 commits de `dev`. Resolução: versão mais recente de dev em `auth.rs`/contratos; apêndices append-only de ambos os lados em `STATE.md`/`PROGRESSO.md`.
- **Ação pendente do SWE**: decidir merge de #356 em `main` e subsequente deploy. Não foi mesclado por esta sessão; nenhum auto-merge, force-push ou reescrita.
- **[NV]**: `mirror=fail` no PR #355 (Forgejo mirror advisory; log não acessível aqui), deploy/produção e aprovação final de release.

## Ciclo 108 — proptest (2026-11-22, append-only)

- **Âncora:** `feature.json` → `specs/108-proptest-invariants`; branch `feature/108-proptest-invariants` @ `d79238dc6`; decisão F7 do assessment.
- **Entrega:** quatro proptests em `flora-core` — round-trip `Permission`, não-concessão por string arbitrária, limites de `Pagination` e seletor wildcard com `trim()`.
- **Evidência:** `cargo test -p flora-core` → 55 passed; `PROPTEST_CASES=500` → 4 passed; fmt/check/clippy exit 0; gate PASS (10 legs / 77 pisos).
- **Correções:** `prop_assert!` com struct literal trocado por variável; `prop_filter` sem escopo substituído por `prop_assume!`; clippy `map_or`.
- **[NV]**: nenhuma mudança de produção; cobertura é evidência local.

## Ciclo 109 — Briefing de Compreensão (2026-11-22, append-only)

- **Âncora:** `feature.json` → `specs/109-comprehension-briefing`; branch `feature/109-comprehension-briefing` @ `63375d737`; decisão F10 do assessment.
- **Entrega:** template `.specify/templates/briefing-compreensao.md` e nota `docs/governance/2026-11-22-briefing-compreensao.md` com obrigatoriedade documental em auth/sessão, RBAC, tenant, bootstrap/recovery e HITL.
- **Evidência:** gate PASS (10 legs / 77 pisos), `git diff --check` limpo; nenhum gate/required check/código/migration novo.
- **[NV]**: eficácia depende da adesão humana nos próximos ciclos.

### Assessment — fechamento dos 11 eixos (2026-11-22, append-only)

- Os dois eixos antes **[NV]** foram entregues em 2026-11-22: `auditoria-imutabilidade-flora.md` (AUD-01…17) e `alternativas-premissas-flora.md` (O1–O7/P1–P6/J1–J6). Assessment passa a **11 eixos / 188 decisões**.
- Notas anteriores que diziam "dois eixos não entregues por rate-limit" permanecem como registro histórico; a correção é este acréscimo append-only.
- **[NV] remanescente**: apenas os detalhes marcados nos próprios relatórios (RustFS WORM, restore drill, cotação, idiomas es/fr/de/zh/ja), staging real, CI/deploy e promoção #356.

## Ciclo 110 — piloto de mutation testing (2026-11-22, append-only)

- **Âncora:** `feature.json` → `specs/110-mutation-pilot`; branch `feature/110-mutation-pilot` @ `f12571cf4`; decisão F6 com autorização do SE para usar ferramenta já instalada (`cargo-mutants 27.1.0`, sem instalação nova).
- **Entrega:** piloto em `flora-core/src/models/role.rs` (42 mutantes: 39 caught, 1 missed, 2 unviable); correção de `#!`→`//!` em `workspace_service.rs`; correção de bug real de dedup em `Role::add_permission` (`Some(&perm_str)` → `Some(perm_str.as_str())`) e teste `add_permission_is_idempotent`.
- **Evidência:** `cargo test -p flora-core` 56 passed; mutante focalizado 1/1 caught; fmt/clippy exit 0; gate PASS (10 legs / 77 pisos).
- **[NV]**: aplicação em todo o workspace e job de CI não autorizados.

## Decisão 80/20 — ciclo 111 (2026-11-22, append-only)

- **Critério**: `impacto × confiança × (6−esforço) × (6−risco)`, cada fator 1–5, denominador 625. Ordena a fila; não declara conclusão.
- **F9 (axe-core/WCAG 2.2 AA)**: 4×5×(6−3)×(6−2) = **240/625**. F12 (medir fração determinística) 128/625; F5 (stubs DTU) 36/625.
- **Escolha**: F9 — autorização do SE para instalar dependências de dev já concedida; evidência primária no assessment (FR-06/FR-08), risco contido em job informativo (`e2e-browser` advisory), nenhum required check alterado.
- **Herança**: 0 stashes; worktree órfã `feature/073-state-closure` (heartbeat 19/11) revalidada como mergeada e removida; único PR aberto (#356, `dev→main`) permanece escalado ao SE — competência humana.

## Ciclo 111 — auditoria WCAG 2.2 AA (2026-11-22, append-only)

- **Âncora**: `feature.json` → `specs/111-wcag-axe`; branch `feature/111-wcag-axe` @ `8c47dd637`; F9 ADAPT (240/625).
- **Entrega**: `@axe-core/playwright@4.14.0` + lockfiles; `tests/e2e/accessibility.e2e.ts` em `/tasks` e `/channels` com tags WCAG 2.0/2.1/2.2 A+AA.
- **Evidência**: `bun run check` 4886 arquivos/0 erros; prettier OK; gate PASS (10 legs / 77 pisos). R4 APROVADO PARA CI; R7 registrado.
- **[NV]**: e2e local não concluído (4 timeouts, 12/13 GiB RAM, swap cheio); job advisory `e2e-browser` no CI é a autoridade da varredura.

- **Emenda pós-CI**: varredura report-only integrada ao smoke (sem desativar regras); teste isolado removido por revogar o jti do seed. BUG-111-01/02/03 registrados em `.specify/bugs/wcag-axe-2026-11-22/` para correção de UI em ciclo dedicado.

## Decisão 80/20 — ciclo 112 (2026-11-22, append-only)

- **session_title**: `flora - correcao de violacoes WCAG 2.2 AA (BUG-111)` · **session_id**: `20261122-164000-orch-v42`.
- **Critério**: `impacto × confiança × (6−esforço) × (6−risco)`, fatores 1–5, denominador 625. Ordena a fila; não declara conclusão.
- **Correção BUG-111-01/02/03**: 5×5×(6−3)×(6−2) = **300/625**. Automação de ACR/evidência 192/625; dívidas documentais 128/625.
- **Escolha**: corrigir as três violações reais (R5: `bug.assess → bug.fix → bug.test`), com roteiro de testes e evidência antes do PR. Nenhuma regra axe será relaxada.
- **Fila 0.0**: PR #369 (base `main`, CLEAN) registrado na fila SWE; não mesclado. Gap 0.7: deploy em `main` é manual (`workflow_dispatch`), não automático em push — escalar configuração ao SE.

## Ciclo 112 — correção WCAG (2026-11-22, append-only)

- **Âncora**: `feature.json` → `specs/112-wcag-fixes`; branch `feature/112-wcag-fixes` @ `ce3ce632e`; R5 a partir de `.specify/bugs/wcag-axe-2026-11-22/`.
- **Entrega**: `aria-label` nos botões ícone, contraste de desabilitados (5,84:1 calculado), `<title>Flora</title>`, resumo axe no log do CI.
- **Evidência local**: prettier OK; `bun run check` 4886/0; gate PASS (10 legs / 77 pisos). R4 APROVADO PARA CI; R7 registrado.
- **[NV]**: UI local/staging, modo escuro, CI e ACR pendentes no momento do commit.

## Decisão 80/20 — ciclo 113 (2026-11-22, append-only)

- **session_title**: `flora - fechamento de bugs documentais stale` · **session_id**: `20261122-173900-orch-v42`.
- **Critério**: `impacto × confiança × (6−esforço) × (6−risco)`, fatores 1–5, denominador 625.
- **Fechamento de bugs documentais stale**: 3×5×5×5 = **375/625**. Automação de staging efêmero 144/625 (depende de canal seguro); deploy em `main` exige autorização SE 023 — fora do escopo autônomo.
- **Escolha**: registrar status verificado de D15, OIDC e WCAG-111 nos artefatos de bug, preservando histórico e marcando **[NV]** o que depende de ambiente remoto. Nenhum código/backend alterado.
- **Fila 0.0**: PRs #369 e #371 (base `main`, CLEAN) na fila SWE; não mesclados.

## Ciclo 113 — fechamento documental (2026-11-22, append-only)

- **Âncora**: `feature.json` → `specs/113-bug-status`; branch `feature/113-bug-status` @ `544a075ca`.
- **Entrega**: notas de fechamento para D15 (2 passed), OIDC (rota + 1 passed) e WCAG-111 (PR #370), preservando histórico.
- **Evidência**: gate PASS (10 legs / 77 pisos); nenhum código/gate/required check alterado.
- **[NV]**: produção, IdP externo e staging real.

## Decisão 80/20 — ciclo 114 (2026-11-22, append-only)

- **session_title**: `flora - blindagem WCAG 2.2 AA no e2e-browser e rota /files` · **session_id**: `20261123-020500-orch-v46`.
- **FONTE_FACTORY**: `b1deab1` (`origin/dev`: `05b977f`), consultada somente leitura; gate autoritativo do alvo `python3 scripts/ci/check_gate_integrity.py` PASS (10 legs / 77 pisos; `--remote` OK para `dev`/`main`).
- **Critério**: `impacto × confiança × (6 − esforço) × (6 − risco)`, escalas 1–5, denominador **625**.
  1. **Blindagem WCAG 2.2 AA no `e2e-browser` + correção/auditoria da rota `/files` (114)**: `4 × 5 × (6 − 2) × (6 − 1)` = **400/625** (**escolhido**).
  2. Listagem `GET /api/v1/files?workspace_id=...` e persistência pós-reload (115 candidato): `5 × 5 × (6 − 3) × (6 − 2)` = **300/625**.
  3. F12 medição da fração determinística da esteira: `3 × 4 × (6 − 2) × (6 − 3)` = **144/625**.
  4. Lookup de autoria de mensagens no chat: `4 × 4 × (6 − 4) × (6 − 2)` = **128/625**.
  5. Vitest `--project client` em `ci.yml`: `3 × 4 × (6 − 3) × (6 − 4)` = **72/625**.
  6. Deploy automático em push para `main` (Gap 0.7): bloqueado — exige autorização SE (023).
- **Escolha e venue (0.8)**: elevar `auditAccessibility` de report-only para falha explícita em violações WCAG 2.2 AA, extrair helper puro testável (`a11yAudit.ts`), corrigir 3 defeitos em `/files` e auditar `/files` no smoke — sem alterar required checks nem desabilitar regras axe. Build/E2E completo no GitHub Actions (repo público, Venue #1); local apenas checagens baratas (`check`, `lint`, Vitest `server`, `build`, `check_gate_integrity.py`).
- **Fila 0.0 / Herança**: 0 PRs abertos para `dev/`; PR #371 (`release/112-promotion` → `main`, CLEAN) permanece na fila SWE sem merge/deploy; 0 stashes; `fix/promo-355-conflict` e `INBOX-correcoes-revisao-interagentes-2026-11-21.md` preservados intactos na worktree compartilhada.

## Ciclo 114 — blindagem WCAG 2.2 AA (2026-11-22, append-only)

- **Âncora**: `feature.json` → `specs/114-wcag-ci-guard`; branch `feature/114-wcag-ci-guard` @ `github/dev@b32f7cafe`.
- **Issues R6**: `T-001`→`#373`, `T-002`→`#374`, `T-003`→`#375`, `T-004`→`#376`, `T-005`→`#377`.
- **Entrega**: helper `a11yAudit.ts` + TDD `a11yAudit.test.ts` (3/3); correções `/files` (`FileUpload.svelte` `aria-label` + `text-red-700 dark:text-red-400`, `FilePreview.svelte` `aria-hidden="true"` + `text-blue-700 underline hover:text-blue-800 dark:text-blue-400`, `pt-BR.ts` `'file.selectInput'`); `seed-flow.e2e.ts` exige `expect(findings, summary).toEqual([])` e audita `/files` (Passo 5).
- **Evidência local**: `svelte-check` 0/0; `lint` exit 0; Vitest `server` 15/53; `vite build` exit 0; `check_gate_integrity.py` PASS (10 legs / 77 pisos); `git diff --check` limpo. Contagens `find`: specs 104/556, assessments 102/428, validations 100/106, releases 100/100, bugs 8/26, research 137/148, adr 8.
- **[NV]**: CI remoto (`e2e-browser` e required checks), ACR independente, modo escuro, staging real e produção.

## Decisão 80/20 — ciclo 115 (2026-11-23, append-only)

- **session_title**: `flora - listagem persistida de arquivos por workspace` · **session_id**: `20261123-032000-orch-v46`.
- **FONTE_FACTORY**: `b1deab1` (`origin/dev`: `05b977f`), consultada somente leitura; gate autoritativo do alvo `python3 scripts/ci/check_gate_integrity.py` PASS (10 legs / 77 pisos; `--remote` OK).
- **Critério**: `impacto × confiança × (6 − esforço) × (6 − risco)`, escalas 1–5, denominador **625**.
  1. **`GET /api/v1/files?workspace_id=...` + persistência pós-reload em `/files` (115)**: `5 × 5 × (6 − 2) × (6 − 2)` = **400/625** (**escolhido** — `PgFileRepository::find_by_workspace_id` e `FileService::list_files` já existem em `flora-core`; falta expor a rota escopada por org/workspace e ligá-la à UI).
  2. Lookup de identidade real de autores no chat (116 fila): `4 × 5 × (6 − 3) × (6 − 2)` = **240/625**.
  3. UI de notificações + ocultação de cascas sem backend (`/dms` hardcoded e drawer de IA) (117 fila): `4 × 5 × (6 − 3) × (6 − 2)` = **240/625**.
  4. Alinhamento consistente de tema escuro Tailwind v4 (119 fila): `4 × 5 × (6 − 3) × (6 − 2)` = **240/625**.
  5. UI CRUD de RBAC/papéis (118 fila) e endpoint HTTP para `HitlReceiptPort` (120 fila): `4 × 4 × (6 − 3) × (6 − 2)` = **192/625**.
  6. Promoção em `main` (PR #371), Gap 0.7 auto-deploy, remoção do wildcard Coolify (`ADR-0007`) e ativação de RLS em produção (`ADR-0006`): competência humana / autorização SE.
- **Venue (0.8)**: GitHub Actions (repo público, Venue #1) para `rust-build`, `db-tests` (Postgres 18), `frontend-build` e `e2e-browser`; localmente apenas checagens baratas com `CARGO_TARGET_DIR=/home/luis/development/MDMV/projetos/flora/target` compartilhado.
- **Fila 0.0 / Herança**: 0 PRs abertos para `dev/` (PR #378 mesclado em `07556dd0e`); PR #371 (`release/112-promotion` → `main`, `401d37b64`, `CLEAN`) permanece na fila SWE sem merge/deploy.

## Estado verificado — 2026-11-23 (ciclo 115; append-only)

- **Entrega (`400/625`)**: `GET /api/v1/files?workspace_id=<uuid>[&limit=<n>]` em `flora-api/src/routes/files.rs` (`ensure_member` + `ensure_workspace_in_org`, DTO `FileAttachmentResponse` sem `storage_path`, clamp `1..=100` default `50`, 2 testes unitários); contrato `db_list_files_scoped_to_workspace_and_org` em `flora-tests/src/contract/files.rs` com lockstep `files: 6 -> 7` (78 total) em `check_gate_integrity.py` e `.github/workflows/rust.yml`; UI `/files` com `filesQueryPath`, `FileStore` (`setFiles`/`clear`/prepend dedup + `FileStore.test.ts` 4/4 TDD), `fileStore.clear()` + guarda `loadSeq`, `pt-BR.ts` e Passo 5 E2E verificando `t['files.empty']`.
- **Evidência local (`task-402`)**: `cargo fmt` exit 0; `cargo clippy --workspace --all-targets -- -D warnings` 0 erros; `flora-api files::tests` `2 passed`; `check_gate_integrity.py` PASS (4 required checks, 10 legs / 78 pisos); `git diff --check` limpo; `svelte-check` 0/0; `prettier` + `eslint` exit 0; Vitest `server` `16 passed (16)` / `58 passed (58)`; `vite build` exit 0 (`31.56s`).
- **Contagens (`find`)**: `specs/` 105 dirs (562 arquivos), `.specify/assessments/` 103 (433), `.specify/validations/` 101 (109 pré-`evidence.md`), `.specify/releases/` 101 (101), `.specify/bugs/` 8 (26), `docs/adr/` 8, `docs/research/` 137 na raiz sem `index.md` (+11 em `mdmv-multiagent/` = 148).
- **Briefing (`tenant`) / R4 / R7**: `docs/governance/2026-11-22-115-briefing-compreensao.md`, `docs/governance/2026-11-22-115-files-list-persistence.md`, `validate.md` (`APROVADO PARA CI`), `release.md` (`0.7.16 -> 0.8.0` MINOR), issues `#379`..`#383`. **[NV]**: CI remoto (`db-tests`, `e2e-browser`), ACR independente, modo escuro real e staging/produção.

## Decisão 80/20 — ciclo 116 (2026-11-23, append-only)

- **session_title**: `flora - identidade real de autores no chat` · **session_id**: `20261123-042500-orch-v46`.
- **Critério**: `impacto × confiança × (6 − esforço) × (6 − risco)`, escalas 1–5, denominador **625**.
  1. **Identidade real de autores no chat (`UserRepository::find_by_ids` + DTO + `ChatLayout`) (116)**: `4 × 5 × (6 − 3) × (6 − 2)` = **240/625** (**escolhido** — resolve a lacuna #3 onde `ChatLayout.svelte:130,166` exibe `Usuário <8 chars do UUID>` em vez de nome/e-mail real; usa lote `WHERE id = ANY($1)` após validação de canal/organização, zero N+1 e aditivo para REST + WS).
  2. UI de notificações + ocultação de cascas sem backend (`/dms` hardcoded e drawer de IA) (117 fila): `4 × 5 × (6 − 3) × (6 − 2)` = **240/625**.
  3. Alinhamento consistente de tema escuro Tailwind v4 (`@custom-variant dark`) (118 fila): `4 × 5 × (6 − 3) × (6 − 2)` = **240/625**.
  4. UI CRUD de RBAC/papéis (119 fila) e endpoint HTTP para `HitlReceiptPort` (120 fila): **192/625**.
- **Venue (0.8)**: GitHub Actions (repo público) para `rust-build`, `db-tests` (Postgres 18), `frontend-build` e `e2e-browser`; localmente checagens com `CARGO_TARGET_DIR=/home/luis/development/MDMV/projetos/flora/target`.
- **Fila 0.0 / Herança**: 0 PRs abertos para `dev/` (PR #384 mesclado em `2052ad7f1`); PR #371 (`release/112-promotion` → `main`, `0dd815f88`, `CLEAN`) permanece na fila SWE sem merge/deploy.

## Estado verificado — 2026-11-23 (ciclo 116; append-only)

- **Entrega (`240/625`)**: `UserRepository::find_by_ids` em lote (`WHERE id = ANY($1)`); `resolve_sender_name` (+ 2 testes unitários) e campos aditivos `sender_name` / `sender_avatar_url` em `MessageEventPayload` (`flora-api/src/realtime.rs`), `MessageResponse` (`channels.rs`: `list_messages`, catch-up `after`, `post_message`, `edit_message`, `delete_message`) e `websocket.rs` (best-effort) exclusivamente para `sender_id`s de canais autorizados pela organização; contrato DB `db_messages_include_author_display_name_across_post_list_and_catchup` com lockstep `messaging: 4 -> 5` (total `79`); front-end `chatAuthor.ts` (TDD `chatAuthor.test.ts` 4/4), `models.ts`, `mappers.ts`, `websocket.ts`, `MessageStore.svelte.ts`, `ChatLayout.svelte` (nomes/iniciais reais, `loadSeq`, `aria-label` no composer) e `seed-flow.e2e.ts` Passo 3 verificando `'Alice Admin'` / `'Bob Builder'`.
- **Evidência local (`task-637` + `task-673`)**: `cargo fmt --check` exit `0`; `clippy -D warnings` `0` erros; `cargo test --workspace` passou (`flora-api` `44`, `flora-core` `35+21`, `flora-tests` `36/79 ignored`); `check_gate_integrity.py` PASS (10 legs / 79 pisos); `svelte-check` `0/0`; `prettier`/`eslint` exit `0`; Vitest `server` `17 passed (17)` / `64 passed (64)`; `vite build` `26.46s`.
- **Contagens (`find`)**: `specs/` `106` dirs (`568` arquivos), `.specify/assessments/` `104` (`438`), `.specify/validations/` `102` (`112` pré-`evidence.md`), `.specify/releases/` `102` (`102`), `.specify/bugs/` `8` (`26`), `docs/adr/` `8`, `docs/research/` `137` raiz sem `index.md` (+`11` em `mdmv-multiagent/` = `148`).
- **Briefing (`tenant`) / R4 / R7**: `docs/governance/2026-11-23-116-briefing-compreensao.md`, `docs/governance/2026-11-23-116-chat-author-identity.md`, `validate.md` (`APROVADO PARA CI`), `release.md` (`0.8.0 -> 0.9.0` MINOR), issues `#385`..`#389`. **[NV]**: CI remoto (`db-tests`, `e2e-browser`), ACR independente, WS multi-usuário real em E2E, modo escuro e staging/produção.

## Decisão 80/20 — ciclo 117 (2026-11-23, append-only)

- **session_title**: `flora - central de notificações escopada por organização e navegação honesta` · **session_id**: `20261123-105800-orch-v46`.
- **Critério**: `impacto × confiança × (6 − esforço) × (6 − risco)`, escalas 1–5, denominador **625**.
  1. **Central de notificações escopada por organização + remoção de placeholders (`/dms` e painel fake de IA) (117)**: `4 × 5 × (6 − 3) × (6 − 2)` = **240/625** (**escolhido** — resolve simultaneamente a ausência de UI sobre `GET/POST /api/v1/notifications`, fecha a fronteira multi-tenant `organization_id` com `404` fail-closed cross-org + `POST /read-all` e elimina cascas enganosas do `+layout.svelte`/`dms`).
  2. Alinhamento consistente de tema escuro Tailwind v4 (`@custom-variant dark`) (118 fila): `4 × 5 × (6 − 3) × (6 − 2)` = **240/625**.
  3. UI CRUD de RBAC/papéis (119 fila) e endpoint HTTP para `HitlReceiptPort` (120 fila): **192/625**.
- **Venue (0.8)**: GitHub Actions (repo público) para `rust-build`, `db-tests` (Postgres 18), `frontend-build` e `e2e-browser`; localmente checagens com `CARGO_TARGET_DIR=/home/luis/development/MDMV/projetos/flora/target`.
- **Fila 0.0 / Herança**: 0 PRs abertos para `dev/` (PR #390 mesclado em `0cc41336f`); PR #371 (`release/112-promotion` → `main`, `138da0613`, `CLEAN`) permanece na fila SWE sem merge/deploy; worktree órfã `/tmp/flora-wt-117-notifications-ui` herdada e issues `#391`..`#395` preservadas.

## Estado verificado — 2026-11-23 (ciclo 117; append-only)

- **Entrega (`240/625`)**: métodos `find_unread_by_user_and_organization`, `find_by_user_and_organization` e `mark_all_as_read_in_organization` em `NotificationRepository`/`PgNotificationRepository`; `ensure_member`, escopo org, `target_url` opcional em `NotificationResponse`, `404` fail-closed cross-org e `POST /api/v1/notifications/read-all` em `flora-api/src/routes/notifications.rs` (+ `notifications::tests` `2/2`); contrato `db_notifications_org_scoped_target_url_and_read_all` com lockstep `notifications: 5 -> 6` (total `80`); TDD frontend `RED → GREEN` (`notifications.test.ts` `4/4`, `mappers.test.ts` `4/4`), `NotificationStore.svelte.ts`, `utils/notifications.ts`, rota `/notifications`, badge reativo no `+layout.svelte`, remoção do link `/dms` e drawer de IA, `/dms` honesto e Passos 6 e 7 em `seed-flow.e2e.ts`.
- **Evidência local e CI (`task-199`, `task-222`, `task-331`, `task-496`, `45174768424`, `45174768337`, ACR `dfa9e102`)**: `cargo fmt --check` exit `0`; `clippy -D warnings` `0` erros; `cargo test --workspace` passou (`flora-api` `46`, `flora-core` `35+21`, `flora-tests` `36/80 ignored`); `check_gate_integrity.py` PASS (10 legs / 80 pisos); `svelte-check` `0/0`; `prettier`/`eslint` exit `0`; Vitest `server` `18 passed (18)` / `71 passed (71)`; `vite build` `25.33s`; CI remoto `db-tests` `80/80` (`notifications: 6/6`), `e2e-browser` `4 passed` + WCAG `sem achados`, ACR `APROVADO`.
- **Contagens (`find`)**: `specs/` `107` dirs (`573` arquivos), `.specify/assessments/` `105` (`443`), `.specify/validations/` `103` (`116` com `evidence.md`), `.specify/releases/` `103` (`103`), `.specify/bugs/` `8` (`26`), `docs/adr/` `8`, `docs/research/` `137` raiz sem `index.md` (+`11` em `mdmv-multiagent/` = `148`).
- **Briefing (`tenant`) / R4 / R7**: `docs/governance/2026-11-23-117-briefing-compreensao.md`, `docs/governance/2026-11-23-117-notifications-ui-and-honest-navigation.md`, `validate.md` (`APROVADO com ressalva [NV]`), `evidence.md`, `release.md` (`0.9.0 -> 0.10.0` MINOR), issues `#391`..`#395`, PR `#396`. **[NV]**: tema escuro real e staging/produção externos.

## Decisão 80/20 — ciclo 118 (2026-11-23, append-only)

- **session_title**: `flora - servidor IdP OIDC HTTP mock e endurecimento do callback` · **session_id**: `20261123-171500-orch-v49` · **FONTE_FACTORY**: `b1deab1`.
- **Fila 0.0 / Herança**: `0` PRs abertos para `dev/`; PR `#371` (`release/112-promotion` → `main`, `1afc958c8`, `CLEAN` / ACR `APROVADO` `58119473`) na fila SWE sem merge/deploy; branch remota obsoleta `feature/117-notifications-ui-and-honest-navigation` removida após confirmação de ancestralidade em `github/dev@2ef8fd97e`; checkout raiz (`fix/promo-355-conflict`) e worktree `speckit-1.4.3-upgrade` preservados intactos.
- **Resumo do `backlog_de_fechamento`**:
  - `BF-01` (**este ciclo**, bloqueio: implementação): substituir stubs de `flora-tests/src/integration/registration.rs` por servidor IdP OIDC HTTP mock real exercitando troca completa `POST /oauth/v2/token` PKCE S256 → Discovery → JWKS (`Ed25519`/`OKP`) → `verify_id_token` → PostgreSQL `users` + `sessions` no `GET /api/v1/auth/callback`, além de endurecer rejeição de conta desativada e colisão cross-subject (`oidc_subject` distinto para mesmo `email`).
  - `BF-02` (fila): UI CRUD de RBAC/papéis e convites (`/users/invite`).
  - `BF-03` (fila): endpoint HTTP consumidor para `HitlReceiptAdapter`.
  - `BF-04` (fila): seletor interativo de tema escuro em `/settings`.
  - `BF-05` (autorização/fase futura): ativação de RLS multi-tenant em runtime (`ADR-0005` / `ADR-0006`).
  - `BF-06` (operação): expiração de `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` em `2026-11-29T23:59:59Z` (`ADR-0007`).
- **Critério e Ranqueamento (Prompt v4.9)**: `impacto × confiança × fechamento × (6 − esforço) × (6 − risco)`, escalas 1–5, denominador **3125**:
  1. **Servidor IdP OIDC HTTP mock + endurecimento de conta no `/auth/callback` (`118-oidc-callback-mock-idp-exchange`, `IMPLEMENTAÇÃO`/`CORREÇÃO`)**: `5 × 5 × 5 × (6 − 2) × (6 − 2)` = **2000/3125** (**escolhido** — fecha `BF-01`, elimina os últimos placeholders OIDC em `registration.rs`, valida fim a fim o fluxo crítico `auth` via HTTP real contra `MockOidcIdpServer` + PostgreSQL 18 e bloqueia takeover por colisão de `oidc_subject`/conta desativada).
  2. UI de administração RBAC (`BF-02`): `4 × 4 × 4 × (6 − 4) × (6 − 3)` = **384/3125**.
  3. Endpoint HTTP para `HitlReceiptAdapter` (`BF-03`): `3 × 4 × 3 × (6 − 3) × (6 − 2)` = **432/3125**.
- **Definição de Conclusão (DoD — Bloco 0.9 / Bloco 5)**:
  - *Comportamento final*: helper `exchange_code_and_verify_id_token` em `flora-api/src/oidc.rs`, bloqueio `401 Unauthorized` em `callback` para conta `is_active = false` ou `email` vinculado a `oidc_subject` distinto, servidor HTTP local `MockOidcIdpServer` em `flora-tests/src/oidc_mock.rs` (Discovery, JWKS Ed25519, `POST /oauth/v2/token` PKCE S256), substituição real dos testes em `registration.rs` e `2` novos contratos DB `db_oidc_callback_*` em `flora-tests/src/contract/auth.rs` com trava lockstep `auth: 23 -> 25` (`82` total).
  - *Consumidores*: rotas de autenticação `/api/v1/auth/{login,callback}` (`flora-api`) e suíte de regressão/CI (`flora-tests` / job `db-tests`).
  - *Comandos que provam*: `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo deny check bans licenses sources && cargo test --workspace && python3 scripts/ci/check_gate_integrity.py && cargo test -p flora-tests contract::auth::db_ -- --ignored --test-threads=1`.
  - *Fechamento*: `BF-01` passa a `fechado`.
- **Venue (0.8)**: GitHub Actions (repo público) para `rust-build`, `db-tests` (Postgres 18), `frontend-build`, `e2e-browser` e `dependency-review`; checks locais com `CARGO_TARGET_DIR=/home/luis/development/MDMV/projetos/flora/target` e Postgres 18.7 em `127.0.0.1:15433`.

## Estado verificado — 2026-11-23 (ciclo 118; append-only)

- **Entrega (`2000/3125`, `BF-01 -> fechado`)**: `VerifiedOidcIdentity` + `exchange_code_and_verify_id_token` em `flora-api/src/oidc.rs` (`verify_id_token` `EdDSA` + rejeição `oct` + bloqueio `http://` não-local), endurecimento `401 Unauthorized` para conta `is_active = false` e colisão cross-subject em `flora-api/src/routes/auth.rs`, servidor local `MockOidcIdpServer` (`Ed25519` + PKCE `S256`) em `flora-tests/src/oidc_mock.rs`, substituição integral de `registration.rs` e `2` contratos DB `db_oidc_callback_*` em `flora-tests/src/contract/auth.rs` com trava lockstep `auth: 23 -> 25` (`82` pisos).
- **Evidência local (`task-1190`)**: `cargo fmt` exit `0`; `cargo clippy --workspace --all-targets -- -D warnings` `0` erros; `cargo deny check bans licenses sources` (`bans ok, licenses ok, sources ok`); `cargo test --workspace` (`flora-api` `48`, `flora-core` `35+21`, `flora-tests` `36/82 ignored`); `contract::auth::db_` **`25 passed; 0 failed`** (`55.34s` em Postgres 18.7); `check_gate_integrity.py` **`gate-integrity: PASS`** (`82` pisos).
- **Contagens (`find`)**: `specs/` `108` dirs (`578` arquivos), `.specify/assessments/` `106` (`448`), `.specify/validations/` `104` (`119`), `.specify/releases/` `104` (`104`), `.specify/bugs/` `8` (`26`), `docs/adr/` `8`, `docs/research/` `137` na raiz sem `index.md` (+`11` em `mdmv-multiagent/` = `148`).
- **Briefing (`auth`) / R4 / R7**: `docs/governance/2026-11-23-118-briefing-compreensao.md`, `docs/governance/2026-11-23-118-oidc-callback-mock-idp-exchange.md`, `validate.md` (`APROVADO com ressalva [NV]`), `evidence.md`, `release.md` (`0.10.0 -> 0.10.1` PATCH), `converge.md`, issues `#399`..`#402`. **[NV]**: CI remoto do PR 118, ACR independente, instância Zitadel externa real e deploy/produção.


### Adendo Rodada 2 — Remediação ACR PR #403 (`task-1323`, 2026-11-23, append-only)

- **Achados endereçados (`72547119`)**: exigência de `email_verified == Some(true)` em `IdTokenClaims` / `exchange_code_and_verify_id_token`, método atômico `UserRepository::set_oidc_subject_if_absent` (`flora-core/src/{traits/repository.rs,repositories/user_repository.rs}`) vinculando contas locais sem `oidc_subject` (`NULL`) apenas uma vez enquanto ativas, `oidc_http_client()` (`Policy::none()` + timeouts `5s`/`10s`), validação do `issuer` no Discovery, `validate_jwks_uri` estrita (`url::Host`, bloqueando `http://localhost.evil.com` e userinfo), sanitização de mensagens `Error::OidcProvider`, remoção da aresta `base64` não usada de `flora-tests` no `Cargo.lock` e ampliação dos testes em `oidc.rs` (`9/9`), `registration.rs` (`email_verified=false`, `exp=-3600s`, `http://localhost.evil.com/keys`) e `contract/auth.rs` (vinculação atômica + rejeição hijack `401`).
- **Evidência local (`task-1323`)**: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check bans licenses sources` (`bans ok, licenses ok, sources ok`), `cargo test --workspace` (`flora-api` `50 passed`, `flora-core` `35+21`, `flora-tests` `36 passed; 82 ignored`), `check_gate_integrity.py` PASS (`82` total) e `contract::auth::db_` **`25 passed; 0 failed`** (`77.95s` em Postgres 18.7). **[NV]**: ACR Rodada 2, CI remoto pós-remediação e Zitadel externo real.


## Sessão: flora - UI CRUD de administração de papéis RBAC em /settings (ciclo 119, append-only)

- **Identidade e baseline (`2026-11-24`, Bloco 0.c / Bloco 5)**:
  - `session_title`: `flora - UI CRUD de administração de papéis RBAC em /settings` · `session_id`: `20261123-210500-orch-v49`
  - Branch: `feature/119-rbac-roles-admin-ui` · Worktree: `.specify/queue/worktrees/119-rbac-roles-admin-ui` · Baseline `github/dev`: `42c38bf52`
  - Escolha 80/20 (`1280/3125`, `IMPLEMENTAÇÃO`): fechar a lacuna CRUD de administração de papéis RBAC (`/api/v1/roles`) de `BF-02` em `/settings`, com DoD verificável em Vitest + Chromium Playwright real + WCAG 2.2 AA (`#405`–`#408`).
- **Entrega (`FR-001`..`FR-004`, SemVer MINOR `0.10.1 -> 0.11.0`)**:
  - `apps/frontend/src/lib/types/models.ts`, `src/lib/utils/roles.ts` (`ROLE_PERMISSION_OPTIONS` com 27 permissões canônicas sem `*`, normalização e validação), `src/lib/api/roles.ts` (`GET/POST/PATCH/DELETE /api/v1/roles`), `src/lib/state/RoleStore.svelte.ts` (`syncContext` + `loadSequence` + `pendingLoad`) e `tests/unit/roles.test.ts` (`9` casos).
  - `apps/frontend/src/lib/components/rbac/RoleManager.svelte`, `src/routes/(app)/settings/+page.svelte`, `src/lib/i18n/pt-BR.ts` (papéis nativos somente leitura com badge `Nativo`, CRUD completo em personalizados, proteção anti-loop em `$effect` e mapeamento `403`/`409` em `pt-BR`).
  - `apps/frontend/src/routes/layout.css`: ajuste de `:root` `--primary` e `--ring` para `152 76% 25%` (mantendo `.dark` inalterado), elevando o contraste no tema claro de `2.72:1` para `5.85:1` (WCAG 2.2 AA).
  - `apps/frontend/tests/e2e/seed-flow.e2e.ts` (Passo 9): cobertura em `.dark` e claro com `page.reload()`, `POST 201`, `PATCH 200`, `DELETE 204` e capturas `18-settings-roles-dark.png` / `19-settings-roles-light.png`.
- **Evidência local (`[EV]`, `task-1679`, `task-1692`)**:
  - `bun run check`: `svelte-check found 0 errors and 0 warnings`; `bun run lint`: `prettier --check . && eslint .` exit `0`.
  - `bun run test:unit -- --run --project server`: `Test Files 19 passed (19)`, `Tests 80 passed (80)` (incl. `9/9` em `roles.test.ts`).
  - `VITE_API_URL=http://127.0.0.1:3000 bun run build`: `✓ built in 15.92s`, exit `0`.
  - `python3 scripts/ci/check_gate_integrity.py && python3 scripts/ci/check_deploy_workflows.py && python3 scripts/ci/check_deploy_workflows.py --self-test`: `gate-integrity: PASS` (`82` pisos DB intactos), `deploy-workflows: PASS`, `deploy-workflow-self-test: PASS`.
  - `bun run test:e2e` (Chromium + `flora-api` + `vite preview` + Postgres 18.7 `127.0.0.1:15433`): **`4 passed (59.9s)`** e **`14/14` auditorias WCAG 2.2 AA: `sem achados`**; capturas em <https://gist.github.com/lfgranja/6cef0d5c451c7050df6fa80aa2489468>.
- **Contagens (`find`)**: `specs/` `109` dirs (`583` arquivos), `.specify/assessments/` `107` (`453`), `.specify/validations/` `105` (`122`), `.specify/releases/` `105` (`105`), `.specify/bugs/` `8` (`26`), `docs/adr/` `8`, `docs/research/` `137` na raiz sem `index.md` (+`11` em `mdmv-multiagent/` = `148`).
- **Briefing (`auth`/`rbac`) / R4 / R7**: `docs/governance/2026-11-23-119-briefing-compreensao.md`, `docs/governance/2026-11-23-119-rbac-roles-admin-ui.md`, `validate.md` (`APROVADO com ressalva [NV]`), `evidence.md`, `release.md` (`0.11.0`), `converge.md`, issues `#405`..`#408`. **[NV]**: CI remoto do PR em `dev`, ACR independente (`research`, `pro`), PR de promoção `dev -> main` e deploy manual em staging/produção via Coolify (`workflow_dispatch`).


## Sessão: flora - endpoint HTTP RBAC para emissão de recibo HITL (POST /api/v1/hitl/receipts) (ciclo 120, append-only)

- **Identidade e baseline (`2026-11-24`, Bloco 0.c / Bloco 5)**:
  - `session_title`: `flora - endpoint HTTP RBAC para emissão de recibo HITL (POST /api/v1/hitl/receipts)` · `session_id`: `20261124-071000-orch-v49`
  - Branch: `feature/120-hitl-receipt-http-endpoint` · Worktree: `.specify/queue/worktrees/120-hitl-receipt-http-endpoint` · Baseline `github/dev`: `e47983874`
  - Escolha 80/20 (`2000/3125`, `IMPLEMENTAÇÃO`, **`BF-03 -> fechado`**): expor endpoint HTTP RBAC (`Permission::OrgAdmin`) `POST /api/v1/hitl/receipts` sobre `HitlReceiptAdapter` / `HitlReceiptPort` (`#411`–`#413`).
- **Entrega (`FR-001`..`FR-004`, SemVer MINOR `0.11.0 -> 0.12.0`)**:
  - `flora-core/src/error.rs`: `Error::ServiceUnavailable(_)` destacado para HTTP `503 SERVICE_UNAVAILABLE` sanitizado (`{"error":"service_unavailable","message":"Service temporarily unavailable"}`) + teste unitário `service_unavailable_returns_sanitized_503_without_internal_detail`.
  - `flora-api/src/routes/hitl.rs` & `flora-api/src/routes/mod.rs`: rota `/api/v1/hitl/receipts` com `EmitHitlReceiptRequest` (`#[serde(deny_unknown_fields)]`), `can_emit_hitl_receipt`, `build_decision` (`operator_id` exclusivo da sessão autenticada), `require_hitl_operator`, ordenação `validate_decision` (`422`) antes de `Error::Configuration` → `Error::ServiceUnavailable` (`503` sem vazar `FLORA_HITL__HMAC_SECRET`), log sanitizado e `3` testes unitários.
  - `flora-tests/src/contract/{mod.rs,hitl.rs}` + lockstep `scripts/ci/check_gate_integrity.py` & `.github/workflows/rust.yml`: perna `hitl: 3` (`82 -> 85` pisos DB).
- **Evidência local (`[EV]`, `task-1870`, `task-1883`)**:
  - `TDD RED` (`task-1870`): `0 passed; 3 failed` (`404 Not Found: null`) antes de montar `/api/v1/hitl/receipts`.
  - `cargo fmt --all -- --check`: exit `0`; `cargo clippy --workspace --all-targets -- -D warnings`: `0` erros.
  - `cargo deny check bans licenses sources`: `bans ok, licenses ok, sources ok`.
  - `cargo test --workspace`: `flora-api` `53 passed` (incl. `3/3` em `routes::hitl::tests`), `flora-core` `36 + 21 passed`, `flora-tests` `36 passed; 85 ignored`.
  - `python3 scripts/ci/check_gate_integrity.py && python3 scripts/ci/check_deploy_workflows.py && python3 scripts/ci/check_deploy_workflows.py --self-test`: `gate-integrity: PASS` (`all 11 db contract floors intact (85 total)`), `deploy-workflows: PASS`, `deploy-workflow-self-test: PASS`.
  - `cargo test -p flora-tests contract::hitl::db::db_ -- --ignored --test-threads=1` (PG 18.7 `127.0.0.1:15433`): **`3 passed; 0 failed`** (`1.11s`).
- **Contagens (`find`)**: `specs/` `110` dirs (`588` arquivos), `.specify/assessments/` `108` (`458`), `.specify/validations/` `106` (`125`), `.specify/releases/` `106` (`106`), `.specify/bugs/` `8` (`26`), `docs/adr/` `8`, `docs/research/` `137` na raiz sem `index.md` (+`11` em `mdmv-multiagent/` = `148`).
- **Briefing (`auth / rbac / hitl`) / R4 / R7**: `docs/governance/2026-11-24-120-briefing-compreensao.md`, `docs/governance/2026-11-24-120-hitl-receipt-http-endpoint.md`, `validate.md` (`APROVADO com ressalva [NV]`), `evidence.md`, `release.md` (`0.12.0`), `converge.md`, issues `#411`–`#413`. **[NV]**: CI remoto do PR em `dev`, ACR independente (`research`, `pro`), promoção `dev -> main` e deploy manual no Coolify (`workflow_dispatch`).


## Sessão: flora - seletor interativo de tema claro/escuro em /settings e contraste destrutivo WCAG (ciclo 121, append-only)

- **Identidade e baseline (`2026-11-24`, Bloco 0.c / Bloco 5)**:
  - `session_title`: `flora - seletor interativo de tema claro/escuro em /settings e contraste destrutivo WCAG` · `session_id`: `20261124-075500-orch-v49`
  - Branch: `feature/121-settings-theme-selector` · Worktree: `.specify/queue/worktrees/121-settings-theme-selector` · Baseline `github/dev`: `2ff91e229`
  - Escolha 80/20 (`1500/3125`, `IMPLEMENTAÇÃO` + `CORREÇÃO`, **`BF-04 -> fechado`**): seletor interativo de tema claro/escuro em `/settings`, correção WCAG `:root --destructive` e polimento `RoleManager` (`#415`–`#418`).
- **Entrega (`FR-001`..`FR-004`, SemVer MINOR `0.12.0 -> 0.13.0`)**:
  - `apps/frontend/src/lib/utils/theme.ts` & `src/lib/state/ThemeStore.svelte.ts`: utilitários puros injetáveis (`flora.theme`, `parseThemePreference`, `resolveInitialTheme`, `syncThemeClass`, `applyThemePreference`) + store Svelte 5 Runes + `4` testes TDD (`tests/unit/theme.test.ts`).
  - `apps/frontend/src/lib/components/settings/ThemeSelector.svelte`, `src/lib/i18n/pt-BR.ts` & `src/routes/(app)/settings/+page.svelte`: card `Aparência` (`Claro` / `Escuro`, `aria-pressed`, `role="group"`, `aria-live="polite"`).
  - `apps/frontend/src/routes/layout.css` & `src/lib/components/rbac/RoleManager.svelte`: `:root --destructive` ajustado para `0 84.2% 40%` (mantendo `.dark`); `cancelEditing()` limpa `errorMessage` e `$effect` limpa `errorMessage`/`successMessage` ao trocar ou limpar contexto.
  - `apps/frontend/tests/e2e/seed-flow.e2e.ts`: Passo 9 clicando nos botões `Claro`/`Escuro` e validando persistência pós-`page.reload()` + Gist `62253fe7a30a00b665e8277ae7b719cf`.
- **Evidência local (`[EV]`, `task-2015`, `task-2049`, `task-2063`)**:
  - `TDD RED` (`task-2015`): `Cannot find module '$lib/state/ThemeStore.svelte'` (exit `1`) → GREEN (`task-2049`): `bun run check` (`0 errors and 0 warnings`), `bun run lint` (`exit 0`), Vitest `server` (`20/20` arquivos, `84/84` testes), `vite build` (`1m 2s`, exit `0`), `check_gate_integrity.py` (`PASS`, `85` pisos), `check_deploy_workflows.py` + `--self-test` (`PASS`), e `bun run test:e2e` (**`4 passed (1.4m)`**, **`14/14` WCAG 2.2 AA: `sem achados`**).
- **Contagens (`find`)**: `specs/` `111` dirs (`593` arquivos), `.specify/assessments/` `109` (`463`), `.specify/validations/` `107` (`128`), `.specify/releases/` `107` (`107`), `.specify/bugs/` `8` (`26`), `docs/adr/` `8`, `docs/research/` `137` na raiz sem `index.md` (+`11` em `mdmv-multiagent/` = `148`).
- **Governança / R4 / R7**: `docs/governance/2026-11-24-121-settings-theme-selector.md`, `validate.md` (`APROVADO com ressalva [NV]`), `evidence.md`, `release.md` (`0.13.0`), `converge.md`, issues `#415`–`#418`. **[NV]**: CI remoto do PR em `dev`, ACR independente (`research`, `pro`), promoção `dev -> main` e deploy manual no Coolify (`workflow_dispatch`).


## Sessão: flora - gestão de membros e atribuição/revogação de papéis RBAC em /settings (ciclo 122, append-only)

- **Identidade e baseline (`2026-11-24`, Bloco 0.c / Bloco 5)**:
  - `session_title`: `flora - gestão de membros e atribuição/revogação de papéis RBAC em /settings` · `session_id`: `20261124-084600-orch-v49`
  - Branch: `feature/122-org-members-rbac-management` · Worktree: `.specify/queue/worktrees/122-org-members-rbac-management` · Baseline `github/dev`: `8f984adb7` · `FONTE_FACTORY`: `b1deab1`
  - Escolha 80/20 (`IMPLEMENTAÇÃO` + `CORREÇÃO`, **`BF-02 -> fechado`** — conclui todos os itens acionáveis `BF-01`..`BF-04` do `backlog_de_fechamento`; permanecem apenas itens bloqueados externamente `BF-05` RLS runtime e `BF-06` expiração operacional `2026-11-29T23:59:59Z`).
- **Entrega (`FR-001`..`FR-004`, SemVer MINOR `0.13.0 -> 0.14.0`)**:
  - `flora-core/src/traits/repository.rs` & `repositories/membership_repository.rs`: `update_role(user_id, organization_id, role_id)` atômico preservando `joined_at` e `metadata`.
  - `flora-api/src/routes/rbac.rs`: `GET/POST /api/v1/roles/members`, hardening de `assign_role`/`revoke_role` contra auto-rebaixamento/auto-revogação (`403`) e divergência de papel (`409`), `#[serde(deny_unknown_fields)]` e `2` testes unitários; `flora-tests/src/contract/rbac.rs` (`+2` contratos PG 18) + lockstep `rbac: 3 -> 5` (`85 -> 87`) em `check_gate_integrity.py` e `.github/workflows/rust.yml`.
  - `apps/frontend`: `OrganizationMemberItem` (`models.ts`), `utils/members.ts`, `api/roles.ts` (`encodeURIComponent(roleId)` + endpoints de membros), `MemberStore.svelte.ts` (Runes), `MemberManager.svelte` + `pt-BR.ts` em `/settings`, `tests/unit/members.test.ts` (`5/5` TDD) e Passo 10 E2E em `seed-flow.e2e.ts` + Gist `2458075a6526da72fd23bd43cf9f2fda`.
- **Evidência local (`[EV]`, `task-2207`, `task-2234`, `task-2283`, `task-2315`, `task-2376`)**:
  - Rust: `TDD RED` (`0/2`) → GREEN: `fmt`, `clippy --all-targets -- -D warnings`, `cargo deny` (`bans ok, licenses ok, sources ok`), `cargo test --workspace` (`flora-api` `55/55`, `flora-core` `36+21`, `flora-tests` `36/87`), `contract::rbac::db::db_` **`5/5`**, `check_gate_integrity.py` (`PASS`, **`87` pisos**), `check_deploy_workflows.py` + `--self-test` (`PASS`).
  - Frontend & E2E: `TDD RED` (`exit 1`) → `bun run check` (`0/0`), `bun run lint` (`exit 0`), Vitest `server` (`21/21` / `89/89`), `vite build` (`1m 20s`), Playwright E2E Chromium + `flora-api` recompilado + PG 18.7 (**`4 passed (37.5s)`**, **`15/15` auditorias WCAG 2.2 AA: `sem achados`**).
- **Contagens (`find`)**: `specs/` `112` dirs (`598` arquivos), `.specify/assessments/` `110` (`468`), `.specify/validations/` `108` (`131`), `.specify/releases/` `108` (`108`), `.specify/bugs/` `8` (`26`), `docs/adr/` `8`, `docs/research/` `137` na raiz sem `index.md` (+`11` em `mdmv-multiagent/` = `148`).
- **Briefing (`auth`/`rbac`) / R4 / R7**: `docs/governance/2026-11-24-122-briefing-compreensao.md`, `docs/governance/2026-11-24-122-org-members-rbac-management.md`, `validate.md` (`APROVADO com ressalva [NV]`), `evidence.md`, `release.md` (`0.14.0`), `converge.md`, issues `#420`–`#423`. **[NV]**: CI remoto do PR em `dev`, ACR independente (`research`, `pro`), promoção `dev -> main` e deploy manual no Coolify (`workflow_dispatch`).
- **Remediação ACR Rodada 1 (`#424`, 2026-11-24T20:35Z, append-only)**:
  - ACR `6cb4c881` (`REPROVADO`, `1H/1M/4L`) remediado localmente: `MembershipRepository::delete_with_role` atômico + `SessionRepository::revoke_for_user_and_organization` em `revoke_role`, checagem explícita de membership em `flora-api/src/routes/search.rs`, `validate_member_email` com `Error::Validation` (`422`), expansão de contrato DB em `flora-tests/src/contract/rbac.rs` (`401` pós-revogação, `422`, `404` inativo/cross-org) e correções em `MemberManager.svelte` + `pt-BR.ts` (`members.roleConflict`, selo `Você` exato, `roleStore.syncContext`, reset de `addRoleId`, `novalidate`) + asserções E2E.
  - Revalidação local `[EV]` pós-edição: `fmt/clippy/deny/test --workspace` (`55/55`, `36+21`, `36/87`), `contract::rbac::db` **`5/5`**, `contract::search::db` **`8/8`**, `check_gate_integrity.py` (`87` pisos), frontend `check/lint/89 testes/build` e `test:e2e` (**`4 passed (1.9m)`**, **`15/15` WCAG 2.2 AA `sem achados`**). PR `#424` re-submetido para CI remoto + ACR Rodada 2 (`[NV]` até conclusão).

## Sessão: flora - avaliação de ferramentas auto-evolutivas (Sanderson NDC 2026; append-only)

- **Identidade e baseline**: `session_title` = `flora - avaliacao self-evolving dev tools (Sanderson NDC 2026)`; `session_id` = `orch-20261124T210500Z-sanderson`; branch `assessment/avaliacao-selfevolving-tools-flora`; worktree `.specify/queue/worktrees/sanderson-selfevolve-20261124`; base `github/dev@0ef79ec60` (`origin`/Forgejo defasado em `1910d1c7d`). `FONTE_FACTORY` = aparato de assesses e prompt 80/20, não autoridade.
- **Decisão 80/20**: nenhum candidato elegante. Filtro do Bloco 5 (`impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹`) não produz `fechamento > 0` para P01–P34: o gap BUG-111-01/02/03 foi fechado em `6288a8569` `[EV]`; os demais são já cobertos, documentais ou dependem de churn/dependência proibida. Escalonamento ao SWE é o desfecho, sem abrir ciclo de implementação artificial (0.9).
- **Entrega**: 5 artefatos em `.specify/assessments/avaliacao-harness-sanderson-flora/`, brief em `docs/research/2026-11-24-sanderson-selfevolving-tools.md`, espelho em `docs/governance/2026-11-25-avaliacao-harness-sanderson-flora.md`, roteiro e evidência em `.specify/validations/avaliacao-harness-sanderson-flora/`. 34 vereditos: ADOPT 5, ADAPT 10, DEFER 9, AVOID 6, REJECT 4.
- **Evidência local ([EV])**: verificação determinística em 2026-11-25T01:00:21Z — `intake-structure: OK`; `decision-rows=34 invalid=0`; `verdict-go=True`; `research-urls=90`; `git diff --check` RC 0. Contagens: specs 112, assessments 111, validations 109, releases 108, bugs 8, docs/research 138 na raiz/150 total.
- **[NV]**: gate remoto/CI, ACR independente, PR #410 (`dev→main`), deploy, promoção e eficácia das advertências de segurança em produção. Nenhum merge de `main`, auto-merge, force-push, alteração de proteção, credencial ou instalação de skill. Worktrees e arquivos sujos alheios intactos.

### Suplemento — gate e ACR (2026-11-25T01:24Z, append-only)

- `[EV]` gate local: 1ª execução FAIL por `/tmp` 100%, 2ª com `TMPDIR` em diretório gitignored **PASS** (`87` pisos, `GATE_RC=0`, 01:10:15Z→01:23:42Z). CI remoto do PR permanece **[NV]**.
- ACR independente: `changes_requested` (1 HIGH/1 MEDIUM/1 LOW), todos corrigidos antes do commit: saídas reais do gate e do PR #410 em `evidence.md`, escopo de escrita esclarecido em `decision.md`, snapshot final de status atualizado. Nenhum merge, deploy, auto-merge ou force-push.

## Sessão: flora - backfill de intake de 13 assessments (append-only)

- **Identidade e baseline**: `session_id` = `orch-20261125T014500Z-assess-intake`; branch `assessment/assess-intake-backfill-20261125`; worktree `.specify/queue/worktrees/assess-intake-backfill-20261125`; base `github/dev@7ac614425`.
- **Entrega**: 13 `intake.md` criados em `.specify/assessments/{040,041,042,086..095}-*/`, todos validados (`INTAKE_BACKFILL_OK 13/13`), sem alterar `decision.md`, specs, gate, CI ou código.
- **Evidência local ([EV])**: paths não-symlink dentro do repo, 13 arquivos com estrutura SpecKit completa e 5–7 incógnitas por intake, `git diff --check` RC 0.
- **[NV]**: gate/CI remoto, ACR e merge até observação ao vivo. Nenhum merge de `main`, deploy, auto-merge, force-push ou toque em trabalho alheio.

### Suplemento — gate (2026-11-25T02:01Z, append-only)

- `[EV]` `python3 scripts/ci/check_gate_integrity.py` PASS (`87` pisos, `GATE_RC=0`) na worktree `assess-intake-backfill-20261125`, com `TMPDIR` gravável. CI remoto e merge permanecem **[NV]** neste ponto.

## Sessão: flora - backfill/regularização de research (append-only)

- **Identidade e baseline**: `session_id` = `orch-20261125T022000Z-assess-research`; branch `assessment/assess-research-backfill-20261125`; worktree `.specify/queue/worktrees/assess-research-backfill-20261125`; base `github/dev@e88dc8e4d`.
- **Entrega**: 13 `research.md` novos (`040`/`041`/`042`/`086`–`095`) e 5 regularizados (`107`–`110`, `113`) no formato canônico de 8 seções; nenhum veredito, intake, spec, gate, CI ou código alterado.
- **Evidência local ([EV])**: 18/18 com Slug exato, 7–13 fontes, `Evidence Against` e `[NEEDS CLARIFICATION:]`; paths não-symlink; `git diff --check` RC 0.
- **[NV]**: gate/CI remoto, ACR e merge até observação ao vivo; nenhum merge de `main`, deploy, auto-merge, force-push ou toque em trabalho alheio.

### Suplemento — gate (2026-11-25T02:29Z, append-only)

- `[EV]` `python3 scripts/ci/check_gate_integrity.py` PASS (`87` pisos, `GATE_RC=0`) na worktree `assess-research-backfill-20261125`, com `TMPDIR` gravável. CI remoto e merge permanecem **[NV]** neste ponto.

## Sessão: flora - backfill/regularização de problem definition (append-only)

- **Identidade e baseline**: `session_id` = `orch-20261125T025000Z-assess-define`; branch `assessment/assess-define-backfill-20261125`; worktree `.specify/queue/worktrees/assess-define-backfill-20261125`; base `github/dev@df75b1bcd`.
- **Entrega**: 42 `problem.md` no formato canônico — 30 novos, 5 reescritos (`010`, `020`–`023`), 7 novos ao lado de `define.md` fino (`107`–`113`). Nenhum veredito/intake/research/spec/gate/CI/código alterado.
- **Evidência local ([EV])**: 42/42 com Slug exato, 8 seções, Users/Stakeholders, métricas com baseline, Cost of Inaction e `[NEEDS CLARIFICATION:]`; paths não-symlink; `git diff --check` RC 0.
- **[NV]**: gate/CI remoto, ACR e merge até observação ao vivo; nenhum merge de `main`, deploy, auto-merge, force-push ou toque em trabalho alheio.

### Suplemento — gate (2026-11-25T03:33Z, append-only)

- `[EV]` gate local: FAIL→FAIL→PASS. Causas ambientais: SIGABRT em `aws-lc-sys` (memória/swap) e ENOSPC em `chrono-tz` (btrfs metadata); após `cargo clean` no target próprio e `CARGO_BUILD_JOBS=2`, `gate-integrity: PASS` (`87` pisos, `GATE_RC=0`, 03:18:11Z→03:32:52Z). Sem código/gate/CI alterados; CI remoto observado via PR #428.
