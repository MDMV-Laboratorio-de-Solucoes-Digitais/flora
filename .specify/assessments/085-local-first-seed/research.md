# Idea Research: First Seed Journey local one-time bootstrap

- **Slug**: 085-local-first-seed
- **Created**: 2026-11-20T01:08:00Z
- **Evidence confidence (overall)**: medium — falha atual e capacidades do repo são de alta confiança; demanda é sinal único do SE

## Users & Demand

- Único sinal observado: o SE clicou em "Sprout Workspace" e reportou "nada acontece"; após correção de feedback, reportou o erro `redirect_uri is not allowed` e escolheu explicitamente a opção de bootstrap local — [source: conversa desta sessão + screenshot anexado] (confidence: high para o sinal, low para generalização).
- Não há dados de telemetria, tickets ou múltiplos usuários neste repo que quantifiquem a demanda — [source: ausência verificada em `.specify/bugs/` e `docs/`] (confidence: medium).

## Prior Art
- `dev-seed.sql` + `scripts/dev-smoke-session.sql` + `flora-mint-session` já permitem entrar na app sem IdP, mas são ferramentas de teste/E2E, não fluxo de produto — [source: `dev-seed.sql`, `scripts/dev-smoke-session.sql`, ciclos 027/029] (confidence: high).
- `docs/research/2026-11-16-030-frontend-e2e-selectors.md:86-88` classifica o First Seed Journey como **decorativo**: os campos são bound mas nunca usados, e ambos os botões apenas disparam o OIDC externo — [source: repo] (confidence: high).
- `flora-tests/src/integration/registration.rs:1-25` contém apenas placeholders de "registration flow with mock OIDC"; não existe mock funcional — [source: repo] (confidence: high).
- Repositórios e modelos para Organization, User, Membership, Role, Workspace e sessão JWT já existem e são usados por contratos — [source: `flora-core/src/models/organization.rs:28-46`, `flora-core/src/repositories/*`, `flora-tests/src/contract/auth.rs:704-748`] (confidence: high).

## Market & Context

- `.env.example:58-66` declara OIDC/Zitadel como **opcional para dev**; `OidcConfig::default()` aponta para `http://localhost:8080` com `allowed_redirect_uris: Vec::new()` — [source: repo] (confidence: high).
- Constituição (linha 138) define o milestone Seed como Postgres-first, com Zitadel/profile pesado fora doSeed mínimo; ADR-0001 reforça mecanismos substitutos para o Seed — [source: `.specify/memory/constitution.md`, `docs/adr/0001-seed-v01-mechanism-substitutions.md`] (confidence: high).
- Custo de não fazer: o botão continua sem utilidade real em instalação sem IdP, e o único caminho de entrada é injeção manual de sessão via ferramenta de dev — [source: observação desta sessão] (confidence: high).

## Data & Constraints

- API respondeu `503` com `OidcProvider("redirect_uri is not allowed")` para `redirect_uri=http://127.0.0.1:5173/callback`; cliques repetidos geraram `429` (limite 5/60 s por IP) — [source: logs `flora-demo-api`, 2026-11-20T00:46:20Z–00:46:33Z; `flora-api/src/middleware/rate_limit.rs:187-209`] (confidence: high).
- Nenhum servidor Zitadel está disponível localmente; `docker images` não mostra imagem Zitadel e o compose não define o serviço — [source: `docker-compose.yml`, `docker images`, `.env.example`] (confidence: high).
- `AuthState.login(token, user, organizationId)` já persiste `flora.auth.v1`; o guard redireciona para `/select-org` quando `organizationId` é nulo — [source: `apps/frontend/src/lib/state/AuthState.svelte.ts`, `apps/frontend/src/routes/(app)/+layout.svelte`] (confidence: high).

## Evidence Against the Idea

- Endpoint não autenticado de criação de organização/admin é superfície de ataque; sem guarda *one-time* atômica ele permitiria takeover — [source: raciocínio de segurança sobre `flora-api/src/routes/*`] (confidence: high, risco).
- A constituição exige identidade OIDC com ID token verificado no milestone Seed; um bootstrap local com e-mail digitado não prova posse do e-mail e pode divergir da promessa de identidade — [source: `.specify/memory/constitution.md:138`] (confidence: medium).
- Demanda observada é de um único usuário; pode ser um artefato apenas do ambiente de demo — [source: conversa desta sessão] (confidence: medium).
- Concorrência: duas requisições simultâneas podem passar por uma checagem "banco vazio" não atômica — [source: padrão de repositório sem transação de bootstrap] (confidence: medium).

## Gaps & Open Questions

- [NEEDS CLARIFICATION: critério exato de "banco vazio" e atomicidade da checagem]
- [NEEDS CLARIFICATION: o bootstrap deve criar token vs. exigir login OIDC depois?]
- [NEEDS CLARIFICATION: nome/papel padrão do admin e workspace inicial]
- [NEEDS CLARIFICATION: como desabilitar/retirar o endpoint após o primeiro uso sem breaking change]

## Sources

- `proc://flora-demo-api` (logs locais, 2026-11-20T00:36Z–00:47Z; policy: fonte local verificada)
- `flora-api/src/routes/auth.rs:84-160` (repo; policy: fonte primária local)
- `flora-core/src/config/oidc.rs:24-40` (repo; policy: fonte primária local)
- `docs/research/2026-11-16-030-frontend-e2e-selectors.md:76-90` (repo; policy: fonte primária local)
- `.env.example:58-66` (repo; policy: fonte primária local)
- `.specify/memory/constitution.md:138` (repo; policy: fonte normativa local)
- `flora-tests/src/integration/registration.rs:1-25` (repo; policy: fonte primária local)
