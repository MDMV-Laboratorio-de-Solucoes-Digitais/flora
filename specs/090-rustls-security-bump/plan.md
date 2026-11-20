# Implementation Plan: Bump de segurança rustls 0.23.43 → 0.23.46 (ciclo 090)

**Branch**: `feature/090-rustls-security-bump` @ `824ba00e9` | **Date**: 2026-11-20 | **Spec**: [spec.md](./spec.md)

**Input**: `/specs/090-rustls-security-bump/spec.md` · Âncora: `.specify/feature.json`

## Summary

Bump **patch** de `rustls` `0.23.43` → `0.23.46` somente no grafo resolvido
(`Cargo.lock:2748`) e na isenção cargo-vet
(`supply-chain/config.toml:1115-1117`). Fecha os 4 advisories RUSTSEC sem
tocar o manifest (`Cargo.toml:189` = `rustls = "0.23"`), o gate ou o diff de
código além desses dois arquivos.

## Technical Context

**Advisories**: `RUSTSEC-2026-0285` (medium, fix `0.23.45`),
`RUSTSEC-2026-0346` (medium, fix `0.23.46`), `RUSTSEC-2026-0348` (low, fix
`0.23.46`), `RUSTSEC-2026-0349` (low, fix `0.23.46`) — 4 alertas no lock raiz.

**Alvos**: `Cargo.lock:2748` → `version = "0.23.46"` (sem `0.23.43` residual);
`supply-chain/config.toml:1115-1117` → `version = "0.23.46"` com
`criteria = "safe-to-deploy"` preservado. `Cargo.toml:189` inalterado.

**Verificação**: `python3 scripts/ci/check_gate_integrity.py` → PASS (10 legs /
58 contratos, exit 0); `cargo test --workspace` → exit 0, zero falhas;
`dependency-review` → limpo. Execução ao vivo: `[NV]`.

**Constraints**: nenhum required check novo, nenhuma mudança de manifest,
workflow, ADR, constituição, `validate/`, `release/` ou `STATE.md`; sem PR,
merge, tag, deploy ou produção neste ciclo.

## Constitution Check

**Gate**: **PASS**. O bump é de dependência, não de processo: o gate
autoritativo permanece `check_gate_integrity.py` (ciclo 088, 10 legs / 58
contratos) e nenhuma proteção de branch é alterada. Critério `safe-to-deploy`
e faixa `0.23` preservados; tabela de complexidade vazia.

## Arquivos tocados

| Arquivo | Mudança | Evidência |
|---|---|---|
| `Cargo.lock` | `:2748` `rustls` `0.23.43` → `0.23.46` | entrada `name = "rustls"`, sem `0.23.43` residual |
| `supply-chain/config.toml` | `:1115-1117` `[[exemptions.rustls]]` `version = "0.23.46"` | `criteria = "safe-to-deploy"` preservado |
| `.specify/assessments/090-rustls-security-bump/decision.md` | novo — verdict **go**, 2026-11-20, 4 advisories, bump patch | baseline `824ba00e9` |
| `specs/090-rustls-security-bump/spec.md` | template preenchido (FR-001..FR-005, SC-001..SC-003) | antes template |
| `specs/090-rustls-security-bump/plan.md` | novo | 2026-11-20 |
| `specs/090-rustls-security-bump/tasks.md` | novo — T001..T006 `[X]` | evidência por tarefa |

**Fora de escopo**: `Cargo.toml`, `.github/workflows/`, `flora-tests/`,
`validate/`, `release/`, `.specify/memory/STATE.md`, ADRs, constituição, PRs e
worktrees alheios.

## Next Phase

Fechamento documental do ciclo 090. Nenhuma fase adicional; `[NV]` reconsulta
de CI do PR/merge/produção e aprovações de maintainer antes de qualquer merge
ou promoção.
