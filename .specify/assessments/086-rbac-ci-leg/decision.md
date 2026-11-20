# Decision: fechar o confidence gap do RBAC no CI (ciclo 086)

- **Slug**: 086-rbac-ci-leg
- **Decidido**: 2026-11-20
- **Verdict**: **go**
- **Branch**: `feature/086-rbac-ci-leg` @ `9b05abb9d`
- **Artefatos revisados**: evidência local (workflow, gate, contratos RBAC) | spec | plan | tasks

## Contexto / Problema

Três contratos DB do RBAC em `flora-tests/src/contract/rbac.rs` —
`db_rbac_list_scoped_to_session_org` (:144),
`db_rbac_create_requires_role_write_and_uses_session_org` (:172) e
`db_rbac_builtin_immutable_custom_patchable` (:223) — estão marcados com
`#[ignore = "requires Postgres; ..."]` (:143, :171, :222). Só rodam quando
`FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` estão definidos. Até este ciclo o
job `db-tests` de `.github/workflows/rust.yml` não tinha nenhuma lega `rbac`,
portanto os três testes nunca eram exercitados no CI. Como registrado no próprio
workflow (:87-88, cycle 037), um `cargo test` filtrado que casa zero testes
termina com exit 0 — a ausência da lega era silenciosa. Resultado: **confidence
gap**: autorização por papel/organização podia regredir sem nenhum sinal vermelho.

## Decisão

Adicionar a lega `run_leg rbac contract::rbac::db::db_ 3` ao job `db-tests` e
registrar o piso `"rbac": 3` no mapa `FLOORS` de
`scripts/ci/check_gate_integrity.py`. O guarda determinístico existente passa a
proteger também esse piso, impedindo remoção ou drift silencioso futuro. Nenhuma
dependência, crate, serviço, rota ou mudança de API: duas linhas de configuração
de CI mais os quatro artefatos mínimos do ciclo.

## Scorecard

| Critério | Nota | Justificativa |
|---|---|---|
| Validade do problema | strong | 3 contratos `#[ignore]` em `rbac.rs` sem lega correspondente no CI; armadilha do filtro vazio já documentada em `rust.yml:87-88` |
| Força da evidência | strong | Diff de 2 linhas, linhas exatas do workflow (:115) e do gate (:51), lista de testes via `cargo test -- --list` embutida no conceito do piso |
| Valor vs. inação | strong | Inação mantém RBAC sem cobertura obrigatória; com a lega, toda regressão de escopo/imutabilidade de papéis falha o `db-tests` |
| Viabilidade | strong | Reusa o helper `run_leg` e o mapa `FLOORS` existentes; nenhuma lógica nova, nenhuma API, nenhum segredo novo |
| Fit estratégico | strong | Mantém ADR-0003 (merge machine decision) íntegro: reforça um required check em vez de criar outro; consistente com os 8 pisos já presentes |
| Risco | strong | Diff inteiramente reversível; piso igual ao número de testes descobertos (3) — nem frouxo (0) nem frágil (>3). `rbac` não entra em `REQUIRED`/`ALLOWED_JOB_IF` |

## Handoff

**Dentro**: `.github/workflows/rust.yml` (+1), `scripts/ci/check_gate_integrity.py`
(+1) e os quatro artefatos 086 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`).

**Fora**: `validate/`, `release/`, `.specify/memory/STATE.md`, constituição, ADRs,
outros pisos/legs, código de aplicação, testes de contrato em si, PRs e worktree
espelho.

**Métricas de sucesso**: lega local com 3 passed; gate PASS com
`all 9 db contract floors intact (54 total)`; `git diff --name-only 9b05abb9d`
limitado aos seis arquivos acima.

**Verificação (evidência local registrada, gates/formatters não reexecutados
nesta avaliação de docs)**: leg `contract::rbac::db::db_` local = 3 passed;
`python3 scripts/ci/check_gate_integrity.py` = PASS com as 9 floors (54).
