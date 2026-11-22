# Feature Specification: piloto de mutation testing

**Branch**: `feature/110-mutation-pilot` · **Created**: 2026-11-22 · **Status**: Specified — go

## User Scenarios

### US1 — Medir sensibilidade (P1)

Executar `cargo mutants` em `flora-core/src/models/role.rs` e registrar mutantes mortos/sobreviventes, sem alterar required checks.

### US2 — Corrigir obstáculo determinístico (P1)

O parser da ferramenta não pode encontrar `#!` no início de arquivos Rust; corrigir para `//!` e confirmar que nenhum outro arquivo tem o mesmo defeito.

## Requirements

- **FR-001** piloto informativo com 42 mutantes em `role.rs`.
- **FR-002** correção do doc comment em `workspace_service.rs`.
- **FR-003** gate do alvo inalterado (10 legs / 77 pisos).

## Success Criteria

- **SC-001** comando executa e produz relatório; **SC-002** `cargo fmt/check/clippy/test` exit 0; **SC-003** gate PASS.

## [NV]

CI job e aplicação em todo o workspace.
