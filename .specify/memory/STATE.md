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
