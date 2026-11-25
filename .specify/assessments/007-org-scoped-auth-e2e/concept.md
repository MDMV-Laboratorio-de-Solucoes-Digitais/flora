# Concept: Sessão com escopo de organização

- **Slug**: 007-org-scoped-auth-e2e
- **Created**: 2026-11-25
- **Recommended option**: Seleção de org com sessão escopada e smoke no modelo Bearer

## Options

### Option A — Seleção automática no login

- **Sketch**: O callback escolhe uma organização por conta própria e já emite uma sessão utilizável, sem pedir escolha à pessoa que entrou.
- **Appetite**: small
- **Trade-offs**: Menor coisa que poderia funcionar para destravar as rotas tenant-scoped imediatamente. Mas decide por conta própria em cenários com mais de uma organização e pode burlar a verificação de membership; a promoção de contexto fica sem uma fronteira explícita de autorização.
- **Rabbit holes**: Escolher "a primeira org" como regra escondida; pular checagem de membership por conveniência; descobrir depois que o token errado já foi emitido para sessões antigas.

### Option B — Seleção de org com sessão escopada e smoke no modelo Bearer

- **Sketch**: Depois de entrar, a pessoa passa por um ponto explícito de escolha de organização; só então sua sessão passa a valer dentro daquela organização e o fluxo `org → workspace → tarefa` pode ser percorrido. A verificação de membership acompanha a promoção de contexto, e o caminho de smoke acompanha o mesmo modelo de autenticação usado pelo runtime.
- **Appetite**: medium
- **Trade-offs**: Desbloqueia as rotas endurecidas no ciclo anterior sem abrir brecha de acesso a organizações alheias, e dá um caminho reproduzível para provar o fluxo. Custo médio: mexe no ponto mais sensível da sessão, com rotação/revogação e risco para tokens existentes; execução integrada com OIDC real, DB e seed permanece NV neste ambiente.
- **Rabbit holes**: Redesenhar todo o serviço de sessão; tornar a rotação uma migração ampla de refresh; perseguir UX de seleção e alinhamento de contrato completo neste recorte; prometer smoke e2e sem ambiente integrado.

### Option C — Plataforma de sessão e interface completa (não fazer)

- **Sketch**: Resolver de uma vez seleção de org, persistência de contexto no frontend, websocket/sync e políticas avançadas de auditoria e refresh.
- **Appetite**: large
- **Trade-offs**: Coerência ampla futura, mas depende de backend estável e extrapola o que pode ser verificado agora; risco alto de retrabalho.
- **Rabbit holes**: Acoplar frontend e realtime antes de a sessão org-scoped existir; adicionar auditoria/rate limit como pré-requisito do desbloqueio.

## Recommendation

Option B. Os goals do `problem.md` são claros: caracterizar o token com organização nula como bloqueio funcional das rotas tenant-scoped, explicar a ausência de atualização/rotação de sessão como limitação estrutural, exigir verificação de membership em qualquer promoção de contexto e reconhecer a quebra do smoke header-based como parte do mesmo problema de verificabilidade. B atende a todos e preserva a distinção entre o que é provável em nível de API e o que depende de alinhamento de UI/WS (deferido/NV), conforme a decisão 007.

## Out of Scope (for the recommended option)

- Desenho do endpoint de seleção, formato do contrato e política exata de rotação/refresh em nível de especificação.
- UX de seleção de organização, persistência de contexto no frontend e wiring de websocket/sync.
- Declarar smoke integrado com OIDC real, Postgres e seed como executado sem evidência.
- Auditoria ponta a ponta de OIDC/JWKS e expansão de RBAC fino.

## Assumptions to Validate

- A política de rotação é aceitável: revogar a sessão atual e criar nova com `jti` diferente, em vez de reutilizar a mesma sessão alterada.
- Existe contrato esperado para o ponto de seleção de organização e alguém o consome; payload/códigos de erro ainda não foram fixados nos artefatos.
- Há ambiente com OIDC/DB/seed disponível para validar o fluxo; caso contrário, o smoke fica documentado como NV.
- A checagem de membership no momento da seleção é suficiente como fronteira de autorização para o recorte atual.
