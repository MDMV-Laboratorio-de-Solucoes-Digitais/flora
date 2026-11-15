# Plan: 007 — Org-Scoped Auth + Verifiable Core Slice

**Branch:** `[007-org-scoped-auth-e2e]` (a criar para PR)  
**Status:** executed (partial) — 2026-11-15

## Phase A — Synthesize & scope
- [x] Consolidar achados disponíveis (auditorias backend/frontend+contrato); incorporar recomendação de endpoint aditivo com rotação de sessão/JWT org-scoped.
- [~] Síntese completa incluindo workflow dark-factory e pesquisa multi-idioma — **NV** no momento da decisão (subagentes pendentes).

## Phase B — Implement
1. [x] Backend: `POST /api/v1/auth/select-organization` com validação de membership/sessão ativa, revogação da sessão atual e emissão de nova sessão+JWT org-scoped (`jti` rotacionado).
2. [x] Atualizar `test-flora.sh` para Bearer/JWT e fluxo de seleção de org + smoke tenant-scoped condicionado a token/ambiente.
3. [ ] Alinhamento mínimo frontend/contrato — **deferido**.
4. [~] Testes focados dedicados — **não adicionados** (fmt/check/suites existentes executados).

## Phase C — Validate & deliver
- [x] `cargo fmt --all -- --check`, `cargo check -p flora-api -p flora-core`, `cargo test -p flora-core --lib` (8), `cargo test -p flora-tests --lib` (54).
- [NV] Smoke integrado com OIDC/DB/seed e CI runtime remoto.
- [~] Artefatos 007 atualizados; preparar PR GitHub (CI verde), merge e sync mirror.

## Follow-ups
- Adicionar teste de contrato/integração para `select-organization` (sucesso, membership negada, sessão inválida, org nil).
- Viabilizar smoke automatizável em dev (seed/token bootstrap) e/ou ajustar frontend para fluxo completo de seleção de org.
- Revisar recomendações dos subagentes pendentes (workflow/pesquisa) e aplicar hardening incremental se necessário.
