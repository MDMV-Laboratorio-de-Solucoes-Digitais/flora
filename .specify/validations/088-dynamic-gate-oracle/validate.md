# Validation: 088 — Oráculo dinâmico de contratos ignorados no gate de CI

**Data**: 2026-11-20 · **Branch**: `feature/088-dynamic-gate-oracle` · **Baseline**: `cbc885383`
**Tipo**: governança de CI — substituição do espelho `FLOORS` ↔ `run_leg` por descoberta dinâmica dos contratos `#[ignore]` dentro de `scripts/ci/check_gate_integrity.py`. Nenhuma mudança de API, runtime, schema, migration, workflow, constituição, ADRs, PR, merge, deploy ou banco de produção.
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py` | **gate-integrity: PASS**, `exit=0` — `all 10 db contract floors intact (58 total)`; a mensagem agora deriva da descoberta (10 legs, 58 contratos), não de soma digitada (FR-001, FR-003, FR-004, SC-001) |
| `python3 -m py_compile scripts/ci/check_gate_integrity.py` | `exit=0` — sintaxe válida do guarda alterado |
| `git diff --stat cbc885383` | `1 file changed, 59 insertions(+), 5 deletions(-)` (1 arquivo, +59/-5) — fronteira restrita ao guarda (FR-005, SC-003) |
| Divergência controlada (piso em `FLOORS`, `run_leg` divergente, contrato `#[ignore]` extra, cargo falhando ou lista vazia) | falha fechada: mensagem de piso/contagem, `exit != 0` (FR-002, FR-004, SC-002) |

### Saídas coladas

Gate autoritativo (saída literal consolidada do handoff):

```
$ python3 scripts/ci/check_gate_integrity.py
gate-integrity: PASS
all 10 db contract floors intact (58 total)
exit=0
```

Sintaxe:

```
$ python3 -m py_compile scripts/ci/check_gate_integrity.py
exit=0
```

Patch:

```
$ git diff --stat cbc885383
scripts/ci/check_gate_integrity.py | 1 file changed, 59 insertions(+), 5 deletions(-)
```

Nota de proveniência: esta passagem R4 **não reexecutou** gates, build, testes, formatters, git, CI ao vivo nem inspeção visual. As saídas acima são a evidência literal consolidada do handoff do main agent: gate dinâmico PASS com `all 10 db contract floors intact (58 total)` (`exit=0`), `python3 -m py_compile` com `exit=0` e diff de `scripts/ci/check_gate_integrity.py` com 1 arquivo, +59/-5. Nenhuma inferência além dessa evidência.

## Fronteira

Contra a base `cbc885383`, um único arquivo de código alterado:

- `scripts/ci/check_gate_integrity.py` — `LEG_RE` extrai as 10 linhas `run_leg <label> <filter> <floor>` de `.github/workflows/rust.yml`; `subprocess.run(["cargo", "test", "-p", "flora-tests", "--", "--ignored", "--list"], cwd=ROOT, capture_output=True, text=True, check=False)` fornece a lista completa; somente linhas de `stdout` terminadas em `: test` são contadas e associadas por `startswith`; cada leg exige rótulo em `FLOORS`, piso igual ao mapa, uma única `run_leg` e contagem exatamente igual ao piso. Cargo falho, lista vazia, ausência de legs e qualquer divergência entram em falha fechada. A mensagem `all {len(FLOORS)} db contract floors intact ({sum(FLOORS.values())} total)` é preservada.
- Checagens estáticas anteriores (`REQUIRED`, jobs, `continue-on-error`, path filters, `cargo build/test --workspace`) e o modo `--remote` permanecem intactos; stdlib apenas, nenhuma dependência nova (FR-005).

Nada mais foi alterado: sem workflow, testes, código de aplicação, artefatos de runtime, config de produção, `STATE.md`, `validate/` ou `release/` fora deste R4. Os artefatos do ciclo 088 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`, `analyze.md`, `converge.md`) são registros documentais e não afetam o sistema.

## [NV]

- **CI do PR/merge/produção**: nenhum PR, tag, deploy ou merge realizado/verificado neste snapshot; execução ao vivo do GitHub Actions, Forgejo, aprovações de maintainer, proteção de branch e ambiente de produção não verificados neste ciclo. O gate local PASS não substitui a CI remota.
- **Reexecução nesta passagem**: gates, testes, formatters, build e comandos git não foram reexecutados ao escrever este R4; cobertura determinística atribuída ao handoff do main agent.
- **DBs de produção**: nenhum banco de produção foi tocado, alterado ou removido; o oráculo apenas enumera testes `#[ignore]` que usam Postgres descartável.
- **Escala da mudança**: um arquivo de CI/gate (+59/-5); sem teste de carga, verificação visual de UI ou alteração de comportamento de runtime a validar.

## Portarias

R1 `go` antes da implementação (confidence gap dos contratos `#[ignore]` órfãos registrado em `.specify/assessments/088-dynamic-gate-oracle/decision.md`, verdict go, 2026-11-20); R2 analyze READ-ONLY com `critical=0` e `requirements_covered=7/7` (`specs/088-dynamic-gate-oracle/analyze.md`); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`; R5 nenhum bug de `.specify/bugs/` acionado neste ciclo; R7 release com SemVer patch `0.4.2 → 0.4.3`, changelog FR-001..FR-005 / T001..T007 e rollback para a validação estática anterior no guarda (remover `LEG_RE`, o `subprocess` de descoberta e a contagem dinâmica), sem DBs de produção. Nenhuma invocação de `/speckit.validate` ou `/speckit.release` — ambos são artefatos escritos diretamente. Nenhuma tag, deploy ou merge executados; PR a critério do maintainer.
