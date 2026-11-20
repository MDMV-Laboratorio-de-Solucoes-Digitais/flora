# Release: 091 — Auditoria viva de governança do CI

**Ciclo**: 091 · **Data**: 2026-11-20 · **Branch**: `feature/091-governance-ci-audit` @ `673c687ea` · **Baseline**: `673c687ea`
**Tipo**: auditoria viva, documental e não mutante do CI — emenda constitucional v1.4.0 (ADR-0003, "CI/CD GREEN basta") aprovada SE e workflow de auditoria advisory; nenhum required check novo, nenhuma credencial exposta. **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- **Constituição v1.4.0** (`.specify/memory/constitution.md`): emenda de 2026-11-20 aprovada SE — merge autônomo em `dev/` com CI/CD GREEN, sem aprovador distinto nem revisão deliberada; gate autoritativo `python3 scripts/ci/check_gate_integrity.py`; Svelte 5 canônico; PGMQ/Valkey/RustFS/Meilisearch/OTLP e Tauri/mobile conforme ADR-0001/0002. De `1.3.0` (2026-11-16) para `1.4.0` (2026-11-20).
- **Workflow de auditoria** (`.github/workflows/branch-protection-audit.yml`): novo, **advisory** (não required), `schedule` + `workflow_dispatch`; executa `check_gate_integrity.py --remote` com `GH_TOKEN: ${{ secrets.FLORA_BRANCH_PROTECTION_TOKEN }}` — secret já registrado, nunca em URL/argumento/log.
- **Auditoria viva**: `check_gate_integrity.py --remote` **PASS** — `dev: live required contexts match` / `main: live required contexts match`; 4 required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`) com *strict*/*enforce_admins*, sem required reviews; zero required checks novos, zero credenciais exibidas.
- **Explicações auditáveis** e **omissão dos scripts SDD**: itens diferidos marcados `[NV]`; 4 scripts da factory fora do gate; 25 alerts (7 high) em `apps/frontend/package-lock.json` **não** promovidos a gate — recomendação de triar/limpar.
- Verificação documental: R4 **APROVADO** em `.specify/validations/091-governance-ci-audit/validate.md`; detalhe em `analyze.md` (READ-ONLY, `critical=0`) e `converge.md` (**Converged**).

## SemVer

**Proposta:** patch `0.4.5` → `0.4.6`. Motivo: mudança governamental/documental sem alteração de API, comportamento de runtime, schema, migration, contrato de produção ou superfície do produto — emenda constitucional e workflow advisory não elevam nenhum required check. Mesmo critério dos precedentes imediatos 089 (`0.4.3 → 0.4.4`) e 090 (`0.4.4 → 0.4.5`) para mudanças de governança/CI sem quebra de contrato. **Nenhuma tag criada**; registro a critério do maintainer.

## Changelog (FR / T)

- `FR-001` auditoria viva registrada: `--remote` PASS com `dev`/`main` alinhados, 4 required checks com *strict*/*enforce_admins*, CI green, gate `check_gate_integrity.py` PASS (10 legs / 58 pisos); PAT do SE e secret referidos por nome, zero valores exibidos, zero required checks novos.
- `FR-002` emenda 2026-11-20 à ADR-0003 vigente — "CI/CD GREEN basta", sem aprovador distinto; constituição v1.4.0 aprovada SE; emendas pendentes (SolidJS vs Svelte 5 / PGMQ obrigatório) permanecem `[NV]` a maintainer.
- `FR-003` deploy em produção registrado como decisão separada via MDMV-Autonoma, fora do gate; **`[NV]`** — somente app Staging no Coolify, sem app de produção/FQDN/revisão; reconsultar antes de promoção/rollback/deploy.
- `FR-004` explicações auditáveis separam fatos verificados de itens diferidos `[NV]` (Tauri/OTLP, leg RBAC, emendas), com justificativa e continuidade das decisões 088–090, sem reescrever histórico.
- `FR-005` 4 scripts SDD da factory omitidos de relatórios como gate/evidência (decisão 089); único gate autoritativo `check_gate_integrity.py`; 25 alerts em `package-lock.json` (7 high) não promovidos a gate.
- `T001` âncora 091 e verdict `go` fixados · `T002` auditoria viva do CI/proteção (`--remote` PASS) · `T003` emenda ADR-0003 confirmada · `T004` deploy produção registrado como decisão separada (`[NV]`) · `T005` explicações auditáveis · `T006` omissão dos scripts SDD · `T007` verificação de gate e fronteira do diff · `T008` fechamento dos artefatos e rastreabilidade (SC-001..SC-005).

## Evidência

`.specify/validations/091-governance-ci-audit/validate.md` (R4 **APROVADO**, saídas literais consolidadas do handoff) · `specs/091-governance-ci-audit/analyze.md` (READ-ONLY, `critical=0`, `requirements_covered=5/5`, `tasks=8/8`) · `specs/091-governance-ci-audit/converge.md` (**Converged**, sem novas tarefas) · baseline `673c687ea` · diff do ciclo: constituição v1.4.0 + workflow advisory + artefatos documentais 091. Nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release.

## Rollback

Reverter a deriva governamental entregue, sem force-push e sem reescrita de histórico:

1. **Constituição**: restaurar `.specify/memory/constitution.md` para **v1.3.0** (estado anterior a 2026-11-20 — `git checkout -- .specify/memory/constitution.md` se as alterações ainda estiverem não commitadas; se commitadas, novo commit de reversão ou `git revert` do commit do ciclo). Isso desfaz a emenda ADR-0003, o Svelte 5 canônico e as notas ADR-0001/0002 registradas em v1.4.0.
2. **Workflow de auditoria**: remover `.github/workflows/branch-protection-audit.yml` (novo neste ciclo) — `rm` se não commitado; `git revert`/commit de reversão se commitado. Nenhum outro workflow é tocado.
3. **Secret `FLORA_BRANCH_PROTECTION_TOKEN`: NÃO reverter.** Permanece registrado no GitHub — ele já existia e não é criado, alterado ou removido por este ciclo; remover quebraria a leitura de proteção e não faz parte do rollback.
4. Após a reversão, confirmar que nenhum required check, gate, ADR restante, código, config de produção ou DB foi alterado — estes não foram tocados pelo ciclo. O gate autoritativo `check_gate_integrity.py` e a proteção de branch devem permanecer como antes (reconsultar estado ao vivo, `[NV]`).

Não há arquivos novos de código, migrations, testes, config de produção nem artefatos de runtime para remover — apenas os documentos do ciclo 091 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`, `analyze.md`, `converge.md`, R4 e este release), que são registros e não afetam o sistema. **Nenhum banco de produção é tocado, alterado ou removido.**

## [NV]

- **CI do PR/merge/produção**: nenhum PR, tag, deploy ou merge realizado; execução ao vivo do GitHub Actions, aprovações de maintainer, proteção de branch e ambiente de produção não verificados além do handoff. Gate local/`--remote` PASS não substitui a CI remota para promoção.
- **Reexecução nesta passagem**: nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release; evidência literal consolidada no R4/handoff do main agent.
- **Deploy em produção**: `[NV]` — somente app Staging existe no Coolify; sem app de produção, FQDN ou revisão.
- **DBs de produção**: nenhum acesso, escrita ou remoção.
- **Escala**: mudança governamental/documental (constituição + workflow advisory + artefatos 091); sem teste de carga, UI ou comportamento de runtime a verificar; alertas de `package-lock.json` (25 / 7 high) seguem como recomendação de triagem, não gate.
