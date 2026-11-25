# Idea Research: Auditoria viva de governança do CI

- **Slug**: 091-governance-ci-audit
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: high — os registros documentais locais são determinísticos e convergentes entre decisão, specs, STATE/PROGRESSO e nota de governança; o estado ao vivo permanece **[NV]**.

## Users & Demand

- A demanda é sinal único do maintainer/SE (`@lfgranja`), registrada no intake e na decisão do ciclo; o gatilho é a emenda de 2026-11-20 à ADR-0003 somada à sequência dos ciclos 086–090. [source: `.specify/assessments/091-governance-ci-audit/intake.md:16-19`; `decision.md:12-25`] (confidence: high)
- O problema capturado é a ausência de **auditoria viva** que confirme coincidência entre governança praticada e documentada em cinco pontos: required checks/proteção real de `dev`/`main`, emenda vigente sem aprovador humano, deploy fora do gate, explicações auditáveis e omissão dos 4 scripts SDD da factory. [source: `intake.md:10-14`; `.specify/assessments/091-governance-ci-audit/decision.md:20-29`] (confidence: high)
- A audiência implícita nos artefatos é maintainer, dono do processo/gate e auditor (as cinco User Stories nomeiam esses papéis). Não há telemetria, tickets ou múltiplos usuários neste repo que quantifiquem a demanda — generalização não suportada. [source: `specs/091-governance-ci-audit/spec.md:17-145` (US1–US5); ausência verificada em `.specify/assessments/` e `docs/` | ASSUMPTION quanto à audiência] (confidence: medium)

## Prior Art

- Ciclos 086–090 reforçaram o CI por via determinística: leg RBAC (086), leg bootstrap (087), oráculo dinâmico do gate (088: 10 legs / 58 pisos), espelho Forgejo advisory (089) e bump de `rustls` `0.23.43 → 0.23.46` (090). [source: `.specify/assessments/091-governance-ci-audit/decision.md:12-15`; `specs/091-governance-ci-audit/analyze.md:22-26`] (confidence: high)
- A autoridade de merge é a ADR-0003 com a seção "Emenda 2026-11-20 — confirmação do SE": a regra antiga (aprovador distinto + revisão deliberada) foi substituída por "CI/CD GREEN basta para merge autônomo em `dev/`"; se o CI não inspirar confiança, o próximo 80/20 imediato é reforçá-lo, não escalar a aprovação humana. [source: `docs/adr/0003-autonomous-merge-under-deterministic-gates.md:37-48`] (confidence: high)
- O gate autoritativo já existente é `scripts/ci/check_gate_integrity.py` (oráculo dinâmico do ciclo 088); a decisão 089 excluiu os 4 scripts SDD da factory do conjunto de gates, e a decisão 091 estendeu a exclusão a relatórios de status. [source: `specs/091-governance-ci-audit/spec.md:194-210`; `docs/governance/2026-11-20-091-relatorios-sem-scripts-sdd.md:46-54`] (confidence: high)
- Já existia um workflow diário de auditoria de proteção de branch, `branch-protection-audit.yml`, explícito como **advisory, não required** — ou seja, a auditoria contínua antecede o ciclo 091 como mecanismo de visibilidade, não como gate. [source: `.github/workflows/branch-protection-audit.yml:1-23`; `.specify/memory/STATE.md:781`] (confidence: high)

## Market & Context

- A constituição passou de v1.3.0 para v1.4.0 em 2026-11-20 com migration plan **documentation-only** (Svelte 5 como frontend canônico, sem SQLite, ADR-0001/0002 para mecanismos diferidos); rollback registrado é reverter para v1.3.0. [source: `.specify/memory/STATE.md:777`; `docs/governance/2026-11-20-091-relatorios-sem-scripts-sdd.md:28-32`] (confidence: high)
- No snapshot do ciclo, a proteção registrada era: 4 required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`) com *strict* e `enforce_admins`, sem required reviews, e `check_gate_integrity.py --remote` PASS com `dev: live required contexts match` / `main: live required contexts match`. [source: `STATE.md:776`; `docs/PROGRESSO.md:245`; `docs/governance/2026-11-20-091-relatorios-sem-scripts-sdd.md:9-15`] (confidence: high para o registro; estado ao vivo **[NV]**)
- Deploy de produção é tratado como decisão separada, fora do gate de merge (ADR-0003), conduzida pela ligação MDMV-Autonoma; o registro do ciclo 091 não reivindica deploy: no Coolify existia apenas a app `Flora-Staging`, sem app/FQDN/revisão de produção. [source: `decision.md:47-51`; `STATE.md:778`; `docs/governance/2026-11-20-091-relatorios-sem-scripts-sdd.md:34-38`] (confidence: high para o registro; **[NV]** para o estado ao vivo)
- Contexto de adversários de gate: os 25 advisories (7 high) em `apps/frontend/package-lock.json` foram registrados como informação, com recomendação explícita de **não promover** checagem de advisories a required check; triagem e recontagem ficaram **[NV]**. [source: `STATE.md:779`; `docs/governance/2026-11-20-091-relatorios-sem-scripts-sdd.md:40-44`] (confidence: high)

## Data & Constraints

- Nenhum valor de credencial aparece nos artefatos: o PAT do SE é descrito como fine-grained `Administration: read`, restrito ao repo `flora`, e o secret `FLORA_BRANCH_PROTECTION_TOKEN` é referido por nome; ele é injetado apenas como `GH_TOKEN` no step de auditoria do workflow. [source: `docs/governance/2026-11-20-091-relatorios-sem-scripts-sdd.md:13,23`; `.github/workflows/branch-protection-audit.yml:20-23`] (confidence: high)
- O workflow de auditoria tem `permissions: {}`, `cron: '23 4 * * *'` e `timeout-minutes: 20`; o job é advisory e uma divergência de contextos gera exit não-zero sem bloquear merge. [source: `.github/workflows/branch-protection-audit.yml:7-23`; `STATE.md:781`] (confidence: high)
- A evidência de gate consolidada é `all 10 db contract floors intact (58 total)`, exit 0; `analyze` do ciclo fechou com `critical=0`, `requirements_covered=5/5`, `tasks=8/8`, `errors=[]`. [source: `specs/091-governance-ci-audit/tasks.md:36-38,86-93`; `specs/091-governance-ci-audit/analyze.md:9-16`] (confidence: high)
- A fronteira documental do ciclo é explícita: nenhum required check novo, nenhum aprovador humano reintroduzido, nenhum toque em `validate/`, `release/`, gate, ADRs, constituição ou credenciais. [source: `.specify/assessments/091-governance-ci-audit/decision.md:61-64,84-90`; `specs/091-governance-ci-audit/plan.md:43-54`] (confidence: high)
- Cobertura idiomática real nos artefatos lidos: **pt-BR_en** — prosa e headings em pt-BR; nomes de código, comandos, check IDs e termos de gate em inglês. [source: artefatos do ciclo lidos diretamente] (confidence: high)
- Estado ao vivo (CI do PR, proteção de branch, Coolify, Forgejo, aprovações de maintainer) não foi observado nesta pesquisa: **[NV]**. [source: `.specify/assessments/091-governance-ci-audit/decision.md:92-95`; `specs/091-governance-ci-audit/converge.md:19`] (confidence: high)

## Evidence Against the Idea

- A auditoria se apoia nos registros do próprio ciclo como fonte daquilo que pretende auditar; neste worktree não há captura reproduzível da leitura `gh` nem das configurações de proteção consultadas, apenas a afirmação consolidada do handoff. Isso enfraquece a pretensão de verificação independente. [source: `specs/091-governance-ci-audit/analyze.md:28-32`; listagem de arquivos do worktree | ASSUMPTION quanto à interpretação de circularidade] (confidence: medium)
- Há tensão interna entre FR-003, que registra deploy em produção via MDMV-Autonoma com health e rollback, e o registro contemporâneo de que só existia a app `Flora-Staging`, sem FQDN ou revisão de produção. A decisão resolve isso marcando o estado ao vivo como **[NV]**, mas os dois enunciados coexistem no mesmo conjunto de artefatos. [source: `.specify/assessments/091-governance-ci-audit/decision.md:47-51`; `STATE.md:778`] (confidence: high)
- O ciclo subsequente (094/095) registra promoção `dev → main`, deploy e FQDN de produção já em 2026-11-20/21, o que torna o registro 091 ("deploy **[NV]**", sem app de produção) uma fotografia datada e não um estado corrente — reutilizá-lo como verdade atual seria erro. [source: `STATE.md:801-805,812`; `docs/PROGRESSO.md:272,277`] (confidence: high)
- Riscos registrados, não observados como fatos: promover script SDD ou advisory a required check, reintroduzir aprovador humano ou expor credencial invalidariam a auditoria. São contra-evidências à ideia na forma de restrição de fronteira, não achados de divergência. [source: `.specify/assessments/091-governance-ci-audit/decision.md:27-29`; `specs/091-governance-ci-audit/spec.md:150-166`] (confidence: high)
- O workflow `branch-protection-audit.yml` é diário e advisory: se a proteção real divergir da ADR-0003, a divergência aparece como falha de job, mas não bloqueia merges. A confiança no CI depende, portanto, de alguém ler esse sinal. [source: `.github/workflows/branch-protection-audit.yml:1-6,20-23`] (confidence: medium)

## Gaps & Open Questions

- [NEEDS CLARIFICATION: Qual é o estado ao vivo dos required checks e da proteção de `dev`/`main` no momento da auditoria, além do registrado como [NV]?]
- [NEEDS CLARIFICATION: Qual divergência concreta entre documentação e prática motivou a auditoria, se houver alguma?]
- [NEEDS CLARIFICATION: Qual é o destino e a reavaliação das demais emendas pendentes registradas como escaladas [NV] a maintainer?]
- [NEEDS CLARIFICATION: Qual é o estado atual do deploy em produção conduzido pela ligação MDMV-Autonoma, incluindo health e imagem de rollback?]
- [NEEDS CLARIFICATION: Como os itens diferidos (Tauri/OTLP, leg RBAC) devem ser acompanhados até uma eventual reavaliação?]
- [NEEDS CLARIFICATION: Qual audiência e periodicidade são esperadas para o relatório de auditoria viva?]

## Sources

- `.specify/assessments/091-governance-ci-audit/intake.md:1-28` (system; policy: fonte primária local; lido 2026-11-25)
- `.specify/assessments/091-governance-ci-audit/decision.md:12-25,33-64,92-95` (system; policy: fonte primária local; lido 2026-11-25)
- `specs/091-governance-ci-audit/spec.md:17-250` (system; policy: fonte primária local; lido 2026-11-25)
- `specs/091-governance-ci-audit/tasks.md:22-107`, `analyze.md:9-36`, `converge.md:8-24` (system; policy: fonte primária local; lido 2026-11-25)
- `docs/adr/0003-autonomous-merge-under-deterministic-gates.md:37-48` (system; policy: fonte normativa local; lido 2026-11-25)
- `docs/governance/2026-11-20-091-relatorios-sem-scripts-sdd.md:7-58` (system; policy: fonte primária local; lido 2026-11-25)
- `.specify/memory/STATE.md:773-782,801-805` e `docs/PROGRESSO.md:242-272` (system; policy: registro append-only local; lido 2026-11-25)
- `.github/workflows/branch-protection-audit.yml:1-23` (system; policy: fonte primária local; lido 2026-11-25)
- `https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora` (host: github.com; policy: allowlisted; não buscado nesta passagem — CI/proteção remotas **[NV]**)
- `https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git` (host: forgejo.mdmv.qzz.io; policy: auto-refused: host not allowlisted; [UNVERIFIED — fetch skipped: host not allowlisted])
