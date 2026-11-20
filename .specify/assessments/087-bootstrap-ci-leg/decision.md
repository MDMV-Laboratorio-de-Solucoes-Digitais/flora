# Decision: fechar o confidence gap do bootstrap no CI (ciclo 087)

- **Slug**: 087-bootstrap-ci-leg
- **Decidido**: 2026-11-20
- **Verdict**: **go**
- **Branch**: `feature/087-bootstrap-ci-leg` @ `367a50e79`
- **Artefatos revisados**: evidência local (workflow, gate, contratos bootstrap) | spec | plan | tasks

## Contexto / Problema

Quatro contratos DB em `flora-tests/src/contract/bootstrap.rs`, no módulo
`tests` (:7), estão marcados com
`#[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL"]`:

- `db_bootstrap_creates_org_admin_role_membership_workspace_and_session` (:138)
- `db_bootstrap_rejects_second_call_without_writes` (:188)
- `db_bootstrap_rejects_when_organization_exists` (:211)
- `db_bootstrap_concurrent_calls_create_single_organization` (:229)

Eles só rodam quando `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` estão
definidos. Até este ciclo o job `db-tests` de `.github/workflows/rust.yml` não
tinha nenhuma lega `bootstrap`. Como registrado no próprio workflow
(:83-84, cycle 037), um `cargo test` filtrado que casa zero testes termina com
exit 0 — a ausência da lega era silenciosa. Resultado: **confidence gap** —
o provisionamento atômico de organização/admin/role/membership/workspace/
sessão, a rejeição de segunda chamada e a concorrência podiam regredir sem
nenhum sinal vermelho.

## Decisão

Adicionar a lega `run_leg bootstrap contract::bootstrap::tests::db_ 4` ao job
`db-tests` e registrar `"bootstrap": 4` no mapa `FLOORS` de
`scripts/ci/check_gate_integrity.py`, preservando `"rbac": 3`. O filtro `db_`
cobre exatamente os 4 testes `db_bootstrap_*` descobertos por
`cargo test -- --list` e casa a regex existente do guarda (linha 118); o filtro
longo `...::db_bootstrap_` não casava essa regex. O guarda determinístico passa
a proteger também o décimo piso (9 → 10 entradas; 54 → 58 no total). Nenhuma
dependência, crate, serviço, rota ou mudança de API: duas linhas de
configuração de CI mais os quatro artefatos mínimos do ciclo.

## Scorecard

| Critério | Nota | Justificativa |
|---|---|---|
| Validade do problema | strong | 4 contratos `#[ignore]` em `bootstrap.rs` sem lega correspondente no CI; armadilha do filtro vazio já documentada em `rust.yml:83-84` |
| Força da evidência | strong | Diff de 2 linhas, linhas exatas do workflow (:116) e do gate (:52), lista de testes via `cargo test -- --list` embutida no conceito do piso |
| Valor vs. inação | strong | Inação mantém o bootstrap sem cobertura obrigatória; com a lega, regressão de provisionamento/idempotência/concorrência falha o `db-tests` |
| Viabilidade | strong | Reusa o helper `run_leg` e o mapa `FLOORS` existentes; nenhuma lógica nova, nenhuma API, nenhum segredo novo |
| Fit estratégico | strong | Mantém ADR-0003 (merge machine decision) íntegro: reforça um required check em vez de criar outro; consistente com os 9 pisos já presentes |
| Risco | strong | Diff inteiramente reversível; piso igual ao número de testes descobertos (4) — nem frouxo (0) nem frágil (>4). `bootstrap` não entra em `REQUIRED`/`ALLOWED_JOB_IF` |

## Handoff

**Dentro**: `.github/workflows/rust.yml` (+1), `scripts/ci/check_gate_integrity.py`
(+1) e os quatro artefatos 087 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`).

**Fora**: `validate/`, `release/`, `.specify/memory/STATE.md`, constituição,
ADRs, outros pisos/legs, código de aplicação, testes de contrato em si, PRs e
worktree espelho.

**Métricas de sucesso**: lega local com 4 passed; gate PASS com
`all 10 db contract floors intact (58 total)`; `git diff --name-only 367a50e79`
limitado aos seis arquivos acima.

**Verificação (evidência local registrada, gates/formatters não reexecutados
nesta avaliação de docs)**: leg `contract::bootstrap::tests::db_` local =
4 passed; `python3 scripts/ci/check_gate_integrity.py` = PASS com as 10 floors
(58).
