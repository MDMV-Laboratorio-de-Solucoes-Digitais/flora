# Concept: 011 (80/20)

## Abordagem proposta
- **Contrato/API (alto valor, baixo risco):** corrigir `POST /api/v1/tasks` para persistir `description` e `assignee_id` do request; manter semânticas existentes.
- **Frontend (alinhamento determinístico):** evoluir `ApiClient` para expor erro tipado com `status` e corpo (JSON/texto) preservando `lockSession()` em 401; ajustar `AuthState` (`setOrganization` limpa `isInvalidated`) e páginas `select-org`/`tasks` para usar contrato/erros de forma mais robusta (guard auth/org em tasks, parsing seguro); atualizar/adequar testes unitários frontend obsoletos (AuthState, interceptor) para refletir comportamento atual.
- **Ergonomia smoke/docs:** adicionar receita concreta de override de portas para `docker compose` em caso de conflito (exemplo isolado), mantendo notas NV/operacionais.
- **Hardening de sessão/tenancy e cobertura DB:** aplicar apenas incrementos mínimos com verificação clara (ex. ajustes pontuais de consistência já cobertos por testes existentes/novos determinísticos). Evitar revalidação DB global no extractor neste ciclo se risco/complexidade (sessão pré-seleção nil-org, impacto amplo) superar evidência de 80/20; registrar como follow-up fundamentado.
- **Verificação:** fmt/check/clippy (se viável)/tests Rust, frontend check/lint/test:unit(server)/build, e smoke compatível documentado quando ambiente permitir; DB-backed env-gated quando aplicável. Evidência em validação com NV explícito.

## Fora de escopo
- OIDC/JWKS e2e real, reescrita de auth/sessão, fanout realtime, upload binário/quotas, busca/notificações wiring, expansão de UI/produto, mudanças de migração aplicada.
