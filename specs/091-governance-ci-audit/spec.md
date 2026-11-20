# Feature Specification: Auditoria viva de governança do CI (ciclo 091)

**Feature Branch**: `feature/091-governance-ci-audit`

**Created**: 2026-11-20

**Status**: Implementado (evidência e escopo registrados em `tasks.md`)

**Input**: Auditoria viva, documental e não mutante da governança do CI após os
ciclos 086–090, confirmando que a prática coincide com a ADR-0003 emendada em
2026-11-20 ("CI/CD GREEN basta para merge autônomo em `dev/`"). Âncora: branch
`feature/091-governance-ci-audit` @ `673c687ea`;
`.specify/feature.json` → `specs/091-governance-ci-audit`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Auditoria viva do CI e da proteção de branch (Priority: P1)

Como mantenedor, quero consultar o estado real do CI/GitHub e confirmar que os
required checks e a proteção de `dev`/`main` correspondem ao documento, para
que a regra "CI green basta" tenha evidência observável e não seja apenas
afirmação de papel.

**Why this priority**: é o núcleo do ciclo. Sem leitura ao vivo, a auditoria
seria especulativa e a confiança no CI — condição da emenda ADR-0003 — não
poderia ser aferida.

**Independent Test**: usar `gh` com o PAT do SE (fine-grained,
`Administration: read`, restrito ao repo `flora`) para ler os checks recentes e
a proteção de `dev`/`main`; confirmar os 4 required checks (`frontend-build`,
`rust-build`, `db-tests`, `dependency-review`) com *strict* e `enforce_admins`,
CI green e gate `scripts/ci/check_gate_integrity.py` PASS (10 legs / 58 pisos).
O secret `FLORA_BRANCH_PROTECTION_TOKEN` já está registrado no GitHub; nenhum
valor de credencial é exibido.

**Acceptance Scenarios**:

1. **Given** o PAT do SE e o secret `FLORA_BRANCH_PROTECTION_TOKEN` disponíveis,
   **When** a proteção de `dev`/`main` é consultada, **Then** os 4 required
   checks aparecem com *strict* e `enforce_admins`, sem required reviews e sem
   check novo.
2. **Given** um run recente em `dev`, **When** o rollup de checks é lido, **Then**
   os checks obrigatórios estão verdes e o gate local fecha PASS
   (10 legs / 58 pisos, exit 0).
3. **Given** qualquer divergência entre estado ao vivo e documentação,
   **When** a auditoria a encontra, **Then** ela é registrada como achado com
   estado `[NV]`, nunca reivindicada como verde.

---

### User Story 2 — Emenda constitucional vigente (Priority: P2)

Como dono do processo, quero confirmar que a emenda de 2026-11-20 à ADR-0003 é
a regra vigente de merge, para que nenhum aprovador distinto ou revisão
deliberada seja reintroduzido como condição de merge.

**Why this priority**: a emenda substitui a regra antiga explicitamente; deixá-la
ambígua reabriria a decisão humana que a ADR-0003 removeu.

**Independent Test**: ler a seção "Emenda 2026-11-20" de
`docs/adr/0003-autonomous-merge-under-deterministic-gates.md` e conferir que a
regra nova prevalece; verificar que nenhum required check, workflow ou
configuração de proteção foi alterado neste ciclo.

**Acceptance Scenarios**:

1. **Given** a regra antiga (aprovador distinto + revisão deliberada), **When** a
   emenda é aplicada, **Then** "CI/CD GREEN basta" prevalece e nenhuma
   aprovação humana é exigida para merge em `dev/`.
2. **Given** demais emendas constitucionais pendentes (ex.: SolidJS vs Svelte 5,
   PGMQ obrigatório), **When** o ciclo 091 é fechado, **Then** elas permanecem
   **escaladas [NV]** a maintainer — não executadas, não reescritas.

---

### User Story 3 — Deploy em produção fora do gate (Priority: P3)

Como responsável por publicação, quero que o deploy em produção seja registrado
como decisão explícita e separada do merge, para que a ADR-0003 não seja
confundida com autorização de deploy automático.

**Why this priority**: ADR-0003 delega o merge ao CI, mas mantém deploy
(GHCR + Coolify) fora do gate; confundir os dois geraria publicação não
deliberada.

**Independent Test**: verificar que o deploy foi conduzido pela ligação
**MDMV-Autonoma**, com health verificado e imagem de rollback preservada, e que
nenhum workflow/gatilho de deploy foi alterado neste ciclo.

**Acceptance Scenarios**:

1. **Given** a ligação MDMV-Autonoma, **When** o deploy em produção é
   registrado, **Then** ele aparece como decisão separada, fora do conjunto de
   required checks, com health e imagem de rollback documentados.
2. **Given** o estado ao vivo da app/deployment, **When** o ciclo documental é
   fechado, **Then** ele é marcado **[NV]** — reconsultar antes de qualquer
   promoção, rollback ou novo deploy.

---

### User Story 4 — Explicações auditáveis (Priority: P4)

Como auditor, quero explicações claras das decisões de governança e dos itens
diferidos, para que fatos verificados não sejam confundidos com itens `[NV]`.

**Why this priority**: a auditoria só é útil se o leitor entender por que
Tauri/OTLP, leg RBAC e emendas pendentes não foram executados e por que o CI
green é suficiente.

**Independent Test**: ler `tasks.md` e confirmar que cada decisão traz
justificativa e que os itens diferidos estão marcados `[NV]`, sem verde
reivindicado sobre estado não observado.

**Acceptance Scenarios**:

1. **Given** um item diferido (ex.: Tauri/OTLP), **When** a explicação é lida,
   **Then** o motivo do diferimento e o estado `[NV]` estão explícitos.
2. **Given** as decisões 089/090, **When** a narrativa do ciclo 091 é lida,
   **Then** a continuidade (SDD fora do gate, bump rustls, oráculo dinâmico)
   está explicada sem reescrever histórico.

---

### User Story 5 — Omissão dos scripts SDD em relatórios (Priority: P5)

Como dono do gate, quero que os 4 scripts SDD da factory não apareçam em
relatórios como gate ou evidência de verificação, para que um `FAIL` de baseline
legada não seja lido como regressão do alvo.

**Why this priority**: a decisão 089 é explícita: esses scripts não são gate do
alvo; citá-los como verificação invalidaria a auditoria.

**Independent Test**: varrer `decision.md`, `spec.md`, `plan.md` e `tasks.md` e
confirmar que os nomes dos 4 scripts SDD não são apresentados como gate ou
evidência de aprovação — apenas como itens deliberadamente omitidos.

**Acceptance Scenarios**:

1. **Given** a decisão 089, **When** os relatórios do ciclo 091 são lidos,
   **Then** os scripts SDD aparecem somente como "fora do gate / omitidos",
   nunca como verificação.
2. **Given** o gate autoritativo, **When** a evidência de verificação é
   listada, **Then** apenas `scripts/ci/check_gate_integrity.py` (10 legs /
   58 pisos) é citado, sem required check novo.

---

### Edge Cases

- **Estado ao vivo divergente**: qualquer diferença entre a documentação e o
  GitHub real é registrada como achado `[NV]`, nunca corrigida alterando gates
  ou proteção neste ciclo.
- **Credenciais**: o PAT do SE e o secret `FLORA_BRANCH_PROTECTION_TOKEN` são
  referidos por nome/local, nunca por valor; nenhum token entra em artefato,
  log ou diff.
- **Required check novo**: promover script SDD, advisory ou job informativo a
  required check rompe ADR-0003 e falha a auditoria.
- **Aprovação humana**: reintroduzir aprovador distinto/revisão deliberada
  contradiz a emenda 2026-11-20.
- **Deploy sem separação**: tratar deploy como parte do gate de merge é fora de
  escopo e falha a auditoria.
- **Emendas pendentes**: SolidJS/Svelte 5 e PGMQ obrigatório exigem maintainer;
  executá-las neste ciclo rompe a fronteira documental.
- **Fronteira do diff**: qualquer toque em `validate/`, `release/` ou
  `.specify/memory/STATE.md` invalida o fechamento — a auditoria é somente leitura
  nesses caminhos.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O projeto DEVE registrar uma **auditoria viva** do CI e da
  proteção de branch, lendo o estado real via `gh` com o PAT do SE
  (`Administration: read`, restrito a `flora`) e usando o secret
  `FLORA_BRANCH_PROTECTION_TOKEN`, confirmando CI green, os 4 required checks
  com *strict*/*enforce_admins* e o gate `check_gate_integrity.py` PASS
  (10 legs / 58 pisos). Nenhum valor de credencial pode ser copiado, gravado ou
  exibido, e nenhum required check novo pode ser criado.
- **FR-002**: A **emenda de 2026-11-20 à ADR-0003** DEVE ser registrada como
  regra vigente — "CI/CD GREEN basta para merge autônomo em `dev/`", sem
  aprovador distinto nem revisão deliberada — prevalecendo sobre a regra antiga
  e sobre registros históricos de "sem merge". As demais emendas constitucionais
  pendentes DEVEM permanecer escaladas **[NV]** a maintainer, sem execução.
- **FR-003**: O **deploy em produção** DEVE ser registrado como decisão
  explícita e separada, conduzida pela ligação **MDMV-Autonoma**, fora do gate
  de merge (ADR-0003), com health verificado e imagem de rollback preservada.
  O estado ao vivo da app/deployment DEVE ser marcado **[NV]** até reconsulta
  antes de promoção, rollback ou novo deploy; nenhum workflow/gatilho de deploy
  pode ser alterado neste ciclo.
- **FR-004**: O ciclo DEVE produzir **explicações auditáveis** que separem fatos
  verificados de itens diferidos **[NV]** (Tauri/OTLP, leg RBAC, emendas
  pendentes), justifiquem a suficiência do CI green e a continuidade das decisões
  088–090, sem reescrever histórico nem reivindicar verde não observado.
- **FR-005**: Os 4 scripts SDD da factory (`check_sdd_completeness.py`,
  `check_converge_append_only.py`, `check_task_id_uniqueness.py`,
  `check_work_order_dag.py`) DEVEM ser **omitidos de relatórios como gate ou
  evidência de verificação**, conforme decisão 089 — seu `FAIL` é baseline
  legada da hub, não regressão. O único gate autoritativo citado DEVE ser
  `scripts/ci/check_gate_integrity.py`; nenhum required check novo é adicionado.

### Key Entities

- **Auditoria viva**: leitura do estado real do CI/GitHub via `gh` com PAT do
  SE; produz achados verificáveis e marca estados não observados como `[NV]`.
- **ADR-0003 (emenda 2026-11-20)**: autoridade de merge — CI green basta;
  jobs informativos e deploy ficam fora do conjunto de required checks.
- **Gate autoritativo**: `scripts/ci/check_gate_integrity.py` com oráculo
  dinâmico (ciclo 088), 10 legs / 58 pisos — única evidência de verificação.
- **Scripts SDD da factory**: 4 verificações de baseline da hub, explicitamente
  fora do gate do alvo (decisão 089) e omitidas como evidência em relatórios.
- **MDMV-Autonoma**: ligação que conduz o deploy em produção; decisão separada
  do merge, com health e rollback.
- **PAT / secret**: PAT do SE (fine-grained, `Administration: read`, restrito a
  `flora`) e secret `FLORA_BRANCH_PROTECTION_TOKEN` — referidos por nome, nunca
  por valor.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `tasks.md` registra T001–T008 `[X]` com cobertura FR-001..FR-005;
  a auditoria viva cita os 4 required checks com *strict*/*enforce_admins*, CI
  green e gate PASS (10 legs / 58 pisos), com zero credenciais exibidas e zero
  required checks novos.
- **SC-002**: a emenda 2026-11-20 à ADR-0003 é citada como vigente em
  `decision.md`, `spec.md` e `tasks.md`; zero aprovadores humanos
  reintroduzidos; emendas pendentes marcadas `[NV]`.
- **SC-003**: o deploy em produção aparece como decisão separada via
  MDMV-Autonoma, com health/rollback documentados e estado ao vivo `[NV]`;
  nenhum workflow de deploy alterado.
- **SC-004**: cada item diferido (Tauri/OTLP, leg RBAC, emendas) traz
  justificativa e marcação `[NV]`, com zero verdes reivindicados sobre estado
  não observado e zero reescritas de histórico.
- **SC-005**: nenhum dos 4 scripts SDD é apresentado como gate/evidência; o
  diff do ciclo contém apenas os 4 artefatos 091 — zero arquivos em
  `validate/`, `release/` ou `.specify/memory/STATE.md`.

## Assumptions

- A regra vigente de merge é a emenda 2026-11-20 à ADR-0003; registros
  históricos de "sem merge" permanecem como histórico, não como autoridade.
- O gate autoritativo continua sendo `check_gate_integrity.py` (oráculo dinâmico
  088, 10 legs / 58 pisos); decisão 089 (SDD fora do gate) permanece válida.
- O PAT do SE e o secret `FLORA_BRANCH_PROTECTION_TOKEN` existem e são
  suficientes para leitura de proteção; nenhum valor é necessário nos artefatos.
- Deploy em produção é conduzido pela ligação MDMV-Autonoma e fica fora do gate
  de merge; estado ao vivo é `[NV]` até reconsulta.
- Evidência de execução é a registrada em `tasks.md`; subagentes de
  documentação não reexecutam gates, testes, build ou formatters e não acessam
  PR/merge/produção — marcado `[NV]` onde aplicável.
