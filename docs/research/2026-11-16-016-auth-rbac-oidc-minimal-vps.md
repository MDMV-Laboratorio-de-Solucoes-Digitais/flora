# 016 — Auth, RBAC e OIDC mínimos e seguros para VPS (Flora, 2026-11-16)

**Escopo:** pesquisa técnica multilíngue (EN, pt-BR, es, de, fr, zh-CN, ja) sobre autenticação/autorização mínima e segura para app colaborativo SMB self-hosted em VPS barata (Rust + Postgres + Svelte), com foco em: sessões/JWT, OIDC opcional vs auth local, RBAC/isolation multi-tenant e hardening 80/20 com debloat de IdP complexo.  
**Cross-check (código/docs):** `flora-api/src/routes/auth.rs`, `flora-api/src/extractors/auth.rs`, `flora-api/src/middleware/{cors.rs,rate_limit.rs,mod.rs}`, `flora-core/src/config/{app.rs,oidc.rs}`, `flora-core/src/services/{auth_service.rs,rbac_service.rs}`, `.env.example`, `OBJETIVO.md`, `ROADMAP.md`.  
**Restrições:** sem alterações em código-fonte da aplicação; recomendações orientadas a evidência pública + estado observado.

## 1) Estado observado (cross-check)

- **Modelo atual (híbrido stateful):** login OIDC com PKCE/state server-side (`/auth/login`, `/auth/callback`) gera sessão em DB (`jti`, `user_id`, `organization_id`) e emite JWT HS256 (`flora-api/src/routes/auth.rs`, `flora-core/src/config/app.rs`).
- **Autoridade de sessão:** extractors validam assinatura/claims e revalidam `jti`/estado/expiração/consistência `user_id`+`organization_id` via `session_service` (server-side authority, boa direção para revogação imediata em VPS pequena).
- **Tenancy:** sessão carrega `organization_id` (inclui placeholder `nil` pré-seleção) e `select-organization` rotaciona `jti`; `AuthExtractor::require_org()` protege rotas que exigem contexto concreto.
- **Gaps relevantes:** callback decodifica `id_token` sem verificação JWKS/assinatura/`iss`/`aud`/`nonce` completa (comentário de produção no código); refresh reutiliza `jti`; logout revoga todas as sessões do usuário; CORS permissivo (`Any`); `AuthService` stub; OIDC local fallback configurável mas não claramente integrado no fluxo.
- **RBAC:** serviço com roles/permissões e cache in-memory; `has_permission_in_workspace` delega a permissão no escopo da organização (workspace-level enforcement fino ainda limitado). Rate limit existe (Valkey + fallback in-memory) com limites por endpoint.
- **Alinhamento estratégico:** `OBJETIVO.md`/`ROADMAP.md` priorizam P0 auth/tenancy consistente, contratos e2e verificáveis, Postgres-first/debloat e perfil Seed operável em VPS modesta.

## 2) Síntese da pesquisa multilíngue (fontes primárias)

| Eixo | Consenso prático (EN/pt-BR/es/de/fr/zh-CN/ja) | Implicação Seed/VPS |
|---|---|---|
| Sessão vs JWT | **Sessões server-side (cookie HttpOnly/Secure/SameSite) são o default mais simples e revogáveis** para monólito/web app; JWT stateless puro complica revogação/logout/comprometimento. Híbrido comum: access curto + refresh rotativo/DB ou sessão opaca + verificação server-side. (OWASP Session/JWT; OpenReplay/Zyvop/Stackblip/LogRocket/Archidepth/JP Blogs; guias pt-BR/es/de/fr; padrões descritos em zh-CN/ja sobre OIDC+JWT+RBAC) | Manter **autoridade server-side por `jti`/sessão em Postgres** e tokens curtos; evitar “JWT 100% stateless” como meta. |
| Cookies/transporte browser | Cookies de sessão: `HttpOnly`, `Secure`, `SameSite=Lax/Strict`, expiração/timeout, rotação de ID em login/elevação, proteção CSRF quando cookie-auth; prefixos `__Host-` recomendados. Bearer em header útil para API/SPA, mas exige cuidado com armazenamento XSS. (OWASP, MDN/RFC refs via cheatsheets) | Para Svelte web, preferir **cookie de sessão** no BFF/same-origin; se manter Bearer JWT, manter curta duração + refresh seguro + rotação e mitigação XSS/CSRF. |
| JWT seguro | Validar `alg` (allowlist), `iss`/`aud`/`exp`/`nbf`/assinatura/JWKS; não confiar em `alg:none`; não colocar segredos no payload; preferir access curto; revogação via denylist/rotação/store. (OWASP JWT, RFC 8725/7519 práticas citadas) | **Obrigatório:** verificação JWKS completa do `id_token` no callback e claims OIDC; fixar algoritmo HS256/RS256 conforme uso. |
| OIDC opcional vs local/IdP | OIDC dá SSO/federação e evita reimplementar login social/enterprise; IdPs completos (Keycloak/Zitadel/Authentik) trazem custo operacional/RAM em VPS pequena. Authelia/forward-auth atende proxy/gateway simples mas não substitui IdP completo multi-app; auth local embutida + OIDC opcional por perfil é viável se escopo for SMB pequeno. (comparativos 2025-2026: SSD Nodes, Keycloakops, OpenSourceLab, Authgear, selfhostable, Casdoor/Compile7) | **Seed 80/20:** auth local simples (ou sessão própria) + **OIDC opcional** (um provider) atrás de feature/profile; evitar Keycloak/Zitadel como dependência obrigatória no Seed. |
| RBAC/multi-tenant | Escopo de tenant deve vir de identidade verificada/sessão, nunca de header/body confiável; checar tenant em toda query/camada; RBAC início simples (roles→perms) + enforce server-side; RLS Postgres útil em modelo shared-table (defense-in-depth) com role/request context testados. (OWASP Multi-Tenant, AWS SaaS/Postgres RLS, WorkOS/BrotCode/SuperTokens patterns) | **P0:** eliminar confiança em `X-User-ID/X-Organization-ID` para authz; centralizar tenant context na sessão verificada; considerar RLS progressiva em tabelas sensíveis. |
| Hardening 80/20 VPS | HTTPS/TLS, segredos fortes/rotacionáveis, rate-limit login/refresh, logs sem segredos, cabeçalhos seguros, backups/restore, least privilege DB, validação de entrada, monitoramento mínimo. (OWASP, guias EN/pt/es/de/fr/zh/ja) | Priorizar controles baratos de alto impacto antes de ampliar features/infra. |

## 3) Recomendações 80/20 (priorizadas)

### P0 — Segurança e correção de tenancy (imediato)
1. **Manter sessão stateful em Postgres como fonte de verdade** (jti ativo, expiração, rotação em `select-organization`/eventos sensíveis); continuar rejeitando token só-assinado sem sessão válida.
2. **Endurecer callback OIDC:** validar assinatura `id_token` via JWKS, `iss/aud/exp/nonce` e binding `state`; restringir `redirect_uri` a allowlist exata; tratar falhas como `401/400` seguros.
3. **Unificar contexto de tenant:** derivar `user_id`/`organization_id` apenas de sessão verificada; remover/deprecar confiança em headers de identidade para rotas críticas e auditar queries por escopo tenant.
4. **Refresh/logout seguros:** adotar rotação de refresh/session com detecção de reuso quando aplicável; revisar política de `revoke_all_for_user` vs logout de dispositivo (documentar trade-off).
5. **Cookies/browser hardening (quando aplicável):** `Secure; HttpOnly; SameSite=Lax|Strict`, TTLs curtos, rotação no login, CSRF token/Origin checks em fluxos cookie-auth; CORS restrito a origens conhecidas em produção.

### P1 — OIDC opcional e debloat operacional
6. **Perfil Seed “auth enxuta”:** local-first/sessão própria com bootstrap admin e recuperação controlada; **OIDC como plugin opcional** (1 provider) ativado por config/perfil, sem exigir Zitadel/Keycloak/AuthentiK no caminho mínimo.
7. **Escolha de IdP (se/quando necessário):** para poucos apps em VPS pequena, preferir integração OIDC direta ou proxy simples; evitar Keycloak (JVM/RAM/operação) no Seed; Authentik/Zitadel apenas se requisitos (SAML/LDAP/flows/multi-tenant SaaS avançado) justificarem custo.
8. **Reduzir superfície:** Postgres como store de sessão/rate-limit fallback aceitável; Valkey/Meilisearch/OIDC externos opcionais por perfil e com degradação documentada.

### P1 — RBAC e isolamento progressivo
9. **RBAC mínimo explícito:** papéis Owner/Admin/Member com permissões por recurso (org/workspace/canal/mensagem/tarefa/arquivo); enforcement centralizado no backend (middleware/policy helper) + testes negativos cross-tenant/cross-role.
10. **Isolamento em profundidade:** padronizar `WHERE organization_id = $tenant` em repositórios críticos; avaliar RLS em tabelas core (mensagens/tarefas/arquivos/memberships) com `current_setting` por transação e testes no mesmo role/path de runtime.

### P2 — Evolução
11. Passkeys/MFA, device/session management UI, SCIM/SSO enterprise, ABAC/ReBAC fino, chaves assimétricas/JWKS próprio para tokens internos — apenas após Seed P0 estável e demanda comprovada.

## 4) NVs (não verificados / validações necessárias)

- **NV-1:** claims/assinatura JWKS do `id_token` e fluxos de erro reais contra provider OIDC em ambiente integrado (código atual indica decode sem verificação completa).
- **NV-2:** comportamento e2e browser/Svelte (armazenamento de token/cookie, CSRF, CORS em origem real, refresh/expiração/revogação multi-aba).
- **NV-3:** isolamento tenant em **todas** as rotas/repositórios críticos sob requests autenticados (auditoria sistemática de queries e testes negativos automatizados).
- **NV-4:** efetividade de RBAC em escopo workspace/canal/arquivo (workspace permission hoje delega a org-level no serviço).
- **NV-5:** semântica operacional de logout/refresh/rotação (`revoke_all_for_user`, reuse detection, device sessions) em produção-like.
- **NV-6:** perfil Seed em VPS real (~2 vCPU/4 GB): footprint com/sem Valkey/Meilisearch/OIDC, limites de pool, latência de lookup de sessão em Postgres.
- **NV-7:** validação de que `local_fallback_enabled` e `AuthService` stub atendem (ou não) a um caminho local seguro; risco de configuração default (`jwt_secret` dev, CORS `Any`).

## 5) Fontes-chave (consultadas)

- OWASP: [Session Management](https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html), [JWT](https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_Cheat_Sheet.html), [Multi-Tenant Security](https://cheatsheetseries.owasp.org/cheatsheets/Multi_Tenant_Security_Cheat_Sheet.html), [ASVS/CSRF refs citadas].
- Sessão vs JWT / arquitetura: [OpenReplay (EN)](https://blog.openreplay.com/authentication-strategy-web-app), [OpenReplay (es)](https://blog.openreplay.com/es/estrategia-authentication-web-app), [OpenReplay (de)](https://blog.openreplay.com/de/authentication-strategie-web-app), [OpenReplay (fr)](https://blog.openreplay.com/fr/strategie-authentication-web-app), [StackBlip](https://stackblip.com/blog/jwt-vs-session-cookies.html), [Zyvop](https://zyvop.com/sessions-vs-jwts-vs-passkeys-how-to-choose-an-authentication-approach-r820r), [Archidepth](https://archidepth.io/learn/reliability-security/authentication), [LogRocket JWT best practices](https://blog.logrocket.com/jwt-authentication-best-practices/).
- Práticas e implementação (pt-BR/es): [Rust + JWT Axum (pt-BR)](https://rustlang.com.br/blog/autenticacao-jwt-rust-axum-2026), [StackPractices (es)](https://stackpractices.com/es/guides/complete-guide-authentication-patterns), [Aadhvika SaaS auth (EN)](https://aadhvika.io/blog/authentication-authorization-saas-jwt-oauth-rbac), [BrotCode SaaS auth](https://brotcode.com/blog/engineering/saas-authentication-sso-oauth-rbac).
- Multi-tenant/RLS: [AWS RLS guidance](https://docs.aws.amazon.com/prescriptive-guidance/latest/saas-multitenant-managed-postgresql/rls.html), [AWS best practices](https://docs.aws.amazon.com/prescriptive-guidance/latest/saas-multitenant-managed-postgresql/best-practices.html), [WorkOS multi-tenant sessions](https://workos.com/blog/multi-tenant-session-management).
- IdP/self-host trade-offs (VPS): [SSD Nodes Keycloak vs Authentik vs Zitadel](https://www.ssdnodes.com/learn/keycloak-vs-authentik-vs-zitadel), [Keycloakops comparison](https://keycloakops.com/posts/keycloak-vs-authentik-vs-authelia/), [SSO OSS比較 (ja)](https://opensourcelab.jp/articles/open-source-sso), [Open Source Lab / self-host SSO guides](https://selfhostable.dev/blog/authentik-vs-authelia-vs-keycloak-self-hosted-sso), [Authgear/Compile7 comparisons](https://www.authgear.com/post/best-open-source-identity-providers), [Casdoor IdP guide](https://www.casdoor.com/blog/self-hosted-identity-provider).
- Contexto multilíngue adicional: [OAuth2/OIDC/JWT/RBAC (zh-CN, Aliyun)](https://developer.aliyun.com/article/1759568), [IAM全景 (zh-CN)](https://quant67.com/post/iam/01-iam-landscape/iam-landscape.html), [認証基盤選定 2026 (ja)](https://tomodahinata.com/blog/auth-platform-selection-2026-cognito-auth0-clerk-supabase), [認証と認可基礎 (ja)](https://lab.main.jp/ai/posts/authentication-and-authorization-fundamentals), guias fr/de sobre sessões/OIDC citados acima.

> Nota: recomendações priorizam o perfil **Seed/VPS barata** (Postgres-first, baixo overhead operacional, revogação confiável) e devem ser validadas por testes/execução (ver NVs) antes de qualquer declaração de conformidade.