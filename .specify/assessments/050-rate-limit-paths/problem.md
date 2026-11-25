# Problem Definition: Limites de taxa nos caminhos certos

- **Slug**: 050-rate-limit-paths
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md (equivalente local; `problem.md` ausente no baseline) | shape.md

## Problem Statement

O rate limiter compara paths sem o prefixo montado no router: `flora-api/src/middleware/rate_limit.rs:159` testa `/auth/login`, `:161` `/auth/refresh` e `:167` procura `/invites` ou `/invitations`, mas `routes/mod.rs:25-29` monta a API sob `/api/v1` com `/login` e `/refresh` em `routes/auth.rs:72,75`, e o frontend chama `POST /api/v1/users/invite` (`UserManager.svelte:18`) (intake.md:9-11; research.md:5-11). O teste vermelho executado mostrou `/api/v1/auth/login`, `/api/v1/auth/refresh` e `/api/v1/users/invite` classificados como `GENERAL` — BUG, enquanto `/health` está correto. Os limites específicos (5/60 login, 20/60 refresh, 10/3600 invite) nunca disparam e tudo cai em `gen:{ip}` (100/min). Os 2 testes existentes em `rate_limit.rs:193-207` cobrem apenas o contador, não o casamento de path (intake.md:14-21; research.md:13). Ranqueamento 80/20 = 5.0.

## Affected Users & Stakeholders

- **Users**: pessoas que autenticam ou refresham sessão e administradores que enviam convites — os limites específicos que deveriam proteger esses fluxos não são aplicados; o limite genérico continua valendo (intake.md:11-19).
- **Stakeholders**: mantenedores da API e responsáveis pela proteção contra força bruta e spam, que precisam de classificação testável sem alterar limites, janelas, estado ou respostas (`define.md:14-16`; `decision.md:9-13`).

## Goals

- Reconhecer `/api/v1/auth/login`, `/api/v1/auth/refresh`, `/api/v1/users/invite` e as formas legadas `/invites`/`/invitations` em uma regra pura e testável (`define.md:7-10`; `research.md:24-28`).
- Manter todos os demais paths no limite genérico (`define.md:11`).
- Cobrir os 4 casos com testes unitários que não dependam de servidor HTTP (`define.md:12`; `shape.md:9-11`).
- Preservar limites, janelas, estado do limitador e mensagens de resposta (`decision.md:9-11`).

## Non-Goals

- Não alterar limites, janelas, estado do limitador ou mensagens de resposta (`define.md:14`; `decision.md:12`).
- Não mexer em rotas, autenticação, CORS, constituição, ADRs ou workflows (`define.md:15`).
- Não casar por expressão regular nem adicionar dependência; não mover a verificação para dentro das rotas (`shape.md:12-13`).
- Não tratar C4/C5/C6 nem mudanças arquiteturais do middleware (`intake.md:31-33`; `shape.md:13`).

## Success Metrics
- Classificação correta dos 4 casos: login, refresh, invite (com variantes legadas) e general (baseline: 3 BUGs e `/health` correto — intake.md:14-19).
- Limites específicos aplicados de fato: 5/60, 20/60 e 10/3600 (baseline: nunca disparavam; `gen:{ip}` 100/min dominava — intake.md:21).
- `cargo test -p flora-api` verde com os novos testes de classificação (baseline: 2 testes cobriam só `check_rate_limit` — research.md:13).
- Nenhuma regressão nos gates Rust/frontend e `check_gate_integrity.py` (baseline: gates verdes — define.md:21-23).
- Comportamento do middleware em produção e estado distribuído do limitador: **[NV]** (research.md:31; decision.md:15).

## Cost of Inaction

Força bruta em login/refresh e spam de convite seguem sem os limites específicos pretendidos, com a falsa sensação de que o middleware já protege esses caminhos. Como os testes cobrem apenas o contador, uma futura mudança de rota pode reintroduzir o problema sem ninguém perceber (intake.md:19-21; shape.md:12).

## Open Questions

- [NEEDS CLARIFICATION: as formas legadas `/invites` e `/invitations` ainda são usadas por algum cliente?] (`research.md:27`; `shape.md:9`)
- [NEEDS CLARIFICATION: os sufixos escolhidos cobrem todas as variantes que precisam de limite específico?] (`research.md:28`)
- [NEEDS CLARIFICATION: o estado distribuído do limitador altera a eficácia dos limites em produção?] (`research.md:31`; **[NV]**)
- [NEEDS CLARIFICATION: runner/CI ao vivo será observado em qual PR?] (`decision.md:15`; **[NV]**)
