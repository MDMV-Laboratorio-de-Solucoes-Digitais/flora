# Problem: 012 — Session Authority Revalidation & Contract Confidence

1. **Session authority parcial:** `AuthExtractor` (e `UserContext`/`OrgContext`) valida apenas assinatura/expiração do JWT; não consulta a linha da sessão (`jti`) no PostgreSQL. Token revogado (logout, troca de organização, `revoke_all_for_user`) ou sessão expirada no servidor continua aceito até o `exp` do JWT. `refresh`/`select-organization` fazem checagens parciais, mas as rotas protegidas em geral não.
2. **Contrato de erro 401 incompleto no cliente:** `ApiClient` lança `ApiError` antes de ler o corpo no 401, descartando `error`/`message`/`field` estruturados; isso reduz a capacidade da UI de distinguir sessão expirada de credencial inválida e contradiz o objetivo do ciclo 011.
3. **Cobertura DB limitada:** os testes DB-backed existentes cobrem `select-organization`, mas não há casos determinísticos para revogação, expiração, sessão ausente ou mismatch de usuário/organização entre token e sessão.
4. **Risco operacional/performance não medido:** uma consulta indexada por request protegido é aceitável para PME/VPS, mas precisa de evidência e de um caminho claro de rollback; cache distribuído seria prematuro.
5. **Perfil Seed/VPS ainda difuso:** serviços opcionais (Valkey/Meilisearch/OIDC) e requisitos mínimos de memória/backup não estão consolidados, dificultando a promessa “qualquer VPS barata”.
