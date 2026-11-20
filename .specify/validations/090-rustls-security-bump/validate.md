# Validation: 090 — Bump de segurança rustls 0.23.43 → 0.23.46

**Data**: 2026-11-20 · **Branch**: `feature/090-rustls-security-bump` · **Baseline**: `824ba00e9`
**Tipo**: atualização de segurança de dependência — lockfile e isenção cargo-vet alinhados para `rustls 0.23.46`; `Cargo.toml:189` permanece na faixa `0.23`. Nenhuma mudança de API, runtime, schema, migration, workflow, constituição, ADRs, PR, merge, deploy ou banco de produção.
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| leitura de `Cargo.lock:2748` | `name = "rustls"` → `version = "0.23.46"`; nenhuma entrada `0.23.43` residual (FR-002, SC-001) |
| leitura de `Cargo.toml:189` | `rustls = "0.23"` inalterado — manifest fora do diff (FR-001) |
| leitura de `supply-chain/config.toml:1115-1117` | `[[exemptions.rustls]]` → `version = "0.23.46"`, `criteria = "safe-to-deploy"` preservado (FR-003, SC-001) |
| `python3 scripts/ci/check_gate_integrity.py` | **gate-integrity: PASS**, `exit=0` — `all 10 db contract floors intact (58 total)` (FR-004) |
| `cargo test --workspace` | `exit=0` — 0 falhas na suíte completa (FR-004) |
| `dependency-review` | limpo — nenhum achado de vulnerabilidade (FR-005) |

### Saídas coladas

Evidência esperada/consolidada do handoff do ciclo:

```
$ grep -n -A2 '^name = "rustls"' Cargo.lock
2748:name = "rustls"
2749-version = "0.23.46"
2750-source = "registry+https://github.com/rust-lang/crates.io-index"

$ sed -n '1115,1117p' supply-chain/config.toml
[[exemptions.rustls]]
version = "0.23.46"
criteria = "safe-to-deploy"

$ python3 scripts/ci/check_gate_integrity.py
gate-integrity: PASS
all 10 db contract floors intact (58 total)
exit=0

$ cargo test --workspace
exit=0  (0 failed)

dependency-review: limpo — 0 achados
```

Nota de proveniência: esta passagem R4 **não reexecutou** gates, build, testes, formatters, git, CI ao vivo nem inspeção visual. As saídas acima são a evidência literal **esperada e consolidada do handoff do main agent**: `rustls 0.23.46` no lock, isenção ajustada para `0.23.46`, gate PASS, `cargo test --workspace` sem falhas e dependency-review limpo. Nenhuma inferência além dessa evidência.

## Fronteira

Contra a base `824ba00e9`, a mudança de código fica restrita a dois arquivos:

- `Cargo.lock` — entrada de `rustls` atualizada de `0.23.43` para `0.23.46`; demais entradas inalteradas. `Cargo.toml:189` mantém `rustls = "0.23"` (FR-001, FR-002).
- `supply-chain/config.toml` — `[[exemptions.rustls]]` em `1115-1117` passa a `version = "0.23.46"`, com `criteria = "safe-to-deploy"` preservado (FR-003).

Nada mais foi alterado: sem manifest, workflow, testes, código de aplicação, artefatos de runtime, config de produção, `STATE.md`, `validate/` ou `release/` fora deste R4. Os artefatos do ciclo 090 (`spec.md`, `analyze.md`, `converge.md`) são registros documentais e não afetam o sistema.

## [NV]

- **CI do PR/merge/produção**: nenhum PR, tag, deploy ou merge realizado/verificado neste snapshot; execução ao vivo do GitHub Actions, aprovações de maintainer, proteção de branch e ambiente de produção não verificados neste ciclo. O gate local PASS não substitui a CI remota.
- **Reexecução nesta passagem**: gates, testes, formatters, build e comandos git não foram reexecutados ao escrever este R4; cobertura determinística atribuída ao handoff do main agent.
- **DBs de produção**: nenhum banco de produção foi tocado, alterado ou removido.
- **Escala da mudança**: 2 arquivos (lockfile + isenção cargo-vet); sem teste de carga, verificação visual de UI ou alteração de comportamento de runtime a validar além da suíte do workspace.

## Portarias

R1 `go` para o bump restrito a lockfile + isenção (faixa `0.23` do manifest inalterada; escopo definido no handoff do ciclo, sem decisão formal materializada em `.specify/assessments/090-.../`); R2 analyze READ-ONLY com `critical=0` e `requirements_covered=6/6` (`specs/090-rustls-security-bump/analyze.md`); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`; R5 nenhum bug de `.specify/bugs/` acionado neste ciclo; R7 release com SemVer patch `0.4.4 → 0.4.5`, changelog FR-001..FR-005 / T001..T006 e rollback para `Cargo.lock` e a isenção anteriores (`rustls 0.23.43`), sem DBs de produção. Nenhuma invocação de `/speckit.validate` ou `/speckit.release` — ambos são artefatos documentais deste ciclo.
