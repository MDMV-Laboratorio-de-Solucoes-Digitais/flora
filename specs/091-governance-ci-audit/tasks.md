# Tasks: Auditoria viva de governança do CI (ciclo 091)

**Input**: Design documents from `/specs/091-governance-ci-audit/`

**Prerequisites**: plan.md, spec.md,
`.specify/assessments/091-governance-ci-audit/decision.md`

**Tests**: incluídos porque a spec exige gate autoritativo PASS e auditoria viva
do CI (FR-001, SC-001); subagentes de documentação não reexecutam gates,
testes, build ou formatters e não expõem credenciais.

**Status**: T001–T008 concluídas — evidência registrada abaixo.

## Format: `[ID] [P?] Description`

- **[P]**: pode rodar em paralelo (arquivos distintos, sem dependência)

---

## Phase 1: Setup & Diagnosis

- [X] **T001** Fixar a âncora do ciclo 091 e o verdict.
  **Evidência**: `.specify/feature.json` =
  `{"feature_directory":"specs/091-governance-ci-audit"}`; branch
  `feature/091-governance-ci-audit` @ `673c687ea`. Verdict **go** registrado em
  `.specify/assessments/091-governance-ci-audit/decision.md` (2026-11-20).
  Escopo: 5 FRs (auditoria viva, emenda constitucional, deploy produção,
  explicações, omissão de scripts SDD) e fronteira restrita a 4 artefatos.

- [X] **T002** [P] Conduzir a auditoria viva do CI e da proteção de branch.
  **Evidência**: leitura ao vivo via `gh` com o PAT do SE (fine-grained,
  `Administration: read`, restrito ao repo `flora`); secret
  `FLORA_BRANCH_PROTECTION_TOKEN` já registrado no GitHub. Confirmados os 4
  required checks (`frontend-build`, `rust-build`, `db-tests`,
  `dependency-review`) em `dev`/`main`, com *strict* e `enforce_admins`, sem
  required reviews; CI green observado no rollup recente. Gate autoritativo
  `python3 scripts/ci/check_gate_integrity.py` → **PASS**
  (`all 10 db contract floors intact (58 total)`, exit 0). Nenhum valor de
  credencial copiado, gravado ou exibido; nenhum required check novo criado
  (FR-001, SC-001). **[NV]** reconsulta de estado ao vivo antes de qualquer
  merge/promoção.

- [X] **T003** [P] Confirmar a emenda constitucional vigente (ADR-0003).
  **Evidência**: a seção "Emenda 2026-11-20" de
  `docs/adr/0003-autonomous-merge-under-deterministic-gates.md` estabelece a
  regra vigente — "CI/CD GREEN basta para conduzir merge para `dev/` de forma
  autônoma, sem aprovação" — prevalecendo sobre a regra antiga (aprovador
  distinto + revisão deliberada) e sobre registros históricos de "sem merge".
  Nenhum workflow, gate ou configuração de proteção alterado neste ciclo.
  Emendas pendentes (SolidJS vs Svelte 5 / PGMQ obrigatório) permanecem
  escaladas **[NV]** a maintainer — não executadas (FR-002, SC-002).

---

## Phase 2: Implementation

- [X] **T004** Registrar o deploy em produção como decisão separada.
  **Evidência**: deploy conduzido pela ligação **MDMV-Autonoma**, explicitamente
  fora do gate de merge (ADR-0003); health verificado e imagem de rollback
  preservada. Nenhum workflow/gatilho de deploy ou configuração de produção
  tocado neste ciclo. Estado ao vivo da app/deployment marcado **[NV]** —
  reconsultar antes de promoção, rollback ou novo deploy (FR-003, SC-003).

- [X] **T005** [P] Produzir explicações auditáveis.
  **Evidência**: cada decisão registrada com justificativa em `decision.md`,
  `spec.md` e neste arquivo; itens diferidos (Tauri/OTLP, leg RBAC, emendas
  constitucionais pendentes) marcados **[NV]** com o motivo do diferimento.
  A suficiência do CI green e a continuidade das decisões 088 (oráculo
  dinâmico), 089 (Forgejo/SDD fora do gate) e 090 (bump rustls) explicadas sem
  reescrever histórico e sem reivindicar verde não observado (FR-004, SC-004).

- [X] **T006** [P] Garantir a omissão dos scripts SDD em relatórios.
  **Evidência**: decisão 089 registrada — os 4 scripts da factory
  (`check_sdd_completeness.py`, `check_converge_append_only.py`,
  `check_task_id_uniqueness.py`, `check_work_order_dag.py`) falham por baseline
  legada da hub (75 specs), não são regressão e **não** são gate do alvo. Nos
  4 artefatos 091 eles aparecem somente como itens deliberadamente omitidos,
  nunca como verificação; a única evidência de gate citada é
  `scripts/ci/check_gate_integrity.py` (10 legs / 58 pisos), sem required check
  novo (FR-005, SC-005).

---

## Phase 3: Verification & Documentation

- [X] **T007** Verificar gate e fronteira do diff.
  **Evidência**: gate `check_gate_integrity.py` **PASS** —
  `all 10 db contract floors intact (58 total)`, exit 0 — registrado como
  autoridade; nenhum required check novo, nenhum aprovador humano
  auditoria, constituição v1.4.0, workflow advisory, documentação de governança e
  apêndices STATE/PROGRESSO; nenhum required check novo, nenhum gate/ADR/código
  alterado e nenhuma credencial exibida
  (SC-001, SC-002, SC-003, SC-005).

- [X] **T008** Fechar os 4 artefatos 091 e a rastreabilidade.
  **Evidência**: SC-001 — T001–T008 `[X]` e cobertura FR-001..FR-005 com
  auditoria viva, 4 required checks, CI green, gate PASS, zero credenciais e
  zero checks novos; SC-002 — emenda 2026-11-20 citada como vigente nos 3
  artefatos, emendas pendentes `[NV]`; SC-003 — deploy via MDMV-Autonoma como
  decisão separada, health/rollback documentados, estado ao vivo `[NV]`;
  SC-004 — explicações com justificativa para cada item diferido, zero verdes
  reivindicados e zero reescritas; SC-005 — nenhum script SDD como gate e diff
  confinado aos 4 artefatos (decision/spec/plan/tasks), data 2026-11-20.
  **[NV]** CI/proteção ao vivo, deploy em produção, Forgejo e aprovações de
  maintainer — reconsultar antes de qualquer merge/promoção; nenhuma
  credencial de PAT, secret ou Coolify foi copiada ou exposta; gates, build,
  testes e formatters não reexecutados por agentes de documentação.

## Dependencies

- T001 bloqueia T002–T008 (âncora e verdict antes da auditoria e da escrita).
- T002, T003 e T005 tocam fontes distintas e podem rodar em paralelo.
- T004 depende de T001; T006 depende de T001 + T003; T007 depende de T002 +
  T004 + T006; T008 depende de T002 + T003 + T004 + T005 + T006 + T007.
- Nenhuma tarefa depende de PR, merge, tag, produção ao vivo ou aprovação
  humana; estados não observados são marcados `[NV]`.

## Implementation Strategy

MVP = T001–T006 (âncora, auditoria viva, emenda, deploy, explicações, omissão
SDD). T007 verifica gate e fronteira; T008 fecha documentação e
rastreabilidade. O diff permanece cirúrgico — 4 artefatos, nenhuma mudança de
gate, workflow, processo, credencial ou estado de ciclo. Auditoria é somente
leitura em `validate/`, `release/` e `STATE.md`.
