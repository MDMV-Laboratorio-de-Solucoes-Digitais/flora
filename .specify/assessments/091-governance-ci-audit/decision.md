# Decision: Auditoria viva de governança do CI (ciclo 091)

- **Slug**: 091-governance-ci-audit
- **Decidido**: 2026-11-20
- **Verdict**: **go**
- **Branch**: `feature/091-governance-ci-audit` @ `673c687ea`
- **Âncora**: `.specify/feature.json` → `specs/091-governance-ci-audit`
- **Artefatos revisados**: spec | plan | tasks

## Contexto / Problema

Os ciclos 086–090 reforçaram o CI de forma determinística: leg RBAC (086),
leg bootstrap (087), oráculo dinâmico do gate (088: 10 legs / 58 pisos),
espelho Forgejo advisory (089) e bump `rustls` `0.23.43 → 0.23.46` (090). A
emenda de 2026-11-20 à ADR-0003, confirmada literalmente pelo SE, estabelece a
regra vigente: **CI/CD GREEN basta para merge autônomo em `dev/`; se o CI não
inspirar confiança, o próximo 80/20 imediato é reforçá-lo — não escalar para
aprovação humana**.

Falta, porém, uma **auditoria viva** que confirme que a governança praticada
coincide com a governança documentada: (a) required checks e proteção de
`dev`/`main` reais, (b) a emenda constitucional vigente aplicada sem aprovador
humano, (c) deploy de produção fora do gate de merge, (d) explicações claras e
auditáveis das decisões e dos itens diferidos, e (e) a omissão dos 4 scripts SDD
da factory em relatórios, conforme decisão explícita do ciclo 089.

O risco a evitar é transformar a auditoria em mutação de processo: promover
scripts SDD ou advisories a required check, reintroduzir aprovação humana,
publicar credenciais, ou tocar `validate/`, `release/` e `.specify/memory/STATE.md`.

## Decisão

Registrar o ciclo 091 como **auditoria viva, documental e não mutante**, com
cinco compromissos rastreáveis:

1. **Auditoria viva (FR-001)**: consultar o estado real do CI/GitHub via `gh` com
   o PAT do SE (fine-grained, `Administration: read`, restrito ao repo `flora`),
   confirmando CI green, os 4 required checks (`frontend-build`, `rust-build`,
   `db-tests`, `dependency-review`) com *strict* e `enforce_admins`, e o gate
   autoritativo `scripts/ci/check_gate_integrity.py` PASS (10 legs / 58 pisos).
   O secret `FLORA_BRANCH_PROTECTION_TOKEN` já está registrado no GitHub; nenhum
   valor de credencial é copiado, gravado ou exibido.
2. **Emenda constitucional (FR-002)**: confirmar a emenda de 2026-11-20 à
   ADR-0003 como regra vigente — CI green basta, sem aprovador distinto nem
   revisão deliberada — e manter as demais emendas pendentes (ex.: SolidJS vs
   Svelte 5 / PGMQ obrigatório) **escaladas [NV]** a maintainer, sem execução.
3. **Deploy em produção (FR-003)**: registrar o deploy via ligação
   **MDMV-Autonoma** como decisão explícita e separada, fora do gate de merge
   (ADR-0003), com verificação de health e imagem de rollback preservada.
   Estado ao vivo da app/deployment permanece **[NV]** — reconsultar antes de
   qualquer promoção, rollback ou novo deploy.
4. **Explicações (FR-004)**: produzir explicações auditáveis, separando fatos
   verificados de itens diferidos **[NV]** (ex.: Tauri/OTLP, leg RBAC, emendas
   pendentes), sem reescrever histórico nem reivindicar verde não observado.
5. **Omissão de scripts SDD em relatórios (FR-005)**: os 4 scripts da factory
   (`check_sdd_completeness.py`, `check_converge_append_only.py`,
   `check_task_id_uniqueness.py`, `check_work_order_dag.py`) **não** aparecem em
   relatórios como gate ou evidência de verificação — decisão 089. O único gate
   autoritativo é `check_gate_integrity.py`; nenhum required check novo é criado.

**Fronteira final**: os 4 artefatos do ciclo 091 mais constituição v1.4.0,
workflow advisory de auditoria e documentação de governança/STATE/PROGRESSO.
Nenhum required check novo, nenhum gate, ADR, código, PR, merge, tag ou
credencial é alterado ou exposto.

## Scorecard

| Critério | Nota | Justificativa |
|---|---|---|
| Validade do problema | strong | Governança determinística precisa de auditoria viva após 086–090; emenda ADR-0003 define regra a conferir |
| Força da evidência | strong | ADR-0003 + emenda 2026-11-20, nota de governança 089, gate 088 (10 legs / 58 pisos), PAT/secret disponíveis no escopo do SE |
| Valor vs. inação | strong | Confirma coerência entre CI real e documentação sem mutar gates, sem aprovação humana e sem vazar credenciais |
| Viabilidade | strong | Auditoria documental com ferramentas já disponíveis (`gh`, gate local, ligação MDMV-Autonoma) |
| Fit estratégico | strong | Preserva ADR-0003 e a decisão 089 (SDD fora do gate); reforça o 80/20 imediato de confiança no CI |
| Risco | strong | Diff confinado a 4 artefatos; estado ao vivo marcado `[NV]`; nenhuma credencial exposta; nenhum required check novo |

## Handoff

**Dentro**: `.specify/assessments/091-governance-ci-audit/decision.md`,
`specs/091-governance-ci-audit/spec.md`,
`specs/091-governance-ci-audit/plan.md`,
`specs/091-governance-ci-audit/tasks.md`.

**Fora**: gate, ADRs, código de aplicação, PRs, merge, tags, produção e credenciais;
`validate/` e `release/` produzidos na fase final do próprio ciclo.

**Métricas de sucesso**: os 5 FRs rastreados em `tasks.md` com T001–T008 `[X]`;
gate `check_gate_integrity.py` PASS (10 legs / 58 pisos) registrado como
autoridade; zero required checks novos; zero aprovadores humanos reintroduzidos;
zero scripts SDD apresentados como gate; nenhuma credencial copiada ou exibida.

**[NV]** CI do PR, proteção de branch ao vivo, estado do deploy em produção,
Forgejo, mensageria cross-session e aprovações de maintainer — reconsultar antes
de qualquer merge, promoção ou rollback. Gates, build, testes e formatters não
foram reexecutados pelos agentes de documentação neste ciclo.
