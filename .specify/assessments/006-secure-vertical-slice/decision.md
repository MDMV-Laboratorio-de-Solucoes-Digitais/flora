# Decision: 006 — Secure Vertical Slice (Auth/Tenancy + Core Flow)

**Date:** 2026-11-15  
**Verdict:** **go (executed, partial slice)**

## Rationale
- **Problem validity:** alta — confiança em headers crus e contexto inconsistente eram P0 de segurança/tenancy.
- **Value vs effort:** alta — unificar extração JWT + membership/tenant checks nas rotas críticas reduz risco imediato e cria base para fatia e2e.
- **Risk:** médio (quebra de clientes header-based, ex. `test-flora.sh`/frontend atual) — aceitável e explícito; compatibilidade de migração completa fica para follow-up.
- **Strategic fit:** alinhado ao norte Macro-alternative seguro em VPS barata.

## Scope executed
- Refactor backend para JWT-derived identity/tenant nas rotas: `workspace`, `tasks`, `channels`/messages, `org`, `files`, `search`, `notifications`, e WS (Bearer-only).
- Endurecimento: `require_org()` para rotas tenant-scoped, membership checks, validação de tenancy de recursos (`workspace_id ∈ org` em criação de canal, checks em update/delete/get onde aplicável).

## Out of scope / deferred
- Mecanismo de token/sessão escopado após seleção de organização (callback ainda emite org `nil`).
- Alinhamento de contrato/frontend e E2E UI (NV).
- Fanout/realtime completo, RBAC fino além do slice, refactors amplos.

## Evidence (local, 2026-11-15)
- `cargo fmt --all -- --check` ✓
- `cargo check -p flora-api -p flora-core` ✓
- `cargo test -p flora-core --lib` ✓ (8 passed)
- `cargo test -p flora-tests --lib` ✓ (54 passed)
- Runtime integrado (DB/services/OIDC) e CI remota: **NV** neste ambiente.
