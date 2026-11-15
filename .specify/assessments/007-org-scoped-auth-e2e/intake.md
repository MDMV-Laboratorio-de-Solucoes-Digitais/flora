# Intake: Flora 80/20 Cycle 007 — Org-Scoped Auth + Verifiable Core Slice

**Date:** 2026-11-15  
**Status:** draft (in progress; research/audits running)

## Captured request
Executar o próximo ciclo 80/20 (após merge do #209/ciclo 006) para aproximar a Flora de um **Seed utilizável e verificável** como alternativa self-hosted ao Macro para PMEs em VPS barata (Rust + PostgreSQL, Svelte 5/Tauri depois), com decisões autônomas e evidência.

## Restated idea
Viabilitar o fluxo pós-login necessário às rotas tenant-scoped endurecidas no ciclo 006: **seleção de organização com emissão de sessão/JWT escopado à org (com verificação de membership)** e entregar um caminho mínimo reproduzível (smoke API e/ou alinhamento mínimo de contrato/frontend) para `org → workspace → task` (ou equivalente), atualizando `test-flora.sh`/contratos conforme o novo modelo de auth.

## Source and type
- **Source:** handoff de `specs/006-secure-vertical-slice/{plan,spec,tasks}.md`, `OBJETIVO.md`, `ROADMAP.md`; lacunas conhecidas (callback emite `organization_id=nil`, frontend/contrato desalinhado, `test-flora.sh` header-based).
- **Type:** ciclo técnico (auth/session scoping + fluxo núcleo verificável).

## Known context (evidence)
- Callback cria sessão/JWT com `org_id=nil` (`flora-api/src/routes/auth.rs`); `AuthExtractor::require_org()` rejeita nil; rotas críticas agora exigem JWT+org+membership.
- Session repo/service não possui update de `organization_id/jti`; refresh reemite token com mesmo `jti` e `session.organization_id`.
- `test-flora.sh` usa `X-User-ID`/`X-Organization-ID` (quebrado pelo modelo novo).

## Unknowns / pending inputs
- Resultados de subagentes em andamento: workflow mdmv-dark-factory (`ses_e1ea41e0fffeDgn1ojT3GM1BXp`), auditoria auth/session/org-scoping (`ses_e1ea41e0dffeWk8vTPsHFXBV5k`), pesquisa multi-idioma de padrões (`ses_e1ea41e08ffeHj1Q40A2fgILJO`), auditoria frontend/contrato (`ses_e1ea41e07ffebb36eOtH32TuIi`).
- Melhor desenho de rotação/reemissão (mesma sessão vs nova sessão/jti) e contrato exato do endpoint de seleção.

## Desired outcome
- Assessment 007 com decisão `go` e escopo acotado.
- Implementação backend de seleção de org + JWT/sessão org-scoped (segura, membership-validada).
- Smoke/caminho verificável documentado e executado quando viável (NV explícito se bloqueado por OIDC/DB real neste ambiente).
- Artefatos SpecKit atualizados e entrega via PR no GitHub (CI verde) com sync ao mirror.
