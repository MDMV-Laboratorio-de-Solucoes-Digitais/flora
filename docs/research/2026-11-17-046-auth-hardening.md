# 046 — Endurecimento de autenticação: logout real + superfície RBAC autenticada

**Data:** 2026-11-17 · **Metodologia:** hub `mdmv-dark-factory` aplicado ao Flora;
6 agentes de decisão (ranking, varredura BOLA, design logout, pesquisa OWASP/RFC, 404s frontend, stubs), auditoria author-blind.

## 1. Decisão 80/20

Com T6 fechado no 045, os 6 agentes ranquearam os gaps restantes. Escolha: **dois fixes de autenticação/autorização**, ambos confirmados em código, autônomos e verificáveis:

1. **R1 (varredura BOLA):** 4 handlers RBAC (`create/get/assign/revoke role`) eram **públicos** — sem `AuthExtractor`, sem org, sem DB; no-op que ecoavam o ID. Risco latente: qualquer implementação futura herdaria um endpoint não autenticado aceitando `role_id`/`user_id` arbitrários (A01:2021/BOLA).
2. **C2 (ranking):** `logout()` do frontend só limpava localStorage → JWT roubado válido até `exp`. Backend fazia `find_by_jti` descartado (no-op) e `revoke_all_for_user` (derrubava todas as sessões).

| Hipótese | Impacto | Confiança | Esforço | Veredito |
|---|---|---|---|---|
| R1 — autenticar handlers RBAC | Alto (superfície) | Alta | Baixo | **FEITO** |
| C2 — logout real (jti) | Alto (session hijack) | Alta | Baixo | **FEITO** |
| R2 — revalidar membership em 7 handlers | Médio | Média | Médio | Follow-up (sem exploit vivo) |
| R3 — validar `thread_id` | Baixo | Alta | Baixo | Follow-up |
| 404s frontend (invite/telemetry) | Médio | Alta | Baixo | Follow-up (features nunca implementadas) |
| Wrapper RLS / promoção db-tests | — | — | — | Não (autorização SE) |

## 2. Evidência primária (OWASP/RFC, EN/PT-BR/ES)

- **Session Management Cheat Sheet / ASVS V7**: logout deve invalidar a sessão **no servidor**; limpar storage mesmo se rede falhar (`try/catch/finally`); logout idempotente (204 com token ausente/inválido); POST não GET.
- **RFC 7009 §2.2**: revogação de token é idempotente; token inválido → 200/204.
- **Multi-device**: "sair deste dispositivo" (padrão) vs "sair de todos" (ação separada) — revogar tudo no logout é regressão de UX e não é o padrão.
- **BOLA/API1:2023**: todo endpoint que recebe um ID deve checar autorização por objeto, inclusive no payload.

## 3. Implementação

**R1** — `rbac.rs`: novo helper puro `can_manage_roles` (exige `OrgAdmin`, não `OrgRead`) + `require_org_admin()` (membership + role da sessão). Os 4 handlers placeholders agora exigem `AuthExtractor` e `require_org_admin()` antes de devolver o no-op. Teste unitário `can_manage_roles_requires_admin_not_just_read`.

**C2** — backend `auth.rs`: `logout` troca `find_by_jti` descartado + `revoke_all_for_user` por `state.session_service.revoke_session(&claims.jti)` (só a sessão atual; o jti é validado no DB a cada request, então o token vira 401). Frontend `AuthState.logout()`: captura o token, limpa o estado **sincronamente** (compatível com os 4 callers fire-and-forget) e dispara o POST best-effort com `try/catch` (async IIFE para satisfazer `unicorn/prefer-await`).

## 4. Verificação

- Contrato DB `db_logout_revokes_only_current_session`: token A → 401 após logout; token B (outra sessão do mesmo usuário) permanece válido. Suíte auth **13 passed** (era 12).
- 4 testes RBAC (incluindo o novo) + frontend 39/39 (check/lint/build).
- Gates: fmt, clippy `--all-targets -D warnings`, deny, `cargo test --workspace` (13 suítes), `check_gate_integrity.py` — todos verdes.
- **Não tocado**: required checks, deploy, RLS enforcement, ADRs, trabalho de outros agentes.
