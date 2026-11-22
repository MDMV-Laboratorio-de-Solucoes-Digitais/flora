# Release: 110 — piloto de mutation testing

**Ciclo**: 110 · **Data**: 2026-11-22 · **Baseline**: `f12571cf4`

## Entregue

Piloto informativo com `cargo-mutants` em `flora-core/src/models/role.rs` (42 mutantes), correção do doc comment `#!`→`//!` em `workspace_service.rs`, do bug real de dedup em `Role::add_permission` e do teste `add_permission_is_idempotent`. Nenhum required check, CI job, migration ou dependência nova.

## SemVer

PATCH `0.7.11` → `0.7.12`: correção de lógica interna de domínio sem mudança de API/contrato/migration.

## Changelog

FR-001 piloto 42 mutantes · FR-002 correção de `#!` e dedup · FR-003 gate inalterado. Decisão F6; sem bug report em `.specify/bugs/` (bug descoberto pelo piloto, registrado no analyze).

## Evidência

R4 APROVADO · 56 testes · mutante focalizado 1/1 caught · fmt/clippy/gate exit 0.

## Rollback

Novo commit de reversão dos três arquivos, sem force-push.

## [NV]

Workspace-wide e CI não autorizados.
