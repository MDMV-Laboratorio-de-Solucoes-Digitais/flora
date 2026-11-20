# Nota de governança — 2026-11-20 (ciclo 091)

**Assunto:** decisão de relatórios sem scripts SDD e registro da auditoria viva de governança do CI.

**Âncora:** branch `feature/091-governance-ci-audit` @ `673c687ea`; `.specify/feature.json` → `specs/091-governance-ci-audit`. Registro append-only; nenhum validation/release existente tocado.

## (a) Auditoria viva — gate autoritativo confirmado em dev/main

Auditoria viva, documental e não mutante: `python3 scripts/ci/check_gate_integrity.py --remote` → **PASS**, com `dev: live required contexts match` e `main: live required contexts match`.

- Os 4 required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`) correspondem ao conjunto da ADR-0003 (emenda 2026-11-20), com *strict* e `enforce_admins`, sem required reviews.
- Gate local: **PASS** — oráculo dinâmico do ciclo 088, `all 10 db contract floors intact (58 total)`, exit 0.
- Leitura ao vivo via `gh` com PAT fine-grained do SE (`Administration: read`, restrito somente ao repositório `flora`). O secret `FLORA_BRANCH_PROTECTION_TOKEN` está registrado no GitHub; nenhum valor de credencial foi copiado, gravado ou exibido.

Nenhum required check novo foi criado. **[NV]** reconsulta de estado ao vivo antes de qualquer decisão de merge/promoção.

## (b) Workflow de auditoria — advisory, não required

`.github/workflows/branch-protection-audit.yml` (YAML válido):

- Gatilhos: `schedule` (`23 4 * * *`, diário) e `workflow_dispatch`.
- `permissions: {}`; job `branch-protection-audit`, `timeout-minutes: 20`.
- `GH_TOKEN: ${{ secrets.FLORA_BRANCH_PROTECTION_TOKEN }}` exposto somente no `env` do step de auditoria, consumido por `gh`; nunca em URL, argumento ou log.
- Comando auditado: `python3 scripts/ci/check_gate_integrity.py --remote`.

O job é **advisory e NÃO required** — a divergência de contextos é tornada visível por exit não-zero, mas não bloqueia merge. Nenhum required check adicionado ao conjunto da ADR-0003.

## (c) Constituição v1.4.0 — aprovação do SE e migration plan

`.specify/memory/constitution.md`: emenda `1.3.0 → 1.4.0` (MINOR, 2026-11-20), **aprovada pelo SE/maintainer** sob §Governance e a ADR-0003 "Emenda 2026-11-20 — confirmação do SE". Não é autocertificação.

Migration plan (curto): emenda **documentation-only** — nenhuma mudança de código, config, workflow, gate, migration ou secret. Documentos downstream devem nomear Svelte 5 como frontend canônico, remover SQLite e citar ADR-0001/ADR-0002 para mecanismos diferidos (PGMQ/Valkey/RustFS/Meilisearch/OTLP) e Tauri/mobile; a ADR-0003 (emenda 2026-11-20) rege merges. Rollback = reverter este arquivo para v1.3.0.

## (d) Deploy de produção — [NV]

**Não há deploy de produção verificado.** No Coolify existe apenas a app `Flora-Staging` (deploy do ciclo 089); não há app de produção, FQDN de produção ou revisão de produção identificados. Nenhum deploy de produção é reivindicado por esta nota.

Deploy está fora do gate de merge (ADR-0003). **[NV]** estado ao vivo: reconsultar app/deployment/FQDN/revisão antes de qualquer promoção, rollback ou novo deploy; não promover a app Staging a produção sem FQDN e revisão confirmados ao vivo.

## (e) Advisories no frontend — recomendação, sem promoção de gate

`apps/frontend/package-lock.json` registra **25 advisories, dos quais 7 high**.

Recomendação registrada: **não promover** checagem de advisories a required check; **triar/limpar** os findings. Nenhum check de advisory entra no conjunto de gates obrigatórios neste ciclo. **[NV]** triagem, remediação e recontagem de advisories.

## (f) Decisão — scripts SDD da factory fora de relatórios

Decisão explícita (ciclo 091), que confirma e reforça a decisão do ciclo 089: os **4 scripts SDD da factory** — `check_sdd_completeness.py`, `check_converge_append_only.py`, `check_task_id_uniqueness.py`, `check_work_order_dag.py` — **não devem sequer constar de relatórios futuros**.

- Os nomes e resultados desses scripts aparecem **apenas neste documento de decisão/governança** (e em registros de decisão equivalentes). **Nunca** em relatórios de status — incluindo `.specify/memory/STATE.md`, `docs/PROGRESSO.md`, resumos, dashboards ou saídas de auditoria.
- Eles falham por **baseline legada da hub** (75 specs), não são regressão do alvo e **não** são gate autoritativo. Nenhum deles deve ser promovido a required check nem citado como evidência de verificação.
- A única autoridade de gate do alvo é `python3 scripts/ci/check_gate_integrity.py` (oráculo dinâmico 088, 10 legs / 58 pisos), com a auditoria `--remote` de (a).

Esta decisão prevalece sobre qualquer leitura que trate a suíte SDD da factory como fonte de verdade de gate ou como conteúdo devido em relatórios de status.

## Histórico e escopo

Documento append-only. Nenhuma linha de registros anteriores (STATE.md, PROGRESSO.md, ADRs, constituição, validations/releases) foi reescrita. Nenhum código, workflow, gate, migration, ADR, constituição, segredo, PR, merge, tag ou deploy foi alterado por esta nota; nenhuma credencial foi exposta. **[NV]** estado ao vivo de CI, proteção de branch, deploy, Forgejo e aprovações de maintainer — reconsultar antes de agir.
