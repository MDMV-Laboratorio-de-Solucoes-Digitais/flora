# Decision: 007 — Org-Scoped Auth + Verifiable Core Slice

**Date:** 2026-11-15  
**Verdict:** **go (executed, partial slice)**

## Rationale
- **Problem validity:** alta — sem seleção/escopo de org, rotas tenant-scoped endurecidas (ciclo 006) permaneciam inutilizáveis pós-login (`org_id=nil`).
- **Value vs effort:** alta — endpoint aditivo de seleção com membership check e rotação de sessão/JWT desbloqueia o caminho de API e reduz risco operacional.
- **Risk:** médio — rotação/revogação afeta tokens/sessões existentes; mitigado por validação de sessão ativa/membership, novo `jti` e escopo mínimo.
- **Strategic fit:** P0 para Seed seguro/verificável em VPS barata.

## Scope executed
- Backend: `POST /api/v1/auth/select-organization` em `flora-api/src/routes/auth.rs` (valida org/membership/sessão, revoga sessão atual, emite nova sessão+JWT org-scoped com `jti` rotacionado).
- Verificação/contrato: `test-flora.sh` reescrito para Bearer/JWT e fluxo de seleção de org + chamadas tenant-scoped (smoke condicionado a token/ambiente).

## Deferred / NV
- Smoke e2e integrado com OIDC/DB/seed e CI runtime remoto: **NV** neste ambiente.
- Alinhamento frontend/UI e contrato completo (org selection UX, persistência de contexto, WS/sync, ajustes de endpoints/shapes): **deferido**.
- Síntese final de subagentes pendentes no momento da decisão (`ses_e1ea41e0fffeDgn1ojT3GM1BXp`, `ses_e1ea41e08ffeHj1Q40A2fgILJO`): recomendações adicionais podem gerar follow-ups.

## Evidence (local, 2026-11-15)
- `cargo fmt --all -- --check` ✓
- `cargo check -p flora-api -p flora-core` ✓
- `cargo test -p flora-core --lib` ✓ (8 passed)
- `cargo test -p flora-tests --lib` ✓ (54 passed)
- Integração runtime/OIDC/DB: **NV**
