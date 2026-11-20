# Validation: 086 — Leg RBAC no CI e piso no gate de integridade

**Data**: 2026-11-20T10:30Z · **Branch**: `feature/086-rbac-ci-leg` · **Baseline**: `9b05abb9d`
**Tipo**: ajuste de governança/CI — exatamente duas linhas: `run_leg rbac contract::rbac::db::db_ 3` no job `db-tests` de `.github/workflows/rust.yml` e `"rbac": 3` em `FLOORS` de `scripts/ci/check_gate_integrity.py`. Nenhuma mudança de runtime, API, schema, migration, constituição, ADRs, PR, merge, deploy ou banco de produção.
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py` | **gate-integrity: PASS**, `exit=0` — `ok all 9 db contract floors intact (54 total)`; os 8 pisos anteriores (total 51) seguem intactos e o novo piso `rbac=3` é reconhecido (FR-002, FR-003) |
| `cargo test -p flora-tests contract::rbac::db::db_ -- --ignored --test-threads=1` (leg `rbac`) | **3 passed** — `rbac: passed=3 floor=3`, `exit=0`; os três contratos `db_rbac_*` PASS (FR-001, FR-003) |
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
  ok  all 9 db contract floors intact (54 total)
exit=0
```

Leg `rbac` (contratos DB-gated, três com `#[ignore = "requires Postgres"]`):

```
== rbac: contract::rbac::db::db_ (floor 3) ==
running 3 tests
test contract::rbac::db::db_rbac_list_scoped_to_session_org ... ok
test contract::rbac::db::db_rbac_create_requires_role_write_and_uses_session_org ... ok
test contract::rbac::db::db_rbac_builtin_immutable_custom_patchable ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured
rbac: passed=3 floor=3
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

Nota de proveniência: esta passagem R4 **não reexecutou** gates, build, testes, formatters, git, CI ao vivo nem inspeção visual. As saídas acima são a evidência literal consolidada do handoff do main agent: gate `PASS` com `all 9 db contract floors intact (54 total)`, leg `rbac` com 3 contratos passed e `git diff --check` limpo (`exit=0`). A linha `test result:` está transcrita até a contagem de `3 passed`; o sufixo `filtered out`/tempo do runner local não foi repassado no handoff e não é inferido aqui. Nenhuma inferência além dessa evidência.

## Fronteira

Contra a base `9b05abb9d`, exatamente duas adições:

- `.github/workflows/rust.yml:115` — `run_leg rbac          contract::rbac::db::db_     3`, depois de `workspace`, no job `db-tests`. A leg usa o mesmo contrato `run_leg()` das oito legs existentes (listagem `cargo test -- --list`, execução `--ignored --test-threads=1`, contagem de `passed;` e comparação com o piso).
- `scripts/ci/check_gate_integrity.py:51` — `"rbac": 3,` em `FLOORS`, ao lado de `workspace: 3`. O gate passa a verificar 9 pisos e 54 contratos no total (era 8/51) e detecta remoção ou divergência da leg no workflow.

Nada mais foi alterado: sem código de aplicação, sem testes novos, sem artefatos de runtime, sem config de produção, sem movimentação de DB.

## [NV]

- **CI do PR/merge/produção**: zero PRs abertos/verificados neste snapshot; execução ao vivo do GitHub Actions, Forgejo, proteção de branch, aprovações, merge e deploy não verificados neste ciclo. O gate local PASS e a leg local 3 passed não substituem o resultado da CI remota.
- **Reexecução nesta passagem**: gates, testes, formatters, build e comandos git não foram reexecutados ao escrever este R4; cobertura determinística atribuída ao handoff do main agent.
- **DBs de produção**: nenhum banco de produção foi tocado, alterado ou removido. Os contratos `db_rbac_*` exigem Postgres descartável fornecido por `FLORA_DATABASE__POSTGRES_URL`; nenhum estado persistente de produção é usado.
- **Escala da mudança**: duas linhas de configuração de CI/gate; sem teste de carga, sem verificação visual de UI e sem alteração de comportamento de runtime a validar.

## Portarias

R1 `go` antes da implementação (RBAC no CI era a próxima pendência autônoma registrada em `.specify/memory/STATE.md`); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`; R5 nenhum bug de `.specify/bugs/` acionado neste ciclo; R7 release com SemVer patch `0.4.0 → 0.4.1`, changelog FR-001..FR-004 / T001..T006 e rollback restrito às duas linhas (leg no workflow e piso no gate), sem DBs de produção. Nenhuma invocação de `/speckit.validate` ou `/speckit.release` — ambos são artefatos escritos diretamente. Nenhuma tag, deploy ou merge executados; PR a critério do maintainer.
