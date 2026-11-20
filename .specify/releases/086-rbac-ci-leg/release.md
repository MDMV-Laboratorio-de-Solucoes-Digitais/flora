# Release: 086 — Leg RBAC no CI e piso no gate de integridade

**Ciclo**: 086 · **Data**: 2026-11-20T10:30Z · **Branch**: `feature/086-rbac-ci-leg` · **Baseline**: `9b05abb9d`
**Tipo**: governança de CI — duas linhas tornam os contratos RBAC obrigatórios no job `db-tests` e protegem o novo piso contra drift no gate autoritativo. **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `.github/workflows/rust.yml` — `run_leg rbac          contract::rbac::db::db_     3` no job `db-tests`, imediatamente após a leg `workspace`. Usa o helper `run_leg()` já existente: lista os `#[ignore]` com `cargo test -- --list`, executa com `--ignored --test-threads=1`, conta os `passed;` e exige `passed >= floor`.
- `scripts/ci/check_gate_integrity.py` — `"rbac": 3,` em `FLOORS`, ao lado de `workspace: 3`. O gate passa de 8 legs/51 contratos para **9 legs/54 contratos** e detecta remoção/divergência da leg no workflow.
- Verificação: **gate-integrity PASS** com `all 9 db contract floors intact (54 total)`; leg `rbac` local **3 passed** (`rbac: passed=3 floor=3`); `git diff --check` sem saída (`exit=0`); `git diff --stat` = `2 files changed, 2 insertions(+)`. Detalhe literal em `.specify/validations/086-rbac-ci-leg/validate.md` (R4 **APROVADO**).

## SemVer

**Proposta:** patch `0.4.0` → `0.4.1`. Motivo: nenhuma alteração de API, comportamento de runtime, schema, migration, contrato de produção ou superfície do produto — apenas cobertura de CI (nova leg de contratos já existentes) e detecção de drift no gate. Mesmo critério do precedente 084 (patch `0.3.4 → 0.3.5` para mudança mecânica sem impacto de runtime); o ciclo 085 entregou `0.3.5 → 0.4.0`. **Nenhuma tag criada**; registro a critério do maintainer.

## Changelog (FR / T)

- `FR-001` o job `db-tests` passa a executar a leg `rbac` com filtro `contract::rbac::db::db_` e piso 3, cobrindo os três contratos `db_rbac_*` (`list_scoped_to_session_org`, `create_requires_role_write_and_uses_session_org`, `builtin_immutable_custom_patchable`) que antes não corriam na CI.
- `FR-002` `FLOORS` declara `"rbac": 3`, fazendo o gate conferir 9 pisos e 54 contratos e falhar se a leg for removida ou divergir do workflow.
- `FR-003` os oito pisos existentes (auth 13, messaging 4, websocket 8, search 8, notifications 5, files 6, tasks 4, workspace 3) permanecem inalterados: total 51 → 54, com gate **PASS** e leg local **3 passed**.
- `FR-004` a mudança se limita a duas linhas em dois arquivos, com `git diff --check` limpo; nada de runtime, documentação de produto, config de produção ou banco é modificado.
- `T001` branch `feature/086-rbac-ci-leg` sobre `9b05abb9d` e alvo ativo confirmado; `T002` inspeção do job `db-tests`, do helper `run_leg()` e dos pisos derivados da listagem de `#[ignore]`; `T003` inclusão de `run_leg rbac ... 3` no workflow; `T004` inclusão de `"rbac": 3` em `FLOORS`; `T005` verificação local da leg (3 passed) e do gate (`PASS`, 9/54); `T006` R4/R7 com `git diff --check` limpo e rollback documentado.

## Evidência

`.specify/validations/086-rbac-ci-leg/validate.md` (R4 **APROVADO**, saídas literais) · baseline `9b05abb9d` · diff do ciclo: `.github/workflows/rust.yml` +1 e `scripts/ci/check_gate_integrity.py` +1.

## Rollback

Reverter **exatamente duas linhas**, sem force-push e sem reescrita de histórico:

1. remover `run_leg rbac          contract::rbac::db::db_     3` de `.github/workflows/rust.yml` (job `db-tests`);
2. remover `"rbac": 3,` de `FLOORS` em `scripts/ci/check_gate_integrity.py`.

Se as alterações ainda estiverem não commitadas, basta `git checkout --` nesses dois arquivos; se já commitadas, usar um novo commit de reversão (ou `git revert` dos commits do ciclo). Não há arquivos novos, migrations, código de aplicação nem outros artefatos do ciclo para remover. Após a reversão, repetir `python3 scripts/ci/check_gate_integrity.py`: o esperado é voltar a `all 8 db contract floors intact (51 total)`. **Nenhum banco de produção é tocado, alterado ou removido** — os contratos `db_rbac_*` usam apenas Postgres descartável via `FLORA_DATABASE__POSTGRES_URL`; nenhum dado de produção participa do rollback. Gate, workflows alheios, ADRs, constituição, STATE, specs de outros ciclos, PRs, rede, segredos e worktrees alheios permanecem intactos.

## [NV]

- **CI do PR/merge/produção**: nenhum PR, tag, deploy ou merge realizado; execução ao vivo do GitHub Actions, Forgejo, aprovações de maintainer, proteção de branch e ambiente de produção não verificados neste ciclo. Gate local PASS e leg local 3 passed não substituem a CI remota.
- **Reexecução nesta passagem**: nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release; evidência literal consolidada no R4/handoff do main agent.
- **DBs de produção**: nenhum acesso, escrita ou remoção; cobertura é somente de contratos DB-gated com banco descartável.
- **Escala**: mudança de configuração de CI/gate; sem teste de carga, UI ou comportamento de runtime a verificar.
