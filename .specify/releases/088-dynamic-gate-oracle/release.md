# Release: 088 — Oráculo dinâmico de contratos ignorados no gate de CI

**Ciclo**: 088 · **Data**: 2026-11-20 · **Branch**: `feature/088-dynamic-gate-oracle` · **Baseline**: `cbc885383`
**Tipo**: governança de CI — o guarda de integridade passa a consultar a lista real de testes `#[ignore]` como fonte de verdade, em vez de comparar duas listas escritas à mão. **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `scripts/ci/check_gate_integrity.py` — `LEG_RE` extrai as 10 linhas `run_leg <label> <filter> <floor>` de `.github/workflows/rust.yml`; `subprocess.run(["cargo", "test", "-p", "flora-tests", "--", "--ignored", "--list"], cwd=ROOT, capture_output=True, text=True, check=False)` descobre os contratos; somente linhas de `stdout` terminadas em `: test` são coletadas e associadas à leg cujo filtro é prefixo do nome (`startswith`). Cada leg exige `label in FLOORS`, `floor == FLOORS[label]`, uma única `run_leg` por rótulo e contagem exatamente igual ao piso. Cargo falho, lista vazia, ausência de legs e qualquer divergência falham fechado. As checagens estáticas (`REQUIRED`, jobs, `continue-on-error`, path filters, `cargo build/test --workspace`) e o modo `--remote` permanecem intactos; stdlib apenas.
- Verificação: **gate-integrity PASS** com `all 10 db contract floors intact (58 total)` (`exit=0`); `python3 -m py_compile` `exit=0`; `git diff --stat cbc885383` = `1 file changed, 59 insertions(+), 5 deletions(-)`. Detalhe literal em `.specify/validations/088-dynamic-gate-oracle/validate.md` (R4 **APROVADO**).

## SemVer

**Proposta:** patch `0.4.2` → `0.4.3`. Motivo: nenhuma alteração de API, comportamento de runtime, schema, migration, contrato de produção ou superfície do produto — apenas a forma como o guarda de CI descobre e conta contratos já existentes. Mesmo critério do precedente imediato 087 (patch `0.4.1 → 0.4.2` para mudança de governança/CI). **Nenhuma tag criada**; registro a critério do maintainer.

## Changelog (FR / T)

- `FR-001` o guarda descobre os contratos ignorados com `cargo test -p flora-tests -- --ignored --list`, `cwd=ROOT`, `capture_output=True`, `text=True`, consumindo somente `stdout`.
- `FR-002` `LEG_RE` extrai de `rust.yml` cada `run_leg <label> <filter> <floor>` com regex ancorada em linha; cada nome terminado em `: test` é associado à leg cujo `filter` é prefixo dele (`startswith`), cobrindo prefixos sobrepostos como `contract::auth::db_`.
- `FR-003` para cada leg o guarda exige rótulo em `FLOORS`, piso do workflow igual a `FLOORS[label]`, exatamente uma `run_leg` por rótulo e quantidade de nomes casados exatamente igual ao piso (10 legs / 58 contratos).
- `FR-004` falha fechada (`exit != 0`) se o cargo falhar, a lista estiver vazia, não houver legs ou qualquer contagem divergir; sucesso imprime `all N db contract floors intact (M total)` — hoje `all 10 db contract floors intact (58 total)`.
- `FR-005` `REQUIRED`, jobs, `continue-on-error`, path filters, `cargo build/test --workspace` e `--remote` preservados; stdlib apenas; diff limitado a `scripts/ci/check_gate_integrity.py` e aos artefatos 088, sem tocar workflow, `validate/`, `release/` ou `STATE.md`.
- `T001` âncora confirmada: `.specify/feature.json` = `specs/088-dynamic-gate-oracle`, branch @ `cbc885383`, único arquivo de código no diff; `T002` diagnóstico do espelho `FLOORS` ↔ `run_leg` e validação da fonte de descoberta (58 contratos, 10 pisos); `T003` descoberta via subprocess no guarda; `T004` validação de cada leg contra a descoberta; `T005` fail-closed e preservação do gate existente; `T006` validação do oráculo e da detecção de divergência (SC-001/SC-002); `T007` validação do escopo do diff e fechamento dos artefatos 088 (SC-003/FR-005).

## Evidência

`.specify/validations/088-dynamic-gate-oracle/validate.md` (R4 **APROVADO**, saídas literais) · `specs/088-dynamic-gate-oracle/analyze.md` (READ-ONLY, `critical=0`, `requirements_covered=7/7`) · `specs/088-dynamic-gate-oracle/converge.md` (**Converged**, sem novas tarefas) · baseline `cbc885383` · diff do ciclo: `scripts/ci/check_gate_integrity.py` `1 file changed, 59 insertions(+), 5 deletions(-)`.

## Rollback

Restaurar a **validação estática anterior** em `scripts/ci/check_gate_integrity.py`, sem force-push e sem reescrita de histórico:

1. remover `LEG_RE` e o laço que extrai as `run_leg` de `rust.yml`;
2. remover o `subprocess.run(... --ignored --list ...)` e o parser de linhas `: test`;
3. remover a contagem dinâmica por `startswith` e a validação por leg, restaurando o loop estático original que compara `FLOORS` com as linhas `run_leg` do workflow;
4. manter `FLOORS` (10 entradas, total 58), `REQUIRED`, jobs, `--remote` e a mensagem `all 10 db contract floors intact (58 total)` — a antiga verificação estática já produzia essa saída.

Se as alterações ainda estiverem não commitadas, basta `git checkout -- scripts/ci/check_gate_integrity.py`; se já commitadas, usar um novo commit de reversão (ou `git revert` do commit do ciclo). Não há arquivos novos de código, migrations, testes, config de produção nem artefatos de runtime para remover — apenas os documentos do ciclo 088 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`, `analyze.md`, `converge.md`, este release e o R4), que são registros e não afetam o sistema. Após a reversão, repetir `python3 scripts/ci/check_gate_integrity.py`: o esperado é o mesmo **PASS** `all 10 db contract floors intact (58 total)`, agora pelo espelho estático. **Nenhum banco de produção é tocado, alterado ou removido**; nenhum dado de produção participa do rollback. Gate alheio, workflows, ADRs, constituição, STATE, specs de outros ciclos, PRs, rede, segredos e worktrees alheios permanecem intactos.

## [NV]

- **CI do PR/merge/produção**: nenhum PR, tag, deploy ou merge realizado; execução ao vivo do GitHub Actions, Forgejo, aprovações de maintainer, proteção de branch e ambiente de produção não verificados neste ciclo. Gate local PASS não substitui a CI remota.
- **Reexecução nesta passagem**: nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release; evidência literal consolidada no R4/handoff do main agent.
- **DBs de produção**: nenhum acesso, escrita ou remoção; o oráculo apenas enumera nomes de testes `#[ignore]` com Postgres descartável.
- **Escala**: mudança em um arquivo de gate/CI (+59/-5); sem teste de carga, UI ou comportamento de runtime a verificar.
