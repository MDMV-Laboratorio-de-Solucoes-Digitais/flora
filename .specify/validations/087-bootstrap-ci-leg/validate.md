# Validation: 087 — Leg bootstrap no CI e piso no gate de integridade

**Data**: 2026-11-20 · **Branch**: `feature/087-bootstrap-ci-leg` · **Baseline**: `367a50e79`
**Tipo**: ajuste de governança/CI — exatamente duas linhas: `run_leg bootstrap     contract::bootstrap::tests::db_ 4` no job `db-tests` de `.github/workflows/rust.yml` e `"bootstrap": 4` em `FLOORS` de `scripts/ci/check_gate_integrity.py`. Nenhuma mudança de runtime, API, schema, migration, constituição, ADRs, PR, merge, deploy ou banco de produção.
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py` | **gate-integrity: PASS**, `exit=0` — `ok all 10 db contract floors intact (58 total)`; os 9 pisos anteriores (total 54) seguem intactos e o novo piso `bootstrap=4` é reconhecido (FR-002, FR-003) |
| `cargo test -p flora-tests -- --list \| grep 'contract::bootstrap::tests::db_' \| wc -l` | `4` — os quatro contratos `db_bootstrap_*` descobertos sob o filtro (FR-001) |
| `cargo test -p flora-tests contract::bootstrap::tests::db_ -- --ignored --test-threads=1` (leg `bootstrap`) | **4 passed** — `bootstrap: passed=4 floor=4`, `exit=0`; os quatro contratos `db_bootstrap_*` PASS (FR-001, FR-003) |
| `git diff --check` | sem saída, `check_exit=0` — nenhum erro de whitespace/patch (FR-004) |
| `git diff --stat` | `2 files changed, 2 insertions(+)` — fronteira restrita ao workflow e ao script do gate (FR-004) |

### Saídas coladas

Gate autoritativo:

```
=== gate de integridade ===
gate-integrity: PASS
  ok  job frontend-build present and non-vacuous
  ok  job rust-build present and non-vacuous
  ok  job db-tests present and non-vacuous
  ok  job dependency-review present and non-vacuous
  ok  rust.yml contains 'cargo build --workspace'
  ok  rust.yml contains 'cargo test --workspace'
  ok  all 10 db contract floors intact (58 total)
exit=0
```

Descoberta dos contratos (filtro `contract::bootstrap::tests::db_`):

```
$ cargo test -p flora-tests -- --list | grep 'contract::bootstrap::tests::db_' | wc -l
4
```

Leg `bootstrap` (contratos DB-gated, quatro com `#[ignore = "requires Postgres"]`):

```
== bootstrap: contract::bootstrap::tests::db_ (floor 4) ==
running 4 tests
test contract::bootstrap::tests::db_bootstrap_creates_org_admin_role_membership_workspace_and_session ... ok
test contract::bootstrap::tests::db_bootstrap_rejects_second_call_without_writes ... ok
test contract::bootstrap::tests::db_bootstrap_rejects_when_organization_exists ... ok
test contract::bootstrap::tests::db_bootstrap_concurrent_calls_create_single_organization ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured
bootstrap: passed=4 floor=4
```

Patch:

```
$ git diff --check
check_exit=0
$ git diff --stat
.github/workflows/rust.yml         | 1 +
scripts/ci/check_gate_integrity.py | 1 +
2 files changed, 2 insertions(+)
```

Nota de proveniência: esta passagem R4 **não reexecutou** gates, build, testes, formatters, git, CI ao vivo nem inspeção visual. As saídas acima são a evidência literal consolidada do handoff do main agent: gate `PASS` com `all 10 db contract floors intact (58 total)` (`exit=0`), listagem com `4` contratos `db_bootstrap_*`, leg `bootstrap` com `4 passed` (`bootstrap: passed=4 floor=4`) e `git diff --check` limpo (`exit=0`). A linha `test result:` está transcrita até a contagem de `4 passed`; o sufixo `filtered out`/tempo do runner local não foi repassado no handoff e não é inferido aqui. Nenhuma inferência além dessa evidência.

## Fronteira

Contra a base `367a50e79`, exatamente duas adições:

- `.github/workflows/rust.yml:116` — `run_leg bootstrap     contract::bootstrap::tests::db_ 4`, depois de `rbac`, no job `db-tests`. A leg usa o mesmo contrato `run_leg()` das nove legs existentes (listagem `cargo test -- --list`, execução `--ignored --test-threads=1`, contagem de `passed;` e comparação com o piso). O filtro usa `::db_` (não `::db_bootstrap_`) para casar a regex do guarda (linha 119) sem alterá-la; a forma longa foi descartada por não casar (`match=False`).
- `scripts/ci/check_gate_integrity.py:52` — `"bootstrap": 4,` em `FLOORS`, ao lado de `"rbac": 3` (preservado). O gate passa a verificar 10 pisos e 58 contratos no total (era 9/54) e detecta remoção ou divergência da leg no workflow.

Nada mais foi alterado: sem código de aplicação, sem testes novos, sem artefatos de runtime, sem config de produção, sem movimentação de DB.

## [NV]

- **CI do PR/merge/produção**: zero PRs abertos/verificados neste snapshot; execução ao vivo do GitHub Actions, Forgejo, proteção de branch, aprovações, merge e deploy não verificados neste ciclo. O gate local PASS, a listagem de 4 contratos e a leg local 4 passed não substituem o resultado da CI remota.
- **Reexecução nesta passagem**: gates, testes, formatters, build e comandos git não foram reexecutados ao escrever este R4; cobertura determinística atribuída ao handoff do main agent.
- **DBs de produção**: nenhum banco de produção foi tocado, alterado ou removido. Os contratos `db_bootstrap_*` exigem Postgres descartável fornecido por `FLORA_DATABASE__POSTGRES_URL` / `DATABASE_URL`; nenhum estado persistente de produção é usado.
- **Escala da mudança**: duas linhas de configuração de CI/gate; sem teste de carga, sem verificação visual de UI e sem alteração de comportamento de runtime a validar.

## Portarias

R1 `go` antes da implementação (confidence gap dos contratos DB de bootstrap registrado em `.specify/assessments/087-bootstrap-ci-leg/decision.md`, verdict go, 2026-11-20); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`; R5 nenhum bug de `.specify/bugs/` acionado neste ciclo; R7 release com SemVer patch `0.4.1 → 0.4.2`, changelog FR-001..FR-004 / T001..T006 e rollback restrito às duas linhas (leg no workflow e piso no gate), sem DBs de produção. Nenhuma invocação de `/speckit.validate` ou `/speckit.release` — ambos são artefatos escritos diretamente. Nenhuma tag, deploy ou merge executados; PR a critério do maintainer.
