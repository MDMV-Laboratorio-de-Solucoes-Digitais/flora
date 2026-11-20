# Release: 087 — Leg bootstrap no CI e piso no gate de integridade

**Ciclo**: 087 · **Data**: 2026-11-20 · **Branch**: `feature/087-bootstrap-ci-leg` · **Baseline**: `367a50e79`
**Tipo**: governança de CI — duas linhas tornam os contratos DB de bootstrap obrigatórios no job `db-tests` e protegem o novo piso contra drift no gate autoritativo. **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `.github/workflows/rust.yml` — `run_leg bootstrap     contract::bootstrap::tests::db_ 4` no job `db-tests`, imediatamente após a leg `rbac`. Usa o helper `run_leg()` já existente: lista os `#[ignore]` com `cargo test -- --list`, executa com `--ignored --test-threads=1`, conta os `passed;` e exige `passed >= floor`. O filtro `::db_` casa a regex do guarda sem relaxá-la.
- `scripts/ci/check_gate_integrity.py` — `"bootstrap": 4,` em `FLOORS`, ao lado de `"rbac": 3` (preservado). O gate passa de 9 legs/54 contratos para **10 legs/58 contratos** e detecta remoção/divergência da leg no workflow.
- Verificação: **gate-integrity PASS** com `all 10 db contract floors intact (58 total)` (`exit=0`); listagem = **4** contratos `db_bootstrap_*`; leg `bootstrap` local **4 passed** (`bootstrap: passed=4 floor=4`); `git diff --check` sem saída (`exit=0`); `git diff --stat` = `2 files changed, 2 insertions(+)`. Detalhe literal em `.specify/validations/087-bootstrap-ci-leg/validate.md` (R4 **APROVADO**).

## SemVer

**Proposta:** patch `0.4.1` → `0.4.2`. Motivo: nenhuma alteração de API, comportamento de runtime, schema, migration, contrato de produção ou superfície do produto — apenas cobertura de CI (nova leg de contratos já existentes) e detecção de drift no gate. Mesmo critério do precedente imediato 086 (patch `0.4.0 → 0.4.1` para mudança idêntica: leg RBAC no CI e piso no gate). **Nenhuma tag criada**; registro a critério do maintainer.

## Changelog (FR / T)

- `FR-001` o job `db-tests` passa a executar a leg `bootstrap` com filtro `contract::bootstrap::tests::db_` e piso mínimo 4, usando o helper `run_leg`, cobrindo os quatro contratos `db_bootstrap_*` (`creates_org_admin_role_membership_workspace_and_session`, `rejects_second_call_without_writes`, `rejects_when_organization_exists`, `concurrent_calls_create_single_organization`) que antes existiam como `#[ignore]` mas nunca corriam na CI.
- `FR-002` `FLOORS` declara `"bootstrap": 4` (10ª entrada, ao lado de `"rbac": 3`), fazendo o gate conferir 10 pisos e 58 contratos e falhar com `rust.yml: db floor for bootstrap (4) missing or changed` se a leg for removida ou divergir do workflow.
- `FR-003` a leg exige `FLORA_DATABASE__POSTGRES_URL` e `DATABASE_URL` e falha cedo (fail-closed) quando ausentes, sem alterar o comportamento das outras 9 legs; os nove pisos existentes (auth 13, messaging 4, websocket 8, search 8, notifications 5, files 6, tasks 4, workspace 3, rbac 3) permanecem inalterados: total 54 → 58, com gate **PASS** e leg local **4 passed**.
- `FR-004` a mudança se limita a duas linhas em dois arquivos de CI/gate, com `git diff --check` limpo; `validate/`, `release/`, `.specify/memory/STATE.md`, runtime, documentação de produto, config de produção e banco não são modificados.
- `T001` âncora confirmada: `.specify/feature.json` = `specs/087-bootstrap-ci-leg`, branch `feature/087-bootstrap-ci-leg` @ `367a50e79`; `T002` inventário dos 4 contratos `#[ignore]` em `flora-tests/src/contract/bootstrap.rs:138/188/211/229` e da ausência da leg no workflow base; `T003` inclusão de `run_leg bootstrap ... 4` em `rust.yml:116` (após `rbac`); `T004` inclusão de `"bootstrap": 4,` em `FLOORS` (:52); `T005` verificação do gate (`PASS`, 10/58) e da listagem (4 contratos); `T006` verificação da leg local (4 passed), do escopo do diff (2 arquivos/2 inserções, zero arquivos em `validate/`, `release/` ou `STATE.md`) e fechamento dos artefatos do ciclo com R4/R7.

## Evidência

`.specify/validations/087-bootstrap-ci-leg/validate.md` (R4 **APROVADO**, saídas literais) · baseline `367a50e79` · diff do ciclo: `.github/workflows/rust.yml` +1 e `scripts/ci/check_gate_integrity.py` +1.

## Rollback

Reverter **exatamente duas linhas**, sem force-push e sem reescrita de histórico:

1. remover `run_leg bootstrap     contract::bootstrap::tests::db_ 4` de `.github/workflows/rust.yml` (job `db-tests`, após a leg `rbac`);
2. remover `"bootstrap": 4,` de `FLOORS` em `scripts/ci/check_gate_integrity.py`.

Se as alterações ainda estiverem não commitadas, basta `git checkout --` nesses dois arquivos; se já commitadas, usar um novo commit de reversão (ou `git revert` dos commits do ciclo). Não há arquivos novos de código, migrations nem outros artefatos de runtime para remover — apenas os artefatos documentais 087 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`, este release e o R4), que são registros do ciclo e não afetam o sistema. Após a reversão, repetir `python3 scripts/ci/check_gate_integrity.py`: o esperado é voltar a `all 9 db contract floors intact (54 total)` (9 legs/54 contratos). **Nenhum banco de produção é tocado, alterado ou removido** — os contratos `db_bootstrap_*` usam apenas Postgres descartável via `FLORA_DATABASE__POSTGRES_URL` / `DATABASE_URL`; nenhum dado de produção participa do rollback. Gate, workflows alheios, ADRs, constituição, STATE, specs de outros ciclos, PRs, rede, segredos e worktrees alheios permanecem intactos.

## [NV]

- **CI do PR/merge/produção**: nenhum PR, tag, deploy ou merge realizado; execução ao vivo do GitHub Actions, Forgejo, aprovações de maintainer, proteção de branch e ambiente de produção não verificados neste ciclo. Gate local PASS, listagem de 4 contratos e leg local 4 passed não substituem a CI remota.
- **Reexecução nesta passagem**: nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release; evidência literal consolidada no R4/handoff do main agent.
- **DBs de produção**: nenhum acesso, escrita ou remoção; cobertura é somente de contratos DB-gated com banco descartável.
- **Escala**: mudança de configuração de CI/gate; sem teste de carga, UI ou comportamento de runtime a verificar.
