# Research: First Seed Journey local one-time bootstrap

**Feature**: 085-local-first-seed | **Date**: 2026-11-20T01:31:00Z

## Decision 1 — Guarda one-time atômica

**Decision**: adquirir `pg_advisory_xact_lock` com chave fixa dentro de uma transação, contar organizações não-sistema (excluindo a placeholder `system` criada pela migration) e, somente se o total for zero, inserir todos os objetos antes do commit. A segunda chamada recebe conflito (409) sem gravar.

**Rationale**: uma checagem fora da transação permite duas requisições concorrentes criarem duas organizações. A trava transacional usa o PostgreSQL que já existe, sem tabela de estado novo, e é liberada no commit/rollback.

**Alternatives considered**: (a) flag em arquivo/config — não é autoritativa em múltiplas instâncias; (b) tabela `bootstrap_state` — mais estado e migração para um evento único; (c) confiar apenas no unique slug — impede duplicata, mas não garante que a segunda chamada não crie usuário/workspace órfãos.

## Decision 2 — Sessão e resposta

**Decision**: o handler insere a linha `sessions` na mesma transação e devolve `{session_token, expires_in, user_id, organization_id, workspace_id}` com status 201, reutilizando `FloraClaims::new` + `encode_token`. O frontend chama `AuthState.login(token, user, organizationId)` e navega para `/tasks`.

**Rationale**: espelha `select_organization` (`flora-api/src/routes/auth.rs:359-419`), que já é o caminho de sessão escopada à organização; evita fluxo de "selecionar org" logo após o bootstrap e não inventa novo formato de token.

**Alternatives considered**: (a) criar dados e exigir login OIDC depois — não funciona sem IdP; (b) devolver token sem sessão no banco — quebra revogação e auditoria; (c) cookie próprio — nova superfície e divergência do contrato atual.

## Decision 3 — Papel e permissões

**Decision**: criar papel `Admin` com `permissions = ["*"]` e `is_builtin = true`, associado ao usuário administrador via `Membership`.

**Rationale**: `Role::has_permission` trata `*` como todas as permissões (`flora-core/src/models/role.rs:64-72`), garantindo que o administrador consiga usar RBAC sem uma lista longa e frágil de permissões.

**Alternatives considered**: (a) papel sem permissões — administrador não conseguiria operar; (b) lista explícita de todas as variantes — acoplamento a enum que evolui.

## Decision 4 — Validação e slug

**Decision**: validar `organization_name` (1–255 caracteres após trim) e `admin_email` (formato de e-mail, lowered); derivar slug de organização lowercase, trocar caracteres não alfanuméricos por `-`, colapsar hífens e remover bordas. E-mail duplicado ou slug conflitante resulta em 409 sem gravação parcial.

**Rationale**: usa as regras já presentes nos modelos (`CreateOrganizationInput`, `User::new`) e evita nomes de slug inválidos sem adicionar dependência.

**Alternatives considered**: aceitar nome arbitrário como slug — gera URLs inválidas; slug numérico aleatório — pior experiencia e não usa o nome informado.

## Decision 5 — Proteção da rota

**Decision**: `POST /api/v1/bootstrap` entra na classe de rate limit `Login` (5 requisições / 60 s por IP) e registra log de auditoria com `organization_id` e `user_id`, sem token.

**Rationale**: é uma rota não autenticada; o limite existente em `flora-api/src/middleware/rate_limit.rs:187-209` é a defesa de baixo custo contra força bruta e repetição.

**Alternatives considered**: deixar no bucket `General` — limite mais frouxo; CAPTCHA — dependência externa fora do perfil Seed.

## Source References

- `flora-api/src/routes/auth.rs:359-419` — padrão de sessão escopada
- `flora-core/src/utils/jwt.rs:82-89` — `encode_token`
- `flora-core/src/models/role.rs:35-72,117-160` — Role, permissões e wildcard
- `flora-core/src/models/user.rs:35-48` — normalização de e-mail
- `flora-api/src/middleware/rate_limit.rs:133-209` — classes e limites
- `.specify/assessments/085-local-first-seed/decision.md` — go e handoff
